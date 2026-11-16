htmd playground
===============

A web page for trying htmd (HTML → Markdown) and pulldown-cmark (Markdown →
HTML) in the browser. Its behavior is specified in
[`../pulldown-cmark-htmd-demo-2/htmddemo2-spec.md`](../pulldown-cmark-htmd-demo-2/htmddemo2-spec.md).

This directory is a Cargo workspace of its own, so the library's build, lint,
and tests never see it:

* `core/` — the options, presets, and permalink format, shared by the other
  two crates.
* `engine/` — the converters. The page loads it as a separate wasm instance,
  so a converter crash (a panic, or a stack overflow on deeply nested input)
  leaves the interface able to report it. `engine/tests/spec.rs` checks the
  spec's Expected results and Acceptance criteria.
* `app/` — the Leptos 0.8 user interface.
* `static/` — the HTML shell, loader script, and stylesheet.

Building and running
--------------------

Needs the `wasm32-unknown-unknown` target and `wasm-bindgen-cli` at the
version of `wasm-bindgen` in `Cargo.lock`:

```
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --locked --version <version from Cargo.lock>
./build.sh
python3 -m http.server 8765 --directory dist
```

Then open <http://localhost:8765/>. Tests run natively:

```
cargo test
cargo clippy --all-targets -- --deny warnings
cargo clippy --target wasm32-unknown-unknown -p htmd-playground -p htmd-playground-engine -- --deny warnings
```

`.github/workflows/playground.yaml` runs the same checks and publishes
`dist/` to GitHub Pages on every push to `main`.
