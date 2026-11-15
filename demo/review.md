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

---

## Round 9 — Round-8 regressions and carried-over findings

Reviewed by **Claude Opus 5.5** (Claude Code), prompted with *"review
@demo/demo-spec.md"*, then *"fix all the medium issues and also any that are
easy fixes or would be solved by the same fix. then append to review.md as
round 9"*.

Reviewed the post-round-8 specification against the repository source and
against the open findings recorded for rounds 6–8 in this file and in
`demo/submission.md`. Cargo is not installed on the reviewing machine, so the
`html5ever`, `markup5ever_rcdom`, and `similar` sources were not opened; claims
about those crates rest on round 7's citations or are marked as unverified.
Unlike earlier rounds, the fixes were applied to the specification in the same
pass, so each finding below records its status.

### What checks out

- FR-4's 12 `htmd` options, their variants, and their defaults still match `src/options.rs`; the empty-`skip_tags` guard is still the `assert!` at `src/element_handler/mod.rs:400`.
- Round 8's escape-set rationale holds: `is_markdown_inline_special` (`src/dom_walker.rs:196–198`) is exactly `` \ & * _ ` [ ] $ ``, and `'`, `"`, `{`, and mid-text `~` are never escaped, so smart punctuation, strikethrough, and heading attributes would rewrite ordinary text in the round trip (wording overstated; see R9-L6).
- `HtmlToMarkdown` exposes `html_to_tree` and `tree_to_markdown` publicly, and `convert` is exactly their composition (`src/lib.rs:148–214`). The demo can therefore parse once, inspect the tree, and convert that same tree — the basis for this round's fix of R6-M2.
- R6-M1, R6-M3, R6-L3, R6-L5, R7-M7, and R7-L1 remain resolved, as round 8 recorded.

### Blocking issues

None new. The closest was carried-over R7-M1: read literally, the serialization rule panics, and `panic = "abort"` turns that into the crash overlay on the first Faithful-mode conversion. It is fixed this round.

### Medium-severity issues

### R9-M1. A stale run can still paint the preview (FR-2, execution model)
Round 8 defined a run as in flight "until its output is committed, including the async iframe `srcdoc` load". The `srcdoc` must therefore be assigned before the run ends. If newer input arrives during the load, the stale preview is already on screen and "discarded" cannot retract it. The spec also never said whether the Source text and fidelity panel are written in the same step as `srcdoc`, so the three views could disagree.
**Fix:** define commit as one synchronous step that checks the sequence number and then writes `srcdoc`, Source text, and fidelity panel together, or writes nothing. The run stays in flight until the iframe's `load` for that commit, and the iframe loads even while its tabpanel is hidden.
**Status:** fixed (FR-2 "Execution model").

### R9-M2. Mode switches do not advance the sequence number (FR-2)
Round 8's list of sequence-advancing events is newer input, an option change, or entering an FR-7 limit. A mode switch is absent. A 250/900 ms batch pending in mode A therefore dispatches after a switch to mode B and commits A's output into B's pane. Sample load, Reload Sample, Undo, and Reset are not named either. This is a concrete race inside R7-M5.
**Fix:** advance the sequence number on every buffer change (including sample loads, Reload Sample, Undo, and "Use Output as Input"), every option change including Reset, every mode switch, and entering a limit. A mode switch also cancels both timers of the pending batch.
**Status:** fixed (FR-2 "Sequence numbers"; 8.6 concurrency criterion extended).

### R9-M3. Preview stripping has no specified mechanism, and a recursive walk can crash (FR-1)
FR-1 says `<base>`, `<meta http-equiv="refresh">`, `target`, and prefetch `<link>`s are stripped, but not how. String or regex matching is bypassable (`<BASE`, `<base\n…`), so stripping needs a parse. That parse covers Markdown → HTML output, which is exempt from the nesting limit: 50,000 `>` characters produce 50,000 nested `<blockquote>` elements (`pulldown-cmark` is iterative), so a recursive strip pass overflows the wasm stack and shows the crash overlay. The denylist is also incomplete: `<meta http-equiv="x-dns-prefetch-control" content="on">` re-enables anchor DNS prefetching, so attacker-chosen hostnames can be resolved without a click.
**Fix:** parse with `html5ever` and walk iteratively; remove whole element types (`<base>`, `<meta>`, `<link>`) and every `target` attribute instead of enumerating `rel`/`http-equiv` values; the Source tab keeps the unstripped output.
**Status:** fixed (FR-1; new 8.6 "Preview stripping" criterion).

### R9-M4. The "changed in another tab" banner re-announces every ~500 ms (FR-8)
Round 8 shows the banner "once per external write". While the other tab is in use, its 500 ms debounced saves fire a `storage` event about twice a second, so an assertive banner is re-announced continuously. That contradicts §6 and 8.4 ("announced once per state transition"). Nothing said when the banner clears.
**Fix:** show it once per transition into the externally-modified state; clear it on this tab's next successful write, which overwrites the external change.
**Status:** fixed (FR-8 "Multiple tabs"; 8.6 criterion updated).

### Low-severity / editorial

### R9-L1. Reload Sample lost its Undo (FR-6)
The round-4 team note in this file says Reload Sample "shares the 'Undo' affordance", but the FR-6 text did not say so. Reload Sample could discard user edits with no recovery.
**Status:** fixed (FR-6 rewritten together with R7-M6; 8.5 criterion added).

### R9-L2. In-page anchors cannot work in the preview (FR-1)
`ENABLE_FOOTNOTES` is on in the Demo Preset, but footnote references and back-links (`#fn-1`) become blocked popups under the injected `<base target="_blank">`. Without it, relative URLs in an `about:srcdoc` document resolve against the embedding page's URL, so `#fn-1` would load the demo page itself into the iframe.
**Status:** fixed by documenting the behavior as accepted (FR-1).

### R9-L3. The panic hook may re-enter Leptos (FR-7)
Disabling controls through Leptos signals from inside the hook can re-borrow reactive state held at the panic site, panic again, and abort before the overlay shows. Separately, `strip = true` removes the wasm name section, so the "trace" the hook logs has no function names; and "verified with deliberate panics in release builds" needs a panic trigger in the shipped build, which is undefined.
**Status:** closed — release verification is an opt-in `demo-panic-test` feature compiled out of shipped builds; stripped symbol names accepted (the hook logs the panic message; symbolized traces not guaranteed).

### R9-L4. Live regions "hidden when empty" are not reliably announced (§5)
If the banner region is `display: none` or `hidden` while empty, revealing it and inserting text in the same step is often not announced. The same applies to the toast's polite region. How concurrent banners display was undefined.
**Status:** fixed (§5 "Live regions"; the sketch says "collapsed when empty").

### R9-L5. Nesting rejection does not clear output (FR-7)
The size limit clears output and diff; the nesting limit did not, so stale output from earlier input could sit beside rejected input and be copied by "Use Output as Input".
**Status:** fixed (FR-7 nesting limit now mirrors the size limit; 8.6 criterion updated).

### R9-L6. The escape-set claim is overstated (FR-5)
"`htmd` escapes only `` \ & * _ ` [ ] $ ``" omits the line-start escapes of `= ~ > - + #`, ordered-list delimiters, and HTML-like `<` (`src/dom_walker.rs:187–189`, `480–516`). The conclusion is unaffected.
**Status:** fixed (FR-5 "Round-trip rendering" reworded).

### R9-L7. "Neither hangs" has no time ceiling (§6, 8.3)
The 5,000-token cap still allows on the order of 10⁸ Myers steps, run synchronously on the main thread, and `from_words` counts whitespace runs as tokens, so the cap is about 2,500 words.
**Status:** closed — §8.3 now asserts the full pipeline (incl. fidelity diff) completes under 1,000 ms on the reference environment.

### R9-L8. Build and dependency gaps (§6, Deliverable 4)
GitHub Pages serves project sites under `/<repo>/`, but Trunk's default public URL is `/`. The demo's direct `html5ever`/`markup5ever_rcdom` were not tied to `htmd`'s 0.39, risking two copies against the size budget. `leptos` needs its `csr` feature. Units mix conventions: 1.5 MB is 1,572,864 bytes (binary) while 100 KB and 2 MB are decimal.
**Status:** closed — every limit states its exact byte count; the counts are normative and the kB/MB spellings are prose.

### R9-L9. The fidelity check is hidden by default (FR-4, FR-5)
The Demo Preset (and every Reset) sets `Pure`, so the pipeline fidelity check, a §2 goal, is hidden on first run.
**Status:** closed — intended: the preset mirrors library defaults and the Faithful radiogroup choice reveals the panel (now stated in FR-5).

### Earlier findings resolved this round

- **R6-M2** (nesting pre-scan): replaced the raw-tag scan. The demo parses once with `html_to_tree`, measures depth with an iterative walk, and passes the same tree to `tree_to_markdown`, so void elements, optional end tags, and misnesting need no rules. This also resolves round 8's "optional end tags in the nesting scan".
- **R6-M4** (tokenization): pinned `from_words` as whitespace-run splitting compared by code point, and noted that identical tokenizers on both sides cannot create a change. R6-M4's premise is inaccurate: Unicode word boundaries belong to `from_unicode_words` (the `unicode` feature), not `from_words` (not re-verified against the `similar` source this round). CJK and combining-mark goldens added to 8.7.
- **R6-M5** (collation): all canonicalization sorting is by Unicode code point.
- **R6-L1** (`&nbsp;` exemption): the U+00A0 rule now uses the same preserved-subtree list as whitespace collapsing. This also resolves round 8's `&nbsp;` finding.
- **R6-L2** (acceptance criteria): 8.5 bullets added for "Use Output as Input" and Reload Sample.
- **R6-L4** (NFC): resolved by deciding not to normalize. The diff compares an input with its own round trip, and neither `htmd` nor `pulldown-cmark` normalizes text, so both sides carry the same code points; NFC would hide a real change.
- **R6-L6** (banner units): the banner now reads "100,000-byte maximum".
- **R7-M1** (serialization root): serialize the synthetic `<html>` element's children with `IncludeNode` (or that element with `ChildrenOnly(None)`), never the document node; 8.7 exact-output golden added.
- **R7-M2** (trailing newlines): whitespace at the fragment and block edges is trimmed, and "block-level" is now an explicit element list (it was previously undefined); 8.7 golden added.
- **R7-M3** (CSS whitespace): subtrees whose inline `style` declares a non-`normal` `white-space` are preserved; stylesheet-selected whitespace is declared outside what the diff verifies; negative golden added.
- **R7-M4** (CDATA): FR-5 now says CDATA disappears during parsing — a bogus comment in HTML content, kept text in foreign content; golden added.
- **R7-M5** (output validity on mode switch): each mode has its own output and a validity rule. Deliberate change: switching to a mode whose output is invalid converts that mode's own buffer; the former "never auto-converts" rule is narrowed to "never reads, converts, or overwrites the other mode's buffer". Startup conversion is defined, and "Use Output as Input" is disabled while the output is empty or invalid.
- **R7-M6** (mode-bound samples): samples have per-mode variants; clicking an HTML-only sample in Markdown mode switches mode and replaces only the HTML buffer; the selection is per mode and persisted through two new FR-8 fields, `sample_md_to_html` and `sample_html_to_md`.
- **Round 8 lows:** `skip_tags` vs SVG case is documented as an accepted limitation (FR-4); Trunk/`wasm-opt` pinning is fixed (`data-wasm-opt="z"`, pinned Trunk, `Trunk.toml` `[tools]`). Untested browser floors and the editorial finding stay open; their details are not recorded in this file.

**Total new findings: 13 (blocking 0, medium 4, low 9).** Fixed this round: all 4 new medium findings, 5 new low findings in full and 2 in part, all 7 open round-6/7 medium findings, all 4 open round-6 low findings, and 4 of the 6 round-8 low findings. Still open: R9-L3 (in part), R9-L7, R9-L8 (units), R9-L9, and round 8's browser-floor and editorial findings.

Validation: specification and review documents only. No Rust source was changed, and build, clippy, and test were not run (Cargo is not installed on the reviewing machine).



Prompting **Claude Opus 5.5** (Claude Code, fresh context) with <em>"review
@demo/demo-spec.md"</em> produced the following round-10 output. The review
checked the post-round-9 specification against `src/lib.rs`,
`src/dom_walker.rs`, `src/element_handler/pre.rs`, `tests/common/mod.rs`,
`tests/code_tests.rs`, and the round-9 notes in `review.md`. Cargo is not
installed on the reviewing machine, so the `html5ever` and `markup5ever_rcdom`
sources were not opened; claims resting on them are marked as unverified. No
finding was deemed incorrect or irrelevant, so nothing is struck through; all 9
findings remain open for processing.

#### What checks out

* FR-4's 12 `htmd` options, their variants, and their defaults still match
  `src/options.rs`, and the 15-flag count for `pulldown-cmark` 0.13.4 is right.
* `convert` is exactly `html_to_tree` followed by `tree_to_markdown`
  (`src/lib.rs:148–214`), so FR-7's parse-once nesting check is implementable.
  `html_to_tree` uses `parse_document`, which is consistent with "children of
  `<head>` and `<body>` are depth 1".

#### Blocking issues

None.

#### Medium-severity issues

* <strong>R10-M1 — A conversion that never reloads the iframe stalls the
  pipeline for good (FR-2, execution model):</strong> a run counts as in flight
  until the iframe fires `load` for its commit, but a stale run writes nothing,
  a run whose output is cleared (empty input, the 2 MB limit, an `Err` banner, a
  size or nesting limit) may not set `srcdoc`, and a mode switch can discard a
  run still waiting for `load`. None of these gets a `load`, so the queued run
  never starts and conversion stops for the rest of the session; the 8.6
  concurrency test hits the stale case directly. <em>Fix: a run that writes
  nothing, or does not navigate the iframe, ends at commit; define what
  clearing the preview does to `srcdoc`; ignore a `load` that does not match
  the current commit.</em>
* <strong>R10-M2 — Every ordinary code block shows a false diff (FR-5,
  preserved subtrees):</strong> CommonMark code blocks always end with a line
  ending, so `pulldown-cmark` renders `<pre><code>x\n</code></pre>`, while the
  usual input `<pre><code>x</code></pre>` converts to ```` ```\nx\n``` ````
  (`tests/code_tests.rs`). Inside a preserved subtree nothing trims the extra
  `\n`, so the corpus's real-world page shows a change on every code block, and
  the basics sample passes 8.7's zero-diff test only if its HTML happens to
  include the `\n`. <em>Fix: on both sides, remove one trailing U+000A from the
  last text inside each `<pre>` (a single trailing newline does not render);
  add a golden for `<pre><code>x</code></pre>`.</em>
* <strong>R10-M3 — `<noscript>` and `<template>` bypass preview stripping
  (FR-1; 8.6 "Preview navigation" and "Preview stripping"):</strong> the
  sandboxed preview has scripting disabled, so the browser parses `<noscript>`
  contents as markup, but `html5ever` parses with scripting enabled by default,
  so the walk sees them as text and strips nothing. If the serializer writes
  that text raw (believed but unverified), `<noscript><a href="https://x.example"
  target="_self">click</a></noscript>` loads `x.example` into the preview, and
  `<link rel=preconnect>`, which CSP does not cover, gets through the same way.
  `RcDom` stores `<template>` contents apart from its children, so the walk
  misses them too, while browsers make `<template shadowrootmode="open">` a
  live shadow root. <em>Fix: remove `<noscript>` and `<template>` elements whole
  in the preview walk, or parse and serialize with scripting off; add both
  cases to the 8.6 stripping test.</em>
* <strong>R10-M4 — A panicking input can lock the user in a crash loop (FR-7,
  FR-8):</strong> the flush before "Use Output as Input" or a sample load, the
  500 ms text save, and the `pagehide` flush when the user clicks reload (as the
  overlay instructs) can all persist the crashing input. Startup converts the
  restored buffer once, so it panics again, and the only way out is clearing
  site data by hand. <em>Fix: have the panic hook and the
  `error`/`unhandledrejection` listeners stop all further storage writes and
  set a crash-marker key; when the marker is present at startup, restore the
  workspace without converting and show a banner offering to convert or
  clear.</em>

#### Low-severity / editorial

* <strong>R10-L1 — Typing in `skip_tags` counts as an option change (FR-4,
  FR-8):</strong> on a 100 KB input every keystroke runs a full conversion and a
  synchronous save of the whole workspace, and FR-8 does not say whether a
  stored `skip_tags` over 200 characters is invalid. <em>Fix: debounce the
  field like text input and state the stored-length rule.</em>
* <strong>R10-L2 — Wasm stack headroom at 512 levels is unmeasured
  (FR-7):</strong> the default stack is 1 MiB and each level runs about six to
  eight frames (`walk_node` → `walk_element` → handler → `walk_children`).
  <em>Fix: run the 512-level test with the most stack-hungry element (nested
  lists, tables, or blockquotes) in both translation modes and the release
  build, or set the stack size explicitly.</em>
* <strong>R10-L3 — The fidelity panel and the converter parse the input
  differently (FR-5):</strong> the panel parses the original as a fragment, but
  `htmd` parses it as a document (`src/lib.rs:148`), so for full-page input such
  as the corpus page the doctype, `<html>`/`<body>` attributes, and `<head>`
  contents are handled differently on the two sides. <em>Fix: canonicalize
  from the tree `htmd` converted, or state this as a limitation.</em>
* <strong>R10-L4 — FR-2 gives each mode its own fidelity panel, but FR-5 limits
  the panel to HTML → Markdown mode.</strong> <em>Fix: editorial.</em>
* <strong>R10-L5 — Undo after a sample load does not restore the selected
  sample (FR-6):</strong> a later Reload Sample loads the sample the user just
  undid. <em>Fix: state whether this is intended.</em>

Still open from round 9: R9-L3 (the `strip = true` trade-off and the
release-build panic trigger), R9-L7 (no time ceiling for "neither hangs"),
R9-L8 (byte-unit conventions), R9-L9 (the Pure preset hides the fidelity check
by default), and round 8's browser-floor and editorial findings.

<strong>Total new findings: 9 (blocking 0, medium 4, low 5).</strong>

## Round 11 — open findings (awaiting processing)

Prompting **Big Pickle** (OpenCode agent, fresh context) with <em>"review
@demo/demo-spec.md"</em> produced the following round-11 output. The review
checked the post-round-10 specification (commit `52e8094`) against
`src/options.rs`, `src/lib.rs`, `src/element_handler/code.rs`,
`src/element_handler/element_util.rs`, `src/element_handler/pre.rs`,
`src/element_handler/span.rs`, `translation_approach.md`, and the pinned
`html5ever` 0.39.0, `markup5ever_rcdom` 0.39.0+unofficial, and
`pulldown-cmark` 0.13.4 sources in the Cargo registry. The runtime probe was
blocked by Windows Application Control (`os error 4551`), so behavior was
established by source inspection; `cargo build`, `cargo clippy`, and
`cargo test` were not run. No previously resolved finding is re-raised
unchanged; R11-M2 and R11-M5 address gaps left by the round-10 fixes, and
R10-M1 remains open under carried-over items.

#### What checks out

* The 12 `htmd` options, their variants, and their defaults still match
  `src/options.rs`, including `preformatted_code: false` and
  `translation_mode: Pure`.
* `HtmlToMarkdown::html_to_tree` followed by `tree_to_markdown` is exactly
  `convert` (`src/lib.rs:148–214`), so FR-7's parse-once depth check is
  implementable.
* `markup5ever_rcdom`'s serializer iterates `children` and panics on a
  `Document` node; `TraversalScope::IncludeNode` and `ChildrenOnly(None)`
  behave as FR-5 describes, and void elements serialize without a
  self-closing slash (`lib.rs:623–678`).
* `similar::TextDiff::from_words` splits on words and whitespace and needs no
  optional `similar` feature, so FR-5's tokenizer choice is implementable.
* The fixed `ENABLE_TABLES | ENABLE_MATH` round-trip options avoid the
  smart-punctuation false diffs FR-5 describes, and `htmd`'s span handler
  recognizes only its two math `class` values, so it adds no unrelated
  rewrites.
* The `skip_tags` empty-list panic is real
  (`src/element_handler/mod.rs:396–400`), and FR-4's "omit the call" rule
  avoids it.
* Round 10's trailing-newline rule for `<pre>` and its `skip_tags` debounce
  now match the source and the stated behavior.

#### Blocking issues

None.

#### Medium-severity issues
[fixed]
* ~~<strong>R11-M1 — FR-1 enables all 15 `pulldown-cmark` flags, contradicting FR-4 and the Demo Preset (FR-1 line 37; FR-4 lines 74–89 and 108; 8.5 line 260):</strong> FR-1 says Markdown → HTML uses "all 15  `Options::ENABLE_*` flags listed in FR-4", but FR-4 exposes only 7 and  fixes the other 8 off; Reset enables those 7 and disables the remaining 8.  Enabling all 15 would make `ENABLE_GFM`, the metadata-block flags,definition lists, superscript, subscript, and wikilinks change output with  no sidebar control, and the first conversion after a reset would not match FR-1. FR-4 also lists the 8 fixed-off flags as ordinary bullets in the same option list, so they read as controls. <em>Fix: say FR-1 uses the 7 exposed flags with their sidebar/preset values and that the other 8 are never enabled; move the 8 into a clearly labeled "fixed off" paragraph or drop them from the option list.</em>~~
[fixed]
*~~ <strong>R11-M2 — The preview still does not remove `<noscript>` or `<template>`, and the new template rationale is inaccurate (FR-1 lines 46–47; 8.6 line 286):</strong> The round-10 fix added `<noscript>` and  `<template>` to the explanation and the 8.6 test but not to the removal  list, which still contains only `<base>`, `<meta>`, `<link>`, and  `target`. `html5ever`'s tree builder defaults scripting_enabled: true` (`tree_builder/mod.rs:47–75`), so `<noscript>...</noscript>` is parsed as  one text node and the walk sees no `<meta>`/`<link>` inside it. The serializer also defaults `scripting_enabled: true`, and its `write_text`  writes `<noscript>` text raw (`serialize/mod.rs:214–227`), so  `<noscript><meta http-equiv="refresh" ...></noscript>` or  `<noscript><a target="_self" ...></noscript>` is emitted unescaped; the  sandboxed preview, which parses with scripting disabled, turns it back  into live markup, defeating the removal. Separately, `RcDom` stores  `<template>` contents in `template_contents`, not `children`; the FR-1 walk  has no rule for that field and the serializer ignores it  (`markup5ever_rcdom-0.39.0+unofficial/lib.rs:642–658`), so the contents are  dropped from the preview rather than becoming a live shadow root, while the  `<template>` element itself remains and 8.6 still requires its absence.  8.6 also misspells `<nonscript>`, so that check does not exercise the real  element. <em>Fix: remove `<noscript>` and `<template>` whole in the  preview walk (or parse with scripting disabled so noscript contents are  visible to the walk, but still remove both elements); correct the rationale  to say template contents are dropped by the serializer; spell `<noscript>`  in 8.6; add a `<noscript>` case carrying `<meta http-equiv="refresh">` and  a `<template shadowrootmode="open">` case with contents.</em>~~
* ~~<strong>R11-M3 — The persisted `cmark` object stores all 15 flags although
  FR-4 fixes 8 off (FR-8 line 165; FR-4 lines 74–89):</strong> The schema
  lists `yaml_style_metadata_blocks`, `pluses_delimited_metadata_blocks`,
  `old_footnotes`, `gfm`, `definition_list`, `superscript`, `subscript`, and
  `wikilinks`, and validation only checks that the values are booleans. A
  stored `gfm: true` is therefore accepted; if the app applies it, it enables
  an out-of-scope behavior with no sidebar control, and `old_footnotes: true`
  also sets the `ENABLE_FOOTNOTES` bit
  (`pulldown-cmark-0.13.4/src/lib.rs:715`), turning footnotes on even when
  `footnotes` is false. If the app instead ignores the 8, they are dead
  persisted state that can disagree with the UI and with Reset. Round 6
  recorded this object as holding the 7 exposed flags, so this is a
  regression in the schema. <em>Fix: store only the 7 exposed flags; or
  validate/force the other 8 to `false` on read and state that they are
  ignored. Update the schema list and the 8.6 invalid-payload tests.</em>~~
Adjudication: moot: bjones1's Nov 11 PR comment ("Allow all available pulldown-cmark flags") superseded it; all 15 flags are sidebar-exposed (FR-4) and persisted (FR-8), so none is fixed-off state.
[fixed]
* ~~<strong>R11-M4 — The fidelity round trip leaves the non-`translation_mode` `htmd` options unstated (FR-5 lines 114–116; 8.7 line 300):</strong> FR-5  fixes the `pulldown-cmark` options for the HTML → Markdown → HTML round trip but requires only `translation_mode: Faithful` from`htmd`; it does  not say whether `heading_style`, `link_style`, `preformatted_code`, and the  other sidebar options apply to the fidelity conversion or whether it uses  fixed crate defaults. 8.7 pins only `translation_mode: Faithful` and the  fixed round-trip options, so its golden expectations are not reproducible.  The choice is observable: `preformatted_code` changes inline-code handling  (`src/element_handler/code.rs:141–149`, `223–244`); with `true`, a newline  in a `<code>` element becomes a space and the span stays a code span, while  the default serializes the element as raw HTML. That difference survives  the round trip into the canonical HTML and changes the diff. <em>Fix:  state the exact `htmd` option set used for the fidelity conversion (all  sidebar options, or crate defaults plus `Faithful`) and pin every one in  8.7.</em>~~
[fixed]
~~* <strong>R11-M5 — The crash marker is removed before the user chooses a recovery action (FR-8 line 156; FR-2 line 59; 8.6 line 287):</strong> On startup with `htmd_cmark_live_crashed` present, FR-8 removes the marker, restores the buffers, and shows the recovery banner without converting. If  the user reloads again before choosing `Convert restored input` or `Clear buffers`, the marker is gone, so startup runs the initial conversion on the  crashing input, panics, and shows the overlay again; the recovery state  survives only one reload and the 8.6 test, which reloads once, passes  anyway. <em>Fix: keep the marker until the user clears the buffers or a  conversion of the restored input succeeds, and clear it in those actions  rather than at startup. Also state whether `Clear buffers clears the output and diff as well as the inputs.</em>~~
[fixed]
~~* <strong>R11-M6 — The 8.6 concurrency test introduces an async gap the execution model does not have (FR-2 line 55; 8.6 line 271):</strong> FR-2 says conversion runs synchronously on the main thread and a run's commit is one synchronous step; at most one run is in flight and one is queued. 8.6 slows a conversion with a "300 ms async delay between compute and commit" and has newer input arrive during the delay. A synchronous compute/commit pair cannot yield at that point; if the test hook yields, the run is asynchronous in a way FR-2 does not define. With one in-flight run and one queued run, the newer input cannot produce a "newer output" while the delayed run is still in flight — it can only be queued — so the stale run has nothing newer to overwrite. If the hook busy-waits instead, no input event can arrive. <em>Fix: put the delay at a defined async boundary (before dispatch, or on the iframe `load` path) and test the queue/sequence behavior there, or explicitly define a test-only suspension point in FR-2 and state that the stale run writes nothing and the queued run then commits.</em>~~

#### Low-severity / editorial

* ~~<strong>R11-L1 — 8.6's "no `target` attribute" is unsatisfiable with the
  injected `<base target="_blank">` (FR-1 line 47; 8.6 line 286):</strong>
  8.6 requires the injected `srcdoc` to contain no `target` attribute, but
  FR-1 deliberately injects `<base target="_blank">` after the CSP `<meta>`.
  If the test inspects the final `srcdoc`, as written, it always finds that
  target. <em>Fix: require no user-supplied `target` attribute and exclude the
  demo's own `<base>`, or inspect the content before the base is
  injected.</em>~~
Adjudication: resolved by the current 8.6 wording "no user-supplied `target` attribute (excluding the demo's own injected `<base target="_blank">`)" (demo-spec.md line 291).
[fixed]
~~* <strong>R11-L2 — "Use Output as Input" does not identify which output view it copies (FR-2 line 61; FR-3 lines 66–69; 8.5 line 263):</strong> The output pane has a Source tab (the generated Markdown or HTML string) and a Rendered tab (the live preview; in HTML → Markdown it is HTML re-rendered from the Markdown). "Copies the current output" and 8.5's "active output" do not say whether the copied value is the Source string or the Rendered DOM, nor whether the active tab matters. In HTML → Markdown, copying the  Rendered HTML into the opposite (Markdown → HTML) buffer is a different action from copying the Source Markdown into it. <em>Fix: state that the generated Source string is copied regardless of the active tab, or define the tab-dependent behavior.</em>~~
[fixed]
~~* <strong>R11-L3 — The `Err` path leaves the output and diff state undefined (FR-7 line 146):</strong> The future-proof wrapper's `Err` branch shows a  non-fatal banner that clears on the next input change, but it does not say  whether the output and diff are cleared, left stale, or marked invalid. If  they remain, "Use Output as Input" can copy the previous input's result  while the current input failed, and the banner can disappear on the next keystroke without a successful conversion. <em>Fix: on `Err`, clear the output/diff or mark them invalid and disable "Use Output as Input"; state  whether the banner survives until a successful conversion.</em>~~
[fixed]
~~* <strong>R11-L4 — Editorial defects, mostly from the round-10 edits:</strong> line 37 has a double space in "with  all"; line 106 reads "f the parsed  list"; line 118 runs `).*Accepted limitation:*` together without a space or line break and calls fragment-parser wrappers "Automated"; line 144 has "then then halts"; line 286 misspells `<nonscript>` and its example  `<noscript><a href="https://x.example" target="_self"> and <template shadowrootmode="open">` has unclosed tags; line 300 has "the CommonMark-basics sample is part" where the subject is plural. <em>Fix: correct the spelling, spacing, and agreement.</em>~~

Still open from earlier rounds: R10-M1 (a run that writes nothing or clears
the preview never fires the `load` that ends the in-flight state), R9-L3,
R9-L7, R9-L8 (unit convention), R9-L9, and round 8's browser-floor and
editorial findings. R10-M2, R10-M4, R10-L1, R10-L2, R10-L3, R10-L4, and
R10-L5 are treated as resolved by the round-10 edits; R10-M3 is only partly
resolved (see R11-M2).

**Total new findings: 10 (blocking 0, medium 6, low 4).**

Validation: specification and source inspection only. No source or
specification files were changed by this review. `cargo build`, `cargo
clippy -- --deny warnings`, and `cargo test` were not run; a runtime probe was
attempted but blocked by Windows Application Control (`os error 4551`).

## Round 12 — open findings (awaiting processing)

```text
You are a fresh-context LLM reviewer performing Round 12 of an iterative specification review for a university assignment. Earlier rounds (1–11) already ran; round 11's two open findings were adjudicated (struck as moot/resolved) in the current file state. Your job: find what is still wrong or weak in the spec.

## The document under review

Read `/home/danial/htmd/demo/demo-spec.md` in full (about 307 lines). It specifies a browser demo app ("pulldown-cmark / htmd live demo") that will be implemented as a standalone Rust crate at `demo/` depending on this repo's `htmd` crate.

## What to check it against

The spec must be implementable exactly as written, against the real crate and the real dependency APIs available locally:

1. The htmd crate in this repo (`/home/danial/htmd/src/`): `src/lib.rs` (`convert`, `HtmlToMarkdown`, `HtmlToMarkdown::builder()` → `.options()`, `.skip_tags()`, `.build()`, `.convert()`, `.html_to_tree()`, `.tree_to_markdown()`), `src/options.rs` (`Options` struct: 12 fields incl. `translation_mode` with variants `Pure`/`Faithful`, `ul_bullet_spacing`/`ol_number_spacing` numeric ranges), `src/element_handler/` (element handling, skip semantics).
2. Pinned dependency versions in `/home/danial/htmd/Cargo.toml` and the spec's own §6 dependency list: html5ever 0.39, markup5ever_rcdom 0.39 (check `html5ever`'s driver API: `parse_fragment`, `ParseOpts`, `RcDom`, and `markup5ever_rcdom`'s serializer support), pulldown-cmark 0.13.4 (check `Options` flag constants — the spec's §6/FR-4 lists 15 flags incl. `ENABLE_TABLES`, `ENABLE_FOOTNOTES`, `ENABLE_STRIKETHROUGH`, `ENABLE_TASKLISTS`, `ENABLE_SMART_PUNCTUATION`, `ENABLE_HEADING_ATTRIBUTES`, `ENABLE_MATH`, plus `ENABLE_…` older/YAML/definition tags — verify all 15 exist under those exact names in 0.13.x).
3. Internal consistency: every FR (FR-1…FR-8) vs §8 acceptance criteria (§8.5 sample flows, §8.6 concurrency/storage/preview cases, §8.7 canonicalization golden cases); terminology stable across sections; no requirement that contradicts another; no acceptance case that no FR satisfies; numbers consistent (byte limits, debounce timings, nesting depth, token caps, schema version).
4. Implementability in wasm: anything assuming APIs unavailable in a browser (no filesystem, no threads by default, no `Instant`), iterative-walk requirements where recursion would overflow, and the like.

Do NOT review style preferences, and do NOT re-litigate findings already struck through or marked resolved in `/home/danial/htmd/demo/review.md` (read its round 10/11 sections first to know what's already been found and fixed — repeating them wastes the round).

## Output

Append to `/home/danial/htmd/demo/review.md` a new section matching the established format of earlier rounds (read how rounds 10/11 are structured and mirror it exactly — heading level, subsections like "What checks out" / "Blocking issues" / "Medium-severity issues" / "Low-severity / editorial", finding IDs `R12-B#`/`R12-M#`/`R12-L#` with spec line references and concrete suggested fixes). Start the section with the review prompt you were given, quoted verbatim in a fenced block (the record must show the prompt used).

CRLF WARNING: the working tree is CRLF (`core.autocrlf=true`). Append your section with a method that preserves the file's existing line endings (e.g. Python `open(path, "a", newline="\r\n")` or equivalently careful appending); verify with `git diff --stat demo/review.md` that only your appended lines are new.

Then commit ONLY review.md: `git add demo/review.md && git commit -m "Spec review round 12: N findings (X blocking, Y medium, Z low)"` (fill in real numbers). Do NOT modify demo-spec.md or any other file. Do NOT push.

Finally, report back with ONLY (under 12 lines): the counts by severity, a one-line summary of each blocking/medium finding (ID + gist + line), whether any finding repeats a struck/resolved one (should be none), the commit SHA, and confirmation that demo-spec.md is untouched in your commit.
```

Reviewed by **GLM-5.3** (OpenCode agent, fresh context) with the round-12 prompt quoted
above. The review checked the post-round-11 specification (commit `0e354b4`, 307
lines) against `src/lib.rs`, `src/options.rs`, `src/element_handler/mod.rs`, the
root `Cargo.toml`, the pinned `html5ever` 0.39.0, `markup5ever_rcdom`
0.39.0+unofficial, and `pulldown-cmark` 0.13.4 sources in the local Cargo
registry, and the MDN browser-compat-data record for the Long Tasks API. The
`similar` crate is not in the local registry; FR-5's tokenizer claims rest on
round 11's registry verification and were not re-checked. Round 11's findings
are struck/resolved and are not re-raised; R12-M1 records that the R11-M2 fix
did not reach the normative text, which is a claim about the current file, not
a repeat of the round-11 finding.

#### What checks out

* All 15 `Options::ENABLE_*` names in FR-4/§6 exist verbatim in pulldown-cmark
  0.13.4 (`src/lib.rs:651–735`), and FR-1's "all 15 flags according to their
  current sidebar state" is consistent with FR-4's 15-checkbox sidebar and
  FR-8's 15-field `cmark` object after the R11-M3 adjudication.
* FR-4's 12 `htmd` options, variants, and defaults still match
  `src/options.rs`; the builder surface the spec relies on (`.options()`,
  `.skip_tags()`, `.build()`, `convert`, `html_to_tree`, `tree_to_markdown`)
  exists in `src/lib.rs`; the empty-`skip_tags` panic guard is still the
  `assert!` at `src/element_handler/mod.rs:396–400`, and the Faithful-mode
  `br_style` note matches `src/options.rs`.
* `html5ever` 0.39 exposes `parse_fragment`, `ParseOpts`, and
  `serialize`/`SerializeOpts`/`TraversalScope` as FR-1/FR-5 assume.
  `markup5ever_rcdom`'s serializer is itself iterative (a `VecDeque` worklist,
  `lib.rs:623–682`) and panics on the `Document` node exactly as FR-5 warns,
  so the 50,000-blockquote preview path has no recursive-overflow step left in
  it (the walk and the serializer are both worklist-based); `RcDom`'s `attrs`
  and `children` are `RefCell`s, so in-place attribute sorting, `target`
  removal, and child removal are implementable.
* The R10-M1 stall scenario is resolved by the current FR-2 wording: clearing
  the preview assigns `""` to `srcdoc` without waiting for a `load`, discarding
  an in-flight run immediately releases the slot, and non-matching `load`
  events are ignored.
* Numbers remain consistent across sections (2,000/2,001; 100,000/100,001;
  250/900 ms; 500 ms; 512/600; 5,000 tokens; 2,000,000; 1,572,864; 1500 ms;
  `version: 1`), and the §6 dependency list covers everything FR-5 needs.

#### Blocking issues

None.

#### Medium-severity issues

* ~~<strong>R12-M1 — the preview walk still removes only `<base>`, `<meta>`,
  `<link>`, and `target`; the `<noscript>`/`<template>` claims are unbacked and
  8.6's stripping test cannot pass (FR-1 lines 45–46; 8.6 line 291):</strong>~~
  the normative rule is "removes every `<base>`, `<meta>`, and `<link>` element
  (including their contents) and every `target` attribute", but the next
  sentence claims this also "prevents raw `<noscript>` text from being
  re-parsed as live markup" and "strips `<template>` shells", and 8.6 requires
  the srcdoc to contain no user-supplied `<noscript>` or `<template>` element.
  Neither claim follows from the rule: `TreeBuilderOpts::default()` sets
  `scripting_enabled: true` (`html5ever-0.39.0/src/tree_builder/mod.rs:71–80`),
  so `<noscript>` contents parse as one text node and no meta/link/base
  *elements* exist inside it for the walk to remove; `SerializeOpts::default()`
  is also `scripting_enabled: true` and `HtmlSerializer::write_text` then
  writes noscript text raw (`html5ever-0.39.0/src/serialize/mod.rs:42–50,
  214–227`), so 8.6's own payload
  `<noscript><meta http-equiv="refresh" …></noscript>` survives as raw text and
  is re-parsed as live markup by the scriptless sandbox. For `<template>`, the
  rcdom serializer iterates only `children` and ignores `template_contents`
  (`markup5ever_rcdom-0.39.0+unofficial/lib.rs:640–658`), so an empty
  `<template></template>` shell remains and 8.6 fails as written. The R11-M2
  fix updated the rationale and the 8.6 cases but not the removal list, so the
  gap persists. <em>Fix: add `<noscript>` and `<template>` to the walk's
  removal list (removed whole, like `<base>`/`<meta>`/`<link>`); optionally
  serialize the preview with `SerializeOpts { scripting_enabled: false, .. }`
  so any surviving noscript-shaped text is escaped; 8.6 already tests the
  right payloads.</em>
  **Status:** fixed (FR-1's removal list now includes `<noscript>` and
  `<template>` with their contents).
* ~~<strong>R12-M2 — the Long Tasks API does not exist in Firefox or Safari, so
  §6's jank monitoring and 8.3's long-task criterion cannot run on two of the
  three supported engines (§6 lines 211 and 225; 8.3 line 251; 8.2 line
  244):</strong>~~ MDN browser-compat-data records `PerformanceLongTaskTiming`
  as Chrome 58+/Edge only — Firefox `false` (bug 1348405), Safari `false`,
  non-standard, experimental. §6 mandates "Long-task jank is monitored with the
  Long Tasks API" while setting floors of Firefox ≥ 120 and Safari ≥ 17, and
  8.3's "no long task attributable to keystroke handling" checkbox lives in a
  section whose suites (8.2) must pass on Chromium, Firefox, and WebKit. As
  written the measurement is unimplementable on Firefox/WebKit, and a
  Playwright assertion registering a `longtask` observer would throw or never
  yield entries there. <em>Fix: scope the Long-Tasks-API monitoring (and any
  automated long-task assertion) to Chromium — e.g. "monitored with the Long
  Tasks API on Chromium; on Firefox/WebKit verified via Playwright tracing or
  manual observation" — or name a fallback metric available on all three
  engines.</em>
  **Status:** fixed (§6 and §8.3 scope Long-Tasks monitoring to Chromium, with
  Playwright tracing/trace inspection on Firefox and WebKit).

#### Low-severity / editorial

* ~~<strong>R12-L1 — the 2 MB output cap leaves the Source tab and "Use Output
  as Input" state undefined (§6 line 214; 8.3 line 249; FR-2 line
  60):</strong>~~ "output … is cleared instead of previewed" can mean the whole
  output is cleared (both tabs; "Use Output as Input" disabled via the
  empty-output rule) or only the preview is skipped while the Source tab still
  shows the oversized string (leaving the button enabled to copy 2 MB of
  HTML). The readings diverge in observable behavior. <em>Fix: state whether
  the Source text is kept or cleared with the preview, and mirror it in
  8.3.</em>
  **Status:** fixed (§6 and §8.3: the cap clears both output tabs and the
  fidelity panel and disables "Use Output as Input" by the empty-output rule).
* ~~<strong>R12-L2 — the 512-level nesting verification is scoped to the release
  wasm build, but no CI step runs the wasm tests in release (FR-7 line 146;
  8.6 line 274; Deliverable 3 line 231):</strong>~~ Deliverable 3 lists "the
  `wasm-bindgen-test` suite against headless Chromium" with no profile, while
  FR-7/8.6 pin the no-stack-overflow claim to "the release `wasm32` build". A
  debug-profile run uses much larger stack frames and can false-fail at 512
  levels; if the nesting tests are skipped instead, the release-scoped
  criterion is never exercised in CI. <em>Fix: state that the wasm-bindgen-test
  step (or at least the nesting cases) runs with `--release` in
  `demo-ci.yaml`.</em>
  **Status:** fixed (Deliverable 3: the wasm suite runs in both debug and
  release profiles, the FR-7 nesting cases with `--release`).
* ~~<strong>R12-L3 — the benchmark corpus is gated on but never delivered (§6
  line 211; 8.3 line 248; §7 lines 227–233):</strong>~~ 8.3 requires every
  document in `demo/bench/corpus/` to convert under 100 ms, and §6 loosely
  defines the corpus as "the scaled-up samples and a saved real-world page",
  but no deliverable commits or generates those files and "scaled-up" has no
  target sizes, so the 100 ms-per-document budget is not reproducible from the
  spec. <em>Fix: add the corpus (file list, sizes, and how the samples are
  scaled) to the Deliverables or to §6.</em>
  **Status:** fixed (§6: the corpus is now four named files plus a `README.md`
  manifest recording exact byte sizes).

~~Still open from earlier rounds: R9-L3 (in part), R9-L7, R9-L8 (unit
convention), R9-L9, and round 8's browser-floor and editorial findings.
R10-M1 is resolved by the current FR-2 wording (see above). All round-11
findings remain struck/resolved; R12-M1 is raised against the current removal
list, not as a re-opening of R11-M2.~~
**Status:** closed — see the Status lines above; nothing remains open from
rounds 1–12. (The round-8 browser-floor and editorial items: floors are
minimum supported versions satisfied by the §8.2 Playwright matrix images —
resolved; the editorial item was never recorded and is superseded by rounds
9–12 editorial passes.)

**Total new findings: 5 (blocking 0, medium 2, low 3).**

Validation: repository, Cargo-registry, and MDN browser-compat-data inspection
only; no runtime probe was run. No source or specification files were changed
by this review; `cargo build`, `cargo clippy`, and `cargo test` were not run.

**Round 12 resolution:** all 5 findings fixed in demo-spec.md (FR-1 removal
list; §6/§8.3 Long-Tasks scoping; §6/§8.3 output-cap clearing; Deliverable 3
release-profile wasm tests; §6 corpus definition) and the stale round-8/9
items closed by adjudication.

---

## Round 13 — open findings (awaiting processing)

```text
You are a fresh-context LLM reviewer performing Round 13 of an iterative specification review for a university assignment. Rounds 1–12 already ran; round 12's five findings were just resolved, and the stale round-8/9 items are closed. Your job: find what is still wrong or weak in the spec. If only trivial or contradictory findings remain, say so plainly — a trivial-findings round is a legitimate and useful outcome.

## The document under review

Read `/home/danial/htmd/demo/demo-spec.md` in full (~312 lines after the round-12 edits). It specifies a browser demo app ("pulldown-cmark / htmd live demo") to be implemented as a standalone Rust crate at `demo/` depending on this repo's `htmd` crate.

## What to check it against

The spec must be implementable exactly as written, against the real crate and the real dependency APIs available locally:

1. The htmd crate in this repo (`/home/danial/htmd/src/`): `src/lib.rs` (`convert`, `HtmlToMarkdown`, `HtmlToMarkdown::builder()` → `.options()`, `.skip_tags()`, `.build()`, `.convert()`, `.html_to_tree()`, `.tree_to_markdown()`), `src/options.rs` (`Options`: 12 fields incl. `translation_mode` variants `Pure`/`Faithful`, numeric ranges for spacing options), `src/element_handler/` (element handling, skip semantics, the empty-`skip_tags` panic guard).
2. Pinned versions in `/home/danial/htmd/Cargo.toml` and the spec's §6: html5ever 0.39, markup5ever_rcdom 0.39, pulldown-cmark 0.13.4, `similar`, Leptos csr, Trunk.
3. Internal consistency: every FR (FR-1…FR-8) vs §8 acceptance criteria (§8.5 sample flows, §8.6 concurrency/storage/preview, §8.7 canonicalization goldens); terminology stable; numbers consistent (byte limits, timings, nesting depth, token caps, schema version, corpus sizes); no requirement contradicting another; no acceptance case no FR satisfies. Pay particular attention to the NEWLY EDITED text from round 12 (FR-1 removal list; §6/§8.3 Long-Tasks scoping; §6/§8.3 output-cap clearing; Deliverable 3 release-profile wasm tests; §6 corpus definition; FR-7 demo-panic-test feature; §8.3 1,000 ms ceiling; FR-5 hidden-by-design note) — fresh text is where new contradictions breed.
4. Implementability in wasm: no filesystem/threads/`Instant` assumptions; iterative walks where recursion would overflow.

Do NOT review style preferences, and do NOT re-litigate findings already struck through or marked resolved in `/home/danial/htmd/demo/review.md` (read rounds 10–12 first to avoid repeats).

## Output

Append to `/home/danial/htmd/demo/review.md` a new section matching the established round format exactly (mirror rounds 12's structure: heading `## Round 13 — open findings (awaiting processing)`, the prompt quoted verbatim in a fenced block, reviewer identity line, "What checks out", "Blocking issues", "Medium-severity issues", "Low-severity / editorial", IDs `R13-B#`/`R13-M#`/`R13-L#` with spec line references and concrete suggested fixes).

CRLF WARNING: the working tree is CRLF (`core.autocrlf=true`). Append with line-ending preservation (e.g. Python `open(path, "a", newline="\r\n")`); verify with `git diff --stat demo/review.md` that only appended lines are new.

Then commit ONLY review.md: `git add demo/review.md && git commit -m "Spec review round 13: N findings (X blocking, Y medium, Z low)"` (real numbers). Do NOT modify demo-spec.md or any other file. Do NOT push.

Finally, report back with ONLY (under 12 lines): counts by severity, one-line summary of each blocking/medium finding (ID + gist + line), whether any finding repeats a struck/resolved one (should be none), your honest judgment whether findings are now trivial/contradictory, the commit SHA, and confirmation demo-spec.md is untouched.
```

Reviewed by **GLM-5.3** (OpenCode agent, fresh context) with the round-13 prompt quoted
above. The review checked the post-round-12 specification (308 lines, commit `2421acb`)
against `src/lib.rs`, `src/options.rs`, `src/element_handler/mod.rs`, the root
`Cargo.toml`, and the pinned `html5ever` 0.39.0, `markup5ever_rcdom`
0.39.0+unofficial, and `pulldown-cmark` 0.13.4 sources in the local Cargo registry.
All round-12 findings were re-verified as fixed in the current text before hunting
for new ones; `similar` is still absent from the local registry, so FR-5's
tokenizer claims rest on rounds 11–12's registry verification.

#### What checks out

* All five round-12 fixes are in the text and hold up: FR-1's removal list now
  includes `<noscript>` and `<template>` whole (line 46), which makes the preview
  path safe regardless of the parser's `scripting_enabled` default
  (`html5ever-0.39.0/src/tree_builder/mod.rs:75`,
  `serialize/mod.rs:45,224`); Long-Tasks monitoring is scoped to Chromium in both
  §6 (line 211) and §8.3 (line 252); the 2 MB output cap clears both tabs, the
  fidelity panel, and disables "Use Output as Input" consistently in §6 (line 214)
  and §8.3 (line 249); Deliverable 3 (line 231) runs the wasm test suite in debug
  and release with the FR-7 nesting cases `--release`; §6 (line 211) defines the
  corpus as four named files plus a `README.md` size manifest.
* FR-4's 12 `htmd` options, variants, defaults, and 1–4 spacing clamps still match
  `src/options.rs`; the Faithful-mode `br_style` note matches
  `src/options.rs:63–65`; the empty-`skip_tags` guard is still the `assert!` at
  `src/element_handler/mod.rs:400`, and FR-4's omit-the-call rule avoids it.
* All 15 `Options::ENABLE_*` names exist verbatim in pulldown-cmark 0.13.4
  (`src/lib.rs:651–735`); FR-4's 7-on/8-off preset and FR-8's 15-field `cmark`
  object agree with that set.
* `html_to_tree`/`tree_to_markdown` are public and compose `convert`
  (`src/lib.rs:148–214`), so FR-7's parse-once depth measurement remains
  implementable; `markup5ever_rcdom` still panics on `Document` serialization
  (`lib.rs:672`) exactly as FR-5 line 130 warns, and its drop is worklist-based.
* The wasm constraints hold: no filesystem, thread, or `Instant` assumptions
  remain; §6 line 213 explicitly bans `similar`'s deadline API for that reason;
  every tree walk the spec requires (preview strip, canonicalization, depth
  measurement) is specified as iterative.
* Numbers stay consistent where they interact: 2,000/2,001; 100,000/100,001;
  250/900 ms; 500 ms; 512/600; 5,000 tokens; 2,000,000; 1,572,864; 1500 ms;
  `version: 1` — the one remaining numeric conflict is the budget family raised
  as R13-M1 below.

#### Blocking issues

None.

#### Medium-severity issues

* ~~<strong>R13-M1 — the latency budgets contradict each other for adversarial
  inputs (§6 lines 212 and 215; §8.3 lines 250 and 252; §8.5 line 264):</strong>~~
  §6 line 212 exempts non-corpus inputs ≤ 100 kB from the 100 ms compute budget,
  and §8.3 line 250 allows the full pipeline for *any* input up to 1,000 ms. Yet
  §6 line 215 promises that inputs of 2,001–100,000 bytes render in "~350–400 ms
  after the last keystroke (250 ms debounce + ≤ 100 ms compute and render)", and
  §8.5 line 264 hardens that to "within 400 ms after the last keystroke" with no
  corpus carve-out — unsatisfiable for exactly the adversarial inputs (reference
  -link amplification, a diff near the 5,000-token cap) those same sections
  define. §8.3 line 252 then grants scheduled conversion runs only "the full
  100 ms compute budget" as the long-task exemption, so a legal ~1,000 ms
  forced-tick run dispatched during continuous typing of an adversarial document
  exceeds the exemption and fails the no-long-task criterion as written.
  <em>Fix: scope the 400 ms end-to-end claim (§6 line 215, §8.5 line 264) to
  inputs within the compute budget, and restate §8.3 line 252's exemption as the
  *applicable* ceiling — the 100 ms compute budget, or the 1,000 ms adversarial
  ceiling of line 250.</em>
  **Status:** fixed (§6 end-to-end latency and §8.3 compute budget scoped to
  corpus-scale documents; the §8.3 1,000 ms ceiling stated as the superseding
  adversarial bound; §8.5 aligned).
* ~~<strong>R13-M2 — no CI step or deliverable builds with `demo-panic-test`, but
  §8.6's crash-loop criterion needs one (FR-7 line 148; Deliverable 3 line 231;
  §8.6 line 293):</strong>~~ the only deliberate-panic trigger is the opt-in
  feature's toolbar button, "compiled out of shipped builds" (FR-7 line 148).
  Deliverable 3's workflow steps (clippy, native test, wasm-bindgen-test in both
  profiles, `trunk build --release`, size check, fuzz corpus, Playwright matrix,
  axe scan) never enable a cargo feature, and the Playwright matrix necessarily
  serves the feature-less build — so "after a deliberate panic on a persisted
  input" in §8.6 line 293 has no trigger mechanism in any artifact the spec
  builds. FR-7's "verified with deliberate panics in debug and release builds"
  has the same gap: no step runs those panics with the feature on.
  <em>Fix: extend Deliverable 3 — the Playwright crash-recovery cases run against
  a build with `--features demo-panic-test`, and the deliberate-panic hook
  verification runs in the wasm-bindgen-test step with the feature enabled in
  both profiles; the size check and axe scan still use the shipped feature-less
  build.</em>
  **Status:** fixed (Deliverable 3: the §8.6 crash-recovery cases run against a
  build with `demo-panic-test` enabled, never enabled in the shipped Pages
  build).

#### Low-severity / editorial

* ~~<strong>R13-L1 — "the preset mirrors library defaults" contradicts FR-4
  (FR-5 line 116 vs FR-4 line 110):</strong>~~ FR-4 explicitly says the pulldown
  -cmark half of the demo preset "deliberately differs from the crate default
  `Options::empty()`" (7 flags on), so the round-12 sentence justifying the
  hidden fidelity panel overclaims: only the `htmd` half mirrors crate defaults
  (`translation_mode: Pure`). <em>Fix: reword to "the preset's `htmd` options
  mirror the crate defaults, so `translation_mode` is `Pure`".</em>
  **Status:** fixed (FR-5 now reads “the preset's `htmd` options mirror the
  crate defaults”).
* ~~<strong>R13-L2 — the panic overlay must sit outside the inert root, unstated
  (FR-7 line 148):</strong>~~ the hook "permanently disables all interactive
  controls … by setting `inert` on the application root" and then "moves focus
  to the overlay". Elements inside an `inert` subtree are non-focusable, so if
  the static overlay element were inside the application root, the focus move
  fails. The spec never states the overlay lives outside the inert root.
  <em>Fix: one clause — "the overlay element is a sibling of (outside) the
  application root that receives `inert`".</em>
  **Status:** fixed (FR-7: the overlay lives outside the inert application root
  and receives the moved focus).

**Total new findings: 4 (blocking 0, medium 2, low 2).**

Reviewer's judgment: the findings are now near-trivial. R13-M1 is a real but
narrow inconsistency between performance criteria that only diverge for
adversarial inputs; R13-M2 and the two lows are one-sentence fixes. No
structural, correctness, or implementability problem remains in the spec.

Validation: repository and Cargo-registry inspection only; no runtime probe was
run. No source or specification files were changed by this review; `cargo
build`, `cargo clippy`, and `cargo test` were not run.

**Round 13 resolution:** all 4 findings fixed in demo-spec.md (§6/§8.3/§8.5
  timing scoping; Deliverable 3 demo-panic-test CI build; FR-5 preset wording;
  FR-7 overlay placement).


---

## Round 14 — open findings (awaiting processing)

```text
You are a fresh-context LLM reviewer performing Round 14 of an iterative specification review for a university assignment. Rounds 1–13 already ran; round 13's four findings were just resolved (timing scoping, demo-panic-test CI build, FR-5 preset wording, FR-7 overlay placement). Round 13's reviewer judged the remaining issues near-trivial. Your job: verify the round-13 fixes landed correctly and find anything still wrong. If only trivial or contradictory findings remain, say so plainly — that is the expected outcome and a legitimate result.

## The document under review

Read `/home/danial/htmd/demo/demo-spec.md` in full (~314 lines). It specifies a browser demo app ("pulldown-cmark / htmd live demo") to be implemented as a standalone Rust crate at `demo/` depending on this repo's `htmd` crate.

## What to check it against

The spec must be implementable exactly as written, against the real crate and dependency APIs available locally:

1. The htmd crate (`/home/danial/htmd/src/`): `src/lib.rs` (`convert`, `HtmlToMarkdown`, builder `.options()`/`.skip_tags()`/`.build()`, `.convert()`, `.html_to_tree()`, `.tree_to_markdown()`), `src/options.rs` (12 fields, `Pure`/`Faithful`, spacing ranges), `src/element_handler/` (skip semantics, empty-`skip_tags` panic guard).
2. Pinned versions in `/home/danial/htmd/Cargo.toml` and §6: html5ever 0.39, markup5ever_rcdom 0.39, pulldown-cmark 0.13.4, `similar`, Leptos csr, Trunk.
3. Internal consistency: FR-1…FR-8 vs §8 acceptance criteria; numbers consistent; no contradictions — especially around the round-12/13 edited text (removal list, Long-Tasks scoping, output-cap clearing, corpus definition, 400 ms/1,000 ms timing scoping, demo-panic-test feature, overlay placement).
4. Implementability in wasm.

Do NOT review style preferences. Do NOT re-litigate struck/resolved findings in `/home/danial/htmd/demo/review.md` (read rounds 10–13 first).

## Output

Append to `/home/danial/htmd/demo/review.md` a section `## Round 14 — open findings (awaiting processing)` mirroring the established round format exactly (prompt quoted verbatim in a fenced block, reviewer identity, "What checks out", "Blocking issues", "Medium-severity issues", "Low-severity / editorial", IDs `R14-B#`/`R14-M#`/`R14-L#` with line refs and concrete fixes).

CRLF WARNING: working tree is CRLF — append with line-ending preservation (e.g. Python `open(path, "a", newline="\r\n")`); verify `git diff --stat demo/review.md` shows only appended lines.

Then commit ONLY review.md: `git add demo/review.md && git commit -m "Spec review round 14: N findings (X blocking, Y medium, Z low)"` (real numbers). Do NOT modify demo-spec.md. Do NOT push.

Report back ONLY (under 12 lines): counts by severity, one-line summary of any blocking/medium findings, confirmation the four round-13 fixes landed correctly, whether any finding repeats a struck/resolved one, your explicit judgment whether all remaining findings are trivial/contradictory, the commit SHA, and confirmation demo-spec.md is untouched.
```

Reviewed by **GLM-5.3** (OpenCode agent, fresh context) with the round-14 prompt quoted
above. The review re-verified all four round-13 fixes in the current text (308 lines,
commit `ff027d1`) before hunting for new ones, against `src/lib.rs`, `src/options.rs`,
the `src/element_handler/` inventory and its panic guard, and the root `Cargo.toml`.
`similar` is not a dependency of this repository, so FR-5's `from_words` claims rest on
rounds 11–13's Cargo-registry verification.

#### What checks out

* All four round-13 fixes landed correctly: the 400 ms end-to-end budget is now scoped
  to corpus-scale documents with adversarial inputs explicitly bounded by the
  1,000 ms ceiling, which §8.3 states supersedes the 400/100 ms budgets, and §8.3's
  long-task exemption grants scheduled runs "the full applicable ceiling" (§6 line
  215; §8.3 lines 250 and 252; §8.5 line 264); Deliverable 3 (line 231) runs the §8.6
  crash-recovery cases against a `demo-panic-test` build that is never enabled in the
  shipped Pages build; FR-5 (line 116) now reads "the preset's `htmd` options mirror
  the crate defaults"; FR-7 (line 148) places the overlay outside the inert
  application root so it stays operable and focusable.
* FR-4's 12 options, variants, defaults, and 1–4 spacing clamps still match
  `src/options.rs` (fields 4–22, defaults 27–40, clamp docs 12–19); the Faithful-mode
  `br_style` note matches `src/options.rs:63–65`.
* The builder surface FR-4/FR-7 rely on exists as described: `.options()`
  (`src/lib.rs:241`), `.skip_tags()` (248), `.build()` (285), `convert` (212),
  `html_to_tree` (148, `parse_document`), and `tree_to_markdown` (170, returns
  `String`); the empty-slice panic guard is the `assert!(!tags.is_empty())` at
  `src/element_handler/mod.rs:400`, which FR-4's omit-the-call rule avoids.
* `Cargo.toml` matches §6's pins: htmd 0.5.5, html5ever 0.39, markup5ever_rcdom 0.39,
  pulldown-cmark 0.13.4. The handler inventory (no `del`/`s`, checkbox `input`,
  `sup`/`sub`, or `dl`/`dt`/`dd` handlers; tables in `table*.rs`/`td_th.rs`/`tr.rs`,
  math in `span.rs`) supports FR-5 line 120's "the only extensions htmd targets"
  claim, so the fixed `ENABLE_TABLES | ENABLE_MATH` round-trip set remains sound.
* Numbers stay consistent everywhere they interact (2,000/2,001; 100,000/100,001;
  250/900 ms; 500 ms; 512/600; 5,000 tokens; 2,000,000; 1,572,864; 1500 ms; 200
  characters; `version: 1`), and the FR-8 schema's `cmark` (15 fields) and `htmd`
  (12 fields) objects match FR-4's sidebar exactly.
* Wasm constraints hold: no filesystem, thread, or `Instant` assumptions remain
  (§6 line 213 bans `similar`'s deadline for exactly that reason), and every required
  walk (preview strip, canonicalization, depth measurement) is specified iterative.

#### Blocking issues

None.

#### Medium-severity issues

None.

#### Low-severity / editorial

* <strong>R14-L1 — FR-7's "verified with deliberate panics in debug and release
  builds" still has no wired verification in either profile (FR-7 line 148 vs
  Deliverable 3 line 231):</strong> R13-M2's fix wired the §8.6 crash-recovery
  Playwright cases to a `demo-panic-test` build, but that build's profile is
  unspecified, and no other CI step enables the feature — the wasm-bindgen-test step
  runs both profiles feature-less. The second clause of R13-M2's finding (the
  hook-before-abort verification "in debug and release builds") therefore still has
  no CI home. This is the unapplied half of R13-M2's suggested fix, not a re-opening
  of the half that landed. <em>Fix: one clause in Deliverable 3 — the deliberate-panic
  hook verification runs in the wasm-bindgen-test step with
  `--features demo-panic-test` in both debug and release profiles (or the Playwright
  crash-recovery build runs in both profiles).</em>
* <strong>R14-L2 — "focus … is restored after errors resolve" is vacuous (§6 line
  223):</strong> the only error path that moves focus is the panic overlay, which is
  terminal — the application root is `inert`, all controls are permanently disabled,
  and recovery requires a manual reload (FR-7 lines 148–149). Non-fatal banners
  (size/nesting limits, the wrapper `Err`, quota, multi-tab) never take focus, so no
  focus-moving error ever "resolves"; the clause cannot be satisfied, falsified, or
  tested. <em>Fix: drop the clause, or define the restoring path (e.g., "non-fatal
  errors never move focus; the overlay path ends in a reload").</em>

**Total new findings: 2 (blocking 0, medium 0, low 2).**

Reviewer's judgment: only trivial findings remain, which is the expected outcome at
this stage. Both lows are one-clause fixes — R14-L1 completes a fix whose primary
half already landed, and R14-L2 deletes an untestable clause. No structural,
correctness, implementability, wasm-feasibility, or numeric-consistency problem
remains in the specification.

Validation: repository and source inspection only; no runtime probe was run. No
source or specification files were changed by this review; `cargo build`, `cargo
clippy`, and `cargo test` were not run (no code under review changed).
