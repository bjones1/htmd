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
* For Playwright: Node.js 22 or newer and npm. Install the pinned test
  dependencies and browsers from `demo/`:

  ```
  npm ci
  npx playwright install --with-deps --no-shell chromium firefox webkit
  ```

Native commands
---------------

The conversion, canonicalization, diff, limits and options logic is plain Rust
and is unit-tested natively, without a browser:

```
cargo test --locked
```

The deep-nesting fixtures can exhaust the default Windows debug test-thread
stack. CI gives native test threads 16 MiB; to use the same setting in
PowerShell:

```
$env:RUST_MIN_STACK = "16777216"
cargo test --locked
```

Clippy, as CI runs it (against the wasm target, warnings denied):

```
cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings
```

To lint the native build including the unit tests' `cfg(test)` code (plain
`cargo clippy` skips test targets):

```
cargo clippy --locked --all-targets -- -D warnings
```

WebAssembly commands
--------------------

### Development server

```
trunk serve --open
```

This builds a debug wasm, serves it at <http://127.0.0.1:8080/>, and rebuilds
and reloads on file changes. Add `--release` to check the optimized build.

### Browser tests (`wasm-bindgen-test`, headless Chromium)

The suite lives in [`src/browser_tests.rs`](src/browser_tests.rs). It covers
the FR-7 nesting cases against the real 1 MiB wasm stack: 512 levels of
nested lists, blockquotes, tables, and a mix of all three, in Pure and
Faithful modes; 600 unclosed `<p>` and 600 `<br>` siblings; 50,000 `>` and
5,000 levels of raw HTML in Markdown rejected by the nesting limit; and the
preview walk on 5,000 levels. It also checks the panic hook's crash steps
(marker, storage halt, overlay, `inert` root, focus) without a real panic.

Install `wasm-bindgen-test-runner` at the same version as `wasm-bindgen` in
`Cargo.lock` (currently 0.2.130), from the
[wasm-bindgen release](https://github.com/wasm-bindgen/wasm-bindgen/releases/tag/0.2.130)
or with `cargo install wasm-bindgen-cli --version 0.2.130 --locked`. Point
Cargo's wasm runner at it and the runner at `chromedriver` (which must match
your Chrome), then run both profiles:

```
export CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner
export CHROMEDRIVER=/path/to/chromedriver
export WASM_BINDGEN_TEST_TIMEOUT=600
cargo test --locked --target wasm32-unknown-unknown
cargo test --locked --target wasm32-unknown-unknown --release
```

The nesting cases run only under `--release`. Debug builds report them as
ignored, because debug stack frames are far larger than the release frames
the 512-level ceiling is sized for. The timeout above is a safety margin over
the runner's 20-second default. The runner is headless by default; set
`NO_HEADLESS=1` to watch it in a visible browser window. Plain `#[test]`
functions are not run in the browser; they run in the native `cargo test`.

### Playwright end-to-end, accessibility, and performance tests

Build the optimized demo before running tests; the test server serves
`dist/` and does not rebuild it. From `demo/`:

```
trunk build --release
npm run check
npm run test:e2e
```

Functional and axe tests run on Chromium, Firefox, and WebKit. Performance
assertions run on desktop Chromium, the reference environment in §8.3 of
the spec. Tests use one worker and no retries. To select a browser or a subset:

```
npm run test:e2e -- --project=chromium
npm run test:a11y
npm run test:perf
```

The suite checks both conversion directions, preview isolation, separate
mode buffers, options/reset, samples, Undo, workspace persistence, UTF-8
limits, and keyboard tabs. Axe scans the empty page, rendered and source
views, the Faithful diff, and a size-limit banner for WCAG 2.1 AA violations.
Scans cover the demo UI, including the iframe's accessible name; arbitrary
user-authored documents inside the sandbox are outside this scope.

Performance tests read files from
[`bench/corpus/`](bench/corpus/) directly. Every document must convert in
strictly less than 100 ms; HTML documents run in both Pure and Faithful modes
so the latter includes canonicalization and diff generation. Three
100,000-byte adversarial cases exercise table-output amplification, a table
just below the output cap (including preview generation), and a divergent
fidelity diff (using skipped `div` content) under 1,000 ms. The limit cases
also verify their banners and check that conversion recovers afterward.
The suite reads the app's `htmd:convert` measure and its
`htmd:convert:start`/`htmd:convert:end` marks after a small warmup conversion.
Compute assertions check individual conversions without averaging. Separate
browser timestamps measure input through the matching preview load and two
parent animation frames (a paint opportunity, not a pixel-paint guarantee):
at most 100 ms for a keystroke reaching exactly 2,000 UTF-8 bytes in both
directions, under 400 ms for each corpus document, and under 1,000 ms for
adversarial documents, including the output-cap clearing path. Preview
matching checks the actual sandbox document and its navigation timing;
unrelated loads cannot finish a measurement. Playwright polling time is
excluded. A delayed-preview regression proves that these measurements
include work after synchronous conversion returns.

The startup test warms the browser cache, navigates again, and requires the
`htmd:ready` mark within 1,500 ms of navigation start. The mark is emitted
after mounting and workspace restoration. The test server permits caching,
and the test checks that JS/WASM assets came from cache. These budgets
require an optimized build on desktop-class hardware without throttling.

The HTML report includes axe results, startup and conversion timings, and
typing-jank measurements. `responsiveness.spec.ts` types near the 100 kB
limit in Markdown, Pure, and Faithful modes in all three browsers and
retains successful traces. Chromium observes Long Tasks and rejects
keystroke tasks reaching 50 ms after subtracting only the measured scheduled
conversion intervals; every input handler must also stay below 50 ms.
A deliberately slow listener proves that the observer detects input jank.
Firefox/WebKit handler timings supplement the required manual trace pass.

Failures retain screenshots and traces in `test-results/`; inspect the
report with `npx playwright show-report`, or a successful typing trace with
`npx playwright show-trace test-results/<typing-test>/trace.zip`.
For the Firefox/WebKit manual pass, inspect the typing filmstrip and action
timeline for dropped text or pauses attributable to keystroke handling,
distinguishing scheduled conversions with the attached `typing-jank` JSON.

For a build with a project-path public URL, set `DEMO_PUBLIC_PATH` to the same
path before testing. For example, in PowerShell:

```
trunk build --release --public-url /htmd/
$env:DEMO_PUBLIC_PATH = "/htmd/"
npm run test:e2e
```

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
```

The §8.6 crash-loop recovery cases live in
[`e2e/crash-recovery.spec.ts`](e2e/crash-recovery.spec.ts) and run only
against such a build. Build it into its own directory, so a panic build
never sits in `dist/`, and point the test server at it. In PowerShell:

```
trunk build --features demo-panic-test --dist dist-panic
$env:DEMO_PANIC_TEST = "1"
$env:DEMO_DIST = "dist-panic"
npm run test:e2e -- --project=chromium
```

Add `--release` to the `trunk build` line for the release profile; CI runs
both. With `DEMO_PANIC_TEST=1` only the crash-recovery spec runs; without it
that spec is skipped. The spec panics on a persisted input and checks the
Rust hook's overlay, `inert` root, and focus. It then reloads twice without
choosing, checking that the workspace is restored with empty outputs, an
empty diff, and the recovery banner. Last, it checks that each of "Convert
restored input" and "Clear buffers" removes the crash marker.

Never enable this feature in a build you deploy: the Pages build in CI is
built without it, and a build with it must not be published.

Continuous integration and deployment
-------------------------------------

`.github/workflows/demo-ci.yaml` runs on changes under `demo/`, to the root
`htmd` library (`src/`, `Cargo.toml`, `Cargo.lock`), or to the workflow itself. It installs
the pinned Trunk, runs the wasm-target Clippy and native tests, builds with
`trunk build --release --public-url /htmd/`, and applies the size check. A
three-browser Playwright matrix tests that same release artifact, running
the end-to-end, axe, and performance checks and uploading reports. Separate
jobs run the `wasm-bindgen-test` suite in headless Chromium in debug and
release, and the crash-recovery spec against `demo-panic-test` builds in
debug and release. Deployment depends on all of these jobs passing. On pushes to `demo-spec`, the workflow
deploys `demo/dist/` to GitHub Pages. The Pages source must
be set to **GitHub Actions** in the repository's Settings → Pages.

The benchmark corpus for the compute budget is in [`bench/corpus/`](bench/corpus/).
