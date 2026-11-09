# htmd / pulldown-cmark live demo — specification

## Team contributions

- **k_muzanbekova:** Initiated the repository fork, created `demo/demo-spec.md` and `demo/review.md`, added teammates to the repository to enable collaboration, and processed review findings B1–B6 into the specification.
- **d_baitakov:** Completed the review file from finding B7 through L10, applied the resolved findings to the specification, added the required submission artifacts (contributions, review history, verification output), and ran the round-5 LLM review cycle.
- **jhg246:** (to be completed)
- **nu_aimagambetov:** (to be completed)

## Review history

| Round | Findings | Main changes made |
| :---: | :---: | :--- |
| **1** | 18 | Narrowed the scope to strict one-way conversion modes, established a compressed WASM asset budget, and defined initial verifiable acceptance criteria. |
| **2** | 26 | Resolved diff pipeline contradictions, added the missing UI-sketch section, formalized the localStorage validation schema, and eliminated runtime panics on empty tag inputs. |
| **3** | 12 | Enforced total preview isolation using a sandboxed `<iframe>` with a strict Content Security Policy to eliminate XSS risks and hidden third-party network requests. |
| **4** | 38 | Verified all settings against `htmd` 0.5.5 and `pulldown-cmark` 0.13.4, expanded the option mapping, resolved reset semantics, and added deep DOM nesting bounds. |
| **5** | 12 | Made performance and size budgets measurable (artifact, command, profile), defined the demo build contract and CI workflow, normalized `skip_tags`, hardened the iframe/panic-hook/debounce contracts, and renumbered sections; the round-5 review then reconciled long-task vs compute budgets, unified storage version handling, added the persisted-state schema, moved the nesting guard to a streaming pre-scan, and pinned the 2 KB threshold. |

### Round context & analysis

1. **Round 1 (conceptual scope):** replaced unstable two-way sync ideas with a deterministic "Use Output as Input" feature, split per-mode state variables, and introduced performance budgets evaluated via the browser's Performance API.
2. **Round 2 (pipeline logic):** designed a word-level text diff built strictly on canonicalized HTML fragments using `html5ever`, dropped unreachable library error wrappers, and fixed concurrency timers to ensure typing does not wipe pending changes.
3. **Round 3 (browser security & sandboxing):** blocked raw HTML passthrough vulnerabilities, isolated event handling vectors (`onclick`, `onerror`), explicitly mapped the `pulldown-cmark` bitflags, and unified the 100 KB limit using UTF-8 byte arrays.
4. **Round 4 (crate alignment & edge cases):** expanded the UI flags to match the 15 bitflags available in `pulldown-cmark` 0.13.4, protected the recursive WASM stack walker against deeply nested elements, and separated "Reset Options" from "Reset Sample" logic.
5. **Round 5 (measurability & buildout):** tied every budget to an artifact and CI command, defined the standalone-crate build contract, added the persisted-state schema and streaming nesting guard, and reconciled the latency criteria (contributed by d_baitakov).

## Key decisions

- Diff pipeline: HTML mode only, `Faithful` translation mode only, canonicalized fragments, `similar::TextDiff::from_words`, automatic (rounds 2–4).
- Storage: single JSON object, `version: 1`, flush-before-write controller (rounds 2–4).
- Canonicalization: `html5ever` fragment parsing, raw-text exemption, foreign-content case preservation (round 4).
- Performance: 100 ms compute-only budget via `performance.mark`/`measure`; size budget = gzipped `.wasm` artifact, checked in CI (round 5).
- Cross-browser: `wasm-bindgen-test` on Chromium; Playwright matrix for Chromium/Firefox/WebKit; Safari manual checklist (round 5).
- Build layout: `demo/` is a standalone crate with a `path` dependency on the root `htmd` crate (round 5).

## 1. Purpose & Target Audience

An interactive, client-side web tool to convert CommonMark (plus selected extensions) Markdown to HTML (via `pulldown-cmark`) and back (via `htmd`). It enables Rust developers and library maintainers to test edge cases, tweak crate options, and inspect translation fidelity in real time within the browser.

## 2. Goals

*   **Convert input automatically** as the user types, enforcing a strict 100 KB ceiling.
*   **Support strict one-way conversion** per mode (Markdown → HTML or HTML → Markdown).
*   **Expose core configuration flags** with a global "Reset Options to Demo Preset" action.
*   **Persist the active workspace** state in localStorage to prevent data loss.
*   **Provide a clear pipeline diff** (HTML → Markdown → HTML) with intermediate views.
*   **Surface conversion failures** (permissive-parse limits, panics, WASM traps) gracefully in the UI without crashing the runtime.

## 3. Non-goals

- User accounts, cloud storage, databases, or shareable link generation.
- Server-side rendering (SSR) or remote backend conversion.
- Advanced rich-text WYSIWYG editor features.
- Dark theme: a single light theme ships; theming is out of scope.

## 4. Functional requirements

### FR-1 — Conversion Modes
- **Markdown → HTML:** Uses `pulldown-cmark` with the seven exposed `Options::ENABLE_*` flags listed in FR-4; all other flags are fixed off and out of scope.
- **HTML → Markdown:** Uses `htmd`. Malformed HTML is handled permissively via `html5ever` without crashing.
- **Preview isolation:** Every rendered preview lives in an `<iframe>` with all of the following:
    * `srcdoc` assigned through the DOM property, never string-interpolated into surrounding HTML;
    * `sandbox=""` — no tokens: no scripts, no same-origin, no forms, no popups, no top navigation;
    * a CSP delivered inside the `srcdoc` document via `<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; style-src 'unsafe-inline';">` (`srcdoc` documents cannot receive response headers);
    * a `title` attribute (accessible name) and `referrerpolicy="no-referrer"`;
    * `img-src data:` deliberately blocks all remote images as a privacy choice; inline data-URI images still render;
    * `<base>` and `<meta http-equiv="refresh">` elements are stripped from preview content before injection as defense in depth.

### FR-2 — Live Adaptive Debounce
Conversion triggers automatically on input changes:
- **Inputs ≤ 2 KB** (2,000 UTF-8 bytes): no debounce; conversion is scheduled on the next animation frame after the keystroke.
- **Inputs > 2 KB and ≤ 100 KB** (2,001–100,000 bytes): debounced with a 250 ms delay after typing stops.
- **Typing Lock Protection:** while typing continues in the debounced range, a conversion runs at most once every 900 ms on the latest input. The forced tick does not apply to the ≤ 2 KB immediate path.
- **Concurrency:** latest-input-wins. At most one conversion runs at a time and at most one is queued; each run carries a sequence number and stale results are discarded. A forced tick that arrives while a conversion is in flight coalesces into the single queued run.
- **Input Isolation:** a separate text buffer per conversion mode. Switching modes never auto-converts or overwrites the opposite buffer; only the explicit "Use Output as Input" and sample-loading actions may overwrite a buffer.
- **State Preservation:** sidebar options, active tabs, and scroll positions remain unaffected by mode switches.
- **Explicit Action:** "Use Output as Input" copies the current output into the opposite mode's buffer, switches to that mode, and immediately converts. If the target buffer was non-empty, an "Undo" toast restores it and remains available until the next user action.

### FR-3 — Output Views
The output pane features two tabs (`Rendered` and `Source`) implementing the ARIA tabs pattern (see accessibility):

| Conversion Mode | **Source** Tab | **Rendered** Tab |
| :--- | :--- | :--- |
| **Markdown → HTML** | The raw, generated HTML string as plain text. | The generated HTML rendered inside a sandboxed `<iframe>` per FR-1. |
| **HTML → Markdown** | The raw, generated Markdown markup text. | The Markdown re-rendered back into HTML via `pulldown-cmark`, inside a sandboxed `<iframe>` per FR-1. |

The Source tab always renders via `textContent` (escaped plain text), never `innerHTML`.

### FR-4 — Configuration & Reset
- **`pulldown-cmark` options** (7 of the 15 flags in 0.13.4 are exposed; the other 8 are fixed off and out of scope):
    * `ENABLE_TABLES` — Tables
    * `ENABLE_FOOTNOTES` — Footnotes
    * `ENABLE_STRIKETHROUGH` — Strikethrough
    * `ENABLE_TASKLISTS` — Task lists
    * `ENABLE_SMART_PUNCTUATION` — Smart punctuation
    * `ENABLE_HEADING_ATTRIBUTES` — Heading attributes
    * `ENABLE_MATH` — Math
- **`htmd` core options** (verified against `htmd` 0.5.5; defaults are the crate defaults):
    *   `heading_style`: `Atx` (Default), `Setex`
    *   `hr_style`: `Asterisks` (Default), `Dashes`, `Underscores`
    *   `br_style`: `TwoSpaces` (Default), `Backslash`
    *   `link_style`: `Inlined` (Default), `InlinedPreferAutolinks`, `Referenced`
    *   `link_reference_style`: `Full` (Default), `Collapsed`, `Shortcut`
    *   `code_block_style`: `Fenced` (Default), `Indented`
    *   `code_block_fence`: `Backticks` (Default), `Tildes`
    *   `bullet_list_marker`: `Asterisk` (Default), `Dash`
    *   `ul_bullet_spacing`: 1–4 (Default: 3)
    *   `ol_number_spacing`: 1–4 (Default: 2)
    *   `preformatted_code`: `bool` (Default: `false`)
    *   `translation_mode`: `Pure` (Default), `Faithful`

    The crate options are independent; no combination is invalid. Note: the crate ignores `br_style` in `Faithful` mode (it emits a literal `<br>` there), so the demo disables the `br_style` control while Faithful is active.
- **`htmd` tag skipping:** a dedicated comma-separated text field maps to `HtmlToMarkdownBuilder::skip_tags`. The field is parsed by splitting on commas, trimming whitespace, lowercasing, then dropping empty parts, duplicates, and entries not matching `[a-z][a-z0-9-]*`; the field is capped at 200 characters. An empty parsed result builds the converter with no skip handler — it never calls `skip_tags` with an empty vector (which panics). Mixed-case input such as `IMG` is accepted and lowercased, because `html5ever` supplies lowercase tag names.
- **Reset action:** the button is labeled "Reset Options to Demo Preset" and restores only the sidebar configuration:
    * `pulldown-cmark`: all 7 exposed flags enabled — the demo preset deliberately differs from the crate default `Options::empty()`;
    * `htmd`: `translation_mode` set to `Pure`, every other option to its crate default listed above;
    * `skip_tags` cleared;
    followed by immediate reconversion. It does not clear or change input text, active mode, active tabs, scroll positions, loaded samples, or error banners.

### FR-5 — Pipeline Fidelity Check (HTML Mode Only)
The pipeline diff (HTML → Markdown → HTML) runs automatically on input changes exclusively when `translation_mode` is set to `Faithful`. In `Pure` mode the fidelity panel is hidden with an explanatory note, because intentional attribute loss would reduce the diff to noise. To prevent false-positive changes during validation, the engine must execute a word-level text diff strictly on canonicalized HTML strings:

*   **Fragment Parsing:** Parse both the original HTML input and the round-tripped HTML output as fragments via `html5ever::parse_fragment` with a `<body>` context using `markup5ever_rcdom::RcDom`. Automated `<html>` or `<head>` wrappers are strictly prohibited.
*   **DOM Normalization:**
    *   **Case Folding:** Convert tag names and attribute keys to lowercase for standard HTML elements only. Foreign content namespaces (SVG and MathML) must preserve their native case-sensitivity (e.g., `viewBox`).
    *   **Attribute Standardization:** Sort all element attributes alphabetically by `namespace` + `local_name`. For `class` attributes, collapse duplicate whitespaces and sort class tokens lexicographically.
    *   **Whitespace Collapsing:** Collapse consecutive whitespaces and line breaks into a single space. Whitespace structures inside `<pre>`, `<code>`, `<textarea>`, `<script>`, and `<style>` blocks must remain untouched. Drop all whitespace-only text nodes located between block-level components.
    *   **Nodes & Entities:** Discard all `Comment`, `Doctype`, `<template>`, and `CDATA` sections. Normalize non-breaking spaces (`&nbsp;` / U+00A0) into standard spaces (U+0020), except when nested inside raw-text elements.
*   **Deterministic Serialization:** Serialize both normalized DOM trees back into clean strings using `html5ever::serialize` with `TraversalScope::IncludeNode`. Output all void elements in lowercase without any trailing self-closing slashes (e.g., `<br>`).
*   **Word-Level Diff:** Run the `similar::TextDiff::from_words` engine (version 2.x) automatically on the canonicalized strings to generate the visual UI diff. The diff is diagnostic — it reports textual divergence, not proof of semantic equivalence.

### FR-6 — Sample Documents
Three samples with fixed content shipped in the binary. Each sample is bound to its intended mode(s); clicking a sample loads it into that mode's buffer, switching the active mode if needed (same "Undo" affordance):
- **CommonMark basics** (both modes): headings, emphasis, lists, links, and fenced code — expected to round-trip losslessly in both modes.
- **Inline HTML** (HTML → Markdown mode): inline spans with attributes and raw HTML blocks — validates block preservation; in `Pure` mode attribute loss is expected and visible; in `Faithful` mode attributes are preserved.
- **Math** (both modes): `$` and `$$` formulas — round-trip when `ENABLE_MATH` is on; render as literal `$` text without warnings when it is off.

Sample actions:
- Clicking a sample replaces the current mode's buffer with the sample content and converts immediately, with the same "Undo" affordance as "Use Output as Input".
- **Reload Sample** restores the currently selected sample's original content (or loads the default sample if none is selected) into the current buffer. It never changes options, mode, or tabs, and is independent of "Reset Options to Demo Preset".

### FR-7 — Error Boundaries & Limits
- **Size limit:** the ceiling is **100,000 UTF-8 bytes** of the input string (measured via `TextEncoder`). Inputs **strictly larger** than the ceiling halt conversion, clear the output and diff, and show the `aria-live="assertive"` banner exactly once per transition into the limit: *"Input exceeds the 100 KB maximum limit for client-side processing."* The oversized text stays in the editor and is persisted; processing resumes automatically once the input is at or below the ceiling.
- **Nesting limit:** HTML → Markdown input whose element nesting exceeds **512 levels** is rejected before conversion with the same non-fatal banner mechanism (*"Input exceeds the supported nesting depth."*). The depth is measured by an iterative scan of the raw input (counting unmatched open tags) **before** any DOM is constructed, so a deep tree is never built or torn down — `RcDom`'s recursive `Drop` would trap the wasm stack on the very input the guard exists to reject. Markdown → HTML input is exempt: `pulldown-cmark`'s parser is iterative.
- **Malformed HTML:** input with syntax errors (e.g., unclosed tags) is not a runtime failure. The engine leverages `html5ever`'s permissive parsing to build a best-effort DOM tree without throwing exceptions; `<script>`/`<style>` contents are treated as raw text.
- **Rust panics:** a single custom panic hook is installed once at startup. It first delegates to `console_error_panic_hook` (only one global hook exists, so it must chain rather than install a second), is null-safe and idempotent (guards element lookups; runs its body once), then un-hides the static fallback overlay element in `index.html` (*"Critical Application Error. Please reload the page."*), permanently disables **all** interactive controls (mode switch, tabs, samples, options, reset, "Use Output as Input"), moves focus to the overlay (`role="alertdialog"`), and logs the trace to the console. The release profile uses `panic = "abort"`; the hook runs before the abort trap and is verified with deliberate panics in debug and release builds.
- **WASM traps & OOM:** low-level traps and out-of-memory errors bypass the Rust panic hook and are intercepted via global `window` `error` and `unhandledrejection` listeners in `index.html`, which show the same static crash overlay. Recovery requires a manual page reload.
- **Future-proof wrapper:** the internal converter wraps execution in a `Result` for forward compatibility with fallible future `htmd` APIs; an `Err` displays a non-fatal banner that clears on the next input change.

### FR-8 — Storage & Persistence
- **Storage key:** use exactly `htmd_cmark_live_workspace`.
- **Safe reads (validation):**
    * Wrap `localStorage.getItem` and `JSON.parse` in `try/catch`.
    * Fall back to in-memory state without crashing if storage is blocked (Incognito).
    * Validate `version == 1`, data types, and enum variants for all fields.
    * Wipe storage and reset to Demo Default Preset with empty inputs if the version is not 1, the JSON is corrupt, fields are invalid, or spacing is outside the 1–4 range.
- **Persisted-state schema:** the stored JSON object contains exactly these fields (unknown fields are ignored on read):
    * `version`: `1` (global)
    * `mode`: `"md-to-html"` or `"html-to-md"` (global)
    * `input_md_to_html`: string (per-mode buffer)
    * `input_html_to_md`: string (per-mode buffer)
    * `active_tab`: `"rendered"` or `"source"` (global)
    * `scroll_md_to_html`, `scroll_html_to_md`: input scroll offset per mode, number ≥ 0, persisted across reloads, default `0`
    * `cmark`: object of the 7 FR-4 booleans (`tables`, `footnotes`, `strikethrough`, `task_lists`, `smart_punctuation`, `heading_attributes`, `math`)
    * `htmd`: object with the FR-4 enum options (`heading_style`, `hr_style`, `br_style`, `link_style`, `link_reference_style`, `code_block_style`, `code_block_fence`, `bullet_list_marker`, `translation_mode`), the integers `ul_bullet_spacing` and `ol_number_spacing` (1–4), and `preformatted_code` (bool)
    * `skip_tags`: string (raw field text)
- **Unified writes (controller):**
    * **State serialization:** save the persisted-state schema above into a single atomic JSON object.
    * **Debounced text save:** queue a 500 ms debounced write upon text entry.
    * **Atomic flush-before-write:** any UI interaction (mode toggle, option switch, tab change) must immediately flush the latest text buffers into the state object, cancel the pending text timer, and execute a synchronous write.
    * **Guaranteed flush:** force-flush all unsaved buffers on `pagehide` and `visibilitychange` events.
    * **Quota guard:** wrap all writes in `try/catch` to ignore `QuotaExceededError`.

## 5. UI sketch

```
+---------------------------------------------------------------------+
| htmd <-> pulldown-cmark demo              active mode: [MD -> HTML]  |
| [Markdown -> HTML] [HTML -> Markdown]                              |
|                [Options] [Reset Options to Demo Preset]             |
+------------------------------+--------------------------------------+
| Input                        | Output                               |
| (textarea, active buffer)    | [Rendered] [Source]                  |
| Samples: [Basics][HTML][Math]|  (sandboxed iframe, or plain text)   |
|                              |                                      |
| [Use Output as Input]        |                                      |
| [Reload Sample]              |                                      |
+------------------------------+--------------------------------------+
| Pipeline Fidelity Check (Faithful mode only, automatic)             |
| Original (canonical) | Round-tripped (canonical) | Word diff        |
+---------------------------------------------------------------------+
| [error banner region, aria-live="assertive", hidden when empty]     |
+---------------------------------------------------------------------+
```

- Responsive: panes stack vertically on narrow screens.
- Keyboard accessible: mode toggle and tabs follow the ARIA patterns in section 6; all controls have labels.
- Single light theme; dark theme is a non-goal.

## 6. Non-functional requirements

### Performance & WASM
- **Compute budget:** one full conversion — including canonicalization and diff generation when active — completes in under 100 ms for documents up to 100 KB, measured with `performance.mark`/`performance.measure` around the conversion call. Long-task jank is monitored with the Long Tasks API; Total Blocking Time is a load-time metric and is not used for conversion latency.
- **End-to-end latency:** inputs ≤ 2 KB (2,000 bytes) render within 100 ms of the keystroke; inputs of 2,001–100,000 bytes render within ~350–400 ms after the last keystroke (250 ms debounce + ≤ 100 ms compute and render).
- **Cold start:** initialization completes within 1500 ms from navigation start to conversion-ready, on a mid-range desktop CPU (4-core laptop, 2020 or newer), without network or CPU throttling, warm cache.
- **Size budget:** the gzipped `demo/dist/*.wasm` artifact must remain strictly under 1.5 MB (1,572,864 bytes), measured in CI by `gzip -6 -c demo/dist/*.wasm | wc -c`. `wasm-opt` runs via Trunk at its default level; gzip level 6 is the CI measurement, not a build setting.
- **Release profile (demo crate):** `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`. The demo is a standalone crate, so the root profile (`debug = true`) does not apply to it.
- **Dependency constraints:** `pulldown-cmark = { default-features = false, features = ["html"] }` prunes the CLI dependency. Direct dependencies: `htmd` (path), `leptos`, `pulldown-cmark`, `html5ever`, `markup5ever_rcdom`, `similar`, `serde`, `serde_json`, `wasm-bindgen`, `js-sys`, `web-sys`, `console_error_panic_hook`. Versions are pinned by the committed `demo/Cargo.lock`; CI builds with `--locked`.

### Accessibility & Compatibility
- **WCAG 2.1 AA (scoped):** full keyboard operability with visible focus indicators, explicit labels on every control, contrast ≥ 4.5:1 for normal text and ≥ 3:1 for large text and UI components, and `aria-live="assertive"` error banners announced once per state transition (not per keystroke).
- **ARIA patterns:** the mode toggle is a radiogroup; the output tabs implement the tabs pattern (`role="tablist"`/`tab`/`tabpanel`, `aria-selected`, `aria-controls`, arrow/Home/End keys, inactive panes hidden); options are grouped in a `fieldset` with `legend`; the preview iframe has an accessible name; focus moves to the crash overlay on panic and is restored after errors resolve.
- **Automated checking:** axe-core runs in CI on the built page; a manual screen-reader pass (NVDA or VoiceOver) is part of the release checklist.
- **Compatibility floors (testable minimums):** Chrome ≥ 120, Firefox ≥ 120, Safari ≥ 17. "Latest stable" applies only to the manual smoke checklist.

## 7. Deliverables

1. `demo/` — a standalone crate (not a workspace member) containing the Leptos CSR application, depending on the root `htmd` crate via `path = "../"`.
2. `demo/README.md` with development and build instructions (native and wasm targets).
3. `.github/workflows/demo-ci.yaml` running on the demo crate: clippy for `wasm32-unknown-unknown` with `--locked -- -D warnings`; native `cargo test --locked`; the `wasm-bindgen-test` suite against headless Chromium; `trunk build --release`; the gzip size check (fails at ≥ 1,572,864 bytes); a small malformed-input fuzz corpus; and the Playwright end-to-end matrix (Chromium, Firefox, WebKit). The root-repo gates (`cargo build`, `cargo clippy -- --deny warnings`, `cargo test`) continue to cover the library and are unaffected by `demo/`.
4. Static-host deployment of `dist/` (GitHub Pages) via the same workflow.
5. This specification, updated to match the implemented behavior.

## 8. Acceptance criteria

### 8.1. Build & Deployment
- [ ] Root-repo gates pass unchanged: `cargo build`, `cargo clippy -- --deny warnings`, and `cargo test`.
- [ ] `trunk build --release` produces a static `dist/` directory deployable on any static host.
- [ ] CI runs an automatic size-check step (GitHub Action or script) that fails if the gzipped `.wasm` artifact reaches or exceeds 1.5 MB (1,572,864 bytes).

### 8.2. Cross-Browser Compatibility
- [ ] The `wasm-bindgen-test` unit suite passes in headless Chromium.
- [ ] The Playwright end-to-end suite passes on Chromium, Firefox, and WebKit.
- [ ] The manual smoke-test checklist passes on latest stable Chrome, Firefox, and Safari (macOS/iOS), with the section 6 floors as minimums.

### 8.3. Performance Benchmarks
- [ ] Conversion (compute-only, including canonicalization and diff) takes under 100 ms for 100 KB documents, measured via `performance.measure`.
- [ ] Inputs ≤ 2 KB: keystroke-to-render end-to-end within 100 ms.
- [ ] No visible input lag when typing documents up to 100 KB: no long task attributable to keystroke handling; scheduled conversion runs may occupy the full 100 ms compute budget.
- [ ] Cold start to conversion-ready within 1500 ms on the reference environment of section 6.

### 8.4. Accessibility (WCAG 2.1 AA, scoped)
- [ ] Full keyboard navigation (tab-focusable controls with visible focus indicators).
- [ ] Accessible labeling: all inputs and option checkboxes have explicitly associated `<label>` or `aria-label` tags.
- [ ] Color contrast meets ≥ 4.5:1 for normal text and ≥ 3:1 for large text and UI components.
- [ ] Error and size-limit banners use `aria-live="assertive"` and are announced once per state transition.
- [ ] Output tabs implement the ARIA tabs pattern with arrow-key navigation; axe-core passes in CI.

### 8.5. Functional Happy Paths
- [ ] Markdown → HTML and HTML → Markdown conversions update the output within the section 8.3 latency budgets: inputs ≤ 2,000 bytes end-to-end within 100 ms; inputs of 2,001–100,000 bytes within 400 ms after the last keystroke.
- [ ] The options panel dynamically updates the active output, and "Reset Options to Demo Preset" restores exactly the FR-4 preset — input text, mode, tabs, scroll positions, samples, and banners untouched.
- [ ] The pipeline fidelity check (Faithful mode) automatically renders the canonicalized original HTML, the canonicalized round-tripped HTML, and a word-level diff of those canonicalized strings only.
- [ ] Sample documents load immediately into the input pane, replace the current buffer with "Undo" available, and update the output state.

### 8.6. Negative & Edge Cases
- [ ] Empty input clears the output pane immediately without console errors or application panics.
- [ ] **Permissive parsing:** malformed HTML (mismatched tags, broken attributes) converts to a best-effort Markdown representation without runtime exceptions or broken application state.
- [ ] **Byte-accurate limit:** an input of exactly 100,000 UTF-8 bytes converts; 100,001 bytes halts conversion, clears output and diff, keeps and persists the text, shows the banner once, and resumes automatically back at the limit.
- [ ] **Nesting depth:** input nested deeper than 512 levels is rejected with the banner before any conversion; input at 512 levels still converts.
- [ ] **Concurrency:** an artificially slowed conversion (test hook adds 300 ms) that finishes after newer input does not overwrite the newer output (latest-input-wins; stale runs discarded).
- [ ] **`skip_tags` parsing:** empty, whitespace-only, mixed-case (`IMG`), duplicate, and unknown entries produce a working converter — no panic on the cleared field.
- [ ] Switching between conversion modes preserves separate text states for each mode without data loss, layout corruption, or accidental re-conversion.
- [ ] Closing or refreshing the browser tab retains typed inputs, selected mode, preview tabs, and sidebar configuration.
- [ ] **Blocked storage:** the app runs and converts normally if `localStorage` is blocked or throws security exceptions.
- [ ] **Malformed JSON:** a non-parseable storage string triggers a clean fallback to defaults without panics.
- [ ] **Type & enum mismatch:** wrong field types or invalid enum variants are rejected, causing a reset to defaults.
- [ ] **Out-of-bound values:** spacing parameters outside the 1–4 range are detected and rejected.
- [ ] **Future version:** storage containing `"version": 2` or higher is dropped and the app boots with the Demo Default Preset and empty inputs (same rule as corrupt or invalid payloads).
- [ ] **Data loss prevention:** type text, immediately toggle a sidebar option within 500 ms, and refresh — the freshly typed text is fully preserved.

### 8.7. Semantic HTML Diff Validation
- [ ] **False-positive avoidance:** the pipeline fidelity diff engine yields zero changes when comparing structurally equivalent HTML fragments that differ *only* by:
    * **Tag & attribute casing:** character case variations (e.g., `<DIV ID="x">` vs `<div id="x">`).
    * **Attribute order:** lexicographical sequence of keys within elements (e.g., `<a href="#" id="x">` vs `<a id="x" href="#">`).
    * **Attribute quoting:** single, double, or omitted quotes compliant with HTML5.
    * **Void element syntax:** self-closing slashes vs open syntax (e.g., `<br />` vs `<br>`).
    * **Entity representations:** equivalent named, decimal, or hexadecimal formats (e.g., `&amp;`, `&#38;`, `&#x26;`) matching their raw character counterparts.
    * **Insignificant whitespace:** collapsible spacing and line breaks between elements, excluding whitespace inside `<pre>` or `<code>` blocks.
- [ ] **Automated regression suite:** a dedicated set of automated "golden tests" (run in `translation_mode: Faithful`) executes in the test pipeline to guarantee these safe variations never trigger visual diff errors; the CommonMark-basics sample is part of the golden corpus (zero diff).
