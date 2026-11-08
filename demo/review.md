# Review — FR-1 (Markdown → HTML)

**Scope:** `demo/demo-spec.md` §5, FR-1. Other requirements are cited only where they constrain FR-1.

**Reference implementation:** `pulldown-cmark` 0.13.4, the version already used by the
library's tests (`Cargo.toml`, `[dev-dependencies]`). The FR-1 snippet is API-correct
for that version: `Parser::new_ext(&str, Options)` and
`html::push_html(&mut String, impl Iterator<Item = Event>)` are the right calls.

**Status:** findings only; this review does not change the specification. Every
finding is open pending a decision.

## Summary

| ID | Severity | Finding | Status |
| --- | --- | --- | --- |
| FR1-01 | Blocker | The rendered preview, as specified, is an XSS and network-request sink; `push_html` is not a sanitizer | Open |
| FR1-02 | High | `options` has no defined type, default set, or mapping from the FR-5 UI | Open |
| FR1-03 | High | “Rendered” vs. “source” is underspecified; the fragment has no document context or isolation | Open |
| FR1-04 | Medium | The dependency/feature declaration is missing; default features pull the CLI-only `getopts` into WASM | Open |
| FR1-05 | Medium | No error contract: Markdown → HTML is infallible, but FR-8 and the acceptance criteria imply otherwise | Open |
| FR1-06 | Medium | Math is emitted as raw TeX spans; FR-7 promises a math sample but there is no typesetting decision | Open |
| FR1-07 | Medium | The round-trip leg must reuse FR-1's converter and options, or the diff measures options rather than fidelity | Open |
| FR1-08 | Medium | The performance target conflicts with the debounce and has no measurement method | Open |
| FR1-09 | Medium | The demo has no conversion tests or lint gate in the acceptance criteria | Open |
| FR1-10 | Low | The 100 KB threshold does not define its unit (UTF-16 code units vs. UTF-8 bytes) | Open |
| FR1-11 | Low | The accessibility and focus behavior of the preview are unspecified | Open |
| FR1-12 | Low | Broken reference links silently become literal text | Open |

---

## Findings

### FR1-01 — The rendered preview, as specified, is an XSS and network-request sink

**Severity:** Blocker

**Where:** FR-1 (“shown … as rendered HTML (in a preview pane)”), FR-5
(`heading_attributes`), and §8 (“No telemetry, no external network requests after
load”).

**Observation:** `pulldown_cmark::html::push_html` is a renderer, not a sanitizer.

- Raw HTML is passed through verbatim. In `pulldown-cmark` 0.13.4,
  `Html(html) | InlineHtml(html) => self.write(&html)` writes the author's markup
  unchanged.
- URLs are not scheme-checked. `escape_href` only percent-encodes unsafe bytes;
  `:` is on its safe list, so `[x](javascript:alert(1))` becomes
  `<a href="javascript:alert(1)">`.
- `ENABLE_HEADING_ATTRIBUTES` accepts arbitrary attribute names
  (`parse_inside_attribute_block` pushes any `key=value`). A heading such as
  `# x { onclick=alert(1) }` therefore becomes
  `<h1 onclick="alert(1)">x</h1>` even when the Markdown contains no raw HTML.
  The `style` attribute is accepted the same way.

Assigning that fragment to `innerHTML` (or Leptos' `inner_html`) executes event
handlers such as `<img src=x onerror=…>` and runs `javascript:` URLs on click.
`<script>` does not execute when inserted with `innerHTML`, but that is not a
mitigation. A malicious document can read the textarea and send its contents to
a third party, and remote `img`/`video`/CSS `url()` sources make requests that
violate the no-external-requests requirement. The primary user story — pasting
representative HTML or Markdown — makes untrusted input the normal case.

**Proposed resolution:**

1. Do not insert converted HTML into the app document with `innerHTML`. Render it
   in a sandboxed iframe: `sandbox=""` (no `allow-scripts`, `allow-forms`,
   `allow-same-origin`, or `allow-top-navigation`), with `srcdoc` set through the
   DOM property rather than an escaped attribute.
2. Add a CSP to the preview document, for example
   `default-src 'none'; img-src data: blob:; style-src 'unsafe-inline'; font-src data:`.
   This blocks remote image and font requests. If remote images are wanted, make
   that an explicit, documented opt-in.
3. If an iframe is rejected, sanitize with a vetted sanitizer (for example
   `ammonia`) and remove `on*` attributes, `javascript:` URLs, `<script>`,
   `<style>`, `<iframe>`, `<object>`, `<embed>`, `<form>`, and
   `<meta http-equiv="refresh">`. Keep the source view unsanitized.
4. Render the source view with `textContent`, never `innerHTML`.
5. Add regression tests for `javascript:` links, raw HTML event handlers, a
   heading `onclick`, and remote image URLs.
6. A sandbox does not stop the iframe from navigating itself: raw
   `<meta http-equiv="refresh">` or a user click can still navigate to an external
   URL. Strip `meta refresh` and intercept link clicks, or document that such
   navigation is user-initiated.

**Status:** Open — needs a decision (iframe vs. sanitizer).

---

### FR1-02 — `options` has no defined type, default set, or mapping

**Severity:** High

**Where:** FR-1 snippet; FR-5 (“sensible defaults”, option list); FR-7 (samples
include tables and math).

**Observation:** The snippet passes an `options` value that the specification
never defines.

- The type is `pulldown_cmark::Options`, a `bitflags` value. It has no `Default`
  implementation in 0.13.4, so “reset to defaults” cannot be implemented with
  `Options::default()`. Use `Options::empty()` as the base and `|`/`insert` the
  flags.
- “Sensible defaults” is not actionable. FR-7's samples include tables and math;
  with `Options::empty()` the table sample renders as literal pipe characters and
  the math sample renders as raw TeX. Either the default set enables the
  extensions the samples use, or the sample loader must set the relevant options.
- Several flags interact, so independent checkboxes are not enough on their own:
  - `ENABLE_OLD_FOOTNOTES` includes `ENABLE_FOOTNOTES` (`(1 << 9) | (1 << 2)`).
  - `ENABLE_SUBSCRIPT` changes how single-tilde text is parsed: with it enabled,
    `~x~` is subscript; with it disabled, `~x~` can become strikethrough.
  - `ENABLE_GFM` only adds blockquote alerts in 0.13.4; tables, strikethrough,
    and task lists remain separate flags.
  - `ENABLE_SMART_PUNCTUATION` rewrites text at parse time, so it changes both
    the source and the rendered output.
- Both crates expose an `Options` type (`htmd::options::Options` and
  `pulldown_cmark::Options`). The demo's `convert.rs`/`options.rs` should alias
  them (for example `PulldownOptions` and `HtmdOptions`) to avoid ambiguity.

**Proposed resolution:**

- Define an explicit `default_markdown_options() -> Options` in the spec and in
  code. For example, either a pure baseline (`Options::empty()`, with samples
  opting in) or a demo default such as
  `ENABLE_TABLES | ENABLE_FOOTNOTES | ENABLE_STRIKETHROUGH | ENABLE_TASKLISTS | ENABLE_MATH | ENABLE_HEADING_ATTRIBUTES`.
- List every flag exposed in v1. The FR-5 list omits the 0.13 additions
  (`ENABLE_GFM`, `ENABLE_DEFINITION_LIST`, `ENABLE_SUPERSCRIPT`,
  `ENABLE_SUBSCRIPT`, `ENABLE_WIKILINKS`, and the two metadata-block flags);
  either include them or state that they are out of scope.
- Define UI rules for implied flags: turning footnotes off must clear both the
  normal and old-footnote bits; document the subscript/strikethrough precedence.
- Document that options are parse-time: changing one re-parses the input and
  changes both views.

**Status:** Open — needs a default-set decision.

---

### FR1-03 — “Rendered” vs. “source” and document context are underspecified

**Severity:** High

**Where:** FR-1 final sentence; FR-4 (“Rendered” and “Source” views).

**Observation:** `push_html` emits an HTML fragment, not a document. It has no
`<!doctype>`, `<html>`, `<head>`, `<body>`, `<base>`, or CSS. “Rendered HTML in a
preview pane” can mean injecting the fragment into the app DOM or into a separate
document, and the two have very different consequences.

- Injecting into the app DOM leaks `<style>`, `id` values, classes, and
  `position: fixed` overlays into the app. It also makes FR1-01 worse.
- Relative URLs resolve against the app URL. Clicking a link navigates away from
  the demo and loses its state, since there is no persistence.
- Code blocks, tables, task-list checkboxes, footnote markers, and math need
  preview CSS. Without it the “rendered” view looks broken.
- The source view must show the exact `String` returned by `push_html`. A
  pretty-printer or re-serializer would make the source/rendered correspondence
  and the FR-6 diff misleading.

**Proposed resolution:**

- Define the preview as a sandboxed iframe containing a minimal document:
  `<!doctype html><html><head><meta charset="utf-8"><style>…preview CSS…</style></head><body>…fragment…</body></html>`.
  Insert the fragment as body content.
- Specify link behavior: intercept clicks and open external links outside the
  demo, or use `<base target="_blank">` with `rel="noopener noreferrer"`. Keep
  `#` links inside the preview.
- Add a preview stylesheet for headings, lists, tables, code blocks, blockquotes,
  task lists, and math. Keep it separate from app CSS.
- State that the source view is the unmodified `push_html` output in a `<pre>`
  and that no pretty-printer is used.
- State that active content (scripts, event handlers) is intentionally disabled
  in the preview; the source view is the fidelity reference for such content.

**Status:** Open — iframe recommended.

---

### FR1-04 — The dependency/feature declaration is missing

**Severity:** Medium

**Observation:** The API call is correct, but the spec does not say how
`pulldown-cmark` is declared.

- `html::push_html` is behind the `html` feature. The crate's default features are
  `getopts` and `html`. `getopts` exists only for the crate's CLI binary and is
  dead weight in a WASM bundle.
- The demo does not pin a version. The library's dev-dependency is
  `pulldown-cmark = "0.13.4"`. A different demo version can change event and HTML
  output and make the round-trip differ from `htmd`'s own tests.
- The `simd` feature is only active on x86_64 (it is gated by
  `target_arch = "x86_64"`), so enabling it for WASM has no benefit.

**Proposed resolution:**

- Add to `demo/Cargo.toml`:
  `pulldown-cmark = { version = "0.13.4", default-features = false, features = ["html"] }`.
- Note in the README why the demo pins the same version as `htmd`'s
  dev-dependencies: it keeps round-trip behavior aligned with the test suite.

**Status:** Open.

---

### FR1-05 — There is no error contract for Markdown → HTML

**Severity:** Medium

**Where:** FR-2, FR-8, the Goals, and the acceptance criterion “Conversion errors
are shown to the user, not panics”.

**Observation:** Markdown → HTML cannot fail for valid UTF-8 input.

- `Parser::new_ext` returns a parser, not a `Result`; parsing always produces
  events.
- `html::push_html` returns `()` and internally unwraps `write_html_fmt`.
  Writing to a `String` cannot fail, so the unwrap is safe only because the
  writer is a `String`.
- Empty input produces `""` naturally.

The spec reads as if both directions can fail. For Markdown → HTML there is
nothing to show, and a stale HTML → Markdown error must be cleared when the
direction changes. If the implementation later switches to `write_html_io` or a
custom writer, errors must be propagated instead of unwrapped.

**Proposed resolution:**

- State in FR-1 that Markdown → HTML is infallible for UTF-8 input and scope the
  error UI to FR-2 (htmd), WASM initialization, and any future I/O.
- Keep the wrapper as `fn markdown_to_html(markdown: &str, options: Options) -> String`,
  not `Result<String>`, and clear the error state on direction or input changes.
- If streaming output is ever used, use `html::write_html_io` and propagate
  `io::Error`.

**Status:** Open.

---

### FR1-06 — The math sample has no typesetting path

**Severity:** Medium

**Where:** FR-1 (rendered output); FR-5 (`math`); FR-7 (samples include math).

**Observation:** With `ENABLE_MATH`, `pulldown-cmark` emits
`<span class="math math-inline">…</span>` and
`<span class="math math-display">…</span>` with the TeX escaped as text. It does
not typeset math. Without KaTeX or MathJax, the math sample shows raw TeX in the
“rendered” pane.

Loading a math renderer from a CDN violates the no-external-requests requirement.
Bundling one adds significant WASM/JS/CSS size. Note also that `math-display` is a
`<span>`, so the preview CSS must set `display: block`.

**Proposed resolution:**

- Decide: bundle a math renderer (and document the size cost), or label math as
  source-only/raw in the preview and adjust FR-7's wording.
- If bundling, run the renderer after every conversion, not only on first mount,
  and include its CSS in the preview document.

**Status:** Open.

---

### FR1-07 — The round-trip leg must reuse FR-1's converter and options

**Severity:** Medium

**Where:** FR-6 (round trip); FR-4 (rendered view in HTML → Markdown mode).

**Observation:** FR-6's second leg is exactly FR-1: “converts that Markdown back
to HTML with pulldown-cmark”. Two consistency problems follow.

- If the round trip uses a different option set than the UI (or
  `Options::empty()`), tables, footnotes, and task lists disappear from the
  round-tripped HTML. The diff then measures an option mismatch, not `htmd`
  fidelity.
- In HTML → Markdown mode, FR-4's “Rendered” view of the Markdown output also
  needs the Markdown → HTML converter. The spec does not say to reuse it, which
  invites a second, divergent conversion path.
- Raw HTML passthrough means the original and round-tripped HTML can differ for
  reasons unrelated to `htmd`. The diff should say what it measures.
- `push_html` emits trailing newlines after block elements. Diffing raw strings
  produces whitespace-only noise.

**Proposed resolution:**

- Use the same `markdown_to_html` function and the same `Options` value for FR-1,
  FR-4's rendered Markdown output, and FR-6's second leg.
- Display the options used for the round trip, or force a documented canonical
  set in round-trip mode.
- Normalize line endings and trailing whitespace in the diff, or use a
  whitespace-insensitive diff, and label the result as diagnostic.

**Status:** Open.

---

### FR1-08 — The performance target conflicts with the debounce

**Severity:** Medium

**Where:** §8 (“first input-to-output conversion under 100 ms”); FR-3 (debounce
150–300 ms).

**Observation:** User-observed latency is debounce plus conversion, so the
current wording cannot be satisfied when the debounce alone is 150–300 ms. The
target presumably means conversion work only, but the spec does not say so.
`push_html` re-parses the whole input on every run; that is fine for 100 KB but
must be measured in a release WASM build. Output can be larger than input
(escaping and generated markup), so memory and time should be measured on the
output as well.

**Proposed resolution:**

- Reword to “conversion work (excluding debounce) completes in under 100 ms for
  inputs up to 100 KB, measured in a release WASM build, at p95 over N runs”.
- Reuse a `String` buffer (`clear()` then `push_html`) or allocate with
  `String::with_capacity` to reduce per-keystroke allocations.
- Keep the debounce at the lower end (150 ms) if the budget is tight.

**Status:** Open.

---

### FR1-09 — The demo has no conversion tests or lint gate

**Severity:** Medium

**Where:** §10 acceptance criteria.

**Observation:** The criteria require `cargo build` for the demo but clippy and
tests only for the library. The demo's `convert.rs` is pure Rust and can be
tested on the host without a browser, yet nothing verifies the FR-1 contract.
Also, `cargo build` without `--target wasm32-unknown-unknown` does not prove the
WASM artifact works.

**Proposed resolution:**

- Add to acceptance/CI:
  - `cargo build --target wasm32-unknown-unknown` for the demo.
  - `cargo clippy --manifest-path demo/Cargo.toml -- --deny warnings`.
  - `cargo test --manifest-path demo/Cargo.toml`.
- Add host tests for: empty input; headings, lists, links, and code blocks;
  tables; footnotes; task lists; raw HTML passthrough; a `javascript:` link and a
  heading `onclick` that must not execute in the preview; the 100 KB threshold;
  and option changes.
- Add a browser smoke test or a manual checklist for the sandboxed preview.

**Status:** Open.

---

### FR1-10 — The 100 KB threshold has no unit

**Severity:** Low

**Where:** FR-8 (100 KB threshold).

**Observation:** The spec does not say whether 100 KB means JavaScript
`string.length` (UTF-16 code units) or UTF-8 bytes (Rust's `str::len`). For
non-ASCII input the two differ, so the UI guard and the conversion guard can
disagree about whether auto-conversion is allowed.

**Proposed resolution:**

- Define the unit explicitly. UTF-8 bytes (`TextEncoder().encode(input).byteLength`
  or `Blob.size`) match Rust's `str::len()`; use the same check for both
  directions before conversion.

**Status:** Open.

---

### FR1-11 — The preview's accessibility and focus behavior are unspecified

**Severity:** Low

**Where:** §8 (WCAG 2.1 AA); §7 (keyboard accessibility).

**Observation:** Arbitrary rendered HTML can include `autofocus`, media that
autoplays, poor heading order, or focus traps. If it is rendered in the app DOM,
it can interfere with the app's keyboard navigation. If it is in an iframe, the
iframe needs an accessible name and a sensible tab order.

**Proposed resolution:**

- Give the preview iframe a `title` and provide the source view as the
  accessible alternative.
- If sanitizing, neutralize `autofocus`, autoplaying media, and `tabindex`;
  sandboxing isolates focus but content must still be readable.
- Announce conversion results with a polite live region, debounced to avoid
  noise.

**Status:** Open.

---

### FR1-12 — Broken reference links silently become literal text

**Severity:** Low

**Observation:** `Parser::new_ext` uses no broken-link callback, so an unresolved
reference link such as `[x][missing]` renders as literal text. Evaluators may
read that as a conversion bug. `Parser::new_with_broken_link_callback` exists if
the demo wants to surface unresolved links explicitly.

**Proposed resolution:**

- Keep the CommonMark behavior (literal text) and document it, or use
  `new_with_broken_link_callback` to emit a link with a visible “unresolved”
  marker. Add a sample or test either way.

**Status:** Open — low priority.

---

## Suggested FR-1 wording

The following is a drop-in replacement for discussion; it does not change the
rest of the specification.

> ### FR-1 — Markdown → HTML
>
> Given Markdown input, render an HTML fragment with `pulldown-cmark`:
>
> ```rust
> use pulldown_cmark::{html, Options, Parser};
>
> fn default_markdown_options() -> Options {
>     Options::ENABLE_TABLES
>         | Options::ENABLE_FOOTNOTES
>         | Options::ENABLE_STRIKETHROUGH
>         | Options::ENABLE_TASKLISTS
>         | Options::ENABLE_MATH
>         | Options::ENABLE_HEADING_ATTRIBUTES
> }
>
> fn markdown_to_html(markdown: &str, options: Options) -> String {
>     let mut output = String::with_capacity(markdown.len());
>     html::push_html(&mut output, Parser::new_ext(markdown, options));
>     output
> }
> ```
>
> - `pulldown-cmark` is pinned to `0.13.4` with
>   `default-features = false, features = ["html"]`.
> - Conversion is infallible for UTF-8 input and returns a fragment, not a
>   document. Empty input returns `""`.
> - The fragment is shown as exact source in a `<pre>` using text content, and
>   rendered only inside a sandboxed iframe with a restrictive CSP (see FR-4).
>   It is never assigned to `innerHTML` in the app document.
> - The default option set is `default_markdown_options()`. Reset restores it.
>   Options are parse-time: changing one re-parses the input and changes both
>   views.
> - The preview document contains its own CSS for headings, lists, tables, code
>   blocks, blockquotes, task lists, and math. Math is rendered as raw TeX unless
>   a bundled renderer is chosen.
> - The same function and option set render the Markdown output in FR-4 and the
>   second leg of FR-6.

## Decisions needed

1. Preview isolation: sandboxed iframe, sanitizer, or both?
2. Default Markdown option set: pure `empty()` baseline, or extensions enabled to
   match the FR-7 samples?
3. Math: bundle a renderer, or show raw TeX and adjust the sample wording?
4. Round-trip options: same as the UI, or a fixed canonical set?
5. 100 KB threshold: UTF-8 bytes or UTF-16 code units?
6. Broken reference links: literal text or an explicit “unresolved” link?

## Verified correct

- The FR-1 snippet matches the `pulldown-cmark` 0.13.4 API.
- `Options` is `Copy` and can be stored directly in a Leptos signal.
- `push_html` writing to a `String` cannot return or panic on an I/O error.
- Empty Markdown produces empty HTML.
- `pulldown-cmark` is pure Rust and is compatible with `wasm32-unknown-unknown`.
- `push_html` emits a fragment, not a full document.
- `push_html` does not sanitize or scheme-check its output; see FR1-01.
