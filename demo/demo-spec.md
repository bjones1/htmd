# htmd / pulldown-cmark live demo — specification

## Key decisions

- Diff pipeline: HTML mode only, `Faithful` translation mode only, canonicalized fragments, `similar::TextDiff::from_words`, automatic (rounds 2–4).
- Storage: single JSON object, `version: 1`, flush-before-write controller (rounds 2–4).
- Canonicalization: `html5ever` fragment parsing, raw-text exemption, foreign-content case preservation (round 4).
- Performance: 100 ms compute-only budget via `performance.mark`/`measure`; size budget = gzipped `.wasm` artifact, checked in CI (round 5).
- Cross-browser: `wasm-bindgen-test` on Chromium; Playwright matrix for Chromium/Firefox/WebKit; Safari manual checklist (round 5).
- Build layout: `demo/` is a standalone crate with a `path` dependency on the root `htmd` crate (round 5).
- Round trip: fixed `ENABLE_TABLES | ENABLE_MATH`; synchronous main-thread conversion; compute budget measured on a benchmark corpus (round 8).
- Output state: per-mode outputs with a validity rule; one synchronous commit step; nesting depth measured on the parsed tree via `html_to_tree`; preview stripping on a parsed DOM (round 9).

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
    * before injection, preview content is parsed with `html5ever` (fragment parsing, `<body>` context, `RcDom`), walked iteratively with an explicit stack, and serialized per FR-5's serialization rule; string or regex matching is not permitted. Markdown → HTML output is exempt from the FR-7 nesting limit, and 50,000 `>` characters render as 50,000 nested `<blockquote>` elements, so a recursive walk would overflow the wasm stack;
    * the walk removes every `<base>`, `<meta>`, and `<link>` element and every `target` attribute as defense in depth. Removing whole element types, rather than listing `rel` and `http-equiv` values, also covers `refresh`, `x-dns-prefetch-control` (which re-enables anchor DNS prefetching), and `dns-prefetch`/`preconnect`/`prefetch`/`prerender` links. The Source tab still shows the unstripped output;
    * the demo injects its own `<base target="_blank">` after its CSP `<meta>`: `sandbox` does not stop the iframe navigating itself to a page the `srcdoc` CSP no longer covers, but it does block the resulting popup;
    * consequently, in-page anchors (footnote references and back-links, links to heading ids) do not navigate in the preview. This is accepted: relative URLs in an `about:srcdoc` document resolve against the embedding page's URL, so even without the injected `<base>`, `#fn-1` would load the demo page itself into the iframe.

### FR-2 — Live Adaptive Debounce
Conversion triggers automatically on input changes:
- **Inputs ≤ 2 KB** (2,000 UTF-8 bytes): no debounce; conversion is scheduled on the next animation frame after the keystroke.
- **Inputs > 2 KB and ≤ 100 KB** (2,001–100,000 bytes): debounced with a 250 ms delay after typing stops.
- **Typing Lock Protection:** for pending edits in the debounced range, schedule conversion at the earlier of 250 ms after the latest edit or 900 ms after the first edit not yet included in a dispatched run. Further edits update the pending input and reset only the 250 ms timer; they never extend the 900 ms deadline. Dispatch clears both timers for that batch; the next edit starts a new batch. If a conversion is already in flight at the deadline, queue the latest input and dispatch it as soon as that run finishes. The forced deadline does not apply to the ≤ 2 KB immediate path.
- **Execution model:** conversion runs synchronously on the main thread (no Web Worker), so FR-7's panic hook and `window` listeners see every failure. A run's *commit* is one synchronous step: it checks the run's sequence number, then writes the preview `srcdoc`, the Source text, and the fidelity panel together, or writes nothing if the run is stale. A run is *in flight* from dispatch until the preview iframe fires `load` for that commit; the iframe stays in the DOM and loads while its tabpanel is hidden, and input events are processed during the load.
- **Sequence numbers:** the sequence number advances on every buffer change (typing, sample load, Reload Sample, Undo, "Use Output as Input"), every option change including Reset, every mode switch, and entering an FR-7 limit. In-flight and queued runs with older numbers are discarded. A mode switch also cancels both timers of the pending batch, so that batch never commits into the other mode's pane.
- **Concurrency:** latest-input-wins. At most one conversion runs at a time and at most one is queued; each run carries a sequence number and stale results are discarded. A forced tick that arrives while a conversion is in flight coalesces into the single queued run.
- **Input Isolation:** a separate text buffer and a separate output (preview, Source text, fidelity panel) per conversion mode. A mode switch never reads, converts, or overwrites the other mode's buffer; only the explicit "Use Output as Input" and sample-loading actions may overwrite a buffer.
- **Output validity:** a mode's output is *valid* while its buffer and the options are unchanged since the commit that produced it. Option changes reconvert only the active mode, so they invalidate the inactive mode's output. Switching to a mode shows its valid output unchanged; if that output is invalid (options changed, or its pending batch was cancelled by the switch), the switch clears it and immediately converts that mode's own buffer. After a workspace is restored at startup, the active mode's buffer is converted once. "Use Output as Input" is disabled while the active mode's output is empty or invalid.
- **State Preservation:** sidebar options, active tabs, and scroll positions remain unaffected by mode switches.
- **Explicit Action:** "Use Output as Input" copies the current output into the opposite mode's buffer, switches to that mode, and immediately converts. If the target buffer was non-empty, an "Undo" toast (`aria-live="polite"`) restores that buffer only, not the mode. It stays until the next buffer edit, mode switch, sample load, "Use Output as Input", or its close button; focus moves, scrolling, and keyboard navigation do not dismiss it.

### FR-3 — Output Views
The output pane features two tabs (`Rendered` and `Source`) implementing the ARIA tabs pattern (see accessibility):

| Conversion Mode | **Source** Tab | **Rendered** Tab |
| :--- | :--- | :--- |
| **Markdown → HTML** | The raw, generated HTML string as plain text. | The generated HTML rendered inside a sandboxed `<iframe>` per FR-1. |
| **HTML → Markdown** | The raw, generated Markdown markup text. | The Markdown re-rendered back into HTML via `pulldown-cmark` with the sidebar's FR-4 flags, inside a sandboxed `<iframe>` per FR-1. |

All user-derived text — the Source tab, the FR-5 fidelity columns (diff tokens as `<ins>`/`<del>`/`<span>` elements), banners, and toasts — renders via `textContent`, never `innerHTML`. Only the sandboxed preview receives markup.

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
- **`htmd` tag skipping:** a dedicated comma-separated text field maps to `HtmlToMarkdownBuilder::skip_tags`. The field is parsed by splitting on commas, trimming whitespace, lowercasing, then dropping empty parts, duplicates, and entries not matching `[a-z][a-z0-9-]*`; the field is capped at 200 characters. An empty parsed result builds the converter with no skip handler — it never calls `skip_tags` with an empty vector (which panics). Mixed-case input such as `IMG` is accepted and lowercased, because `html5ever` supplies lowercase tag names for HTML elements. Consequently SVG elements whose `html5ever` names are camelCase (`foreignObject`, `clipPath`, `linearGradient`, ...) cannot be skipped; this is an accepted limitation.
- **Reset action:** the button is labeled "Reset Options to Demo Preset" and restores only the sidebar configuration:
    * `pulldown-cmark`: all 7 exposed flags enabled — the demo preset deliberately differs from the crate default `Options::empty()`;
    * `htmd`: `translation_mode` set to `Pure`, every other option to its crate default listed above;
    * `skip_tags` cleared;
    followed by immediate reconversion. It does not clear or change input text, active mode, active tabs, scroll positions, loaded samples, or error banners.

### FR-5 — Pipeline Fidelity Check (HTML Mode Only)
The pipeline diff (HTML → Markdown → HTML) runs automatically on input changes exclusively when `translation_mode` is set to `Faithful`. In `Pure` mode the fidelity panel is hidden with an explanatory note, because intentional attribute loss would reduce the diff to noise. To prevent false-positive changes during validation, the engine must execute a word-level text diff strictly on canonicalized HTML strings:

*   **Round-trip rendering:** `htmd`'s Markdown is rendered back with exactly `Options::ENABLE_TABLES | Options::ENABLE_MATH`, ignoring the sidebar — the extensions `htmd` emits and tests. `htmd` backslash-escapes `` \ & * _ ` [ ] $ `` anywhere in text, and `= ~ > - + #`, ordered-list delimiters, and HTML-like `<` only where they would open a construct (`src/dom_walker.rs`); it never escapes `'`, `"`, or `{`, so smart punctuation, strikethrough, and heading attributes would rewrite ordinary text (`don't` → `don’t`) into false diffs.
*   **Fragment Parsing:** Parse both the original HTML input and the round-tripped HTML output as fragments via `html5ever::parse_fragment` with a `<body>` context using `markup5ever_rcdom::RcDom`. Fragment parsing places a synthetic `<html>` element under the `RcDom` document; the fragment's top-level nodes are that element's children. Automated `<html>` or `<head>` wrappers are strictly prohibited in the canonical output. Every canonicalization walk is iterative (explicit stack).
*   **DOM Normalization:**
    *   **Case Folding:** Convert tag names and attribute keys to lowercase for standard HTML elements only. Foreign content namespaces (SVG and MathML) must preserve their native case-sensitivity (e.g., `viewBox`).
    *   **Attribute Standardization:** Sort all element attributes by `namespace` URL, then `local_name`. For `class` attributes, collapse duplicate whitespace and sort class tokens. All canonicalization sorting compares by Unicode code point (Rust `str` ordering), never locale collation, so every browser produces identical output.
    *   **Preserved subtrees:** `<pre>`, `<code>`, `<textarea>`, `<script>`, and `<style>` elements, and any element whose `style` attribute declares `white-space` with a value other than `normal`, are preserved with all their descendants: neither the whitespace rules nor the U+00A0 rule below applies inside them. CSS `white-space` set by a stylesheet selector is not detected; whitespace preserved that way is outside what the diff verifies.
    *   **Whitespace Collapsing:** Outside preserved subtrees, collapse consecutive whitespace and line breaks into a single space. Drop a whitespace-only text node when it is a child of the fragment root or when its previous or next sibling is a block-level element. Then trim leading whitespace from the first child, and trailing whitespace from the last child, of the fragment root and of every block-level element when that child is a text node, and drop text nodes left empty — so `<p>a</p>` and `<p>a</p>\n` (`pulldown-cmark`'s trailing newline) canonicalize identically, and an all-whitespace fragment canonicalizes to the empty string. Block-level elements are `address`, `article`, `aside`, `blockquote`, `caption`, `dd`, `details`, `dialog`, `div`, `dl`, `dt`, `fieldset`, `figcaption`, `figure`, `footer`, `form`, `h1`–`h6`, `header`, `hgroup`, `hr`, `li`, `main`, `nav`, `ol`, `p`, `pre`, `section`, `summary`, `table`, `tbody`, `td`, `tfoot`, `th`, `thead`, `tr`, and `ul`.
    *   **Nodes & Entities:** Discard all `Comment` and `Doctype` nodes, and `<template>` elements with their contents. CDATA sections are not DOM nodes: in HTML content `html5ever` parses `<![CDATA[…]]>` as a bogus comment, which the comment rule discards; in SVG and MathML content it parses the contents as ordinary text, which is kept. Outside preserved subtrees, replace non-breaking spaces (`&nbsp;` / U+00A0) with standard spaces (U+0020) before whitespace collapsing. No Unicode normalization (NFC/NFKC) is applied: entities decode to the same code points on both sides and neither `htmd` nor `pulldown-cmark` normalizes text, so a normalization difference is a real change the diff must show.
*   **Deterministic Serialization:** Serialize each top-level node (each child of the synthetic `<html>` element) in document order with `html5ever::serialize` and `TraversalScope::IncludeNode`, concatenating the results; equivalently, serialize the synthetic `<html>` element with `TraversalScope::ChildrenOnly(None)`. Never serialize the `RcDom` document node itself: `markup5ever_rcdom` panics on it, which `panic = "abort"` turns into the crash overlay. Output all void elements in lowercase without any trailing self-closing slashes (e.g., `<br>`).
*   **Word-Level Diff:** Run the `similar::TextDiff::from_words` engine (version 2.x) automatically on the canonicalized strings to generate the visual UI diff. `from_words` splits text into alternating runs of whitespace and non-whitespace characters, needs no optional `similar` feature, and compares tokens by exact code points. Both sides use the same tokenizer, so tokenization alone cannot produce a change; text without spaces (e.g., CJK) yields coarse tokens, not false ones. The diff is diagnostic — it reports textual divergence, not proof of semantic equivalence.

### FR-6 — Sample Documents
Three samples with fixed content shipped in the binary. A sample has one variant per mode it supports: a Markdown variant for Markdown → HTML and an HTML variant for HTML → Markdown.
- **CommonMark basics** (both modes): headings, emphasis, lists, links, and fenced code — expected to round-trip losslessly in both modes.
- **Inline HTML** (HTML → Markdown only): inline spans with attributes and raw HTML blocks — validates block preservation; in `Pure` mode attribute loss is expected and visible; in `Faithful` mode attributes are preserved.
- **Math** (both modes): `$` and `$$` formulas — round-trip when `ENABLE_MATH` is on; render as literal `$` text without warnings when it is off.

Sample actions:
- Clicking a sample loads its variant for the current mode into the current buffer. If the sample has no variant for the current mode (Inline HTML clicked in Markdown → HTML mode), the click switches to the sample's mode and loads it into that mode's buffer; the other buffer is untouched. The load converts immediately, offers the same "Undo" affordance as "Use Output as Input" (restoring only the replaced buffer, not the mode), and becomes that mode's selected sample.
- The selected sample is tracked per mode and persisted (FR-8). Each mode's default sample is CommonMark basics.
- **Reload Sample** loads the current mode's selected sample (or that mode's default if none is selected) into the current buffer, converts immediately, and offers the same "Undo" affordance. It never changes options, mode, or tabs, and is independent of "Reset Options to Demo Preset".

### FR-7 — Error Boundaries & Limits
- **Size limit:** the ceiling is **100,000 UTF-8 bytes** of the input string (measured via `TextEncoder`). Inputs **strictly larger** than the ceiling halt conversion, clear the output and diff, and show the `aria-live="assertive"` banner exactly once per transition into the limit: *"Input exceeds the 100,000-byte maximum for client-side processing."* The oversized text stays in the editor and is persisted; processing resumes automatically once the input is at or below the ceiling.
- **Nesting limit:** HTML → Markdown input whose element nesting exceeds **512 levels** is rejected before conversion exactly like the size limit: conversion halts, the output and diff are cleared, the banner *"Input exceeds the supported nesting depth."* is shown once per transition into the limit, and processing resumes automatically once the input is back within it. The demo parses the input once with `HtmlToMarkdown::html_to_tree` (`html5ever`'s tokenizer and tree builder are iterative, and `RcDom` drops nodes through a worklist), measures the maximum element depth with an iterative walk (children of `<head>` and `<body>` are depth 1), and passes the same tree to `tree_to_markdown` only when that depth is ≤ 512. Measuring the parsed tree rather than scanning raw tags counts exactly what the converter will walk, so void elements, optional end tags, misnested tags, and `<` inside attributes, comments, or raw text need no special rules. The limit protects the recursive conversion traversal in `src/dom_walker.rs`. Markdown → HTML input is exempt: `pulldown-cmark`'s parser is iterative, and so is FR-1's preview walk.
- **Malformed HTML:** input with syntax errors (e.g., unclosed tags) is not a runtime failure. The engine leverages `html5ever`'s permissive parsing to build a best-effort DOM tree without throwing exceptions; `<script>`/`<style>` contents are treated as raw text.
- **Rust panics:** a single custom panic hook is installed once at startup. It first delegates to `console_error_panic_hook` (only one global hook exists, so it must chain rather than install a second), is null-safe and idempotent (guards element lookups; runs its body once), then un-hides the static fallback overlay element in `index.html` (*"Critical Application Error. Please reload the page."*), permanently disables **all** interactive controls (mode switch, tabs, samples, options, reset, "Use Output as Input") by setting `inert` on the application root, moves focus to the overlay (`role="alertdialog"`), and logs the trace to the console. The hook uses only raw `web-sys` DOM calls, never Leptos signals or other reactive state: that state may be borrowed at the panic site, and re-entering it would panic again and abort before the overlay shows. The release profile uses `panic = "abort"`; the hook runs before the abort trap and is verified with deliberate panics in debug and release builds.
- **WASM traps & OOM:** low-level traps and out-of-memory errors bypass the Rust panic hook and are intercepted via global `window` `error` and `unhandledrejection` listeners in `index.html`, which show the same static crash overlay. Recovery requires a manual page reload.
- **Future-proof wrapper:** the internal converter wraps execution in a `Result` for forward compatibility with fallible future `htmd` APIs; an `Err` displays a non-fatal banner that clears on the next input change.

### FR-8 — Storage & Persistence
- **Storage key:** use exactly `htmd_cmark_live_workspace`.
- **Initial workspace:** Without a saved workspace, start in Markdown → HTML mode on the Rendered tab, with empty inputs and output, zero input scroll offsets, the Demo Preset applied, and no sample selected or preloaded.
- **Safe reads (validation):**
    * Wrap `localStorage.getItem` and `JSON.parse` in `try/catch`.
    * Fall back to in-memory state without crashing if storage is blocked (Incognito).
    * Validate `version == 1`, data types, and enum variants for all fields.
    * Wipe storage and reset to Demo Preset with empty inputs if the version is not 1, the JSON is corrupt, fields are invalid, or spacing is outside the 1–4 range.
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
    * `sample_md_to_html`: `"basics"`, `"math"`, or `null`; `sample_html_to_md`: `"basics"`, `"inline_html"`, `"math"`, or `null` — the per-mode selected sample (FR-6), default `null`
- **Unified writes (controller):**
    * **State serialization:** save the persisted-state schema above into a single atomic JSON object.
    * **Debounced text save:** queue a 500 ms debounced write upon text entry.
    * **Atomic flush-before-write:** any UI interaction (mode toggle, option switch, tab change) must immediately flush the latest text buffers into the state object, cancel the pending text timer, and execute a synchronous write.
    * **Guaranteed flush:** force-flush all unsaved buffers on `pagehide` and `visibilitychange` events.
    * **Quota guard:** wrap all writes in `try/catch`; a failed write (e.g., `QuotaExceededError`) shows the non-fatal banner *"Workspace could not be saved; recent changes will be lost on reload."* once per transition into failure, cleared by the next successful write.
    * **Multiple tabs:** on a `storage` event for the key, a tab keeps its own state and shows the non-fatal banner *"This workspace was changed in another tab; saving here will overwrite it."* once per transition into the externally-modified state. The other tab's debounced saves fire a `storage` event about every 500 ms while it is in use, so further external writes while the banner shows do not re-announce it. The banner clears after this tab's next successful write, which overwrites the external change; writes stay last-writer-wins.

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
| [error banner region, aria-live="assertive", collapsed when empty]  |
+---------------------------------------------------------------------+
```

- Responsive: panes stack vertically on narrow screens.
- Keyboard accessible: mode toggle and tabs follow the ARIA patterns in section 6; all controls have labels.
- Live regions: the banner region and the toast's `aria-live="polite"` region are always in the accessibility tree and only visually collapsed when empty, never `display: none` or `hidden`, because screen readers often skip content inserted while a region is being revealed. Concurrent banners stack in the single banner region, one child element per message, each announced once when added.
- Single light theme; dark theme is a non-goal.

## 6. Non-functional requirements

### Performance & WASM
- **Compute budget:** one full conversion — including canonicalization and diff generation when active — completes in under 100 ms for every document in the benchmark corpus `demo/bench/corpus/` (realistic documents ≤ 100 KB in both modes: the scaled-up samples and a saved real-world page), measured with `performance.mark`/`performance.measure` around the conversion call. Long-task jank is monitored with the Long Tasks API; Total Blocking Time is a load-time metric and is not used for conversion latency.
- **Adversarial bounds:** inputs ≤ 100 KB outside the corpus are exempt from the compute budget but bounded:
    * After stripping the common token prefix and suffix, a diff whose middle exceeds 5,000 tokens on either side is skipped; the panel shows both canonical strings with *"Diff too large to display."* Do not use `similar`'s deadline: `std::time::Instant::now` panics on `wasm32-unknown-unknown`.
    * Markdown → HTML output over 2,000,000 UTF-8 bytes (e.g., reference-link amplification) is cleared instead of previewed, with the non-fatal banner *"Output exceeds the 2 MB display limit."*
- **End-to-end latency:** inputs ≤ 2 KB (2,000 bytes) render within 100 ms of the keystroke; inputs of 2,001–100,000 bytes render within ~350–400 ms after the last keystroke (250 ms debounce + ≤ 100 ms compute and render).
- **Cold start:** initialization completes within 1500 ms from navigation start to conversion-ready, on a mid-range desktop CPU (4-core laptop, 2020 or newer), without network or CPU throttling, warm cache.
- **Size budget:** the gzipped `demo/dist/*.wasm` artifact must remain strictly under 1.5 MB (1,572,864 bytes), measured in CI by `gzip -6 -c demo/dist/*.wasm | wc -c`. `wasm-opt` runs via Trunk at level `z` (`data-wasm-opt="z"` on the `rust` link in `index.html`); CI installs a pinned Trunk version, and `Trunk.toml` `[tools]` pins the `wasm-bindgen` and `wasm-opt` versions Trunk downloads. Gzip level 6 is the CI measurement, not a build setting.
- **Release profile (demo crate):** `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`, `strip = true`. The demo is a standalone crate, so the root profile (`debug = true`) does not apply to it.
- **Dependency constraints:** `pulldown-cmark = { default-features = false, features = ["html"] }` prunes the CLI dependency. Direct dependencies: `htmd` (path), `leptos` (feature `csr`), `pulldown-cmark`, `html5ever` and `markup5ever_rcdom` (the same minor versions as the root `htmd` crate, currently 0.39, so only one copy is linked into the wasm), `similar`, `serde`, `serde_json`, `wasm-bindgen`, `js-sys`, `web-sys`, `console_error_panic_hook`. Versions are pinned by the committed `demo/Cargo.lock`; CI builds with `--locked`.

### Accessibility & Compatibility
- **WCAG 2.1 AA (scoped):** full keyboard operability with visible focus indicators, explicit labels on every control, contrast ≥ 4.5:1 for normal text and ≥ 3:1 for large text and UI components, and `aria-live="assertive"` error banners announced once per state transition (not per keystroke).
- **ARIA patterns:** the mode toggle is a radiogroup; the output tabs implement the tabs pattern (`role="tablist"`/`tab`/`tabpanel`, `aria-selected`, `aria-controls`, arrow/Home/End keys, inactive panes hidden); options are grouped in a `fieldset` with `legend`; the preview iframe has an accessible name; focus moves to the crash overlay on panic and is restored after errors resolve.
- **Automated checking:** axe-core runs in CI on the built page; a manual screen-reader pass (NVDA or VoiceOver) is part of the release checklist.
- **Compatibility floors (testable minimums):** Chrome ≥ 120, Firefox ≥ 120, Safari ≥ 17. "Latest stable" applies only to the manual smoke checklist.

## 7. Deliverables

1. `demo/` — a standalone crate (not a workspace member) containing the Leptos CSR application, depending on the root `htmd` crate via `path = "../"`.
2. `demo/README.md` with development and build instructions (native and wasm targets).
3. `.github/workflows/demo-ci.yaml` running on the demo crate: clippy for `wasm32-unknown-unknown` with `--locked -- -D warnings`; native `cargo test --locked`; the `wasm-bindgen-test` suite against headless Chromium; `trunk build --release`; the gzip size check (fails at ≥ 1,572,864 bytes); a small malformed-input fuzz corpus; the Playwright end-to-end matrix (Chromium, Firefox, WebKit); and an axe-core accessibility scan of the built page driven by Playwright, failing CI on violations. The root-repo gates (`cargo build`, `cargo clippy -- --deny warnings`, `cargo test`) continue to cover the library and are unaffected by `demo/`.
4. Static-host deployment of `dist/` (GitHub Pages) via the same workflow, built with `trunk build --release --public-url` set to the Pages project path (e.g., `/htmd/`): Trunk's default `/` breaks asset URLs on a project site.
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
- [ ] Conversion (compute-only, including canonicalization and diff) takes under 100 ms for every document in the `demo/bench/corpus/` benchmark corpus, measured via `performance.measure`.
- [ ] **Adversarial bounds:** 100 KB of reference-link amplification shows the 2 MB banner, and a fully divergent 100 KB diff shows "Diff too large to display"; neither hangs or panics.
- [ ] Inputs ≤ 2 KB: keystroke-to-render end-to-end within 100 ms.
- [ ] No visible input lag when typing documents up to 100 KB: no long task attributable to keystroke handling; scheduled conversion runs may occupy the full 100 ms compute budget.
- [ ] Cold start to conversion-ready within 1500 ms on the reference environment of section 6.

### 8.4. Accessibility (WCAG 2.1 AA, scoped)
- [ ] Full keyboard navigation (tab-focusable controls with visible focus indicators).
- [ ] Accessible labeling: all inputs and option checkboxes have explicitly associated `<label>` or `aria-label` tags.
- [ ] Color contrast meets ≥ 4.5:1 for normal text and ≥ 3:1 for large text and UI components.
- [ ] Error and size-limit banners use `aria-live="assertive"` and are announced once per state transition.
- [ ] Output tabs implement the ARIA tabs pattern with arrow-key navigation; axe-core passes in CI.
- [ ] Tabbing from the editor to the "Undo" toast does not dismiss it; Undo restores only the overwritten buffer.

### 8.5. Functional Happy Paths
- [ ] Markdown → HTML and HTML → Markdown conversions update the output within the section 8.3 latency budgets: inputs ≤ 2,000 bytes end-to-end within 100 ms; inputs of 2,001–100,000 bytes within 400 ms after the last keystroke.
- [ ] The options panel dynamically updates the active output, and "Reset Options to Demo Preset" restores exactly the FR-4 preset — input text, mode, tabs, scroll positions, samples, and banners untouched.
- [ ] The pipeline fidelity check (Faithful mode) automatically renders the canonicalized original HTML, the canonicalized round-tripped HTML, and a word-level diff of those canonicalized strings only.
- [ ] Sample documents load immediately into the input pane, replace the current buffer with "Undo" available, and update the output state. Clicking Inline HTML in Markdown → HTML mode switches to HTML → Markdown and replaces only the HTML buffer; Undo restores that buffer and leaves the mode unchanged.
- [ ] "Use Output as Input" copies the active output into the other mode's buffer, switches to that mode, and converts; with a non-empty target buffer, Undo restores that buffer only. The button is disabled while the active output is empty or invalid.
- [ ] After editing a loaded sample, Reload Sample restores the current mode's selected sample (CommonMark basics if none is selected), offers Undo, and leaves options, mode, and tabs unchanged; the selection survives a reload of the page.

### 8.6. Negative & Edge Cases
- [ ] Empty input clears the output pane immediately without console errors or application panics.
- [ ] **Permissive parsing:** malformed HTML (mismatched tags, broken attributes) converts to a best-effort Markdown representation without runtime exceptions or broken application state.
- [ ] **Byte-accurate limit:** an input of exactly 100,000 UTF-8 bytes converts; 100,001 bytes halts conversion, clears output and diff, keeps and persists the text, shows the banner once, and resumes automatically back at the limit.
- [ ] **Nesting depth:** input nested deeper than 512 levels is rejected with the banner before any conversion, clearing the output and diff; input at 512 levels still converts, and so do 600 unclosed `<p>` tags and 600 `<br>` siblings.
- [ ] **Concurrency:** a conversion slowed by a test hook (300 ms async delay between compute and commit) that finishes after newer input neither overwrites the newer output nor refills output cleared by crossing the size limit during the delay. A debounced batch pending when the mode is switched never commits into the other mode's pane.
- [ ] **Forced deadline:** with controlled timers, valid input above 2,000 bytes, and edits every 100 ms for three seconds, the first conversion dispatch occurs by 900 ms. Each later pending batch has the same maximum wait while the converter is idle. With an in-flight run, the latest queued input dispatches as soon as that run finishes; no additional runs accumulate.
- [ ] **`skip_tags` parsing:** empty, whitespace-only, mixed-case (`IMG`), duplicate, and unknown entries produce a working converter — no panic on the cleared field.
- [ ] Switching between conversion modes preserves separate text states for each mode without data loss or layout corruption. With results in both modes, a switch shows the target mode's output unchanged and converts nothing; after an option change in one mode, switching to the other reconverts only that mode's own buffer.
- [ ] Closing or refreshing the browser tab retains typed inputs, selected mode, preview tabs, and sidebar configuration.
- [ ] **Blocked storage:** the app runs and converts normally if `localStorage` is blocked or throws security exceptions.
- [ ] **Malformed JSON:** a non-parseable storage string triggers a clean fallback to defaults without panics.
- [ ] **Type & enum mismatch:** wrong field types or invalid enum variants are rejected, causing a reset to defaults.
- [ ] **Out-of-bound values:** spacing parameters outside the 1–4 range are detected and rejected.
- [ ] **Future version:** storage containing `"version": 2` or higher is dropped and the app boots with the Demo Preset and empty inputs (same rule as corrupt or invalid payloads).
- [ ] **Data loss prevention:** type text, immediately toggle a sidebar option within 500 ms, and refresh — the freshly typed text is fully preserved.
- [ ] **Quota failure:** with `localStorage.setItem` throwing `QuotaExceededError`, editing shows the "could not be saved" banner once while conversion continues; the next successful write clears it.
- [ ] **Multiple tabs:** typing in a second tab shows the "changed in another tab" banner in the first tab once, however many writes arrive, without altering its buffers; the first tab's next successful write clears it.
- [ ] **Escaped views:** `<img src=x onerror=alert(1)>` input appears as literal text in the Source tab and every fidelity view; nothing runs outside the preview.
- [ ] **Preview navigation:** clicking a preview link, even with `target="_self"`, neither navigates the preview nor makes a network request.
- [ ] **Preview stripping:** the injected `srcdoc` contains no user `<base>`, `<meta>` (including `x-dns-prefetch-control`), or `<link>` element and no `target` attribute; 50,000 `>` characters in Markdown → HTML mode render a preview without a stack overflow.

### 8.7. Semantic HTML Diff Validation
- [ ] **False-positive avoidance:** the pipeline fidelity diff engine yields zero changes when comparing structurally equivalent HTML fragments that differ *only* by:
    * **Tag & attribute casing:** character case variations (e.g., `<DIV ID="x">` vs `<div id="x">`).
    * **Attribute order:** lexicographical sequence of keys within elements (e.g., `<a href="#" id="x">` vs `<a id="x" href="#">`).
    * **Attribute quoting:** single, double, or omitted quotes compliant with HTML5.
    * **Void element syntax:** self-closing slashes vs open syntax (e.g., `<br />` vs `<br>`).
    * **Entity representations:** equivalent named, decimal, or hexadecimal formats (e.g., `&amp;`, `&#38;`, `&#x26;`) matching their raw character counterparts.
    * **Insignificant whitespace:** collapsible spacing and line breaks between elements, and leading or trailing whitespace of the fragment and of block-level elements (e.g., `<p>a</p>` vs `<p>a</p>\n`), excluding whitespace inside FR-5 preserved subtrees.
    * **CDATA in foreign content:** `<svg><text><![CDATA[a<b]]></text></svg>` vs `<svg><text>a&lt;b</text></svg>`.
- [ ] **Exact canonical output:** an empty fragment canonicalizes to the empty string and `<p>a</p>` to `<p>a</p>`, with no `<html>` wrapper and no panic.
- [ ] **Diff precision:** `<p><span style="white-space: pre">a  b</span></p>` yields a nonzero diff after the round trip, and a CJK paragraph and a word with a combining mark (U+0301) round-trip with zero diff.
- [ ] **Automated regression suite:** a dedicated set of automated "golden tests" (run in `translation_mode: Faithful`, with the fixed FR-5 round-trip options) executes in the test pipeline to guarantee these safe variations never trigger visual diff errors; the CommonMark-basics sample is part of the golden corpus (zero diff).
