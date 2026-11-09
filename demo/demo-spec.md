# htmd / pulldown-cmark live demo — specification

## 1. Purpose & Target Audience
An interactive, client-side web tool to convert CommonMark Markdown to HTML (via `pulldown-cmark`) and back (via `htmd`). It enables Rust developers and library maintainers to test edge cases, tweak crate options, and inspect translation fidelity in real-time within the browser. 

## 2. Goals

*   **Convert input automatically** as the user types, enforcing a strict 100 KB ceiling.
*   **Support strict one-way conversion** per mode (Markdown → HTML or HTML → Markdown).
*   **Expose core configuration flags** with a global "Reset to Defaults" action.
*   **Persist the active workspace** state in localStorage to prevent data loss.
*   **Provide a clear pipeline diff** (HTML → Markdown → HTML) with intermediate views.
*   **Surface conversion errors** gracefully in the UI without crashing the WASM runtime.

## 3. Non-goals

- User accounts, cloud storage, databases, or shareable link generation.
- Server-side rendering (SSR) or remote backend conversion.
- Advanced rich-text WYSIWYG editor features.

## 4. Functional requirements

### FR-1 — Conversion Modes
- **Markdown → HTML:** Uses `pulldown-cmark`. The preview pane is injected via `srcdoc` into a sandboxed `<iframe>` with a strict CSP (`default-src 'none'; img-src data:; style-src 'unsafe-inline';`).
- **HTML → Markdown:** Uses `htmd`. Malformed HTML must be handled permissively via `html5ever` without crashing.

### FR-2 — Live Adaptive Debounce
Conversion triggers automatically on input changes:
- **Inputs < 2 KB:** Immediate execution (0 ms delay) on every keystroke.
- **Inputs 2 KB – 100 KB:** Debounced with a 250 ms delay after typing stops.
- **Typing Lock Protection:** Forced conversion triggered every 900 ms during continuous typing.
- **Input Isolation:** Preserves a separate text input buffer per conversion mode. Switching modes does not auto-convert or overwrite the opposite buffer.
- **State Preservation:** Sidebar configurations, active tabs, and scroll positions remain unaffected by mode switches.
- **Explicit Action:** A manual "Use Output as Input" button promotes generated results for reverse editing.


### FR-3 — Output Views
The output pane features two tabs (`Rendered` and `Source`):

| Conversion Mode | **Source** Tab | **Rendered** Tab |
| :--- | :--- | :--- |
| **Markdown → HTML** | Displays the raw, generated HTML string as plain text. | Displays the generated HTML rendered inside a sandboxed `<iframe>`. |
| **HTML → Markdown** | Displays the raw, generated Markdown markup text. | Displays the Markdown re-rendered back into HTML via `pulldown-cmark` inside a sandboxed `<iframe>`. |

### FR-4 — Configuration & Reset
- **`pulldown-cmark` Options:** Tables, footnotes, strikethrough, task lists, smart punctuation, heading attributes, math.
- **`htmd` Core Options:** Exposes official crate options with exact variants and defaults:
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
- **`htmd` Tag Skipping:** Dedicated comma-separated text input field maps to `HtmlToMarkdownBuilder::skip_tags`.
- **Reset Action:** A "Reset to Defaults" button applies the **Demo Default Preset** (All 7 `pulldown-cmark` options Enabled, `translation_mode` set to `Pure`, all other options to table defaults, `skip_tags` cleared). It does **not** clear input text, change the active mode, or alter active tabs.

### FR-5 — Pipeline Fidelity Check (HTML Mode Only)
Implement a deterministic HTML canonicalization pipeline to clean both HTML outputs *before* passing them to the text diffing engine:

1. **Parse & Normalize:** Parse both HTML fragments into a structured DOM tree using `html5ever`.
2. **Standardize Components:** 
   * Convert all tag names and attribute keys to lowercase.
   * Sort all element attributes alphabetically by name.
   * Normalize text nodes by collapsing consecutive whitespaces and standardizing HTML entities.
   * Serialize void elements using standard lowercase syntax without trailing slashes (`<br>`).
3. **Deterministic Serialization:** Serialize the normalized DOM tree back into a clean string.
4. **Word-Level Diff:** Run a word-level `similar::TextDiff` engine exclusively on the canonicalized strings to generate the UI diff.


### FR-6 — Sample Documents
- **Inline HTML:** Validates block preservation.
- **Math Blocks:** Validates `$` and `$$` formulas. Renders as raw text without warnings if `ENABLE_MATH` is disabled.
- **Reset Sample:** Provides a quick action button to restore the default sample layout.

### FR-7 — Error Boundaries & Limits
- **Size Limit:** Inputs > 100 KB halt processing and trigger an `aria-live="assertive"` banner: *"Input exceeds the 100 KB maximum limit for client-side processing."*
- **Malformed HTML:** Permissively parsed via `html5ever` without breaking application state.
- **Standard Conversion Failures:** Any io::Result<String> error from htmd::convert must be caught, mapped to a non-fatal UI banner, and cleared on the next input change. WASM runtime and inputs must remain interactive.
- **Critical Panic Hook:** The application must register a custom panic hook that complements console_error_panic_hook. Because a WASM panic triggers a panic = "abort" state that poisons the Leptos reactive engine, the hook must interact directly with the DOM outside the WASM-controlled tree. It must un-hide a pre-existing static overlay elements in index.html ("Critical Application Error. Please reload the page."), permanently disable all converter inputs, and log the trace to the browser console. Recovery requires a manual page reload.


### FR-8 — Storage & Persistence
- **Storage Key:** Use exactly `htmd_cmark_live_workspace`.
- **Safe Reads (Validation):**
    * Wrap `localStorage.getItem` and `JSON.parse` in `try/catch`. 
    * If storage is blocked (Incognito), switch to in-memory state without crashing.
    * Check `version == 1` and validate types/ranges for every field.
    * If version is mismatch/missing, JSON is broken, enums are wrong, or spacing is out of 1–4 range: drop storage and reset to Demo Default Preset with empty inputs.
- **Unified Writes (Controller):**
    * **Instant sync write:** For mode switches, active tabs, sidebar options, and `skip_tags`.
    * **Debounced text write:** 500 ms delay for input text areas.
    * **Timer clear:** Any instant write cancels any pending debounced text timer.
    * **Guaranteed flush:** Force-flush text buffers on `pagehide` and `visibilitychange` events.
    * **Quota Guard:** Wrap writes in `try/catch` to safely ignore `QuotaExceededError`.

## 6. UI sketch

```
+---------------------------------------------------------------+

| htmd <-> pulldown-cmark demo                                  |
| [Markdown -> HTML]  [HTML -> Markdown]     [Options] [Reset]  |
+---------------------------+-----------------------------------+

| Input                     | Output                            |
| (textarea)                | [Rendered] [Source]               |
|                           |                                   |
| [Use Output as Input]     |                                   |
+---------------------------+-----------------------------------+

| Round trip: [Run]   Original | Round-tripped | Diff           |
+---------------------------------------------------------------+
```

- Responsive: panes stack vertically on narrow screens.
- Keyboard accessible: toggle and tabs reachable by keyboard, labels on controls.
- Light and dark theme optional.

## 7. Non-functional requirements

### Performance & WASM
- **Compute Budget:** Pure processing and rendering time must remain under 100 ms for documents up to 100 KB.
- **WASM Initialization:** Cold start must complete within 1500 ms.
- **Measurement:** Audited via the browser's Performance API (Total Blocking Time).
- **Size Profile:** The final `.wasm` file must remain under **1.5 MB (gzipped)** using default level 6 compression.
- **Dependency Constraints:** `pulldown-cmark` must be configured with `default-features = false, features = ["html"]` to prune CLI bloat. All dependencies (`leptos`, `pulldown-cmark`, `htmd`, `similar`, `wasm-bindgen`, `web-sys`, `console_error_panic_hook`) must be strictly version-pinned.

### Accessibility & Compatibility
- **WCAG 2.1 AA Compliance:** Full keyboard focusability, explicit labels, and a minimum contrast ratio of 4.5:1 (text) / 3:1 (UI components).
- **Compatibility:** Latest stable versions of Chrome (v120+), Firefox (v120+), and Safari (v17+).

## 8. Deliverables

1. `demo/` crate with the Leptos CSR application.
2. `demo/README.md` with development and build instructions.
3. Deployment workflow or documented manual deployment (static hosting).
4. This specification, updated to match the implemented behavior.

## 9. Acceptance criteria

### 9.1. Build & Deployment
- [ ] `cargo build`, `cargo clippy -- --deny warnings`, and `cargo test` pass successfully.
- [ ] `trunk build --release` produces a static `dist/` directory deployable on any static host, with the final compiled `.wasm` file falling strictly within the 1.5 MB gzipped size budget.
- [ ] В CI настроен автоматический шаг проверки размера (Size Check GitHub Action или скрипт), который падает (fails), если размер gzipped `.wasm` превышает 1.5 МБ.

### 9.2. Cross-Browser Compatibility (Headless & Manual)
- [ ] Automated tests pass in headless Chromium, Firefox, and WebKit via `wasm-bindgen-test`.
- [ ] Manual smoke-test checklist passes in latest stable versions of Chrome, Firefox, and Safari (macOS/iOS).

### 9.3. Performance Benchmarks
- [ ] First input-to-output conversion takes under 100 ms, measured via the browser's Performance API.
- [ ] No visible input lag when typing large documents (up to 100 KB).

### 9.4. Accessibility (WCAG 2.1 AA)
- [ ] Full keyboard navigation (Tab-focusable controls with visible focus indicators).
- [ ] Accessible labeling: all inputs and option checkboxes have explicitly associated `<label>` or `aria-label` tags.
- [ ] Color contrast meets a minimum ratio of 4.5:1 for standard text and 3:1 for UI components.
- [ ] Error messages and size limit banners use `aria-live="assertive"` to be instantly announced by screen readers.

### 9.5. Functional Happy Paths
- [ ] Markdown → HTML and HTML → Markdown conversions visually update the output pane within 350–400 ms End-to-End after the last keystroke (calculating a fixed 250 ms debounce delay + maximum 100 ms compute and render budget).
- [ ] Options panel dynamically updates the active conversion output and resets completely to defaults via a single click.
- [ ] Round-trip view successfully renders original HTML, round-tripped HTML, and a word- or character-level diff generated by the similar crate directly on the raw HTML outputs.
- [ ] Clickable sample documents load immediately into the input pane and update the output state.

### 9.6. Negative & Edge Cases
- [ ] Empty input clears the output pane immediately without throwing console errors or application panics.
- [ ] Malformed HTML input (e.g., unclosed tags, unescaped scripts) is handled safely by `htmd` without crashing the WASM runtime.
- [ ] Input exceeding 100 KB immediately halts conversion, clears the output, and renders the standard error banner: "Input exceeds the 100 KB maximum limit for client-side processing."
- [ ] Conflicting option combinations resolve gracefully without freezing the UI.
- [ ] Switching between conversion modes preserves separate text states for each mode without data loss, layout corruption, or accidental re-conversion.
- [ ] Closing or refreshing the browser tab retains the user's typed inputs, selected mode, preview tabs, and custom sidebar configuration settings.
- [ ] **Blocked Storage:** The app runs and converts text normally if `localStorage` is blocked or throws security exceptions.
- [ ] **Malformed JSON:** Non-parseable string in storage triggers a clean fallback to defaults without panics.
- [ ] **Type & Enum Mismatch:** Wrong field types or invalid enum variants in JSON are rejected, causing a reset to defaults.
- [ ] **Out-of-Bound Values:** Spacing parameters outside the 1–4 range are detected and clamped/rejected.
- [ ] **Future Version:** If storage contains `"version": 2` or higher, the app safely ignores it and boots with the current Version 1 defaults.

### 9.7. Semantic HTML Diff Validation
- [ ] **False-Positive Avoidance:** The pipeline fidelity diff remains completely empty (asserts zero changes) when comparing HTML inputs that differ *only* by:
- [ ] **Automated Regression Suite:** A dedicated suite of automated "golden tests" is executed in the test pipeline to guarantee these safe variations never trigger visual diff errors.