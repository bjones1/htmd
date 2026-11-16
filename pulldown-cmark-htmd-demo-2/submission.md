Project 2 submission
====================

Team name: pulldown-cmark / htmd demo 2

The specification consists of the file
[htmddemo2-spec.md](htmddemo2-spec.md). The implementation is in
[`playground/`](../playground/README.md). Files as "acceptance-criteria.md", "instructor-feedback.md", "submission-template.md" are deemed unnecessary for final submission, as they were derived from the initial pull request feedback and literate-programming-book repository and are not included in the final commits.

Team members:

1. Daniil Kaimekin (da_kaimekin@kbtu.kz), GitHub userids: dkaimekin and
   dkaimekinmasters;
2. Ademi Bakhytova (ad_bakhytova@kbtu.kz), GitHub userid: adeemss;
3. Aruzhan Akhanayeva (ar_akhanayeva@kbtu.kz), GitHub userid: Akhanayeva;
4. Alisher Akim (ali_akim@kbtu.kz); the team could not reach him.

Review rounds are numbered across the whole project. Rounds 1–3 were the
draft's spec reviews, round 4 was the instructor's review of the draft pull
request, and rounds 5–10 were the final submission's spec reviews.
Implementation review rounds are numbered separately.

The spec's Build constraint publishes to the team's fork
(`https://dkaimekinmasters.github.io/paradigms-htmd/`); a push to `main` of
the instructor's fork would publish to that fork's own Pages address.

Draft submission
----------------

### Daniil Kaimekin

GitHub userids: dkaimekin and dkaimekinmasters

#### Contributions

Initial spec draft, after exploring wasm-markdown, the CommonMark dingus and
Leptos; repository setup; spec review round 1; the pull request to the
instructor.

#### Build and run test

The following summarizes LLM output for the prompt "Verify that you can
compile and run the project in the current git repo."

The project compiles and runs. All three CI commands from AGENTS.md pass:
`cargo build` finished cleanly; `cargo clippy -- --deny warnings` gave no
warnings; `cargo test` passed every suite (the largest had 91 and 37 tests),
with 1 test ignored and 0 failed. htmd is a library, so its three examples
were run: `simple` passed all its built-in checks, `table` printed a
correctly formatted Markdown table, and `page-to-markdown` converted the
saved Hacker News page in 25 ms and the Wikipedia page in 310 ms.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 15 | Fixed the version mismatch, number input clamping, deployment URL, and CI job mismatch. |

#### Last specification review

Prompting Claude Opus 5.5, medium effort, with:

> Review a spec at @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md . Find
> logical inconsistencies. DO not change any files yet.

produced the following output; strikethrough marks findings fixed in the
spec:

> **Repo mismatches:** ~~version 0.5.6 vs. `Cargo.toml` 0.5.5~~; `Options`
> does derive `Clone, PartialEq`; ~~spacing clamp 1–8 vs. library's 1–4~~;
> ~~deploy URL doesn't match repo~~.
>
> **Contradictions:** ~~clippy command missing `--`~~; ~~`deploy` waits on
> PR-only `check`~~; optional compression vs. mandatory deflate; "only
> allowed edit" vs. extra repo edits; 1 MB input vs. URL permalinks;
> debounce vs. single `Memo`.
>
> **Gaps:** no `<hr>` example; `br_style` ignored in Faithful mode;
> `skip_tags` default and permalink undefined; build-time version reads
> demo's own; "must not panic" vs. `panic = "abort"`.

### Ademi Bakhytova

GitHub userid: adeemss

#### Contributions

Spec review round 2 (30 findings) and the resulting spec fixes.

#### Build and run test

The following summarizes LLM output for the prompt "Verify that you can
compile and run the project in the current git repo."

The project compiles and runs with Rust 1.99.1.

| Check | Result |
| --- | --- |
| `cargo build` | builds cleanly |
| `cargo clippy -- --deny warnings` | no warnings |
| `cargo test` | 267 tests passed, 1 marked `#[ignore]` and skipped, 0 failed; the largest suites had 91 and 36 tests |

htmd is a library, so its examples were run: `cargo run --example simple`
prints nothing, which is expected, since it is only `assert_eq!` checks and
exits without errors; `cargo run --example table` prints a Markdown table
converted from HTML; `cargo run --example page-to-markdown` converted the
saved Hacker News page in 25–81 ms and the Wikipedia article in 561–652 ms
across runs, overwriting `output_hacker_news.md` and `output_wikipedia.md`,
which git ignores.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 2 | 30 (6 contradictions, 10 technical issues, 8 ambiguities, 6 missing requirements) | Corrected htmd API assumptions, improved WASM panic handling, fixed the GitHub Pages deployment path, added CSP protection for sandboxed previews, clarified permalink encoding, and improved CI configuration, testing requirements and UI accessibility; 5 design decisions (C1, C2, C3, C7, D1) were left for team confirmation, and B5/C8 kept documented caveats. |

#### Last specification review

Round 2. Its full LLM output was not saved in the repository; the findings
are summarized in the table above, and the resulting spec changes are in
commit `bc19149`.

### Aruzhan Akhanayeva

GitHub userid: Akhanayeva

#### Contributions

Spec review round 3 (27 findings) and the resulting spec fixes.

#### Build and run test

The following summarizes LLM output for the prompt "Verify that you can
compile and run the project in the current git repo."

The project compiles and runs with Rust 1.99.1. All three CI commands from
AGENTS.md pass: `cargo build` finished cleanly; `cargo clippy -- --deny
warnings` gave no warnings; `cargo test` passed 268 tests, with 0 failed and
1 ignored (the largest suites had 91 and 37 tests). htmd is a library, so its
three examples were run: `simple` passed all its built-in checks, `table`
printed a correctly aligned Markdown table, and `page-to-markdown` converted
the saved Hacker News page in 21 ms and the Wikipedia page in 385 ms.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 3 | 27 (4 contradictions, 8 technical errors, 10 ambiguities, 5 missing requirements) | Fixed the default-state crash (`skip_tags(vec![])` panics), closed the preview frames' meta-refresh navigation hole, froze the permalink `v=1` defaults with a versioning rule, defined empty-input, strict-decoding and `hashchange` failure handling, added a crash-loop escape link, split the status line, made deploy reuse the checked build with a checksum-pinned Trunk, specified option-combination and size-limit tests, and added accessibility, article-licence and getrandom 0.2 requirements. |

#### Last specification review

Round 3. Its full LLM output was not saved in the repository; the findings
are summarized in the table above, and the resulting spec changes are in
commit `dca4636`.

### Alisher Akim

The team could not reach Alisher, so he has no contributions, build and run
test, or review rounds.

Final submission
----------------

Round 4, the instructor's review of the draft pull request (an overall
review, 11 inline spec comments and 3 submission comments), led to a rewrite
of the spec: it now states behavior, not implementation; adds the Markdown →
HTML direction with a one-editor direction switch; lists every htmd and
pulldown-cmark option with its control and default; adds expected-results
tables checked against htmd 0.5.5 and pulldown-cmark 0.13.4; and drops the
API tables, repository layout, Leptos idioms, file tree, CI and
implementation-order sections, including most round 2 and 3 additions.

### Daniil Kaimekin

GitHub userids: dkaimekin and dkaimekinmasters

#### Contributions

Rewrote the spec after the instructor's review (round 4); ran spec review
rounds 5, 6, 8, 9 and 10 and made the resulting fixes; generated the first
implementation in `playground/` (core, engine and app crates, build script,
and Pages workflow) and ran implementation review round 3.

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 4 (instructor's review of the draft) | 15 (an overall review, 11 inline spec comments, 3 submission comments) | Rewrote the spec to state behavior, not implementation: added the Markdown → HTML direction with a one-editor direction switch, listed every htmd and pulldown-cmark option with its control and default, added expected-results tables checked against htmd 0.5.5 and pulldown-cmark 0.13.4, and dropped the API tables, repository layout, Leptos idioms, file tree, CI and implementation-order sections. |
| 5 | 30 (12 inconsistencies, 15 incomplete scenarios, 3 vague objectives); 13 fixed, 17 left as low-priority implementation detail | Kept large input out of permalinks and Share; defined the round-trip criterion; Markdown presets turn on their extension; switch, Copy and Share use up-to-date output; rendered tabs cannot restyle the page and links do nothing; one source for defaults; two-way footnote coupling; Copy input on crash; measurable size and speed targets. |
| 6 (converters run) | 12 (7 findings, 5 smaller points); 11 fixed, 5 earlier open points left to the implementer | A separate 32,768-character limit for links; saved state kept after `#`; no history entries except hand edits; switch and Copy unavailable while output is stale; 6 verified Expected results rows; footnote preset wording; both library versions in the header; hangs out of scope. |
| 8 (repetition and one plausible result) | 25 (8 repeated rules, 3 contradictions from repetition, 3 conciseness cuts, 9 open behaviors, 2 smaller points); 25 fixed, 2 of the cuts adapted | Cut the acceptance criteria to 2, moving their test inputs into Safety; defined the *default example* once; removed repeated rules; specified invalid addresses, how long "Convert anyway" lasts, crash recovery, a large direction switch, when Examples and Clear confirm, code-point length and what the timing covers, number-field clamping, tab selection across a switch, Markdown preset names and order, header links, and Share's manual-copy fallback. |
| 9 ([latest-review.md](latest-review.md), converters run) | 8 (1 contradiction, 3 gaps, 4 trivial points); 4 fixed, 4 trivial points left | Tabs keep their last content only while conversion is paused, not after a crash; noted that the skip tags `SCRIPT` row relies on the playground ignoring case; the double-switch criterion ignores a trailing newline; an address with no saved state loads the default example, and a loaded address replaces pending keystrokes. |
| 10 (converters checked by the implementation's tests) | 9 spec findings (2 low, 7 trivial), plus 10 on this file | No spec changes yet: proposed one-line fixes for the "published package" wording, which the Pages workflow contradicts, and for what Share copies after an invalid link; left 7 trivial points; listed this file's per-member gaps, inverted strikethrough and wrong size figure for the team. |

#### Last specification review

Prompting Claude Opus 5.5, in Claude Code, with:

> Perform one last specification review on my behalf. Assess the
> @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md and
> @pulldown-cmark-htmd-demo-2/submission.md against
> @pulldown-cmark-htmd-demo-2/acceptance-criteria.md and
> @pulldown-cmark-htmd-demo-2/submission-template.md .

produced the round 10 review, shortened here. No finding was judged
incorrect, so nothing is struck through:

> **Converters checked.** Every Expected results row, the Round trip and both
> acceptance criteria pass as automated tests in
> `playground/engine/tests/spec.rs`, built with pulldown-cmark 0.13.6.
>
> **Spec, low (one-line fixes):**
>
> 1. "Does not change … its published package" contradicts the Build
>    constraint: the Pages workflow lives in `.github/workflows/`, which
>    `cargo package` includes in htmd's crate.
> 2. What Share copies after an invalid link is unspecified: the address
>    keeps the invalid link until the next update, so Share could copy it.
>
> **Spec, trivial (left as they are):** whether the timing counts as a
> status change to announce; a long Skip tags text alone can exceed the
> address limit; Reference style changes a preset only with referenced
> links; and round 9's four trivial points.
>
> **Verdict:** with the two one-line fixes, only trivial findings remain.
>
> **submission.md:** per-member gaps in the final section (no final spec
> review for Ademi, no own implementation review for Daniil or Aruzhan, no
> saved output for Aruzhan's last spec review); strikethrough in the draft
> marks fixed findings, the opposite of the assignment's meaning; the 871 KB
> size figure should be CI's 576,635 bytes; and smaller wording and
> consistency points.

#### Implementation review rounds

Daniil generated the implementation; rounds 1 and 2 are listed under Ademi
Bakhytova.

| Round | Findings | Main changes made |
| --- | --- | --- |
| 3 (at `0c9bcb1`) | 7 (3 low, 4 trivial); none fixed | No code changes: recorded that iOS Safari zooms into the editor and option fields, whose text is under 16 px; that the Pages deploy fails until Pages is enabled on the fork; and that `implementation-review.md` overstates the site's size; left 4 trivial points. |

#### Last implementation review

Prompting Claude Opus 5.5, in Claude Code, with:

> Perform one last implementation review round on my behalf. Add your
> findings in the implementation review rounds on my behalf. Record the
> implementation review under "last implementation review" on my behalf.
> Assess the @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md and
> @pulldown-cmark-htmd-demo-2/submission.md against
> @pulldown-cmark-htmd-demo-2/acceptance-criteria.md and
> @pulldown-cmark-htmd-demo-2/submission-template.md .

produced the round 3 review, shortened here. No finding was judged
incorrect, so nothing is struck through:

> **Checks run.** `cargo test --locked` passes 32 tests, including every
> Expected results row, the Round trip and both acceptance criteria. CI's
> size check measures the site at 576,635 bytes with gzip. Round 1's fixes
> (Share after an invalid link, the status live region, one output panel,
> no ARIA menu claim, the size check) hold. Earlier in-browser checks found
> the Article example converts in 0.2 ms, hostile HTML in the rendered tabs
> makes no requests, and a stack overflow shows the crash message.
>
> **Low:**
>
> 1. The editor (0.88rem) and the option fields (0.92rem) are under 16 px,
>    so iOS Safari zooms the page when one is focused (Constraints:
>    Browsers, phone).
> 2. The deploy job fails on every push to `main` because GitHub Pages is
>    not enabled on the fork, so the Build constraint is not met yet.
> 3. `implementation-review.md` gives the site as about 871 KB with gzip;
>    that is the raw wasm before `wasm-bindgen`, not the published site.
>
> **Trivial, left as they are:** a very long Skip tags text alone can push
> the address past 32,768 characters; browser behavior (typing pause,
> address, sandbox) has no automated test; the workflow's actions target
> the deprecated Node.js 20; and round 2's points on timing and live-region
> announcements.

### Ademi Bakhytova

GitHub userid: adeemss

#### Contributions

Ran implementation review rounds 1 and 2 and made the resulting fixes to
`playground/` and its workflow; reorganized this file by team member,
following the template, and corrected its draft and final sections.

#### Specification review rounds

None in the final submission; Ademi's spec review (round 2) was part of the
draft.

#### Last specification review

None in the final submission.

#### Implementation review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 7 (1 behavior bug, 3 accessibility issues, 1 unchecked constraint, 2 trivial points); 5 fixed, 2 left | Share writes the shown state before copying, so it no longer spreads an invalid link; the whole status line is a live region; one output panel that every tab controls; the Examples button no longer claims an ARIA menu; the workflow fails when the site exceeds 1 MB with gzip. |
| 2 ([implementation-review.md](implementation-review.md)) | 4 (all trivial); none fixed | Confirmed the round 1 fixes and full spec coverage; left the timing of the cross-instance call, frequent polite announcements, optional `wasm-opt`, and the fork's Pages address. |

#### Last implementation review

Prompting Claude Opus 5.5, in Claude Code, in Russian; in English:

> Review the implementation in playground/ against
> @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md and fix the problems found.

produced the round 2 review, in full in
[implementation-review.md](implementation-review.md). No finding was judged
incorrect, so nothing is struck through:

> **Checks run.** `cargo test --locked` passes 32 tests, including every
> Expected results row, the Round trip and both acceptance criteria; clippy
> gives no warnings natively or for wasm32; the site is about 871 KB with
> gzip, under the 1 MB limit; the htmd library is unchanged.
>
> **Trivial points, left as they are:**
>
> 1. The timing includes crossing into the converter's wasm instance, well
>    under a millisecond.
> 2. The live region speaks after every conversion; screen readers coalesce
>    polite updates.
> 3. `build.sh` runs `wasm-opt` only when installed; the new size check
>    guards the limit.
> 4. The Pages address belongs to the team's fork.
>
> Browser-only constraints (the 10 ms Article timing in Chrome, contrast,
> screen reader output, phone layouts) were checked by reading the code,
> not measured.

### Aruzhan Akhanayeva

GitHub userid: Akhanayeva

#### Contributions

Checked the rewritten spec against every instructor comment and made the
remaining fixes to the spec and this file (round 7); added behavior tests to
the implementation (`playground/engine/tests/behavior.rs`).

#### Specification review rounds

| Round | Findings | Main changes made |
| --- | --- | --- |
| 7 | 13 (instructor comments and review findings still open after round 6: 3 in submission.md, 1 inline spec comment, 7 review findings, 2 repetitions) | Removed editor detail the instructor called too detailed; made Clear confirm like Examples; gave Copy and Share a manual-copy fallback; specified when the address changes, that Back to an address with no state shows the default example, and that loading an address wins over a pending update; named deep nesting as a crash the page must survive; required titles on rendered tabs; removed two repeated statements; fixed this file's draft/final placement. |

#### Last specification review

Round 7. Its full LLM output was not saved in the repository; the findings
are summarized in the table above, and the resulting changes are in commit
`d6f83f7`.

#### Implementation review rounds

None; the implementation review rounds are listed under Ademi Bakhytova.

#### Last implementation review

See Ademi Bakhytova.

### Alisher Akim

The team could not reach Alisher, so he has no contributions or review
rounds.
