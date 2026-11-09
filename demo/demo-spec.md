# htmd / pulldown-cmark live demo — specification

## 1. Purpose

An interactive, client-side web tool that converts CommonMark Markdown (with
the `pulldown-cmark` extensions) to HTML using `pulldown-cmark`, and HTML back
to Markdown using `htmd`. It lets developers compare both conversion
directions via a toggle, see how each crate's options change the output, and
inspect `htmd`'s faithfulness with an HTML → Markdown → HTML round-trip diff.

The layout is inspired by the CommonMark dingus, but the tool is not a strict
conformance checker. The application, including `htmd` and `pulldown-cmark`,
compiles to `wasm32-unknown-unknown`, running entirely in the browser with no
backend (deployable as static files). Conversion is live but debounced, and inputs are strictly capped at 100 KB to guarantee UI responsiveness.

## 2. Goals

- Let a user type or paste content in one pane and see the converted result update as they type in the other, debounced, with a strict size ceiling of 100 KB.
- Support both conversion directions, selected strictly with a direction toggle (one-way conversion per mode).
- Expose the conversion options that materially change the output (the subset in FR-5 for the first release), with a dedicated reset action.
- **Session Protection (UX-Guard):** Prevent accidental data loss from browser reloads by temporarily storing the active workspace layout and input text in the browser's local context, adhering to the boundary in Section 3.
- Let users inspect htmd's fidelity with an HTML → Markdown → HTML round trip that shows the original, the round-tripped HTML, and their differences; treat the diff as diagnostic, not proof.
- Show conversion errors and empty input in the UI instead of panicking.
- Be deployable as static files (no backend).

## 3. Non-goals

- User accounts, cloud persistence, database storage, or remote document sharing link generation. (Note: Transient, client-side session auto-recovery is included strictly for user experience validation).
- Server-side rendering or server-side conversion.
- Editing features beyond plain text input (syntax highlighting is optional).
- Replacing the CommonMark dingus or the crates' own test suites.

## 4. Target users

### Primary
- **`htmd` evaluators** — Rust developers deciding whether to adopt `htmd` for HTML → Markdown conversion. They need to paste representative HTML, see faithful Markdown quickly, inspect how options change the output, and check a round-trip diff.

### Secondary
- **`pulldown-cmark` evaluators** — developers checking Markdown → HTML output and extension flags.
- **`htmd` contributors** — maintainers and contributors reproducing edge cases and capturing before/after output.

### Tertiary (v1 scope: samples and side-by-side views only)
- **Students and technical writers** comparing Markdown and HTML semantics. v1 does not include tutorials or guided explanations.

---

### General Prerequisites & Scope Boundary
* **Prior Knowledge:** All audiences are assumed to know Markdown and HTML. No Rust or WASM knowledge is required to use the demo.
* **Accessibility:** All audiences include keyboard and assistive-technology users.
* **Non-Users / Out of Scope:** The tool is not intended for non-technical authors, production conversion pipelines, or CommonMark conformance testing.

## 5. Functional requirements

### FR-1 — Markdown → HTML

Given Markdown input, render HTML with `pulldown-cmark` (`default-features = false`, `features = ["html"]`):

```rust
let mut options = pulldown_cmark::Options::empty();
options.insert(pulldown_cmark::Options::ENABLE_TABLES | pulldown_cmark::Options::ENABLE_MATH);

let mut html = String::new();
pulldown_cmark::html::push_html(&mut html, pulldown_cmark::Parser::new_ext(markdown, options));
```

**Constraints:**
* **Source View:** Rendered strictly via `textContent`.
* **Preview Pane:** Injected via `srcdoc` into a sandboxed `<iframe>` (`sandbox=""`), wrapped in a full HTML document enforcing CSP: `<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; style-src 'unsafe-inline';">`.

### FR-2 — HTML → Markdown

Given HTML input, convert with `htmd`:

```rust
htmd::HtmlToMarkdown::builder()
    .options(options)
    .build()
    .convert(html) // -> io::Result<String>
```

Errors are surfaced in the UI rather than causing a panic.

### FR-3 — Live conversion

- **Adaptive Debounce:** Conversion runs automatically on every input change. To ensure optimal UX:
  - For small inputs (under 2 KB), conversion triggers **immediately** without delay.
  - For inputs between 2 KB and 100 KB, updates are **debounced** (target 150–300 ms) to prevent UI stuttering.
- The direction is selected with a toggle (Markdown → HTML / HTML → Markdown). One pane acts as a fixed input text area, and the other acts as the output view based on the active mode. Two-way simultaneous sync is explicitly excluded from the scope.

### FR-4 — Views

Each output pane offers:

- **Rendered** view (HTML rendered in the browser), and
- **Source** view (raw HTML or raw Markdown text).

### FR-5 — Options

Expose a subset of options for each crate, with sensible defaults:

- `htmd`: `heading_style`, `translation_mode` (`Faithful` vs. pure), `bullet_list_marker`, `code_block_style`, `skip_tags`, `use_html_blocks`, and any other options the crate exposes.
- `pulldown-cmark`: tables, footnotes, strikethrough, task lists, smart punctuation, heading attributes, math, and the other extension flags.

**Reset Scope:** Options are visually grouped by conversion direction. The "Reset to Defaults" action is global: it resets the configuration flags for **both** directions simultaneously to prevent hidden side effects during mode switches.

### FR-6 — Round-trip check

For HTML → Markdown mode, provide a "Round trip" action that:

1. converts HTML to Markdown with `htmd`, then
2. converts that Markdown back to HTML with `pulldown-cmark`, ensuring it uses the **exact same extension flags** (tables, strikethrough, etc.) currently enabled for `htmd` in the options panel, and
3. compares the original HTML with the round-tripped HTML.

To prevent raw text "noise" caused by insignificant variations (such as whitespace, attribute ordering, entity encoding, or equivalent tags like `<b>` vs `<strong>`), the comparison engine must:
* **Normalize both HTML outputs** by parsing them into DOM trees (via browser APIs or a lightweight parser) and re-serializing them to a canonical string format before diffing.
* **Compute the diff using the `similar` crate** based on a line-by-line or token-based text comparison of the normalized outputs.

The round-trip diff is presented strictly as a diagnostic tool for visibility into crate alignment, not as strict proof of compliance.

### FR-7 — Samples
- The application ships a built-in set of click-to-load example documents representing standard and complex formatting elements.
- **Regression Test Assertions:** To ensure stability, these samples function as regression benchmarks with strictly defined expected outputs:
  - **Inline HTML:** Elements like `<div>content</div>` in Markdown must render as raw block/inline HTML in `pulldown-cmark`, and convert back to valid HTML blocks or pass-through strings via `htmd`.
  - **Math Blocks:** Mathematical formulas enclosed in `$` (inline) or `$$` (block) require the `pulldown-cmark` math extension to be active. Since `htmd` passes unrecognized or raw structures down, math elements must explicitly output as text-wrapped mathematical content (e.g., preserving `$` anchors) without stripping or corrupting formulas during the HTML → Markdown conversion.
- A single-click "Reset to Default Sample" action is provided to restore the initial text area state.

### FR-8 — Error and empty-state handling
- Empty input produces empty output without errors.
- **Malformed HTML Behavior:** Since `html5ever` (via `htmd`) is highly permissive, malformed HTML will not explicitly fail; the UI will gracefully display the resulting parsed/converted layout (even if visually distorted) without breaking the application state.
- **WASM Panic Recovery & Hook:** To prevent silent failures in production, the application must register `console_error_panic_hook` at initialization. In the event of an unhandled Rust panic—which leaves the WASM instance unusable—the UI must catch the failure at the root level, halt further operations, and display a fullscreen fallback overlay instructing the user to reload the page ("Critical Application Error. Please reload the page.").
- 100 KB Input Cap: To prevent browser main thread blocking, the application enforces a strict input size limit of 100 KB.

## 6. Architecture

- **Framework:** [Leptos](https://leptos.dev/) with client-side rendering
  (CSR). No `ssr` feature, no server functions.
- **Target:** `wasm32-unknown-unknown`.
- **Build tool:** [Trunk](https://trunkrs.dev/) (`trunk serve` for development, `trunk build --release` for deployment).
- **State:** Leptos signals (`RwSignal`/`Signal`) for input text, direction, options, and output.
- **Crates:**
  - `leptos`
  - `htmd` (path dependency to the workspace root or a published version)
  - `pulldown-cmark`
  - `wasm-bindgen`, `web-sys` (as needed)
- **Suggested layout:**

```
demo/
  demo-spec.md          # this document
  Cargo.toml
  Trunk.toml
  index.html
  src/
    main.rs             # mounts the Leptos app
    app.rs              # layout, direction toggle, panes
    convert.rs          # thin wrappers around htmd and pulldown-cmark
    options.rs          # option state and UI
    components/
      editor.rs         # textarea + toolbar
      preview.rs        # rendered/source output
      diff.rs           # round-trip diff
  styles/
    main.css
```
- **Dependency direction & Workspace:** The project is organized as a Cargo Workspace within a single repository to satisfy the acceptance criteria (allowing a unified `cargo build` for both the library and the demo). The `demo` crate depends on the local `htmd` library crate strictly via a path dependency (`path = "../"` or relative to the workspace root). The library must not depend on the demo.


## 7. UI sketch

```
+---------------------------------------------------------------+
| htmd <-> pulldown-cmark demo                                  |
| [Markdown -> HTML]  [HTML -> Markdown]     [Options] [Reset]  |
+---------------------------+-----------------------------------+
| Input                     | Output                            |
| (textarea)                | [Rendered] [Source]               |
|                           |                                   |
|                           |                                   |
+---------------------------+-----------------------------------+
| Round trip: [Run]   Original | Round-tripped | Diff           |
+---------------------------------------------------------------+
```

- Responsive: panes stack vertically on narrow screens.
- Keyboard accessible: toggle and tabs reachable by keyboard, labels on controls.
- Light and dark theme optional.

## 8. Non-functional requirements
- **Performance:** First input-to-output conversion under 100 ms for documents up to 100 KB; no visible lag while typing.
- **Performance Measurement:** Verification must be performed using the browser's Performance API (measuring total blocking time between input events and DOM update commits).
- **Size & Build Profile:** The release production `.wasm` payload must maintain a strict size budget **under 1.5 MB (gzipped)**. To meet this target and catch size regressions, the release build must enforce aggressive optimization flags.
- **Compatibility:** Latest stable versions of Chrome (v120+), Firefox (v120+), and Safari (v17+) with WebAssembly enabled, verified via automated headless testing and manual checklists.

## 9. Deliverables

1. `demo/` crate with the Leptos CSR application.
2. `demo/README.md` with development and build instructions.
3. Deployment workflow or documented manual deployment (static hosting).
4. This specification, updated to match the implemented behavior.

## 10. Acceptance criteria

### 10.1. Build & Deployment
- [ ] `cargo build` succeeds for the library and the demo workspace without errors.
- [ ] `cargo clippy -- --deny warnings` and `cargo test` pass for the library.
- [ ] `trunk build --release` produces a static `dist/` directory deployable on any static host, with the final compiled `.wasm` file falling strictly within the 1.5 MB gzipped size budget.

### 10.2. Cross-Browser Compatibility (Headless & Manual)
- [ ] Automated tests pass in headless Chromium, Firefox, and WebKit via `wasm-bindgen-test`.
- [ ] Manual smoke-test checklist passes in latest stable versions of Chrome, Firefox, and Safari (macOS/iOS).

### 10.3. Performance Benchmarks
- [ ] First input-to-output conversion takes under 100 ms, measured via the browser's Performance API.
- [ ] Benchmark text document (exactly 95 KB complex Markdown/HTML) causes no visible lag or main-thread blocking during continuous typing.

### 10.4. Accessibility (WCAG 2.1 AA)
- [ ] Full keyboard navigation: all interactive elements (toggles, tabs, checkboxes) are focusable using the `Tab` key.
- [ ] Focus order is logical, moving sequentially from inputs to options and outputs, with high-visibility focus indicators.
- [ ] Accessible labeling: all inputs and option checkboxes have explicitly associated `<label>` or `aria-label` tags.
- [ ] Color contrast meets a minimum ratio of 4.5:1 for standard text and 3:1 for UI components.
- [ ] Error messages and size limit banners use `aria-live="assertive"` to be instantly announced by screen readers.

### 10.5. Functional Happy Paths
- [ ] Markdown → HTML and HTML → Markdown convert live within 150–300 ms after typing stops.
- [ ] Options panel dynamically updates the active conversion output and resets completely to defaults via a single click.
- [ ] Round-trip view successfully renders original HTML, round-tripped HTML, and a line-by-line diff generated by the `similar` crate after DOM tree normalization.
- [ ] Clickable sample documents load immediately into the input pane and update the output state.

### 10.6. Negative & Edge Cases
- [ ] Empty input clears the output pane immediately without throwing console errors or application panics.
- [ ] Malformed HTML input (e.g., unclosed tags, unescaped scripts) is handled safely by `htmd` without crashing the WASM runtime.
- [ ] Input exceeding 100 KB immediately halts conversion, clears the output, and renders the standard error banner: "Input exceeds the 100 KB maximum limit for client-side processing."
- [ ] Conflicting option combinations (e.g., specific combinations of `translation_mode` and `heading_style`) resolve gracefully without freezing the UI.

## 11. Milestones
0. **M0 — WASM Spike (Verification):** Create a minimal standalone target or test to compile the 
1. **M1 — Scaffold:** Leptos CSR app builds to WASM and renders a static page.
2. **M2 — Conversion:** both directions work with hard-coded sample text.
3. **M3 — Live UI:** panes, direction toggle, debounced updates, rendered and source views.
4. **M4 — Options and errors:** options panel, error states, reset.
5. **M5 — Round trip and samples:** diff view and sample documents.
6. **M6 — Polish and deploy:** styling, accessibility, static deployment, and final documentation.
### Milestone Review Process
The milestone review occurs at the end of M6 (Polish and deploy) before final release acceptance. The project team compiles internal findings and resolutions into a temporary `review.md` process artifact to log code quality and spec alignment. Final verification and closure of the review findings are resolved and signed off by the Project Lead / Course Instructor (or External Auditor) to prevent a conflict of interest.

## 12. Open questions

- Which static host should the team use (GitHub Pages, Netlify, Cloudflare Pages)?
- Which options must be exposed in the first release?

## 13. References

- CommonMark dingus: <https://spec.commonmark.org/dingus/>
- `htmd` translation approach: [`translation_approach.md`](../translation_approach.md)
- `htmd` on crates.io: <https://crates.io/crates/htmd>
- `pulldown-cmark` on crates.io: <https://crates.io/crates/pulldown-cmark>
- Leptos book: <https://book.leptos.dev/>
- Trunk: <https://trunkrs.dev/>