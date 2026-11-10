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