htmd live demo
==============

A browser demo of [`htmd`](../README.md): paste or type HTML and watch it
convert to Markdown as you type, with a rendered preview and a fidelity panel
which round-trips the output through
[`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) and diffs the
result against the input. It is a client-side-only
[Leptos](https://leptos.dev/) (CSR) app compiled to WebAssembly and bundled by
[Trunk](https://trunkrs.dev/); the design is specified in
[`demo-spec.md`](demo-spec.md).

`demo/` is a standalone crate, not a member of the root workspace. It has its
own `Cargo.lock`, which pins every dependency version and is used with
`--locked` in CI. Run the commands below from `demo/` unless stated otherwise.
The root-repo gates (`cargo build`, `cargo clippy -- --deny warnings`,
`cargo test`) cover only the `htmd` library and are unaffected by this crate.

Prerequisites
-------------

* A stable Rust toolchain (edition 2024), installed via `rustup`.
* The WebAssembly target:

  ```
  rustup target add wasm32-unknown-unknown
  ```

* Trunk, pinned to the version CI installs:

  ```
  cargo install trunk --version 0.22.1 --locked
  ```

  Prebuilt binaries are also on the
  [Trunk releases page](https://github.com/trunk-rs/trunk/releases/tag/v0.22.1).
  You do not need to install `wasm-bindgen` or `wasm-opt` for builds: Trunk
  downloads the versions pinned in `Trunk.toml` (`[tools]`). The
  `wasm-bindgen` CLI version there must match the `wasm-bindgen` crate version
  in `Cargo.lock`, or the build fails.
* For the browser test suite only: Chrome or Chromium, a matching
  `chromedriver`, and `wasm-bindgen-cli` at the same version as the crate:

  ```
  cargo install wasm-bindgen-cli --version 0.2.130 --locked
  ```

Native commands
---------------

The conversion, canonicalization, diff, limits and options logic is plain Rust
and is unit-tested natively, without a browser:

```
cargo test --locked
```

Clippy, as CI runs it (against the wasm target, warnings denied):

```
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
```

`cargo clippy --locked -- -D warnings` checks the native build, which also
compiles the unit tests' `cfg(test)` code.

WebAssembly commands
--------------------

### Development server

```
trunk serve --open
```

This builds a debug wasm, serves it at <http://127.0.0.1:8080/>, and rebuilds
and reloads on file changes. Add `--release` to check the optimized build.

### Browser tests (`wasm-bindgen-test`, headless Chromium)

Point Cargo's wasm runner at `wasm-bindgen-test-runner` and the runner at
`chromedriver`, then run the suite in both profiles:

```
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
export CHROMEDRIVER=/path/to/chromedriver
cargo test --locked --target wasm32-unknown-unknown
cargo test --locked --target wasm32-unknown-unknown --release
```

The runner is headless by default; set `NO_HEADLESS=1` to watch it in a
visible browser window. At minimum the FR-7 nesting cases must pass under
`--release`. The crash-recovery cases (§8.6)
need the `demo-panic-test` feature; see [Panic testing](#panic-testing).

### Release build

GitHub Pages serves this repository as a project site under `/htmd/`, so the
deployable build must set Trunk's public URL to that path. Trunk's default of
`/` produces absolute asset URLs which 404 on a project site.

```
trunk build --release --public-url /htmd/
```

The output in `demo/dist/` is a static site deployable on any static host;
for a host serving from its root, omit `--public-url`.

### Size check

CI fails the build if the gzipped wasm reaches 1.5 MB (1,572,864 bytes). To
reproduce the check after a release build, from the repository root:

```
gzip -6 -c demo/dist/*.wasm | wc -c
```

The result must be strictly less than `1572864`. Gzip level 6 is only the
measurement; the shipped file is uncompressed and compressed by the host.

Panic testing
-------------

The `demo-panic-test` Cargo feature adds a "Deliberate panic" button to the
toolbar, used to verify the crash overlay and the reload-recovery path
(FR-7:148, FR-8) in both profiles:

```
trunk serve --features demo-panic-test
trunk build --release --features demo-panic-test
cargo test --locked --target wasm32-unknown-unknown --features demo-panic-test
```

Never enable this feature in a build you deploy: the Pages build in CI is
built without it, and a `dist/` built with it must not be published.

Continuous integration and deployment
-------------------------------------

`.github/workflows/demo-ci.yaml` runs on changes under `demo/`. It installs
the pinned Trunk, runs the wasm-target Clippy and native tests, builds with
`trunk build --release --public-url /htmd/`, applies the size check, and on
pushes to `main` deploys `demo/dist/` to GitHub Pages. The Pages source must
be set to **GitHub Actions** in the repository's Settings → Pages.

The benchmark corpus for the compute budget is in [`bench/corpus/`](bench/corpus/).
