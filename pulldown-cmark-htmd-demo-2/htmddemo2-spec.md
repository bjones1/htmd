# Spec: htmd Live Demo (Leptos, client-side WASM)

> **Audience:** an LLM coding agent working inside a checkout of `dkaimekinmasters/paradigms-htmd`.
> **Status:** draft. Facts about htmd were checked against htmd **0.5.5** and Leptos **0.8.x** (Nov 2026). Re-check `src/options.rs` before coding, since the option set may have changed.

---

## 1. Goal

Add an interactive, in-browser playground for htmd, modeled on the CommonMark "dingus" (spec.commonmark.org/dingus). The user types or pastes **HTML** on the left and sees the **Markdown** that htmd produces on the right. The output updates live as they type. htmd is compiled to WebAssembly, so no server is needed.

The demo is a static site built with **Leptos (CSR only)** and **Trunk**, and deployed to GitHub Pages.

### 1.1 Non-goals

- No SSR, hydration, server functions, or `cargo-leptos`.
- Do not change htmd's public API or behavior. If the library needs a change to compile for `wasm32-unknown-unknown`, stop and report it rather than patching around it.
- No custom element handlers in the UI, because they are Rust closures and can't be configured at runtime.
- No user accounts, persistence beyond the URL, or analytics.

---

## 2. Background the agent needs

### 2.1 htmd API (verified, 0.5.6)

```rust
use htmd::{HtmlToMarkdown, options::*};

let converter = HtmlToMarkdown::builder()
    .options(Options { heading_style: HeadingStyle::Setex, ..Default::default() })
    .skip_tags(vec!["script", "style"])
    .build();
let md: String = converter.convert(html)?;   // returns Result
```

`htmd::options::Options` fields and defaults:

| Field | Type | Variants | Default |
|---|---|---|---|
| `heading_style` | `HeadingStyle` | `Atx`, `Setex` | `Atx` |
| `hr_style` | `HrStyle` | `Dashes`, `Asterisks`, `Underscores` | `Asterisks` |
| `br_style` | `BrStyle` | `TwoSpaces`, `Backslash` | `TwoSpaces` |
| `link_style` | `LinkStyle` | `Inlined`, `InlinedPreferAutolinks`, `Referenced` | `Inlined` |
| `link_reference_style` | `LinkReferenceStyle` | `Full`, `Collapsed`, `Shortcut` | `Full` |
| `code_block_style` | `CodeBlockStyle` | `Indented`, `Fenced` | `Fenced` |
| `code_block_fence` | `CodeBlockFence` | `Tildes`, `Backticks` | `Backticks` |
| `bullet_list_marker` | `BulletListMarker` | `Asterisk`, `Dash` | `Asterisk` |
| `ul_bullet_spacing` | `u8` | | `3` |
| `ol_number_spacing` | `u8` | | `2` |
| `preformatted_code` | `bool` | | `false` |
| `translation_mode` | `TranslationMode` | `Pure`, `Faithful` | `Pure` |

Notes:
- `Options` derives only `Debug`, not `Clone` or `PartialEq`. The enums derive `Clone, Copy, PartialEq, Debug`. Keep the demo's own state in a demo-side struct and build `Options` from it on each conversion.
- `skip_tags` is a builder method, not an `Options` field.
- The library depends on `html5ever`, `markup5ever_rcdom`, and `phf`. These should build for wasm32. Confirm this first (see §8, step 1).
- The crate uses **edition 2024**, so the toolchain must be Rust 1.85 or newer.

### 2.2 Repo shape

htmd is a single crate, not a workspace. The top level has `src/`, `tests/`, `benches/`, `examples/`, `.github/`, `Cargo.toml`, and `README.md`.

### 2.3 Leptos 0.8 CSR essentials

- `leptos = { version = "0.8", features = ["csr"] }`
- `use leptos::prelude::*;`
- Entry point: `leptos::mount::mount_to_body(App);`
- Signals: `let (get, set) = signal(x);`, `RwSignal::new(x)`, `Memo::new(move |_| ...)`
- Input binding: `<textarea prop:value=text on:input:target=move |ev| set_text.set(ev.target().value()) />`
- Use 0.8 idioms. Do not use 0.6-era APIs such as `create_signal` or the `cx` parameter.

---

## 3. Placement and packaging

```
demo/
├── Cargo.toml          # standalone crate; contains an empty [workspace] table
├── Trunk.toml
├── index.html          # Trunk entry
├── style.css
├── README.md           # how to run and deploy
└── src/
    ├── main.rs         # mount_to_body
    ├── app.rs          # <App/> layout
    ├── state.rs        # DemoOptions + conversion (pure Rust, host-testable)
    ├── permalink.rs    # URL hash encode/decode (pure Rust, host-testable)
    ├── examples.rs     # built-in sample HTML snippets
    └── components/
        ├── editor.rs
        ├── output.rs   # tabs: Markdown / Preview / Input
        └── options_panel.rs
```

Requirements:
- `demo/Cargo.toml` depends on the library with `htmd = { path = ".." }`, so the demo always tracks the current source.
- Put an empty `[workspace]` table in `demo/Cargo.toml` so Cargo treats it as its own root. This keeps the demo from affecting `cargo build` or `cargo test` at the repo root.
- Add `exclude = ["demo"]` (or extend the existing `exclude` list) under `[package]` in the root `Cargo.toml`, so the demo doesn't ship to crates.io. This is the **only** allowed edit to library files.
- Demo dependencies:
  - `leptos` 0.8 (`csr`)
  - `pulldown-cmark`, for the Markdown→HTML preview (default features off, `html` on)
  - `console_error_panic_hook`
  - `web-sys` with the needed features (`Window`, `Location`, `History`, `Navigator`, `Clipboard`, `Performance`)
  - `base64`, for permalinks
  - `miniz_oxide` or `lz-str`, optional, for compressing permalinks
  - Avoid anything that pulls in `getrandom`. If something does, enable its wasm JS backend.
- Release profile in `demo/Cargo.toml`: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`. In `index.html`, use `<link data-trunk rel="rust" data-wasm-opt="z" />`.

---

## 4. Functional requirements

### 4.1 Layout

- A header with the title "htmd playground", the htmd version (read from `htmd`'s Cargo metadata at build time, or hardcoded from `Cargo.toml` with a TODO), and links to GitHub, docs.rs, and crates.io.
- A two-column body: **input on the left, output on the right**, each about 50% wide. On narrow screens (under 800 px) the columns stack vertically.
- An options panel in a collapsible row above the panes or in a sidebar. It starts expanded on desktop and collapsed on mobile.
- A status line under the output that shows conversion time in ms (from `performance.now()`), the output character count, or an error message.

### 4.2 Input pane

- A monospace `<textarea>` with `spellcheck="false"`, an "HTML" label, and tab-key insertion. Syntax highlighting is not required for v1.
- An **Examples** dropdown that replaces the input with a preset. It must include at least these presets: Basic, Headings & paragraphs, Lists (nested, ordered with start), Links & images, Code (inline and block with a language class), Tables, Blockquotes, Mixed / real-world article snippet, and Edge cases (empty tags, entities, whitespace, `<br>`, comments, `<script>`/`<style>`).
- A **Clear** button.

### 4.3 Live conversion

- Convert on every input change and every options change. Debounce input by about 100 ms. Options changes apply immediately.
- Run the conversion in a `Memo` derived from `(input, options)`. Building a new `HtmlToMarkdown` on each run is acceptable, since construction is cheap.
- If `convert` returns `Err`, keep showing the **last successful output** dimmed and put the error in the status line. The app must not panic.
- Guard against very large input. Above 1 MB, show a warning and require a "Convert anyway" click.

### 4.4 Output pane: three tabs

1. **Markdown** (default): the raw htmd output in a read-only monospace `<pre>`, with a **Copy** button (Clipboard API, with a fallback message if it's unavailable).
2. **Preview**: the Markdown output rendered back to HTML with `pulldown-cmark` (enable tables and strikethrough). This shows round-trip fidelity, which matters most for `TranslationMode::Faithful`.
3. **Input**: the user's original HTML, rendered.

**Security, which is mandatory:** Tabs 2 and 3 must render inside `<iframe sandbox srcdoc=...>`, and the `sandbox` attribute must not include `allow-scripts` or `allow-same-origin`. Never use `inner_html` with user-controlled content in the main document. The iframe should get a small embedded stylesheet so the preview looks readable.

### 4.5 Options panel

Show one control for every `Options` field listed in §2.1:
- Each enum gets a `<select>` or a segmented radio group. The labels are human-readable, for example "ATX (`# Heading`)" and "Setext (underlined)".
- `ul_bullet_spacing` and `ol_number_spacing` get number inputs clamped to 1–4.
- `preformatted_code` gets a checkbox.
- `skip_tags` gets a text input that takes a comma- or space-separated list of tag names. Trim and lowercase the names, and drop empty entries.
- Hide `code_block_fence` when `code_block_style == Indented`, and hide `link_reference_style` unless `link_style == Referenced`. Hidden values are kept in state, not reset.
- Add a **Reset to defaults** button.
- Each control has a tooltip or short help text taken from the htmd doc comments where they exist.

### 4.6 Permalinks

- The URL hash stores the full state (input plus every option that differs from its default). Example form: `#v=1&o=<compact options>&h=<base64url(deflate(html))>`.
- Update the hash with `history.replaceState`, debounced to about 500 ms, so typing doesn't flood the browser history.
- On load, decode the hash if present. If decoding fails, fall back to the default example and show a non-blocking notice.
- Add a **Share** button that copies the full URL.
- Version the format with `v=1` so it can change later.

### 4.7 Keyboard and accessibility

- All controls are reachable by keyboard, and the tabs use proper `role="tablist"` and `role="tab"` attributes with arrow-key switching.
- Visible focus styles, labels tied to their inputs, and WCAG AA contrast.
- Respect `prefers-color-scheme` with light and dark themes using CSS variables. No UI framework or Tailwind; plain CSS in `style.css`.

---

## 5. Non-functional requirements

- **Bundle size:** aim for under 1 MB of gzipped `.wasm`, and report the actual size in the PR.
- **Performance:** converting the "real-world article" example should take under 10 ms on a mid-range laptop.
- **Browsers:** current Chrome, Firefox, and Safari. No IE.
- **No network calls** at runtime other than loading the site's own assets.
- **Code quality:** `cargo clippy --target wasm32-unknown-unknown -- -D warnings` and `cargo fmt --check` must pass in `demo/`.

---

## 6. Testing

- `state.rs` and `permalink.rs` must be pure Rust with no `web-sys`, so their tests run with plain `cargo test` in `demo/`. Required tests:
  - Each `DemoOptions` setting maps to the right `htmd::options::Options` value.
  - Every option round-trips through encode/decode, including Unicode and empty input.
  - Malformed hashes decode to `Err` without panicking.
  - Each example converts without error under default options.
- Optional: one smoke test with `wasm-bindgen-test` in headless browser mode that mounts the app and checks that the output contains `# ` for an `<h1>` input.
- Manual QA checklist, included in `demo/README.md`: change every option, use every tab, open a permalink in a fresh tab, check mobile layout, check dark mode, and confirm that `<script>alert(1)</script>` in the input never runs.

---

## 7. CI and deployment

Add `.github/workflows/demo.yml`:
- **Triggers:** pushes to the default branch and pull requests that touch `src/**`, `Cargo.toml`, or `demo/**`.
- **Job `check`** (runs on PRs and default branch commits): install the `wasm32-unknown-unknown` target, then in `demo/` run `cargo fmt --check`, `cargo clippy --target wasm32-unknown-unknown -- -D warnings`, `cargo test`, and `trunk build --release`. This job's main value is catching library changes that break wasm builds.
- **Job `deploy`** (default branch only, after `check` passes): run `trunk build --release --public-url /htmd/`, upload `demo/dist` with `actions/upload-pages-artifact`, and deploy with `actions/deploy-pages`. Permissions: `pages: write` and `id-token: write`.
- Install Trunk from a prebuilt binary (`cargo binstall trunk` or the release tarball) rather than `cargo install`, which compiles from source and takes minutes.
- Do not change the existing CI workflows.

---

## 8. Implementation order

1. **Spike:** create a bare `demo/` crate that calls `htmd::convert("<h1>x</h1>")` and build it with `cargo build --target wasm32-unknown-unknown`. If it fails, stop and report the cause, for example a dependency that doesn't support wasm.
2. Add the Trunk and Leptos skeleton: `index.html`, `main.rs`, and a textarea with the raw Markdown output.
3. Write `state.rs` with `DemoOptions` and its tests, then build the options panel.
4. Add the output tabs and sandboxed iframes, then the `pulldown-cmark` preview.
5. Add the examples and status line (timing and errors).
6. Write `permalink.rs` with its tests and wire up hash sync.
7. Polish: responsive layout, dark mode, accessibility pass, bundle-size tuning.
8. Set up CI and deployment, then the README updates.

Keep each step a separate commit.

---

## 9. Acceptance criteria

- [ ] `cd demo && trunk serve` opens a working playground. `<h1>Hi</h1>` shows `# Hi`, and switching to Setext shows `Hi\n==`.
- [ ] Every `Options` field and `skip_tags` can be changed in the UI and has a visible effect on at least one example.
- [ ] The Preview and Input tabs render in sandboxed iframes, and no script from the input runs.
- [ ] A permalink pasted into a new tab reproduces the input and options exactly.
- [ ] `cargo test`, `cargo publish --dry-run`, and `cargo package --list` at the repo root are unaffected (no `demo/` files are packaged).
- [ ] The `demo.yml` check job passes, and the deploy job publishes to GitHub Pages.
- [ ] The PR description reports the gzipped wasm size and the conversion time for the article example.
