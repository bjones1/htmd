Project 2 submission
====================

Team name: pulldown-cmark / htmd demo 2

Team members are:
1. Daniil Kaimekin (da_kaimekin@kbtu.kz), GitHub userids: dkaimekin and
   dkaimekinmasters;
2. Ademi Bakhytova (ad_bakhytova@kbtu.kz), GitHub userid: adeemss;
3. Aruzhan Akhanayeva (ar_akhanayeva@kbtu.kz), GitHub userid: Akhanayeva;
4. Alisher Akim (ali_akim@kbtu.kz) (couldn't connect with them, unfortunately);

The specification consists of
the file [htmddemo2-spec.md](htmddemo2-spec.md).

Draft submission
----------------

### Contributions

* Daniil: initial spec draft; wasm-markdown, CommonMark and Leptos exploration;
  repository setup; pull request to the instructor.
* Ademi: spec review round 2 (30 findings) and the resulting spec fixes.
* Aruzhan: spec review round 3 (27 findings) and the resulting spec fixes.


### Compiler test

The following statements summarize LLM output on each team member's computer,
based on the prompt "Verify that you can compile and run the project in the
current git repo."

#### Daniil

The project compiles and runs. All three CI commands from AGENTS.md pass:

cargo build: finished cleanly.
cargo clippy -- --deny warnings: no warnings.
cargo test: every test suite passed. The largest suites had 91 and 37 tests, with 1 test ignored and 0 failed.

htmd is a library, so to check it at runtime I ran its three examples:

simple: all its built-in checks passed.
table: printed a correctly formatted Markdown table.
page-to-markdown: converted the saved Hacker News page in 25ms and the Wikipedia page in 310ms.

#### Ademi

| Check | Result |
| --- | --- |
| `cargo build` | ✅ builds (Rust 1.99.1) |
| `cargo clippy -- --deny warnings` | ✅ no warnings |
| `cargo test` | ✅ 267 tests passed, 1 marked `#[ignore]` and skipped, 0 failed |

Examples:

* `cargo run --example simple` prints nothing, which is expected: the example
  is just `assert_eq!` checks, and it exits without errors.
* `cargo run --example table` prints a Markdown table converted from HTML.
* `cargo run --example page-to-markdown` converts a Hacker News page to
  Markdown in 25 ms and a Wikipedia article in 561 ms. It overwrites
  `output_hacker_news.md` and `output_wikipedia.md`, which git ignores.

#### Aruzhan

The project compiles and runs with Rust 1.99.1. All three CI commands from AGENTS.md pass:

cargo build: finished cleanly.
cargo clippy -- --deny warnings: no warnings.
cargo test: 268 tests passed, 0 failed, 1 ignored. The largest suites had 91 and 37 tests.

htmd is a library, so to check it at runtime I ran its three examples:

simple: all its built-in checks passed.
table: printed a correctly aligned Markdown table.
page-to-markdown: converted the saved Hacker News page in 21ms and the Wikipedia page in 385ms.

### Spec review rounds

| round | findings | main changes made |
|---|---|---|
| 1 | 15 | fixed version mismatch, number input clamping, deployment url, CI job mismatch|
| 2 (Ademi Bakhytova) | 30 (6 contradictions, 10 technical issues, 8 ambiguities, 6 missing requirements) | corrected htmd API assumptions, improved WASM panic handling, fixed the GitHub Pages deployment path, added CSP protection for sandboxed previews, clarified permalink encoding, improved CI configuration, testing requirements, and UI accessibility; 5 design decisions (C1, C2, C3, C7, D1) need team confirmation, and B5/C8 keep documented caveats |
| 3 (Aruzhan Akhanayeva) | 27 (4 contradictions, 8 technical errors, 10 ambiguities, 5 missing requirements) | fixed the default-state crash (`skip_tags(vec![])` panics), closed the preview frames' meta-refresh navigation hole, froze the permalink `v=1` defaults and added a versioning rule, defined empty-input and strict-decoding permalink rules and `hashchange` failure handling, corrected the crash banner and added a crash-loop escape link, dropped strikethrough from the Preview and the redundant root `Cargo.toml` edit, split the status line into conversion and notice parts, made the deploy job reuse the check job's verified build with a checksum-pinned Trunk, specified option-combination and size-limit tests, and added accessibility (live regions, ARIA tabs), article-licence, and getrandom 0.2 requirements; marked round 2's open decisions as provisional in the spec header |
| **4 (instructor review, bjones1)** | **15 (overall review, 11 inline spec comments, 3 submission comments)** | **rewrote the spec to state behavior, not implementation; added the Markdown → HTML direction with a one-editor direction switch; listed every htmd and pulldown-cmark option with its control and default (Pure mode, math on); added expected-results tables checked against htmd 0.5.5 and pulldown-cmark 0.13.4; removed the API tables, repo layout, Leptos idioms, file tree, CI and implementation-order sections, including most round 2 and 3 additions** |

### Last LLM review

Prompting Claude Opus 5.5, Medium effort with:
> Review a spec at @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md . Find logical inconsistencies. DO not change any files yet.

LLM output on last review:
> **Repo mismatches:** ~~version 0.5.6 vs. `Cargo.toml` 0.5.5~~; `Options` does derive `Clone, PartialEq`; ~~spacing clamp 1–8 vs. library's 1–4~~; ~~deploy URL doesn't match repo~~.
>
> **Contradictions:** ~~clippy command missing `--`~~; ~~`deploy` waits on PR-only `check`~~; optional compression vs. mandatory deflate; "only allowed edit" vs. extra repo edits; 1 MB input vs. URL permalinks; debounce vs. single `Memo`.
>
> **Gaps:** no `<hr>` example; `br_style` ignored in Faithful mode; `skip_tags` default and permalink undefined; build-time version reads demo's own; "must not panic" vs. `panic = "abort"`.

Final submission (Excluding implementation, as discussed at the 1-to-1 and group calls)
----------------

### Contributions

* Daniil: rewrite of the spec after the instructor's review (round 4); spec
  review rounds 5, 6, 8 and 9, with the resulting fixes to the spec and this
  file.
* Ademi: (to be completed for the final submission)
* Aruzhan: check of the rewritten spec against every instructor comment, with
  the remaining fixes to the spec and this file (round 7).
* Alisher: (to be completed for the final submission)

### Spec review rounds

Round numbers continue from the draft; round 4 was the instructor's review.
The reviews for rounds 5, 6 and 8 were earlier versions of
[latest-review.md](latest-review.md), kept in the git history.

| round | findings | main changes made |
|---|---|---|
| 5 | 30 (12 inconsistencies, 15 incomplete scenarios, 3 vague objectives); 13 fixed, 17 left as low-priority implementation detail | large input is kept out of permalinks and Share; defined the round-trip criterion; Markdown presets turn on their extension; switch, Copy and Share use up-to-date output; rendered tabs can't restyle the page and links do nothing; a single source for defaults; two-way footnote coupling; Copy input on crash; measurable size and speed targets; more acceptance criteria |
| 6 (converters run) | 12 (7 findings, 5 smaller points); 11 fixed, the 5 earlier open points left to the implementer | separate 32,768-character share limit for links; saved state kept after `#`; no history entries except hand edits; switch and Copy unavailable while output is stale; 6 verified Expected results rows (list number spacing, `<noscript>`, Faithful line break, display math, upper-case skip tags); footnote preset wording; both library versions in the header; hangs out of scope |
| 7 (Aruzhan Akhanayeva) | 13 (instructor comments and review findings still open after round 6: 3 in submission.md, 1 inline spec comment, 7 review findings, 2 repetitions) | removed the editor detail the instructor called too detailed (font, spell check, Tab/Esc); made Clear confirm like Examples; gave Copy and Share a manual-copy fallback; specified that the address changes only after user edits, that Back to an address with no state shows the default example, and that loading an address wins over a pending update; named deep nesting (stack overflow) as a crash the page must survive; required titles on rendered tabs; removed two repeated statements (narrow-screen width, network rule); filled in Contributions, removed the duplicate heading and leftover LLM lines, fixed draft/final placement, added a compiler test |
| 8 (DRY and one plausible result) | 25 (8 repeated rules, 3 contradictions from repetition, 3 conciseness cuts, 9 open behaviors, 2 smaller points); 25 fixed, 2 of the cuts adapted | cut acceptance criteria from 7 to 2, moving their test inputs into Safety; defined the *default example* once; removed Share from the stale-output rule and the repeated address-update trigger; trimmed Out of scope; shortened (not removed) the pulldown-cmark visibility rationale and dropped the measured stack depth; specified that an address the page cannot fully read is invalid as a whole, how long "Convert anyway" lasts, crash recovery (convert again only after reload), a large direction switch, when Examples and Clear confirm, characters as code points and what the timing covers, number-field clamping, tab selection across a switch, Markdown preset names and order, the header's crate links, and Share's manual-copy fallback |
| 9 ([latest-review.md](latest-review.md), converters run) | 8 (1 contradiction, 3 gaps, 4 trivial points); 4 fixed, 4 trivial points left as they are | tabs keep their last content only while conversion is paused, not after a crash; noted that the skip tags `SCRIPT` row relies on the playground ignoring case; the double-switch criterion ignores a trailing newline; restored that an address with no saved state loads the default example and that a loaded address replaces pending keystrokes; all Expected results rows and the Round trip verified against htmd 0.5.5 and pulldown-cmark 0.13.4 |

### Implementation review rounds

(Table showing implementation review rounds)

### Last LLM spec review

Prompting Claude Fable 5.2 with:
> Review the spec at @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md . Evaluate against @pulldown-cmark-htmd-demo-2/acceptance-criteria.md and submission template.

LLM output on last review (round 9, in full in
[latest-review.md](latest-review.md)); no finding was judged incorrect, so
nothing is struck through:

> **Converters run.** Every row under Expected results, and the Round trip,
> gives the stated output with htmd 0.5.5 and pulldown-cmark 0.13.4. The option
> defaults and the 15 extensions match the libraries.
>
> **Findings, fixed:**
>
> 1. Crash and stale output contradict each other: tabs were to keep their
>    last content while the output is stale, but stale includes "after a
>    crash", when a message replaces the output pane.
> 2. The skip tags `SCRIPT` row is not htmd's own output: htmd matches tag
>    names by case, so the row holds only through the playground's Skip tags
>    rule.
> 3. The double-switch criterion fails on a trailing newline, which is not
>    "whitespace between tags".
> 4. Two address rules were missing: what Back or Forward to an address with
>    no saved state shows, and whether a loaded address or a pending typing
>    update wins.
>
> **Trivial points, left as they are:**
>
> 5. "Valid tag name" under Skip tags is not defined.
> 6. The 10 ms speed target names the browser but no hardware.
> 7. The spec does not say whether the address still updates after a crash.
> 8. The Pages address belongs to the team's fork, while the pull request
>    targets the instructor's fork.
>
> **Submission file:** team name and GitHub userids missing; rounds 5–8 belong
> to the final submission; the final review table and last review were empty;
> the round 8 row misstated two changes; Ademi's compiler test was a table
> pasted as loose lines; the draft's "Last LLM review" shows round 1, not
> round 3; final contributions and commits are missing for two members.

### Last LLM implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)