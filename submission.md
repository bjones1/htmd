# Project 2 draft submission: htmd subscript 2

Team: Pavel Shegay, Viacheslav Sokolov, Arsen Zhanserkenov. Arsen Sailau did
not take part in this submission.

## Contributions

- Pavel Shegay: Subscript section framing and introduction, including the example showing how to escape a literal `~` in text.
- Viacheslav Sokolov: the rules for translating `<sub>` to tildes or keeping it
  as HTML (characters on both sides of each delimiter, character references,
  neighboring tildes), first as table rows and then, after the instructor
  review, as prose.
- Arsen Zhanserkenov: empty and whitespace-only `<sub>`, nesting, and the
  Approach steps.

## Build check

### Viacheslav Sokolov

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Verify that you can
compile and run the project in the current git repo.` produced:

> **Verified successfully.** Using Cargo from `C:\Users\Admin\.cargo\bin`
> because it wasn't on `PATH`:
>
> - `cargo build` — passed
> - `cargo clippy -- --deny warnings` — passed
> - `cargo test` — passed: 267 tests, 1 ignored
> - `cargo run --example simple` — ran successfully
>
> The working tree remains clean.

### Pavel Shegay

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Verify that you can compile and run the project in the current git repo.` produced:
> Verified successfully: `cargo build --locked` compiled the project, and `cargo run --locked --example simple` ran the simple example and exited with code 0. No source files were changed.

### Arsen Zhanserkenov

Verified in this Codex session on 2026-11-10 using Rust/Cargo 1.99.1
on Windows (MSVC toolchain):

- `cargo build` — passed.
- `cargo clippy -- --deny warnings` — passed.
- `cargo test` — passed: 267 tests, 1 ignored, 0 failures (including doctests).
- `cargo run --example simple` — completed successfully with exit code 0.

`git diff --check` also passed. These checks validate the current project;
they do not establish that the proposed Subscript encoding is implemented.

## Review rounds

### Viacheslav Sokolov

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 4 (1 accepted, 3 rejected) | Reworded the sentence on why in-word subscripts need HTML so it no longer claims that nothing can fix them. |
| 2 | 2 (2 accepted) | After the instructor review, rewrote the rows and explanation as prose rules; encoded edge whitespace as a reference to the original character and kept HTML next to another element's tilde delimiter. |

### Pavel Shegay

| Round | Findings | Main changes made                                                                                                                                                                                                                     |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 2        | Updated the Subscript introduction in `translation_approach.md` to clarify that tilde syntax is a proposed pulldown-cmark extension encoding, not behavior htmd currently emits, and to qualify strikethrough by its required option. |
| 2     | 2        | Updated the Subscript introduction in `translation_approach.md` to explain single-tilde delimiter constraints and clarify that the examples are proposed behavior, not current htmd output.                                           |
| 3     | 0        | No changes, confirmed round 2 fixes.                                                                                                                                                                                                  |

### Arsen Zhanserkenov

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 (Codex self-review) | 1 accepted | Clarified that selecting HTML in an earlier step prevents later delimiter selection; escaping still applies to the content. |
| 2 (Codex self-review) | 0 additional static findings | Checked the added table rows against the final algorithm and the existing inline/adjacency rules. Runtime validation was not performed. |

## Last review

### Viacheslav Sokolov

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review only the paragraphs
I rewrote in the Subscript section of translation_approach.md (from
"Translating `<sub>` to tildes" to "kept for that future improvement"). Check
them against the pulldown-cmark subscript extension (Options::ENABLE_SUBSCRIPT)
and the CommonMark rules for `_` delimiter runs. Do not edit any files. List
any factual errors, unclear wording, or missing cases as numbered findings.`
produced the following output; strikethrough indicates LLM output that I
deemed incorrect or irrelevant.

> 1. **Whitespace preservation:** The instruction to encode any whitespace edge
>    as `&#32;` preserves only an ordinary space. For tabs, line breaks, or
>    other Unicode whitespace, it changes the subscript's content. Specify that
>    this applies only to U+0020, or encode the original whitespace character.
> 2. **Adjacent tilde runs:** The outside-character rule treats `~` as safe
>    punctuation, but pulldown-cmark matches tilde delimiters only when their
>    run lengths agree. For example, `<sub>a</sub>~` would produce `~a~~`, whose
>    opening and closing runs do not match; the subscript won't parse. The
>    procedure needs a guard or handling for neighboring tildes.

Both findings were accepted after checking them with pulldown-cmark 0.13.4:
`a ~&#160;b~ c` keeps the no-break space, `~a~\~` parses, and `~~x~~~a~` does
not.

### Pavel Shegay

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review the “introduction” subsection of “Subscript” in translation_approach.md for round 3. Check it against the CommonMark spec, pulldown-cmark with ENABLE_SUBSCRIPT, and the existing “Emphasis and strong emphasis” section. Write any numbered findings to review_round_3.md. Do not edit translation_approach.md. If there are no findings, record that and briefly say whether another review round seems necessary.`
produced the following output:
> The introduction now distinguishes CommonMark from pulldown-cmark's
> `ENABLE_SUBSCRIPT` extension, notes the single-tilde flanking and intraword
> constraint, clarifies that the examples are proposed rather than current
> htmd output, and qualifies strikethrough by `ENABLE_STRIKETHROUGH`. No
> further changes to `translation_approach.md` are indicated by this review.


### Arsen Zhanserkenov

This was a Codex self-review within the editing session, not an independent
Copilot Fusion review. The user requested completion of Arsen's part; no
separate review prompt or external reviewer response is claimed.

Scope: the empty/whitespace-only and nesting explanations, the added table
rows, and the Approach steps in the Subscript section. The review compared
them with the document's raw HTML inline and emphasis rules and inspected
[pulldown-cmark 0.13.4 delimiter checks](https://github.com/pulldown-cmark/pulldown-cmark/blob/v0.13.4/pulldown-cmark/src/firstpass.rs#L2138-L2223).

1. Accepted and fixed: make the HTML fallback terminal for delimiter selection,
   while still applying text escaping to content inside retained tags.

Final static pass: no additional findings in the added material. Nested and
adjacent subscripts preserve wrappers; element children use the stated
conservative fallback; the algorithm explicitly describes proposed Faithful
behavior. `git diff --check` passed. The Markdown examples have not been run
through pulldown-cmark in this environment, and the subsequent project build checks passed as recorded above.
If the assignment requires an independent Copilot review, that review is still
outstanding.
