# Spec: htmd Live Demo (Leptos, client-side WASM)

> **Audience:** an LLM coding agent working inside a checkout of `dkaimekinmasters/paradigms-htmd`.
> **Status:** draft. Facts about htmd were checked against this repository's source at htmd **0.5.5** (`Cargo.toml`, `src/lib.rs`, `src/options.rs`) and against Leptos **0.8.x** (Nov 2026). Re-check `src/options.rs` and `src/lib.rs` before coding, since the option set may have changed.

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

### 2.1 htmd API (verified, 0.5.5)

```rust
use htmd::{HtmlToMarkdown, options::*};

let converter = HtmlToMarkdown::builder()
    .options(Options { heading_style: HeadingStyle::Setex, ..Default::default() })
    .skip_tags(vec!["script", "style"])   // Vec<&str>
    .scripting_enabled(true)              // default true
    .build();
let md: String = converter.convert(html)?;   // returns std::io::Result<String>
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

Builder-only settings (not `Options` fields):

| Setting | Type | Library default | Effect |
|---|---|---|---|
| `skip_tags` | `Vec<&str>` | none skipped | Drops the listed elements and their content. |
| `scripting_enabled` | `bool` | `true` | If true, html5ever parses `<noscript>` content as raw text; if false, as normal DOM. |

Notes:
- `Options` derives `Debug, Clone, PartialEq`; the enums derive `Clone, Copy, PartialEq, Debug`. The demo still keeps its own `DemoOptions` struct, because the UI state must also hold the two builder-only settings and must encode to and decode from permalinks (§4.6). `DemoOptions` builds `Options` and the builder on each conversion.
- The library already clamps `ul_bullet_spacing` and `ol_number_spacing` to 1–4.
- `BrStyle` is ignored in `TranslationMode::Faithful`; in that mode `<br>` is always written as `<br>` (see the `BrStyle` doc comment).
- `convert` returns `std::io::Result`, but in 0.5.5 `html_to_tree` always returns `Ok`, so `convert` never returns `Err` in practice. The realistic failure is a **panic** inside the library (see §4.3).
- The library depends on `html5ever`, `markup5ever_rcdom`, and `phf`. These should build for wasm32. Confirm this first (see §8, step 1).
- The crate uses **edition 2024**, so the toolchain must be Rust 1.85 or newer. CI uses the latest stable toolchain, as the existing `ci.yaml` does.

### 2.2 Repo shape

htmd is a single crate, not a workspace. The top level has `src/`, `tests/`, `benches/`, `examples/`, `.github/`, `Cargo.toml`, `README.md`, `AGENTS.md`, and `translation_approach.md`. The default branch is `main`.

### 2.3 Leptos 0.8 CSR essentials

- `leptos = { version = "0.8", features = ["csr"] }`
- `use leptos::prelude::*;`
- Entry point: `leptos::mount::mount_to_body(App);`
- Signals: `let (get, set) = signal(x);`, `RwSignal::new(x)`, `Memo::new(move |_| ...)`
- Input binding: `<textarea prop:value=text on:input:target=move |ev| set_text.set(ev.target().value()) />`
- Timers: `set_timeout_with_handle` returns a handle whose `clear()` cancels a pending timer; use it for debouncing.
- Use 0.8 idioms. Do not use 0.6-era APIs such as `create_signal` or the `cx` parameter.

### 2.4 Repository conventions

`AGENTS.md` applies to everything under `demo/`: its commenting guide, `cargo fmt` with default settings, and CRLF line endings in the working tree (write CRLF explicitly; `cargo fmt` rewrites files to LF).

---

## 3. Placement and packaging

```
demo/
├── Cargo.toml          # standalone crate; contains an empty [workspace] table
├── Cargo.lock          # committed (the demo is an application)
├── .gitignore          # target/ and dist/
├── .cargo/config.toml  # only if needed for getrandom or web-sys cfg flags
├── build.rs            # exports HTMD_VERSION and BUILD_COMMIT
├── Trunk.toml
├── index.html          # Trunk entry; also holds the static crash banner (§4.3)
├── style.css
├── README.md           # how to run and deploy, and the manual QA checklist
└── src/
    ├── lib.rs          # declares all modules below; host-testable
    ├── main.rs         # panic hooks + mount_to_body only
    ├── app.rs          # <App/> layout, debounce, timing
    ├── state.rs        # DemoOptions + conversion (pure Rust, no web-sys)
    ├── permalink.rs    # URL hash encode/decode (pure Rust, no web-sys)
    ├── examples.rs     # built-in sample HTML snippets (pure Rust)
    └── components/
        ├── editor.rs
        ├── output.rs   # tabs: Markdown / Preview / Input
        └── options_panel.rs
```

Requirements:
- `demo/Cargo.toml` depends on the library with `htmd = { path = ".." }`, so the demo always tracks the current source.
- Put an empty `[workspace]` table in `demo/Cargo.toml` so Cargo treats it as its own root. This keeps the demo from affecting `cargo build` or `cargo test` at the repo root.
- **Files outside `demo/` that may change** (and no others):
  1. Root `Cargo.toml`: add `exclude = ["demo"]` under `[package]` (there is no existing `exclude` list), so the demo doesn't ship to crates.io. This is the only edit to library files.
  2. New file `.github/workflows/demo.yml` (§7).
  The root `README.md`, `.gitignore`, and existing workflows stay unchanged; the demo's ignore rules live in `demo/.gitignore`.
- Demo dependencies:
  - `leptos` 0.8 (`csr`)
  - `pulldown-cmark` 0.13 (the version the root crate's tests use), for the Markdown→HTML preview (default features off, `html` on)
  - `console_error_panic_hook`
  - `wasm-bindgen-futures`, to await the Clipboard API's promises
  - `web-sys` with at least these features: `Window`, `Document`, `Element`, `HtmlElement`, `Location`, `History`, `Navigator`, `Clipboard`, `Performance`, `HtmlTextAreaElement`, `HtmlSelectElement`, `HtmlInputElement`, `KeyboardEvent`, `HashChangeEvent`. Add others as needed. Use a `web-sys` version in which the `Clipboard` binding is stable; if it requires `--cfg=web_sys_unstable_apis`, set that in `demo/.cargo/config.toml` and say so in `demo/README.md`.
  - `base64` (URL-safe alphabet, no padding) and `miniz_oxide` (raw DEFLATE), for permalinks
  - Build dependency: `toml`, for `build.rs`
  - Avoid anything that pulls in `getrandom`. Check with `cargo tree -i getrandom --target wasm32-unknown-unknown` in `demo/`. If it appears and can't be removed, enable getrandom 0.3's `wasm_js` feature **and** set `rustflags = ['--cfg', 'getrandom_backend="wasm_js"']` for the wasm32 target in `demo/.cargo/config.toml`.
- Release profile in `demo/Cargo.toml`: `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `panic = "abort"`. In `index.html`, use `<link data-trunk rel="rust" data-wasm-opt="z" />`.
- `build.rs` reads `../Cargo.toml` with `toml`, and emits `cargo:rustc-env=HTMD_VERSION=<package.version>` and `cargo:rustc-env=BUILD_COMMIT=<first 7 chars of $GITHUB_SHA, or "local">`, plus `cargo:rerun-if-changed=../Cargo.toml` and `cargo:rerun-if-env-changed=GITHUB_SHA`. (`env!("CARGO_PKG_VERSION")` would give the demo's own version, not htmd's.)

---

## 4. Functional requirements

### 4.1 Layout

- A header with the title "htmd playground", the text "htmd `HTMD_VERSION` (built from `BUILD_COMMIT`)", and three links:
  - GitHub: `https://github.com/dkaimekinmasters/paradigms-htmd` (the source the demo is built from)
  - docs.rs: `https://docs.rs/htmd`, labelled "docs (latest release)"
  - crates.io: `https://crates.io/crates/htmd`, labelled "crates.io (latest release)"
- A two-column body: **input on the left, output on the right**, each about 50% wide. Below 800 px viewport width the columns stack vertically, input first.
- An options panel in a collapsible row **above the panes**. It starts expanded at 800 px and wider, and collapsed below 800 px.
- A status line under the output that shows either the conversion time in ms (one decimal, from `performance.now()`) and the output length in characters (Unicode scalar values, `str::chars().count()`), or a warning/error message.

### 4.2 Input pane

- A monospace `<textarea>` with `spellcheck="false"` and an "HTML" `<label>`. Syntax highlighting is not required for v1.
- Tab-key insertion: Tab inserts a tab character. **Esc** turns this off until the textarea next gains focus, so that Tab and Shift+Tab move focus as normal. Show the hint "Esc, then Tab, to leave the editor" under the textarea while it has focus.
- An **Examples** dropdown that replaces the input with a preset. It must include at least these presets, with at least the listed content:

  | Preset | Must contain |
  |---|---|
  | Basic | a paragraph with `<strong>` and `<em>` |
  | Headings & paragraphs | `<h1>`, `<h2>`, `<h3>` and paragraphs |
  | Lists (nested, ordered with start) | nested `<ul>`, `<ol start="3">` with 10+ items |
  | Links & images | a link whose text equals its URL, a titled link, 2+ other links, an `<img>` with alt text |
  | Code | inline `<code>` with runs of 2+ spaces, `<pre><code class="language-rust">` |
  | Tables | a table with a header row |
  | Blockquotes | nested blockquotes |
  | Mixed / real-world article snippet | 15–25 KB of article HTML, including `<hr>`, `<br>`, headings, lists, links, and a code block |
  | Edge cases | empty tags, entities, whitespace runs, `<br>`, comments, `<script>`/`<style>`, `<noscript>` with markup inside |

- A **Clear** button.

### 4.3 Live conversion

- Convert on every input change and every options change. Input changes go through a `debounced_input` signal that `app.rs` updates ~100 ms after the last keystroke (restart a `set_timeout_with_handle` timer on each input). Options changes apply immediately.
- The conversion runs in a `Memo` in `app.rs` derived from `(debounced_input, options)`. Its closure reads `performance.now()` before and after calling the pure function `state::convert(&str, &DemoOptions) -> Result<String, String>`. Building a new `HtmlToMarkdown` on each run is acceptable.
- If `state::convert` returns `Err` (defensive; not expected with 0.5.5), keep showing the **last successful output** dimmed and put the error in the status line.
- **Panics:** on wasm32 a panic cannot be caught, and it leaves the wasm instance unusable. `main.rs` installs a panic hook that calls `console_error_panic_hook::hook` and then, using `web-sys` directly (not signals), un-hides a static banner in `index.html`: "The converter crashed. Reload the page; your input is in the URL." The host tests in §6 must show that no example panics under any tested option combination.
- **Large input:** measure size as UTF-8 bytes (`str::len()`). Above 1 MiB (1,048,576 bytes), pause conversion, show a warning in the status line, and show a "Convert anyway" button. The confirmation holds until the input next drops to 1 MiB or less. This applies to typing, pasting, and loading a permalink.

### 4.4 Output pane: three tabs

1. **Markdown** (default): the raw htmd output in a read-only monospace `<pre>`, with a **Copy** button (Clipboard API, with a fallback message if it's unavailable or the page isn't a secure context).
2. **Preview**: the Markdown output rendered back to HTML with `pulldown-cmark` (enable tables and strikethrough). This shows round-trip fidelity, which matters most for `TranslationMode::Faithful`.
3. **Input**: the user's original HTML, rendered.

**Security, which is mandatory:**
- Tabs 2 and 3 render inside `<iframe sandbox srcdoc=...>`. The `sandbox` attribute is present with no tokens (in particular no `allow-scripts`, `allow-same-origin`, `allow-popups`, or `allow-top-navigation`).
- Never use `inner_html` with user-controlled content in the main document.
- Every `srcdoc` document starts with this fixed head, before any user-derived content:

  ```html
  <!doctype html><html><head>
  <meta http-equiv="Content-Security-Policy"
        content="default-src 'none'; style-src 'unsafe-inline'; img-src data:; font-src data:">
  <base target="_blank">
  <style>/* small embedded stylesheet */</style>
  </head><body>
  ```

  The CSP stops user HTML (and Faithful-mode raw HTML) from loading remote images, styles, or fonts, which would break the no-network rule in §5 and expose viewers' IP addresses. Remote images therefore show as broken; that is intended. `<base target="_blank">` makes ordinary links do nothing, because the sandbox blocks popups. A link with its own `target="_self"` can still navigate the frame; the next conversion re-sets `srcdoc` and restores it.
- Each iframe fills the pane's height and scrolls internally; it is not auto-sized, since that would need scripts.

### 4.5 Options panel

Show one control for every `Options` field listed in §2.1 and for both builder-only settings:
- Each enum gets a `<select>` or a segmented radio group. The labels are human-readable, for example "ATX (`# Heading`)" and "Setext (underlined)".
- `ul_bullet_spacing` and `ol_number_spacing` get number inputs clamped to 1–4.
- `preformatted_code` gets a checkbox.
- `scripting_enabled` gets a checkbox labelled "Treat `<noscript>` content as raw text", checked by default.
- `skip_tags` gets a text input that takes a comma- or space-separated list of tag names. Trim and lowercase the names, drop empty entries, and drop entries that don't match `[a-z][a-z0-9-]*` (show which were dropped under the input).
- Dependent controls:
  - Hide `code_block_fence` when `code_block_style == Indented`.
  - Hide `link_reference_style` unless `link_style == Referenced`.
  - Disable `br_style` when `translation_mode == Faithful`, with the note "Faithful mode always writes `<br>`".
  - Hidden or disabled values are kept in state, not reset.
- **Demo defaults** equal the library defaults: `Options::default()`, an empty `skip_tags`, and `scripting_enabled = true`. In particular, `translation_mode` starts as `Pure`.
- Add a **Reset to defaults** button that restores all of the above, including `skip_tags` and `scripting_enabled`. It doesn't change the input.
- Each control has a short help text. Write it by hand; where htmd has a doc comment for the field or variant, base the text on it.

### 4.6 Permalinks

- The URL hash stores the full state: the input, plus every setting (from §4.5) that differs from its demo default.
- **Format `v=1`**, which is fixed once released:

  ```
  #v=1&o=<options>&s=<skip_tags>&h=<html>
  ```

  - Parameters are separated by `&`, may appear in any order, and each may appear at most once. `v` is required. `o`, `s`, and `h` are omitted when they would be empty.
  - `o`: comma-separated `key:value` pairs, only for settings that differ from the default:

    | Key | Setting | Values |
    |---|---|---|
    | `hs` | `heading_style` | `atx`, `setext` |
    | `hr` | `hr_style` | `dashes`, `asterisks`, `underscores` |
    | `br` | `br_style` | `spaces`, `backslash` |
    | `ls` | `link_style` | `inline`, `autolink`, `ref` |
    | `lr` | `link_reference_style` | `full`, `collapsed`, `shortcut` |
    | `cb` | `code_block_style` | `indented`, `fenced` |
    | `cf` | `code_block_fence` | `tildes`, `backticks` |
    | `bm` | `bullet_list_marker` | `asterisk`, `dash` |
    | `us` | `ul_bullet_spacing` | `1`–`4` |
    | `os` | `ol_number_spacing` | `1`–`4` |
    | `pc` | `preformatted_code` | `0`, `1` |
    | `tm` | `translation_mode` | `pure`, `faithful` |
    | `se` | `scripting_enabled` | `0`, `1` |

  - `s`: the normalized `skip_tags` joined with `,` (the allowed tag-name characters are URL-safe).
  - `h`: `base64url_nopad(raw_deflate(utf8(html)))`, using `miniz_oxide` at level 6.
- Decoding returns `Err` (never panics) for: a missing or unknown `v`, an unknown parameter, a duplicate parameter, an unknown `o` key or value, an invalid tag name in `s`, invalid base64, invalid DEFLATE data, a decompressed size over 2 MiB (use `decompress_to_vec_with_limit`), or invalid UTF-8.
- Update the hash with `history.replaceState`, debounced to about 500 ms, so typing doesn't flood the browser history.
- If the encoded hash would be longer than 32,768 characters, stop updating it, and show the notice "Input too large for a permalink" until it fits again.
- On load, decode the hash if present. If decoding fails, fall back to the default example and show a non-blocking notice. Also listen for `hashchange` (fired by manual edits and Back/Forward, not by `replaceState`) and load the new state the same way.
- Add a **Share** button. It first flushes any pending hash update, then copies the full URL. If the state is over the size limit, it shows the size notice instead of copying.

### 4.7 Keyboard and accessibility

- All controls are reachable by keyboard; the editor follows the Esc rule in §4.2. The tabs use proper `role="tablist"`, `role="tab"`, and `role="tabpanel"` attributes with arrow-key switching.
- Visible focus styles, labels tied to their inputs, and WCAG AA contrast.
- Respect `prefers-color-scheme` with light and dark themes using CSS variables. No UI framework or Tailwind; plain CSS in `style.css`.

---

## 5. Non-functional requirements

- **Bundle size:** aim for under 1 MB of gzipped `.wasm`, and report the actual size in the PR.
- **Performance:** converting the "Mixed / real-world article snippet" example (15–25 KB) should take under 10 ms. Measure in a `trunk build --release` build in current desktop Chrome, as the median of 20 conversions read from the status line, and report the machine's CPU in the PR.
- **Browsers:** current Chrome, Firefox, and Safari. No IE.
- **No network calls** at runtime other than loading the site's own assets, including from the preview iframes (enforced by the CSP in §4.4).
- **Code quality:** in `demo/`, these must pass:
  - `cargo fmt --check`
  - `cargo clippy --target wasm32-unknown-unknown -- -D warnings`
  - `cargo clippy --all-targets -- -D warnings` (host; this is the only lint run that covers `#[cfg(test)]` code)

---

## 6. Testing

- `state.rs`, `permalink.rs`, and `examples.rs` must be pure Rust with no `web-sys`, so their tests run with plain `cargo test` in `demo/` on the host. Required tests:
  - Each `DemoOptions` setting maps to the right `htmd::options::Options` value or builder call.
  - Every setting round-trips through permalink encode/decode, including Unicode input, empty input, and non-empty `skip_tags`.
  - The `o` encoding omits settings that equal their defaults.
  - Each malformed-hash case listed in §4.6 decodes to `Err` without panicking.
  - Every example converts without error or panic under the default settings, under each setting changed alone to each of its non-default values, and with every setting at a non-default value at once.
  - Coverage: for every setting, at least one example gives different output for two of its values, with any dependent settings applied (for example `link_style = Referenced` when testing `link_reference_style`, and `translation_mode = Pure` when testing `br_style`).
- Optional: one smoke test with `wasm-bindgen-test` in headless browser mode that mounts the app and checks that the output contains `# ` for an `<h1>` input.
- Manual QA checklist, included in `demo/README.md`: change every setting, use every tab, open a permalink in a fresh tab, edit the hash by hand, check mobile layout, check dark mode, leave the editor with Esc then Tab, confirm that `<script>alert(1)</script>` in the input never runs, and confirm in the browser's network panel that `<img src="https://example.com/x.png">` in the input makes no request.

---

## 7. CI and deployment

Add `.github/workflows/demo.yml`:
- **Triggers:**

  ```yaml
  on:
    push:
      branches: [main]
      paths: ["src/**", "Cargo.toml", "demo/**", ".github/workflows/demo.yml"]
    pull_request:
      paths: ["src/**", "Cargo.toml", "demo/**", ".github/workflows/demo.yml"]
    workflow_dispatch:
  ```

- **Workflow permissions:** `contents: read`.
- **Job `check`** (runs for every trigger): install the latest stable toolchain with the `wasm32-unknown-unknown` target, install Trunk, then in `demo/` run `cargo fmt --check`, `cargo clippy --target wasm32-unknown-unknown -- -D warnings`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `trunk build --release`. This job's main value is catching library changes that break wasm builds.
- **Job `deploy`** (`needs: check`, and only if `github.event_name != 'pull_request' && github.ref == 'refs/heads/main'`):
  - `permissions: { contents: read, pages: write, id-token: write }`
  - `environment: { name: github-pages, url: ${{ steps.deployment.outputs.page_url }} }`
  - `concurrency: { group: pages, cancel-in-progress: false }`
  - Run `trunk build --release --public-url "/${{ github.event.repository.name }}/"` (the site is served from `/<repo>/`, i.e. `/paradigms-htmd/`), upload `demo/dist` with `actions/upload-pages-artifact`, and deploy with `actions/deploy-pages` (step `id: deployment`).
- Install Trunk in both jobs from a pinned prebuilt release tarball from `trunk-rs/trunk` on GitHub, rather than `cargo install`, which compiles from source and takes minutes.
- **One-time manual setup**, documented in `demo/README.md`: in the repository's Settings → Pages, set the source to "GitHub Actions". The deploy job fails until this is done.
- Do not change the existing CI workflows.

---

## 8. Implementation order

1. **Spike:** create a bare `demo/` crate that calls `htmd::convert("<h1>x</h1>")` and build it with `cargo build --target wasm32-unknown-unknown`. Run `cargo tree -i getrandom --target wasm32-unknown-unknown`. If the build fails, stop and report the cause, for example a dependency that doesn't support wasm.
2. Add the Trunk and Leptos skeleton: `index.html`, `lib.rs`/`main.rs`, `build.rs`, the panic hook and crash banner, and a textarea with the raw Markdown output.
3. Write `state.rs` with `DemoOptions` and its tests, then build the options panel.
4. Add the output tabs and sandboxed iframes with the CSP head, then the `pulldown-cmark` preview.
5. Add the examples (with the coverage tests) and the status line (timing, errors, size guard).
6. Write `permalink.rs` with its tests and wire up hash sync, `hashchange`, and Share.
7. Polish: responsive layout, dark mode, accessibility pass, bundle-size tuning.
8. Set up CI and deployment, then write `demo/README.md` (run, deploy, Pages setup, QA checklist).

Keep each step a separate commit.

---

## 9. Acceptance criteria

- [ ] `cd demo && trunk serve` opens a working playground. `<h1>Hi</h1>` shows `# Hi`, and switching to Setext shows `Hi\n==`.
- [ ] Every setting in §4.5 can be changed in the UI and has a visible effect on at least one example, with any dependent settings applied (checked by the coverage test in §6).
- [ ] The Preview and Input tabs render in sandboxed iframes with the CSP head; no script from the input runs and no remote resource from the input is requested.
- [ ] A permalink pasted into a new tab reproduces the input and settings exactly.
- [ ] A panic forced in a local build (a temporary `panic!` in `state::convert`, not committed) shows the crash banner instead of a silently frozen page.
- [ ] `cargo build`, `cargo clippy -- --deny warnings`, and `cargo test` at the repo root are unaffected, and `cargo package --list` at the repo root lists no `demo/` files.
- [ ] The `demo.yml` check job passes, and the deploy job publishes to `https://<owner>.github.io/paradigms-htmd/` with all assets loading.
- [ ] The PR description reports the gzipped wasm size and the conversion time for the article example, measured as in §5.
