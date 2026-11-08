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
backend (deployable as static files). Conversion is live but debounced, and inputs over 100 KB require a manual "Convert" action.

## 2. Goals

- Let a user type or paste content in one pane and see the converted result update as they type in the other, debounced, with a manual Convert action for inputs over 100 KB.
- Support both conversion directions, selected with a toggle; two-way synchronization is optional.
- Expose the conversion options that materially change the output (the subset in FR-5 for the first release), with a reset to defaults.
- Let users inspect htmd's fidelity with an HTML → Markdown → HTML round trip that shows the original, the round-tripped HTML, and their differences; treat the diff as diagnostic, not proof.
- Show conversion errors and empty input in the UI instead of panicking.
- Be deployable as static files (no backend).

## 3. Non-goals

- User accounts, persistence, or sharing of documents.
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

- Conversion runs on every input change (debounced, target 150–300 ms, to keep
  typing responsive on large documents).
- The direction is selected with a toggle (Markdown → HTML / HTML → Markdown).
- Optional: two-way sync, where editing either pane re-renders the other.
  Guard against feedback loops (an edit programmatically setting the other
  pane must not trigger another conversion).

### FR-4 — Views

Each output pane offers:

- **Rendered** view (HTML rendered in the browser), and
- **Source** view (raw HTML or raw Markdown text).

### FR-5 — Options

Expose a subset of options for each crate, with sensible defaults:

- `htmd`: `heading_style`, `translation_mode` (`Faithful` vs. pure),
  `bullet_list_marker`, `code_block_style`, `skip_tags`, `use_html_blocks`,
  and any other options the crate exposes.
- `pulldown-cmark`: tables, footnotes, strikethrough, task lists, smart
  punctuation, heading attributes, math, and the other extension flags.

Options are grouped by conversion direction and reset to defaults with one
action.

### FR-6 — Round-trip check

For HTML → Markdown mode, provide a "Round trip" action that:

1. converts HTML to Markdown with `htmd`, then
2. converts that Markdown back to HTML with `pulldown-cmark`, and
3. shows the original HTML, the round-tripped HTML, and a diff.

This makes the relationship between the two crates visible and mirrors how
`htmd` tests itself.

### FR-7 — Samples

Ship a small set of example documents (headings, lists, tables, links, code
blocks, inline HTML, math) that can be loaded with one click, plus a reset
action.

### FR-8 — Error and empty-state handling

- Empty input produces empty output without errors.
- 100 KB Threshold: To stop the browser thread from locking up, auto-conversion only works on inputs under 100 KB.
- If an input exceeds 100 KB, disable auto-sync, show a clear warning banner, and require the user to hit a manual "Convert" button.

## 6. Architecture

- **Framework:** [Leptos](https://leptos.dev/) with client-side rendering
  (CSR). No `ssr` feature, no server functions.
- **Target:** `wasm32-unknown-unknown`.
- **Build tool:** [Trunk](https://trunkrs.dev/) (`trunk serve` for development,
  `trunk build --release` for deployment).
- **State:** Leptos signals (`RwSignal`/`Signal`) for input text, direction,
  options, and output.
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

- **Dependency direction:** `demo` depends on the `htmd` library; the library
  must not depend on the demo. The demo is a separate crate (own
  `Cargo.toml`), not a member of the library's package unless the maintainers
  prefer a workspace.

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
- Keyboard accessible: toggle and tabs reachable by keyboard, labels on
  controls.
- Light and dark theme optional.

## 8. Non-functional requirements

- **Performance:** first input-to-output conversion under 100 ms for documents
  up to roughly 100 KB; no visible lag while typing.
- **Size:** document the release `.wasm` size; prefer `wasm-opt` if available.
- **Compatibility:** current versions of Chrome, Firefox, and Safari with
  WebAssembly enabled.
- **Accessibility:** WCAG 2.1 AA where practical.
- **No telemetry**, no external network requests after load.

## 9. Deliverables

1. `demo/` crate with the Leptos CSR application.
2. `demo/README.md` with development and build instructions.
3. Deployment workflow or documented manual deployment (static hosting).
4. This specification, updated to match the implemented behavior.
5. `review.md` with the team's review findings and resolutions.

## 10. Acceptance criteria

- [ ] `cargo build` succeeds for the library and the demo.
- [ ] `cargo clippy -- --deny warnings` and `cargo test` pass for the library.
- [ ] `trunk build --release` produces a static `dist/` that runs from any
      static host.
- [ ] Markdown → HTML and HTML → Markdown both convert live as the user types.
- [ ] Options change the output and can be reset.
- [ ] Conversion errors are shown to the user, not panics.
- [ ] Round-trip view shows original, round-tripped, and diff output.
- [ ] Sample documents load and reset correctly.
- [ ] Works in Chrome, Firefox, and Safari.

## 11. Milestones

1. **M1 — Scaffold:** Leptos CSR app builds to WASM and renders a static page.
2. **M2 — Conversion:** both directions work with hard-coded sample text.
3. **M3 — Live UI:** panes, direction toggle, debounced updates, rendered and
   source views.
4. **M4 — Options and errors:** options panel, error states, reset.
5. **M5 — Round trip and samples:** diff view and sample documents.
6. **M6 — Polish and deploy:** styling, accessibility, static deployment,
   documentation, `review.md`.

## 12. Open questions

- Should the demo live in this repository under `demo/`, or in a separate
  repository?
- Should it depend on the local `htmd` crate by path, or on the crates.io
  release?
- Which static host should the team use (GitHub Pages, Netlify, Cloudflare
  Pages)?
- Is two-way synchronization required, or is a direction toggle sufficient?
- Which options must be exposed in the first release?

## 13. References

- CommonMark dingus: <https://spec.commonmark.org/dingus/>
- `htmd` translation approach: [`translation_approach.md`](../translation_approach.md)
- `htmd` on crates.io: <https://crates.io/crates/htmd>
- `pulldown-cmark` on crates.io: <https://crates.io/crates/pulldown-cmark>
- Leptos book: <https://book.leptos.dev/>
- Trunk: <https://trunkrs.dev/>