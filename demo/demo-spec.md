Contributions to the team's pull request
- [k_muzanbekova]: Initiated the repository fork, created the critical demo-spec.md and review.md files, added your teammates to the repository to enable collaboration, and began the initial code review process for the project specification.
Completed the specification analysis and performance audit up to and including point B6, made the corresponding edits to the `review.md` file, and moved the irrelevant points into the specification.
- [d_baitakov]: (Conducted the initial audit of the performance and size specification, completed the review file up to section B7-B10, transferred completed items into the main spec, and systematically applied strikethroughs to the resolved points.) Thats example
- [jhg246@msstate.edu]:
- [nu_aimagambetov]:

### Review History
 
The following table summarizes the design and review iterations for the `htmd / pulldown-cmark live demo` specification. It tracks the document's evolution from initial conceptual sketches to a production-ready specification optimized for WebAssembly runtime stability and browser security.
 
| Round | Findings | Main changes made |
| :---: | :---: | :--- |
| **1** | **18** | Narrowed the scope to strict one-way conversion modes, established a compressed WASM asset budget, and defined initial verifiable acceptance criteria. |
| **2** | **26** | Resolved diff pipeline contradictions, added the missing Section 5 UI sketch, formalized the localStorage validation schema, and eliminated runtime panics on empty tag inputs. |
| **3** | **12** | Enforced total preview isolation using a sandboxed `<iframe>` with a strict Content Security Policy to eliminate XSS risks and hidden third-party network requests. |
| **4** | **38** | Verified all settings against `htmd 0.5.5` and `pulldown-cmark 0.13.4`, expanded the option mapping to 15 flags, resolved reset semantics, and added deep DOM nesting bounds. |

---
 
### Round Context & Analysis
 
1. **Round 1 (Conceptual Scope):** Replaced unstable two-way sync ideas with a deterministic "Use Output as Input" feature, split per-mode state variables, and introduced performance budgets evaluated via the browser's Performance API.
2. **Round 2 (Pipeline Logic):** Designed a word-level text diff built strictly on canonicalized HTML fragments using `html5ever`, dropped unreachable library error wrappers, and fixed concurrency timers to ensure typing does not wipe pending changes.
3. **Round 3 (Browser Security & Sandboxing):** Blocked raw HTML passthrough vulnerabilities, isolated event handling vectors (`onclick`, `onerror`), explicitly mapped the `pulldown-cmark` bitflags, and unified the 100 KB limit using UTF-8 byte arrays.
4. **Round 4 (Crate Alignment & Edge Cases):** Expanded the UI flags to match all 15 bitflags available in `pulldown-cmark 0.13.4`, protected the recursive WASM stack walker against deeply nested elements, and separated "Reset Options" from "Reset Sample" logic.

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
The pipeline diff (HTML → Markdown → HTML) runs automatically on input changes exclusively when `translation_mode` is set to `Faithful`. To prevent false-positive changes during validation, the engine must execute a word-level text diff strictly on canonicalized HTML strings:

*   **Fragment Parsing:** Parse both the original HTML input and the round-tripped HTML output as fragments via `html5ever::parse_fragment` with a `<body>` or `<div>` context using `markup5ever_rcdom::RcDom`. Automated `<html>` or `<head>` wrappers are strictly prohibited.
*   **DOM Normalization:**
    *   **Case Folding:** Convert tag names and attribute keys to lowercase for standard HTML elements only. Foreign content namespaces (SVG and MathML) must preserve their native case-sensitivity (e.g., `viewBox`).
    *   **Attribute Standardization:** Sort all element attributes alphabetically by `namespace` + `local_name`. For `class` attributes, collapse duplicate whitespaces and sort class tokens lexicographically.
    *   **Whitespace Collapsing:** Collapse consecutive whitespaces and line breaks into a single space. Whitespace structures inside `<pre>`, `<code>`, `<textarea>`, `<script>`, and `<style>` blocks must remain untouched. Drop all whitespace-only text nodes located between block-level components.
    *   **Nodes & Entities:** Discard all `Comment`, `Doctype`, `<template>`, and `CDATA` sections. Normalize non-breaking spaces (`&nbsp;` / `U+00A0`) into standard spaces (`U+0020`), except when nested inside raw-text elements.
*   **Deterministic Serialization:** Serialize both normalized DOM trees back into clean strings using `html5ever::serialize` with `TraversalScope::IncludeNode`. Output all void elements in lowercase without any trailing self-closing slashes (e.g., `<br>`).
*   **Word-Level Diff:** Run the `similar::TextDiff::from_words` engine (version 2.x) automatically on the canonicalized strings to generate the visual UI diff.

### FR-6 — Sample Documents
- **Inline HTML:** Validates block preservation.
- **Math Blocks:** Validates `$` and `$$` formulas. Renders as raw text without warnings if `ENABLE_MATH` is disabled.
- **Reset Sample:** Provides a quick action button to restore the default sample layout.

### FR-7 — Error Boundaries & Limits
- **Size Limit:** Inputs > 100 KB halt processing and trigger an `aria-live="assertive"` banner: *"Input exceeds the 100 KB maximum limit for client-side processing."*
- **Malformed HTML Handling:** HTML input with syntax errors (e.g., unclosed tags) is not treated as a runtime failure. The engine leverages `html5ever`'s permissive parsing to build a best-effort DOM tree without throwing exceptions.
- **Runtime Error Isolation:** 
    * *Rust Panics:* The application must register a custom panic hook complementing `console_error_panic_hook`. Since a WASM panic aborts and poisons the Leptos reactive engine, the hook must interact directly with the raw DOM outside the Leptos tree. It must un-hide a static fallback overlay in `index.html` (*"Critical Application Error. Please reload the page."*), permanently disable all converter inputs, and log the trace.
    * *WASM Traps & OOM:* Low-level WebAssembly traps and Out-of-Memory (OOM) errors that bypass the Rust panic hook must be intercepted via a global JavaScript `window.addEventListener('error', ...)` listener in `index.html` to trigger the same static crash overlay.
- **Future-Proof Error Interface:** To ensure forward compatibility with potential fallible APIs in future major versions of `htmd`, the internal converter wrapper must wrap execution in a `Result` type. If an `Err` variant is encountered, it must display a non-fatal UI banner that clears automatically on the next input change.

### FR-8 — Storage & Persistence
- **Storage Key:** Use exactly `htmd_cmark_live_workspace`.
- **Safe Reads (Validation):**
    * Wrap `localStorage.getItem` and `JSON.parse` in `try/catch`. 
    * Fall back to in-memory state without crashing if storage is blocked (Incognito).
    * Validate `version == 1`, data types, and enum variants for all fields.
    * Wipe storage and reset to Demo Default Preset if JSON is corrupt, fields are invalid, or spacing is outside the 1–4 range.
- **Unified Writes (Controller):**
    * **State Serialization:** Save all inputs, active tabs, layout modes, and sidebar settings into a single atomic JSON object.
    * **Debounced Text Save:** Queue a 500 ms debounced write upon text entry.
    * **Atomic Flush-Before-Write:** Any UI interaction (mode toggle, option switch, tab change) must immediately flush the latest text buffers into the state object, cancel the pending text timer, and execute a synchronous write.
    * **Guaranteed Flush:** Force-flush all unsaved buffers on `pagehide` and `visibilitychange` events.
    * **Quota Guard:** Wrap all writes in `try/catch` to ignore `QuotaExceededError`.

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
| Pipeline Fidelity Check (Calculated Automatically)            |
| Original (Canonical) | Round-tripped (Canonical) | Word Diff  |
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
- [ ] The pipeline fidelity check automatically renders the canonicalized original HTML, the canonicalized round-tripped HTML, and a word-level diff generated by the `similar` crate exclusively on those canonicalized strings.
- [ ] Clickable sample documents load immediately into the input pane and update the output state.

### 9.6. Negative & Edge Cases
- [ ] Empty input clears the output pane immediately without throwing console errors or application panics.
- [ ] **Permissive Parsing Verification:** Malformed HTML input (e.g., mismatched tags, broken attributes) compiles into a valid, best-effort Markdown representation without throwing runtime exceptions or breaking application state.
- [ ] Input exceeding 100 KB immediately halts conversion, clears the output, and renders the standard error banner: *"Input exceeds the 100 KB maximum limit for client-side processing."*
- [ ] Conflicting option combinations resolve gracefully without freezing the UI.
- [ ] Switching between conversion modes preserves separate text states for each mode without data loss, layout corruption, or accidental re-conversion.
- [ ] Closing or refreshing the browser tab retains the user's typed inputs, selected mode, preview tabs, and custom sidebar configuration settings.
- [ ] **Blocked Storage:** The app runs and converts text normally if `localStorage` is blocked or throws security exceptions.
- [ ] **Malformed JSON:** Non-parseable string in storage triggers a clean fallback to defaults without panics.
- [ ] **Type & Enum Mismatch:** Wrong field types or invalid enum variants in JSON are rejected, causing a reset to defaults.
- [ ] **Out-of-Bound Values:** Spacing parameters outside the 1–4 range are detected and clamped/rejected.
- [ ] **Future Version:** If storage contains `"version": 2` or higher, the app safely ignores it and boots with the current Version 1 defaults.
- [ ] **Data Loss Prevention:** Type text, immediately toggle a sidebar option within 500 ms, and refresh the browser — the freshly typed text must be fully preserved.

### 9.7. Semantic HTML Diff Validation
- [ ] **False-Positive Avoidance:** The pipeline fidelity diff engine yields zero changes when comparing structurally equivalent HTML fragments that differ *only* by:
    * **Tag & Attribute Casing:** Character case variations (e.g., `<DIV ID="x">` vs `<div id="x">`).
    * **Attribute Order:** Lexicographical sequence of keys within elements (e.g., `<a href="#" id="x">` vs `<a id="x" href="#">`).
    * **Attribute Quoting:** Single, double, or omitted quotes compliant with HTML5 specs.
    * **Void Element Syntax:** Self-closing slashes vs open syntax (e.g., `<br />` vs `<br>`).
    * **Entity Representations:** Equivalent named, decimal, or hexadecimal formats (e.g., `&amp;`, `&#38;`, `&#x26;`) matching their raw character counterparts.
    * **Insignificant Whitespace:** Collapsible spacing and line breaks between elements, excluding whitespace inside `<pre>` or `<code>` blocks.
- [ ] **Automated Regression Suite:** A dedicated set of automated "golden tests" is executed in the test pipeline to guarantee these safe variations never trigger visual diff errors.
