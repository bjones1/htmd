Implementation review: htmd and pulldown-cmark playground
=========================================================

This is implementation review round 2 of `playground/`, dated 2026-11-17. It
replaces the round 1 review of the same date; round 1's findings are listed
at the end.

Model: Claude Opus 5.5, in Claude Code. The prompt, given in Russian, asked
it to review the implementation in `playground/` against
[htmddemo2-spec.md](htmddemo2-spec.md), fix the findings, and review again
until only trivial findings remained. In English:

> Review the implementation in playground/ against
> @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md and fix the problems found.

Summary
-------

Only trivial points remain. Every behavior in the spec has code that
implements it, and the round 1 fixes hold. Constraints that need a browser to
measure (the 10 ms Article timing in Chrome, WCAG contrast, screen reader
output, phone layouts) were checked by reading the code and stylesheet, not
measured.

Checks run
----------

* `cargo test --locked`: 32 tests pass, including `engine/tests/spec.rs`
  (every Expected results row, the Round trip, and both acceptance criteria)
  and `engine/tests/behavior.rs`.
* `cargo clippy --locked --all-targets -- --deny warnings` and the wasm32
  clippy command from the workflow: no warnings.
* `cargo build --release --locked --target wasm32-unknown-unknown`: the two
  wasm files are 521,487 and 346,624 bytes with gzip before wasm-bindgen, and
  the static files 3,332 bytes, about 871 KB in all, under the 1 MB limit.
* The htmd library at the repository root is unchanged.

Spec coverage
-------------

| Spec section | Where | Result |
| --- | --- | --- |
| Page layout, direction switch, input and output panes | `app/src/app.rs` | as specified |
| Status line | `app/src/app.rs` (`StatusLine`) | as specified |
| Conversion (100 ms pause, options at once) | `app/src/state.rs` | as specified |
| Options, defaults, hidden values kept, footnotes rule | `core/src/settings.rs`, `app/src/options_panel.rs` | as specified |
| Examples | `core/src/presets.rs` | as specified |
| Expected results, Round trip | `engine/tests/spec.rs` | all rows pass |
| Permalinks | `core/src/codec.rs`, `app/src/address.rs`, `app/src/state.rs` | as specified |
| Limits and failures | `app/src/state.rs`, `engine/src/lib.rs`, `static/main.js` | as specified; the converter runs in a wasm instance of its own, so a stack overflow cannot take the interface down |
| Safety | `app/src/frame.rs`, `static/index.html` | sandboxed frames, CSP in the page and the frames, links, refreshes, `<base>`, `<link>` and nested frames removed |
| Size | `.github/workflows/playground.yaml` | checked on every build (round 1, finding 3) |

Findings
--------

All four are trivial and left as they are.

1. **The timing includes crossing into the converter's instance**
   (Status line). `convert` in `app/src/state.rs` times the call into the
   engine, which also encodes and decodes the settings and copies the
   strings between the two wasm instances. This adds well under a
   millisecond and is part of what the user waits for.
2. **The live region speaks after every conversion** (Constraints:
   Accessibility). Since round 1 the timing and length are announced, as
   the spec requires; while typing, polite announcements may be frequent.
   Screen readers coalesce polite updates, so this is left as specified.
3. **`build.sh` runs `wasm-opt` only when it is installed**, and the
   workflow does not install it. The site is under the size limit without
   it, and the size check added in round 1 fails the build if that changes.
4. **The Pages address belongs to the team's fork** (Constraints: Build),
   while the pull request targets the instructor's fork. This is the same
   point as finding 8 of the last spec review.

Round 1 findings
----------------

Seven findings: five fixed, two trivial points left.

1. **Share copied an invalid link** (Permalinks). After an address the page
   could not read, the address keeps the bad link until the next update, so
   Share copied a link that opens to the "invalid link" notice. *Fixed:*
   Share first writes the shown state to the address (`app/src/app.rs`).
2. **The timing and length were not announced** (Constraints:
   Accessibility). Only the notices were inside the `role="status"` region.
   *Fixed:* the whole status line is the live region.
3. **Nothing checked the size limit** (Constraints: Size and speed). The
   site measured about 871 KB with gzip, about 130 KB under the limit, and
   CI does not run `wasm-opt`. *Fixed:* the workflow fails when `dist/`
   exceeds 1,000,000 bytes with gzip.
4. **Tabs pointed at panels that do not exist** (Constraints:
   Accessibility). Every tab's `aria-controls` named its own panel id, but
   only the selected tab's panel was in the page. *Fixed:* one panel,
   `output-panel`, which every tab controls.
5. **The Examples button announced a menu that is not one** (Constraints:
   Accessibility). `aria-haspopup="true"` tells assistive technology to
   expect an ARIA menu, but the list is a disclosure of buttons. *Fixed:*
   the attribute is removed; `aria-expanded` remains.
6. **The timing includes crossing into the converter's instance.** Left;
   round 2, finding 1.
7. **The interface reaches the converter through a global**
   (`htmdPlaygroundEngine`, set in `static/main.js`). This is what keeps the
   two wasm instances apart, and the page loads no other scripts. Left as
   designed.
