# Review: htmd / pulldown-cmark live demo specification

## Rounds 1–4 provenance

Prompting **Big Pickle OpenCode (Version 2.2.6)**, standard effort, with
`Review @demo/demo-spec.md, writing findings to @review.md` produced the
findings below (rounds 1–4). Resolved items are struck through with team
notes. Rounds 5 and 6 were run by d_baitakov with a fresh-context GLM-5.3
OpenCode subagent (round 5 resolved, round 6 open at the end of this file).
The professor's submission artifacts — contributions, compiler-test output,
the review-round table, and the last LLM review with its prompt and model —
live in [submission.md](submission.md).

Reviewed `demo/demo-spec.md` (187 lines) against the `htmd` source in this
repository (`htmd` 0.5.5, `src/options.rs`, `src/lib.rs`,
`src/element_handler/`), `pulldown-cmark` 0.13.4, the repository build policy in
`AGENTS.md`, and the current `translation_approach.md`.

The spec is substantially better than the earlier draft: the option list is
concrete, the storage and error requirements are specific, and the acceptance
criteria now include negative cases. The remaining problems are mainly
contradictions between sections, requirements that cannot be verified as
written, and a few implementation traps that would surface immediately when the
demo is built.

The `demo/` directory currently contains only this spec and `review.md`, so no
implementation was available to review. Findings are ordered by severity.

## What checks out

These claims were verified against the pinned crates and need no change:

- The `htmd` option names, variants, and defaults in FR-4 match
  `src/options.rs` exactly, including `ul_bullet_spacing: 3`,
  `ol_number_spacing: 2`, `preformatted_code: false`, and
  `translation_mode: Pure`.
- `pulldown-cmark = { default-features = false, features = ["html"] }` is valid
  for 0.13.4: its defaults are `["getopts", "html"]`, so disabling defaults
  removes the CLI dependency. (`simd` is not a default feature in 0.13.4, so it
  is not lost by this setting.)
- `html5ever` is already the parser used by `htmd`, so the "permissive
  malformed HTML" requirement is realistic.
- `htmd` already special-cases pulldown-cmark's math spans
  (`src/element_handler/span.rs`), so the math sample can round-trip as `$` and
  `$$` rather than being irretrievably lost.
- `style-src 'unsafe-inline'` in the iframe CSP is needed because
  pulldown-cmark emits table alignment as inline `style` attributes.

---

## Blocking issues

~~### B1. Acceptance criterion 9.7 is unfinished (lines 185–187)~~

The first bullet ends with "when comparing HTML inputs that differ *only* by:"
and then the list is missing. The next bullet is a separate "Automated
Regression Suite" requirement. As written, the central false-positive test
cannot be implemented or accepted, because the allowed safe variations are
undefined.

**Fix:** enumerate the variations, for example: tag-name case, attribute order,
attribute quoting, self-closing vs. void syntax, equivalent entity spellings,
insignificant whitespace between block elements, and comments/doctype if they
are in scope. State explicitly which of these are *not* considered equivalent
(for example, whitespace inside `<pre>`). Then make the golden-test bullet refer
to that list.

[team note: added to spec!]

~~### B2. The diff requirements contradict each other (FR-5 lines 63–73; 9.5 line 169; UI line 117)~~

There are four independent conflicts:

- FR-5 says the diff runs **exclusively on canonicalized strings**; 9.5 says the
  diff is generated **directly on the raw HTML outputs**.
- FR-5 specifies a **word-level** diff; 9.5 allows "word- or character-level".
- FR-5 is **HTML mode only**; 9.5 describes a general round-trip view, and the
  UI sketch has a manual "Round trip: [Run]" button, while FR-2 says conversion
  is automatic.
- The goal at line 12 says the pipeline diff shows intermediate views, but FR-3
  only defines two output tabs; it does not say which view is the intermediate
  Markdown in HTML mode.

A reviewer cannot tell whether the delivered diff is canonicalized or raw,
automatic or manual, or which mode is being tested.

**Fix:** define one pipeline: HTML mode only; take original HTML and
round-tripped HTML; canonicalize both; run `similar::TextDiff::from_words` (or
`from_chars`, but pick one); state whether the diff is automatic or on demand;
and update 9.5 and the UI sketch to match. State that the diff is diagnostic,
not proof of semantic equivalence.

[team note: added to spec!]

~~### B3. The "standard conversion failure" path cannot occur (FR-7 line 84; goal line 13; 9.6 line 174)~~

In the pinned `htmd` 0.5.5, `convert` has no reachable `Err` path:

- `html_to_tree` always returns `Ok(dom.document)` (`src/lib.rs` 148–166).
- `tree_to_markdown` returns `String`, not a `Result` (`src/lib.rs` 170–209).
- `convert` is just `Ok(self.tree_to_markdown(&self.html_to_tree(html)?))`
  (`src/lib.rs` 212–214).
- Serialization failures inside handlers are converted to strings
  (`src/element_handler/element_util.rs` 84–85).

So "any `io::Result<String>` error from `htmd::convert`" is dead code, and an
acceptance test for a standard conversion failure cannot be written. The real
failure modes are Rust panics and WebAssembly traps, which FR-7 already covers
separately. The goal "surface conversion errors gracefully" is therefore also
not measurable.

**Fix:** reword the requirement to distinguish (a) malformed HTML, which is
parsed permissively and is not an error, (b) Rust panics, handled by the panic
hook, and (c) wasm traps/OOM, which bypass a Rust panic hook and need a separate
JS-level error listener or an explicit non-goal. Keep the `io::Result` handling
only if the demo is allowed to depend on a future `htmd` version that can fail,
and say so.

[team note: added to spec!]

~~### B4. "Instant write cancels pending text timer" can lose text (FR-8 lines 95–99)~~

If the user types and then changes an option, switches mode, or changes a tab
within 500 ms, the instant write cancels the pending text write. The latest text
is then not stored until the next text change or a `pagehide`; a crash or a
mobile tab kill loses it. This also interacts badly with FR-2's separate
per-mode input buffers, which are changed by mode switches.

**Fix:** make the controller flush pending text *before* performing an instant
write, or serialize all fields in a single write and cancel only the redundant
queued write. Do not cancel a write whose data has not yet been persisted.
Acceptance should include: "type, immediately toggle an option, reload — text is
preserved."

[team note: added to spec!]

~~### B5. The persisted schema is not defined (FR-8 lines 89–100; 9.6 lines 179–183)~~

"Validate types/ranges for every field" cannot be implemented or tested without
a schema. The current text does not define:

- the exact JSON shape, including both per-mode input buffers, both output
  views, active tabs, scroll positions, all option fields, and `skip_tags`;
- whether tabs and scroll positions are per-mode or global;
- what happens to unknown fields;
- whether an invalid/future version is deleted or merely ignored;
- whether the app may overwrite a `version: 2` payload with `version: 1`.

Acceptance 9.6 says future versions are "safely ignored", but if the app then
writes version 1 state back, it has not ignored the future payload; it has
destroyed it. That matters if a newer deployment of the demo is later opened in
an older cached bundle.

**Fix:** include a versioned JSON example and per-field validation table in the
spec. Define unsupported-version behavior as read-only/no-write or migrate, not
"boot and overwrite". Also wrap *all* storage access, not just `getItem` and
`JSON.parse`: in Safari private browsing, reading the `localStorage` property
itself and `setItem` can throw `SecurityError`, not only `QuotaExceededError`.

[teame note: not relevant!]

~~### B6. The canonicalization pipeline is underspecified and has correctness risks (FR-5 lines 63–73)~~

The pipeline is central to the round-trip feature, but several decisions are
missing or wrong:

- **Fragment vs. document parsing.** `htmd`'s `html_to_tree` uses
  `parse_document`, which injects `<html><head><body>` wrappers. If the demo
  reuses it for canonicalization, the diff will be dominated by wrapper
  differences and the original fragment structure will change. Specify
  `parse_fragment` with an explicit context element (for example `<body>` or
  `<div>`) and a fragment serializer, or explicitly strip the document
  wrappers.
- **Missing dependencies.** This design needs direct dependencies on
  `html5ever` and `markup5ever_rcdom` (for `SerializableHandle` and serializer
  options); neither is in the dependency list at line 132. The spec should name
  the serializer API (`serialize`, `SerializeOpts`,
  `TraversalScope::IncludeNode`) so the output is reproducible.
- **Whitespace collapsing is destructive in raw-text elements.** "Collapse
  consecutive whitespaces" must exempt `<pre>`, `<code>`, `<textarea>`,
  `<script>`, and `<style>`, or it will hide real differences and change the
  meaning of the sample. It must also define what happens to whitespace-only
  text nodes between block elements.
- **Case folding is unsafe for foreign content.** html5ever already lowercases
  HTML tag and attribute names, but SVG/MathML attributes are case-sensitive
  (`viewBox`, `xlink:href`). Lowercasing all attributes corrupts them. Sort
  attributes by namespace plus local name, and preserve foreign-content case.
- **Entity "standardization" is undefined.** The parser already decodes
  character references into text; the serializer re-encodes them. Say whether
  `&nbsp;`/U+00A0 is normalized to a space, whether numeric and named references
  are considered equal, and whether that normalization applies inside
  `<pre>`.
- **Comments, doctypes, `<template>`, and CDATA are not mentioned.** Decide
  whether they are removed, preserved, or compared; otherwise the zero-diff
  criterion depends on unspecified parser behavior.
- **Attribute values are not normalized.** Attribute keys are sorted, but
  `class` token order, duplicate whitespace in values, and style declaration
  order can still produce noise. Either normalize these deliberately or state
  that they are out of scope.
- **Wrong diff API.** The usual `similar::TextDiff::from_lines` construction is
  line-based; a word-level diff needs `TextDiff::from_words` (or `from_chars`).
  Pin the `similar` version and state the tokenization choice.
- **Mode matters.** In `translation_mode: Pure` (the demo default), `htmd`
  intentionally drops attributes, so the diff can never be empty for most
  attribute-bearing HTML. The zero-diff acceptance corpus must be defined for a
  specific mode (probably `Faithful`), or it will fail by design.

**Fix:** turn FR-5 into an algorithm with explicit parse mode, serializer
options, normalization rules, preserved-element list, foreign-content policy,
comment policy, and diff tokenizer. Add golden inputs/outputs for each rule.

[team note: fixed!]

~~### B7. Size and performance budgets are not measurable as written (section 7 lines 127–132; 9.1–9.3 lines 147–158)~~

- **1.5 MB gzipped is not tied to an artifact or command.** Say whether it is
  the `.wasm` file alone or the total transfer (wasm + JS glue + CSS), and give
  the exact command and compression level (for example `gzip -6 -c
  demo/dist/*.wasm | wc -c`). "Default level 6 compression" conflates gzip with
  `wasm-opt`, which has no level 6.
- **The release profile is unstated.** The root `Cargo.toml` has
  `[profile.release] debug = true` (lines 39–40). If the demo is a workspace
  member, it inherits that and may ship debug info; if it is a separate crate,
  it needs its own profile. Require `opt-level = "z"`, `lto`, `codegen-units =
  1`, `panic = "abort"`, stripped debug info, and the chosen `wasm-opt`/Trunk
  settings.
- **TBT is the wrong metric for conversion latency.** Total Blocking Time
  measures main-thread blocking during page load, not the time to convert one
  document. Use `performance.mark`/`measure` around the conversion and Long
  Tasks API for jank, with a named CPU profile.
- **9.3 and 9.5 contradict each other.** 9.3 requires the first conversion
  under 100 ms; 9.5 requires 350–400 ms end-to-end after the last keystroke
  (250 ms debounce + 100 ms compute). State that 100 ms is compute-only and
  350–400 ms is end-to-end for the 2–100 KB debounced range; for inputs under
  2 KB, the immediate path should be under 100 ms end-to-end.
- **The budget excludes work the demo actually does.** Canonicalization parses
  and serializes two documents, and the diff can generate thousands of DOM
  nodes. Either include those in the compute budget or define separate budgets.
- **Cold start within 1500 ms has no environment.** Specify device, CPU
  throttling, network profile, and whether it is measured to first paint,
  first input, or conversion-ready.

[team note: fixed! — budget tied to artifact and command (`gzip -6 -c demo/dist/*.wasm | wc -c`, strictly under 1,572,864 bytes); demo release profile defined (`opt-level = "z"`, `lto`, `codegen-units = 1`, `panic = "abort"`, `strip`); TBT replaced by `performance.mark`/`measure` + Long Tasks API; compute-only vs end-to-end budgets reconciled; canonicalization and diff included in the compute budget; cold-start reference environment defined.]

~~### B8. The cross-browser automation claim is not supported as written (9.2 lines 152–155)~~

`wasm-bindgen-test`'s browser runner is WebDriver-based. It does not provide a
WebKit browser out of the box; WebKit on Linux CI needs a WebKit build and
driver (for example through Playwright), and macOS needs `safaridriver`. As
written, "Automated tests pass in headless Chromium, Firefox, and WebKit via
`wasm-bindgen-test`" is likely not achievable without additional tooling that
the spec does not mention.

**Fix:** name the runner and drivers (for example Playwright for all three, or
`wasm-bindgen-test` + chromedriver/geckodriver and Playwright/WebKit only), add
the CI matrix, or move Safari/WebKit to the manual checklist. Also reconcile
"latest stable" in 9.2 with the version floors in section 7 (Chrome 120+,
Firefox 120+, Safari 17+); "latest stable" changes over time and is not a
testable minimum.

[team note: fixed! — `wasm-bindgen-test` scoped to headless Chromium; Playwright end-to-end matrix (Chromium/Firefox/WebKit) added; Safari stays on the manual checklist; "latest stable" narrowed to the manual checklist with version floors as the testable minimums.]

~~### B9. The dependency and build contract is incomplete (line 132; 138–150)~~

- The dependency list omits `html5ever` and `markup5ever_rcdom` (required by
  FR-5), a JSON/serialization dependency for storage, `js-sys` if needed for
  timers, and test/build tooling (`trunk`, `wasm-bindgen-test`, browser
  drivers, `wasm-opt`).
- "All dependencies must be strictly version-pinned" lists no versions for
  `leptos`, `similar`, `wasm-bindgen`, `web-sys`, or
  `console_error_panic_hook`. Either give exact versions and require a
  committed `Cargo.lock` with `--locked`, or say that the lock file is the
  pinning mechanism.
- Whether `htmd` is a path dependency on this repository or a crates.io
  dependency is not stated. A path dependency cannot be version-pinned in the
  same sense, and it determines whether the demo tests the local changes.
- Whether the demo is a workspace member is not stated. The root commands
  `cargo build`, `cargo clippy -- --deny warnings`, and `cargo test` do not
  build a separate `demo/` crate. Specify the package name and commands, and
  run clippy/tests for the wasm target and the native target as appropriate.
- No CI workflow for the demo or the required size-check step is described.
  State the workflow file, the size command, and the failure threshold.

[team note: fixed! — `demo/` declared a standalone crate (not a workspace member) with a `path` dependency on the root `htmd`; dependency list completed (`html5ever`, `markup5ever_rcdom`, `serde`/`serde_json`, `js-sys`); pinning via the committed `demo/Cargo.lock` with `--locked`; `.github/workflows/demo-ci.yaml` steps and the size-check threshold specified in Deliverables.]

~~### B10. `skip_tags` handling can panic or silently do nothing (FR-4 line 60)~~

`HtmlToMarkdownBuilder::skip_tags` forwards to `add_handler`, which asserts
`!tags.is_empty()` (`src/element_handler/mod.rs` 396–408). If the UI maps an
empty comma-separated field to `skip_tags(vec![])`, the demo panics on the
"cleared" case. `skip_tags` also takes `Vec<&str>` and keys handlers by the
exact string; html5ever supplies lowercase tag names, so `IMG` will not match
`img`.

**Fix:** specify that the field is split on commas, each part is trimmed and
lowercased, empty parts are dropped and duplicates removed, and an empty result
builds a converter with no skip handler instead of calling `skip_tags` with an
empty vector. State the behavior for invalid tag names and very long input.
Add tests for empty, whitespace-only, mixed-case, duplicate, and unknown tags.

[team note: fixed! — parsing specified (split, trim, lowercase, drop empty/duplicate entries not matching `[a-z][a-z0-9-]*`, 200-char cap); empty result builds a converter with no skip handler; empty/mixed-case/duplicate/unknown cases pinned in acceptance 8.6.]

---

## Medium-severity issues

~~### M1. Section 5 is missing (line 21 → line 102)~~

The document jumps from section 4 to section 6. Either add the missing section
or renumber 6–9. The FR numbering is independent, so this is editorial, but it
looks like content was deleted during revision.

[team note: fixed! — sections renumbered 5–8; front matter (contributions, history, key decisions) is unnumbered.]

~~### M2. Reset semantics conflict and are ambiguous (FR-4 line 61; 9.5 line 168)~~

FR-4 says "Reset to Defaults" does **not** clear input text, change the mode, or
alter active tabs. 9.5 says it "resets completely to defaults via a single
click." Also, the "Demo Default Preset" enables all seven pulldown-cmark
options, which is not the crates' default (`Options::empty()`), so calling it
"defaults" is misleading.

**Fix:** rename it consistently (for example "Reset options to demo preset"),
enumerate exactly what it resets, and say whether it triggers a reconversion,
resets scroll positions, clears the opposite buffer, or closes the samples. If
the intended behavior is "reset options only", remove "completely" from 9.5.

~~### M3. The input model and UI sketch do not match (FR-2 lines 29–34; UI lines 102–123)~~

FR-2 requires a separate text buffer per mode, but the sketch shows one Input
pane and one Output pane, with no mode/buffer indicator. It also omits the
"Use Output as Input" button and the sample controls. "Use Output as Input"
must overwrite the *other* mode's buffer, which directly conflicts with the
"switching modes does not overwrite the opposite buffer" rule. The same
question applies to loading a sample.

**Fix:** define the overwrite policy (confirmation, undo, or versioned buffer),
say whether "Use Output as Input" switches modes and immediately converts, and
say whether sample loading replaces the current buffer or opens a new one.
Update the sketch to show the real controls and the active buffer.

[team note: fixed! — sketch updated (active-mode indicator, samples, Reload Sample, error banner region); overwrite policy defined: only explicit actions ("Use Output as Input", sample loading) may overwrite the opposite/current buffer, each with an "Undo" toast; plain mode switches never overwrite; "Use Output as Input" switches mode and converts immediately.]

~~### M4. The size-limit behavior is inconsistent (FR-7 line 82; 9.6 line 175)~~

FR-7 says "halt processing and trigger" a banner; 9.6 says it also clears the
output. FR-7 does not say whether the oversized text is kept, whether it is
persisted, or how the user recovers. "100 KB" is not defined as 100,000 or
102,400, or as UTF-8 bytes versus JavaScript string length (UTF-16 code units).

**Fix:** define the threshold and comparator (`>`, not `>=`), state that the
input text is retained and persisted (or deliberately not), that the output and
diff are cleared, that the banner is announced once per transition, and that
processing resumes automatically when the input falls back under the limit.
Test both a one-byte-over and a multi-byte Unicode document.

[team note: fixed! — limit defined as 100,000 UTF-8 bytes measured via `TextEncoder`, strict `>` comparator; oversized text is retained and persisted; output and diff cleared; banner announced once per transition into the limit; automatic resume at or below the ceiling; exact-boundary and multi-byte tests pinned in acceptance 8.6.]

~~### M5. Debounce and forced-conversion rules need concurrency semantics (FR-2 lines 27–34)~~

"Forced conversion every 900 ms during continuous typing" is not enough. What
happens if a conversion takes longer than 900 ms? Do forced ticks queue, cancel,
or skip? If a stale conversion finishes after a newer one, which output wins?
"0 ms delay" is not literally achievable; the event handler can schedule work
immediately, but rendering happens later. The 900 ms rule should apply only to
the 2–100 KB range, otherwise it is redundant with immediate conversion.

**Fix:** specify latest-input-wins, a single in-flight conversion, cancellation
or coalescing of queued work, and the scope of the 900 ms timer. Define
"immediate" as "no debounce; run on the next microtask/frame". Add a test where
a conversion is artificially slow and typing continues.

[team note: fixed! — latest-input-wins with per-run sequence numbers and stale-result discarding; at most one in-flight plus one queued run; forced ticks coalesce into the queued run; the 900 ms rule scoped to the debounced range; "immediate" defined as scheduled on the next animation frame; a slowed-conversion test added to acceptance 8.6.]

~~### M6. The panic hook requirement needs precision (FR-7 line 85)~~

- A Rust panic hook only runs for Rust panics. wasm traps (`unreachable`, stack
  overflow, OOM) and JavaScript exceptions do not necessarily invoke it, so the
  overlay cannot be a universal "critical error" mechanism. Either scope the
  requirement to Rust panics or add a `window` `error`/`unhandledrejection`
  listener for the rest.
- Only one global hook can be installed. `console_error_panic_hook::set_once()`
  installs one; a later `std::panic::set_hook` replaces it. "Complements" must
  mean the custom hook calls `console_error_panic_hook::hook(info)` (or the
  previously installed hook), not that both are installed independently.
- The hook must be null-safe and idempotent: the overlay may not be parsed yet,
  and the hook can run more than once. Disable *all* interactive controls
  (mode switch, tabs, samples, reset, "Use Output as Input"), not just the
  converter inputs, and give the overlay `role="alertdialog"` and focus.
- Confirm that the wasm build actually uses `panic = "abort"` and that the hook
  is invoked before the abort trap. Test with a deliberate panic in debug and
  release builds.

[team note: fixed! — the hook chains `console_error_panic_hook` instead of installing a second hook; is null-safe and idempotent; disables all interactive controls, not just inputs; overlay gets `role="alertdialog"` and receives focus; `panic = "abort"` set in the demo release profile with hook-before-abort ordering verified by deliberate panics in debug and release; wasm traps/OOM handled separately by the JS listeners.]

~~### M7. The iframe security contract is incomplete (FR-1 line 24; FR-3 lines 42–43)~~

"Sandboxed `<iframe>` with a strict CSP" does not say how either is applied.
`srcdoc` documents cannot receive response headers, so the CSP must be a
`<meta http-equiv="Content-Security-Policy">` in the generated document (or be
inherited from the parent policy, which is not guaranteed). The `sandbox`
attribute and its token list are not specified.

**Fix:** require `sandbox=""` with no `allow-scripts`, no `allow-same-origin`,
no `allow-forms`, no `allow-popups`, and no `allow-top-navigation`; set the
CSP inside the `srcdoc` document; assign `srcdoc` via the DOM property or
attribute-escape it; add an iframe `title`; and add `referrerpolicy`. Decide
whether `img-src data:` intentionally blocks all remote images; if not, add
`https:` with a clear privacy note. Raw HTML can also contain `<base>` or
`<meta http-equiv="refresh">`; the sandbox stops script execution but not
self-navigation inside the iframe, so strip or neutralize those if that matters.
Finally, state that the Source tab uses `textContent`/escaped text, never
`innerHTML`.

[team note: fixed! — `sandbox=""` with no tokens; CSP delivered via `<meta http-equiv>` inside the `srcdoc` document; `srcdoc` assigned through the DOM property; `title` and `referrerpolicy="no-referrer"` required; `img-src data:` documented as a deliberate privacy choice; `<base>` and `<meta http-equiv="refresh">` stripped before injection; Source tab pinned to `textContent` only.]

~~### M8. Fidelity expectations and sample outputs are undefined (FR-5, FR-6, 9.7)~~

Because the demo default is `translation_mode: Pure`, attributes are
intentionally dropped. The diff therefore shows expected losses, and the
zero-diff criterion must be run in a named mode. The samples ("Inline HTML",
"Math Blocks", "Reset Sample") have no exact content or expected result, and
the math requirement says only what happens when `ENABLE_MATH` is disabled,
while the default preset enables it.

**Fix:** define each sample's input and expected HTML/Markdown/round-trip output
under both math settings, and state which mode the golden diff tests use
(probably `Faithful`). Include known-lossy constructs in the sample set so users
can see what the diff is supposed to reveal, rather than treating every
difference as a bug.

[team note: fixed! — three samples defined with content and expected results (basics lossless; inline HTML with expected `Pure`-mode attribute loss; math under both `ENABLE_MATH` settings); the fidelity check and golden tests are scoped to `translation_mode: Faithful`, and the panel is hidden with a note in `Pure` mode; known-lossy constructs are part of the samples by design.]

~~### M9. The accessibility contract is weaker than "WCAG 2.1 AA" (lines 134–136; 9.4)~~

The spec says "WCAG 2.1 AA compliance" but lists only labels, keyboard
reachability, contrast, and live regions. Missing: the ARIA tabs pattern
(`role="tablist"`, `role="tab"`, `aria-selected`, `aria-controls`, arrow/Home/End
keys, hidden panes), an accessible name for the preview iframe, fieldset/legend
grouping for options, focus management after mode switches and errors, and
contrast verification in both themes. Assertive announcements can also become
noisy if the same banner is re-announced on every keystroke over the limit.

**Fix:** either narrow the claim to the listed checks or add the missing ARIA
and keyboard requirements, an axe/Lighthouse check plus a manual screen-reader
check, and the rule that each error transition is announced once. Include large
text at 3:1, not only normal text at 4.5:1.

[team note: fixed! — the claim is scoped: ARIA tabs pattern (tablist/tab/tabpanel, arrow/Home/End keys), radiogroup for the mode toggle, fieldset/legend for options, accessible name for the iframe, focus management on mode switch and errors, announce-once rule for banners, 3:1 for large text and UI components, axe-core in CI plus a manual screen-reader pass in the release checklist.]

~~### M10. Pathological-input tests are underspecified (FR-1 line 25; 9.6 line 174)~~

"Unclosed tags, unescaped scripts" is not a good malformed-HTML list;
`<script>` is valid. More importantly, `htmd`'s DOM walk is recursive
(`src/dom_walker.rs`), so deeply nested input can overflow the wasm stack even
when html5ever parses it successfully.

**Fix:** enumerate cases: unclosed/misnested tags, null/control characters,
`<template>`, `<svg>`/`<math>` foreign content, `<plaintext>`, very large
attributes, deeply nested elements, script/style contents, and XSS payloads.
Define the expected result for each (render, degrade, or reject) and add a depth
limit or a catch for stack overflow. Run a small fuzz corpus in CI.

[team note: fixed! — the nesting limit is 512 levels with pre-conversion rejection via the banner (a wasm stack overflow cannot be caught reliably); malformed input is permissively parsed with script/style treated as raw text; a small fuzz corpus runs in the demo CI workflow.]

~~### M11. "Reset Sample" is undefined (FR-6 line 79)~~

"Restore the default sample layout" does not say whether it restores the sample
selection, the input text, the mode, the options, or the output. It also
conflicts with FR-4's rule that reset does not clear input.

**Fix:** define it as a separate action from "Reset options", name exactly what
it changes, and state the overwrite/confirmation behavior.

[team note: fixed! — renamed "Reload Sample": restores the currently selected sample's original content (or loads the default sample), never changes options, mode, or tabs, is fully independent of "Reset Options to Demo Preset", and shares the "Undo" affordance.]

---

## Low-severity / editorial

- ~~**L1.** Section numbering skips 5 (line 21 to line 102). Renumber 6–9 or add
  the missing section.~~ [team note: fixed — renumbered 5–8, see M1.]
- ~~**L2.** "un-hide a pre-existing static overlay elements" (line 85) is
  ungrammatical; use "un-hide the pre-existing static overlay element".~~
  [team note: already fixed in the B3 rewrite ("un-hides the static fallback
  overlay element").]
- ~~**L3.** "0 ms delay" (line 29), "default level 6 compression" (line 131), and
  "any io::Result<String> error" (line 84) are imprecise; see B3, B5, and B7.~~
  [team note: fixed — "no debounce; next animation frame"; gzip level 6 is the
  CI measurement only; `io::Result` replaced by the future-proof `Result`
  wrapper in FR-7.]
- ~~**L4.** The UI sketch omits the samples, "Use Output as Input", "Reset
  Sample", the error banner, and the active buffer indicator. Add them so the
  sketch matches FR-2/FR-4/FR-6/FR-7.~~ [team note: fixed — sketch updated, see
  M3.]
- ~~**L5.** Reconcile "latest stable" (9.2) with the minimum versions in section 7.
  A minimum-version floor and a "latest stable" smoke test are different
  requirements.~~ [team note: fixed — floors are the testable minimums; "latest
  stable" applies only to the manual checklist, see B8.]
- ~~**L6.** If a light/dark theme is implemented, require contrast checks in both
  themes; "optional" should not appear in an acceptance criterion.~~
  [team note: fixed — a single light theme ships; dark theme moved to
  non-goals.]
- ~~**L7.** `pulldown-cmark` 0.13.4 has 15 option flags, not 7. The spec's seven
  are a valid subset, but list their exact `Options::ENABLE_*` names and say
  whether the other flags are fixed off or intentionally out of scope. In
  particular, `ENABLE_SMART_PUNCTUATION` and `ENABLE_HEADING_ATTRIBUTES` change
  output in ways users may not expect from a "defaults" preset.~~
  [team note: fixed — the seven `ENABLE_*` names are listed; the other 8 flags
  are declared fixed off and out of scope; the preset-vs-crate-default
  difference is stated explicitly.]
- ~~**L8.** FR-4 says the reset applies "all other options to table defaults"; the
  table mixes `htmd` and `pulldown-cmark` options. Split the table by crate so
  the default source is unambiguous.~~ [team note: fixed — FR-4 lists the two
  crates separately with their own default sources.]
- ~~**L9.** The purpose says "CommonMark Markdown to HTML", but the demo default
  enables GFM/extensions (tables, task lists, smart punctuation, math). Say
  "CommonMark plus selected extensions" if that is the intent.~~
  [team note: fixed — purpose now reads "CommonMark (plus selected
  extensions)".]
- ~~**L10.** Add a short "Decisions required before implementation" list at the
  top: diff pipeline (B2), storage schema (B5), canonicalization rules (B6),
  size/perf measurement (B7), cross-browser tooling (B8), and demo/workspace
  layout (B9).~~ [team note: fixed — a "Key decisions" list added at the top of
  the spec recording each resolved position and its review round.]

---

## Round 6 — open findings (awaiting processing)

Prompting **GLM-5.3** (OpenCode `general` subagent, fresh context, standard
effort), with the same review instructions as round 5 (*"review
demo/demo-spec.md for internal consistency, verifiability, technical accuracy
against the repo source, completeness gaps, and editorial problems; do not
re-raise findings already recorded as resolved"*) produced the following
output. These findings are **not yet applied** — they are left open for the
next team member to process into the spec.

### What checks out

- FR-4's `htmd` option list matches `src/options.rs` exactly: all 12 names, every variant, and every default (Atx, Asterisks, TwoSpaces, Inlined, Full, Fenced, Backticks, Asterisk, spacing 3/2, `preformatted_code: false`, `translation_mode: Pure`); the 1–4 spacing ranges match the clamp bounds documented in `src/options.rs`.
- FR-4's `br_style`-ignored-in-Faithful note is accurate (`src/element_handler/br.rs:12-17` emits literal `<br>` in Faithful mode), and disabling the control is a sound consequence.
- The `skip_tags` panic claim is correct: `skip_tags` forwards to `add_handler`, which asserts `!tags.is_empty()` (`src/lib.rs:248-250`, `src/element_handler/mod.rs:396-400`), and it takes `Vec<&str>` keyed by exact string, so FR-4's lowercasing rationale and empty-result rule are grounded.
- FR-7's "future-proof `Result` wrapper" framing is accurate: `convert` returns `io::Result<String>` (`src/lib.rs:212`) with no reachable `Err` (`html_to_tree` always `Ok` at `src/lib.rs:148-166`; `tree_to_markdown` returns `String`).
- `htmd` 0.5.5 matches `Cargo.toml:3`; root `Cargo.toml` has `[profile.release] debug = true` and no `[workspace]`, so §6's "root profile does not apply" and Deliverable 1's standalone-crate/`path` claims hold.
- 8.1's gate commands match `AGENTS.md` exactly.
- The math-sample round-trip claim is grounded in `src/element_handler/span.rs:17-23` (`math math-inline`/`math math-display` → `$`/`$$`).
- Numbers are internally consistent across the document: 1.5 MB = 1,572,864 bytes with strictly-under/fail-at-≥ in §6, Deliverable 3, and 8.1; 100 KB = 100,000 UTF-8 bytes with strict `>` and exact-boundary tests in FR-2/FR-7/8.6; 100 ms compute vs 100 ms e2e (≤2,000 bytes) vs 400 ms e2e (2,001–100,000 bytes) in FR-2/§6/8.3/8.5; 1500 ms cold start in §6/8.3; 512 nesting levels in FR-7/8.6; 500 ms storage debounce in FR-8/8.6.
- The FR-8 persisted schema's `cmark` object has exactly the 7 FR-4 flags and its `htmd` object has exactly FR-4's 12 options; unknown-field and version-wipe rules are stated once and used consistently.
- Section numbering (1–8) and all cross-references ("section 6" from §5/8.2/8.3, "section 8.3" from 8.5) resolve correctly.

### Blocking issues

None.

### Medium-severity issues

### R6-M1. CI workflow omits the axe-core step it is required to run (Deliverable 3 vs §6 Accessibility, 8.4)
Deliverable 3 enumerates the demo CI workflow steps (clippy, native test, wasm-bindgen-test, trunk build, size check, fuzz corpus, Playwright matrix) but does not include axe-core, while §6 requires "axe-core runs in CI on the built page" and 8.4 requires "axe-core passes in CI". As specified, the workflow cannot verify the criterion.
**Fix:** Add an explicit axe-core step to Deliverable 3's workflow list (e.g., an axe scan of the built page driven by the Playwright suite).

### R6-M2. Nesting pre-scan algorithm is underspecified and can miscount both ways (FR-7)
"The depth is measured by an iterative scan of the raw input (counting unmatched open tags)" does not define handling of void elements (513 `<br>` siblings would falsely reject), self-closing syntax, closing tags decrementing the count, `<` inside attribute values (`title="a<b"`), or `<` inside raw-text/comment content — and an under-count lets through exactly the stack-trap input the guard exists to reject. Since the demo's purpose is pasting edge-case HTML, these inputs are expected, not exotic.
**Fix:** Specify tokenizer-aware scan rules: skip void and self-closed elements, decrement on close tags, ignore `<` inside attribute values, comments, and raw-text (`script`/`style`/`textarea`) bodies; or state that the scan reuses an incremental html5ever tokenizer driven iteratively rather than raw string counting.

### R6-M3. "No visible input lag" criterion is unadjudicable for the forced tick (8.3 vs FR-2)
8.3 requires "no long task attributable to keystroke handling" while allowing scheduled conversions the full 100 ms budget; FR-2's 900 ms forced tick fires *during continuous typing*, so it is both caused by typing and a scheduled conversion run. A tester observing a ~100 ms main-thread block mid-typing cannot decide whether it violates the criterion.
**Fix:** State explicitly whether forced-tick conversions count as keystroke handling (e.g., "the long-task rule covers input-event handling only; forced-tick conversions are exempt but must complete within the 100 ms compute budget"), or reword the headline claim to match.

### Low-severity / editorial

### R6-L1. `&nbsp;` normalization exemption is inconsistent with the whitespace exemption list (FR-5)
Whitespace collapsing exempts `<pre>`, `<code>`, `<textarea>`, `<script>`, `<style>`, but nbsp normalization exempts only "raw-text elements" — an undefined term, and `<pre>`/`<code>` are not raw-text elements. As written, `&nbsp;` inside `<pre>`/`<code>` is normalized to a space, hiding a real difference the diff should show.
**Fix:** Apply the same preserved-element list to the nbsp rule (or define "raw-text elements" and justify the narrower scope).

### R6-L2. "Use Output as Input" and "Reload Sample" lack acceptance criteria (FR-2, FR-6 vs section 8)
FR-2's "Use Output as Input" (copy direction, mode switch, immediate conversion, Undo toast) and FR-6's "Reload Sample" are fully specified but have no corresponding test in section 8, unlike samples loading (8.5), reset (8.5), and storage (8.6).
**Fix:** Add 8.5/8.6 bullets covering the "Use Output as Input" round-trip with Undo restore, and Reload Sample restoring original sample content without touching options/mode/tabs.

### R6-L3. Preset naming variance (FR-4/§5 vs FR-8/8.6)
The same configuration is called "Demo Preset" (button label "Reset Options to Demo Preset" in FR-4 and the sketch) and "Demo Default Preset" (FR-8 wipe rule, 8.6 future-version boot). An implementer could read these as two different presets.
**Fix:** Pick one name and use it everywhere.

### Additional findings (added to this round by d_baitakov, from a specification-quality checklist; also open for processing)

### R6-M4. Word tokenization for the diff is undefined (FR-5, 8.7)
`similar::TextDiff::from_words` splits by Unicode word boundaries only with the crate's unicode feature; the spec pins "similar 2.x" but not the feature set, and never defines what a "word" is. For CJK text (no spaces), combining marks, and ZWJ sequences, tokenization can differ between the original and round-tripped sides, producing false diffs — e.g., a Kazakh word with a combining stress mark (ма́ма, U+0301) may split differently after round-trip.
**Fix:** pin the tokenizer (UAX #29 word boundaries via the `unicode` feature), state that tokens compare by code point, and add combining-mark and CJK cases to the golden corpus.

### R6-M5. "Alphabetically" / "lexicographically" sorting is collation-undefined (FR-5)
FR-5 sorts attributes and class tokens but never says whether order is by Unicode code point or locale-aware collation. Code point order is not alphabetical order in non-Latin scripts (Kazakh ә sorts differently by code point than in the Kazakh alphabet), and two browsers with different locale settings must produce identical canonical output, so the sort must not depend on locale.
**Fix:** state that all sorting in canonicalization is by Unicode code point (UTF-8 byte order), never locale collation.

### R6-L4. Unicode normalization of text nodes is unspecified (FR-5)
Equivalent NFC and NFD spellings (composed á vs a + U+0301) are not normalized before diffing, so decomposed text surviving from `&#x301;`-style entities can create false positives.
**Fix:** state that canonicalization applies NFC (not NFKC, which would also fold ligatures like ﬁ) to text nodes outside preserved elements, and add an NFC golden case.

### R6-L5. First-run state is undefined (FR-8 vs FR-2/FR-4)
FR-8 covers corrupt and future-version payloads, but a first run with no stored key at all is not addressed: the initial mode (Markdown → HTML?), the initial output tab (Rendered?), and whether any sample is preloaded are never stated.
**Fix:** state first-run defaults explicitly (suggested: mode = Markdown → HTML, tab = Rendered, empty inputs, Demo Default Preset options, no sample auto-loaded).

### R6-L6. Banner text says "100 KB" but the limit is 100,000 bytes (FR-7)
The user-facing banner says "100 KB", which users can reasonably read as 102,400 bytes (100 KiB); the enforced limit is 100,000 bytes. An input of 100,400 bytes is rejected while the banner says the limit is "100 KB".
**Fix:** change the banner text to "100,000 bytes", or state that the KB wording is deliberate and accept the mismatch.

**Total findings: 11 (blocking 0, medium 5, low 6) — 6 from the subagent review above, 5 added from the specification-quality checklist pass.**

---

## Round 7 — Various issues

Reviewed by **Codex** (GPT-6 Sol Extra High), using the prompt *"Review demo/demo-spec.md, write findings to demo/review.md, using the same format."*

Reviewed the current 255-line specification against the repository source,
`translation_approach.md`, and the locally cached sources for
`html5ever` 0.39.0, `markup5ever_rcdom` 0.39.0+unofficial, and
`pulldown-cmark` 0.13.4. This is a specification review: `demo/` contains
Markdown documents, with no application implementation to exercise. The round-6
findings remain open and are not counted again below. No specification changes
have been applied.

### What checks out

- FR-4's option variants, defaults, and spacing bounds still agree with `src/options.rs`; the Faithful-mode `br_style` restriction agrees with `src/element_handler/br.rs`.
- The empty-`skip_tags` guard is justified by the builder and handler-registration assertion in `src/lib.rs` and `src/element_handler/mod.rs`.
- The standalone-crate layout correctly avoids inheriting the root release profile. The root has no workspace declaration.
- FR-5 correctly distinguishes a diagnostic text diff from proof of semantic equivalence, and consistently limits the live pipeline to HTML input with Faithful translation.

### Blocking issues

None.

### Medium-severity issues

### R7-M1. Fragment serialization does not identify a safe serialization root (FR-5, lines 99–105)

`html5ever` 0.39.0's `TreeBuilder::new_for_fragment` creates a synthetic
`<html>` element beneath `RcDom.document`
(`html5ever/src/tree_builder/mod.rs:179–218`). Fragment parsing alone therefore
does not satisfy the prohibition on emitted wrappers. With the required
`TraversalScope::IncludeNode`, serializing that element emits the wrapper;
serializing `dom.document` instead panics
(`markup5ever_rcdom/lib.rs:623–672`). The spec never identifies which handles
to serialize. This is an implementation trap even for `<p>a</p>`.

**Fix:** explicitly serialize each child of the synthetic root with
`IncludeNode`, in document order, or serialize that root with
`ChildrenOnly(None)`. Add exact-output cases for an empty fragment and
`<p>a</p>` that prohibit an emitted wrapper and a panic. The relevant behavior
is visible in the [RcDom serializer source](https://docs.rs/crate/markup5ever_rcdom/latest/source/lib.rs).

### R7-M2. Renderer-added trailing newlines still cause false-positive diffs (FR-5, line 103; 8.7, lines 248–255)

The rule removes whitespace-only text nodes only when they are *between*
block-level components. For input `<p>a</p>`, Faithful conversion produces
`a`, and `pulldown-cmark` renders `<p>a</p>\n`
(`pulldown-cmark/src/html.rs:396–397`). The final newline is a trailing text
node, not a node between blocks. Collapsing it produces a trailing space, so
the canonical strings still differ. A minimal lossless paragraph can therefore
show a change despite 8.7's whitespace-equivalence requirement.

**Fix:** define treatment of leading and trailing fragment whitespace, including
the all-whitespace fragment, without trimming preserved-element contents. Add
goldens comparing `<p>a</p>`, `<p>a</p>\n`, and the actual round-trip output.

### R7-M3. Tag-only whitespace exemptions hide real losses in styled inline content (FR-5, line 103)

The five-tag exemption does not cover whitespace preserved through CSS. For
example, `<p><span style="white-space: pre">a  b</span></p>` displays two spaces.
Faithful inline serialization walks the span's text through
`compress_whitespace` (`src/element_handler/element_util.rs`,
`src/dom_walker.rs`), reducing it to one. The proposed canonicalizer then
collapses the original to the same text, hiding this real rendering change.
CSS `white-space` is inherited and can preserve spaces on ordinary elements;
see [CSS Text, white-space](https://www.w3.org/TR/css-text-3/#white-space-property).
This is separate from R6-L1's non-breaking-space exemption mismatch.

**Fix:** define a conservative policy for CSS-dependent whitespace. Preserve
text in affected subtrees, or explicitly mark such fragments outside the
supported fidelity check instead of treating their whitespace as insignificant.
Add a negative golden asserting a nonzero diff for the example's two-space
versus one-space forms.

### R7-M4. CDATA cannot be discarded as a DOM node after HTML parsing (FR-5, line 104)

FR-5 places CDATA removal in DOM normalization, but `RcDom::NodeData` has no
CDATA variant. In SVG/MathML, `html5ever` tokenizes CDATA contents as ordinary
text, losing their source boundaries. For example,
`<svg><text><![CDATA[a<b]]></text></svg>` and
`<svg><text>a&lt;b</text></svg>` reach the same text representation. A DOM walk
cannot discard the former's contents while preserving the latter's. The
[HTML tokenizer's CDATA rules](https://html.spec.whatwg.org/multipage/parsing.html#cdata-section-state)
require character tokens.

**Fix:** state that CDATA delimiters disappear during parsing while their
foreign-content text is retained. Continue dropping actual comment nodes,
including bogus comments produced by CDATA syntax in HTML content. Add a golden
for the equivalent SVG forms above.

### R7-M5. Mode switches preserve inputs but leave output validity undefined (FR-2, lines 50–53; FR-8, lines 133–142; 8.6, line 238)

Plain mode switches must never convert, but no per-mode output cache or clearing
rule is defined. Convert Markdown in mode A, convert unrelated HTML in mode B,
then switch back to A: the spec does not say whether to restore A's output,
clear the pane, or leave B's output visible. Global options can also change
while a mode is inactive, invalidating any cached output. Since "Use Output as
Input" copies the current output, displaying an unrelated or outdated result can
then overwrite the other buffer with the wrong content. The persisted schema
contains inputs and options but no outputs, and no initial conversion after
restoring a valid workspace is specified either.

**Fix:** define output state per mode and its validity against the input and
options used to produce it. On switching modes, restore a valid cached result
or show a defined empty/pending state; specify how stale results become fresh
without violating the no-conversion-on-switch rule. Disable "Use Output as
Input" while no valid result exists, and explicitly define initial conversion
after workspace restoration. Test switching after both modes have results and
after changing shared options.

### R7-M6. Mode-bound samples conflict with the current-buffer and Reload Sample rules (FR-6, lines 109–116)

Line 109 binds samples to intended modes and switches modes when necessary,
while line 115 says clicking a sample replaces the current mode's buffer.
Starting in Markdown mode and clicking the HTML-only sample therefore has two
different destinations. A second conflict occurs after loading the HTML sample
and manually switching to Markdown mode: Reload Sample must restore the
selected sample into the current buffer without changing mode, which would put
HTML-only sample content in the wrong editor. The schema also does not say
whether sample selection is per mode or global.

**Fix:** make sample selection per mode, with explicit Markdown and HTML
variants for samples supporting both modes. Specify that an HTML-only sample
switches to and replaces only the HTML buffer; Reload Sample uses the current
mode's selection or a named default for that mode. Add cross-mode sample and
reload cases, including the buffer restored by Undo. This addresses conflicting
behavior, beyond R6-L2's missing acceptance bullets.

### R7-M7. The 900 ms rule permits conversion starvation during continuous typing (FR-2, lines 48–50)

"At most once every 900 ms" sets a maximum frequency, but guarantees no run at
all. An ordinary 250 ms trailing debounce that resets on every keystroke
satisfies that wording while never converting during uninterrupted typing.
Neither the first forced-tick deadline nor whether edits reset it is defined,
so the stated Typing Lock Protection can be absent in a conforming
implementation. R6-M3 concerns attribution of long tasks; it does not establish
this missing progress guarantee.

**Fix:** specify a maximum wait of 900 ms from the first pending edit or the
previous forced dispatch, with new edits updating the queued snapshot rather
than resetting that deadline. Preserve the existing single-queued-run rule.
Test input events every 100 ms for several seconds and assert conversion
dispatches before typing stops.

### Low-severity / editorial

### R7-L1. The nesting-limit rationale incorrectly describes RcDom destruction (FR-7, line 120)

The pinned `markup5ever_rcdom` 0.39.0+unofficial implements `Node::drop` with
an explicit worklist (`lib.rs:268–283`), rather than recursively dropping an
intact child tree. The claim that its recursive destructor would trap the wasm
stack is inaccurate. The library's recursive conversion walk still justifies
a depth limit.

**Fix:** retain the limit, but base its rationale on `src/dom_walker.rs` and
measured wasm conversion behavior. Do not use the incorrect destructor claim
to rule out an iterative DOM-depth validation strategy when resolving R6-M2.

**Total new findings: 8 (blocking 0, medium 7, low 1). The 11 round-6 findings remain open separately.**

Validation: repository and dependency-source inspection only. No Rust source or
specification was changed; build, clippy, and test commands were not run for
this review-document update.
