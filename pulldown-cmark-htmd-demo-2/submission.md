Project 2 submission
====================

Team members are:
1. Daniil Kaimekin (da_kaimekin@kbtu.kz);
2. Ademi Bakhytova (ad_bakhytova@kbtu.kz);
3. Aruzhan Akhanayeva (ar_akhanayeva@kbtu.kz);
4. Alisher Akim (ali_akim@kbtu.kz) (couldn't connect with them, unfortunately);

The specification consists of
the file [htmddemo2-spec.md](htmddemo2-spec.md).

Draft submission
----------------

### Contributions

* Daniil: (Andy's contributions)
* Ademi: (Bob's contributions)
* Aruzhan: (Victoria's contributions)


### Compiler test

The following statements summarize LLM output on each team member's computer,
based on the prompt "Verify that you can compile and run the project in the
current git repo."

#### Daniil

Build, clippy and tests all pass. Now I’m running the examples to confirm the library works at runtime.

The project compiles and runs. All three CI commands from AGENTS.md pass:

cargo build: finished cleanly.
cargo clippy -- --deny warnings: no warnings.
cargo test: every test suite passed. The largest suites had 91 and 37 tests, with 1 test ignored and 0 failed.

htmd is a library, so to check it at runtime I ran its three examples:

simple: all its built-in checks passed.
table: printed a correctly formatted Markdown table.
page-to-markdown: converted the saved Hacker News page in 25ms and the Wikipedia page in 310ms.

None of this changed your working tree. Your existing edits in pulldown-cmark-htmd-demo-2/ are still there and uncommitted.

#### Daniil

(Summarized LLM output)

#### Ademi

(Summarized LLM output)

#### Aruzhan

(Summarized LLM output)

### Spec review rounds

| round | findings | main changes made |
|---|---|---|
| 1 | 15 | fixed version mismatch, number input clamping, deployment url, CI job mismatch|
| 2 (Ademi Bakhytova) | 30 (6 contradictions, 10 technical issues, 8 ambiguities, 6 missing requirements) | corrected htmd API assumptions, improved WASM panic handling, fixed the GitHub Pages deployment path, added CSP protection for sandboxed previews, clarified permalink encoding, improved CI configuration, testing requirements, and UI accessibility; 5 design decisions (C1, C2, C3, C7, D1) need team confirmation, and B5/C8 keep documented caveats |
| 3 (Aruzhan Akhanayeva) | 27 (4 contradictions, 8 technical errors, 10 ambiguities, 5 missing requirements) | fixed the default-state crash (`skip_tags(vec![])` panics), closed the preview frames' meta-refresh navigation hole, froze the permalink `v=1` defaults and added a versioning rule, defined empty-input and strict-decoding permalink rules and `hashchange` failure handling, corrected the crash banner and added a crash-loop escape link, dropped strikethrough from the Preview and the redundant root `Cargo.toml` edit, split the status line into conversion and notice parts, made the deploy job reuse the check job's verified build with a checksum-pinned Trunk, specified option-combination and size-limit tests, and added accessibility (live regions, ARIA tabs), article-licence, and getrandom 0.2 requirements; marked round 2's open decisions as provisional in the spec header |
| **4 (instructor review, bjones1)** | **15 (overall review, 11 inline spec comments, 3 submission comments)** | **rewrote the spec to state behavior, not implementation; added the Markdown → HTML direction with a one-editor direction switch; listed every htmd and pulldown-cmark option with its control and default (Pure mode, math on); added expected-results tables checked against htmd 0.5.5 and pulldown-cmark 0.13.4; removed the API tables, repo layout, Leptos idioms, file tree, CI and implementation-order sections, including most round 2 and 3 additions** |
| 5 ([latest-review.md](latest-review.md)) | 30 (12 inconsistencies, 15 incomplete scenarios, 3 vague objectives); 13 fixed, 17 left as low-priority implementation detail | large input is kept out of permalinks and Share; defined the round-trip criterion; Markdown presets turn on their extension; switch, Copy and Share use up-to-date output; rendered tabs can't restyle the page and links do nothing; a single source for defaults; two-way footnote coupling; Copy input on crash; measurable size and speed targets; more acceptance criteria |

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

* Daniil: Initial spec draft, wasm-markdown exploration, commonmark exploration, leptos exploration.
* Ademi: (Ademi's contributions)
* Aruzhan: (Aruzhan's contributions)
* Alisher: (Alisher's contributions)

### Spec review rounds

(Table showing spec review rounds)

### Implementation review rounds

(Table showing implementation review rounds)

### Last LLM spec review

(LLM output, possibly with strikethrough)

### Last LLM implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)