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
| 3 | 7 (2 accepted, 5 rejected) | Defined the neighboring characters as those of the emitted Markdown and removed a repeated statement of the `_` rule. |

### Pavel Shegay

| Round | Findings | Main changes made                                                                                                                                                                                                                     |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 2        | Updated the Subscript introduction in `translation_approach.md` to clarify that tilde syntax is a proposed pulldown-cmark extension encoding, not behavior htmd currently emits, and to qualify strikethrough by its required option. |
| 2     | 2        | Updated the Subscript introduction in `translation_approach.md` to explain single-tilde delimiter constraints and clarify that the examples are proposed behavior, not current htmd output.                                           |
| 3     | 0        | No changes, confirmed round 2 fixes.                                                                                                                                                                                              |
| 4     | 0        | Added a link to the emphasis rules, clarified that `~` follows `_` rather than `*`, noted that literal tildes must be escaped outside `<sub>`, distinguished `ENABLE_STRIKETHROUGH` from `ENABLE_SUBSCRIPT`, and changed “examples” to “rules.” |

### Arsen Zhanserkenov

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 (Codex self-review) | 1 accepted | Clarified that selecting HTML in an earlier step prevents later delimiter selection; escaping still applies to the content. |
| 2 (Codex self-review) | 0 additional static findings | Checked the added table rows against the final algorithm and the existing inline/adjacency rules. Runtime validation was not performed. |
| 3 (Codex review) | 1 accepted | Specified that Pure drops an empty subscript after moving its edge whitespace, avoiding the non-subscript `~~` form. |
| 4 (Codex review) | 0 serious findings | Confirmed the final Pure and Faithful rules, including the conservative fallback for unavailable neighboring output. |

## Last review

### Viacheslav Sokolov

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review only the
paragraphs I wrote in the Subscript section of translation_approach.md (from
"Translating `<sub>` to tildes" to "Step 4 of the approach below says when
these steps apply."). Check them against the pulldown-cmark subscript
extension (Options::ENABLE_SUBSCRIPT), the CommonMark rules for `_` delimiter
runs, and the rest of the Subscript section for contradictions or repetition.
Do not edit any files. List any factual errors, unclear wording, or missing
cases as numbered findings.` produced the following output; strikethrough
indicates LLM output that I deemed incorrect or irrelevant.

> 1. ~~"Literal tildes in ordinary Markdown text ... must be escaped as `\~`"
>    is too absolute. In pulldown-cmark, `~` is special only when it can
>    participate in a valid subscript/strikethrough delimiter run; bare `~` or
>    `~~` in ordinary text may still remain literal. For example, with
>    `Options::ENABLE_SUBSCRIPT` alone, `~~a~~` is parsed as literal text, not
>    as a subscript or as a required escape target. The section should say
>    "escape a literal `~` when it could start/close a delimiter run," not "all
>    literal tildes must be escaped."~~ (Outside my paragraphs; escaping every
>    literal tilde is the conservative rule the instructor asked for.)
> 2. ~~The summary "a single `~` follows the rules for `_`, not `*`" is not
>    the whole flanking story. pulldown-cmark adds a special-case for adjacent
>    `~` runs: a single `~` may still be treated as opening/closing when the
>    previous or next character is another `~` and the other side is not
>    punctuation. That matters in cases like `~a~~b~` or `~~x~~~a~`, where the
>    decision depends on the whole delimiter run, not just "`_`-style"
>    neighbors. The section should mention this extra rule or it will miss real
>    parser behavior.~~ (Checked with pulldown-cmark 0.13.4: `~a~~b~` and
>    `~~x~~~a~` give no subscript; the rule already keeps HTML next to another
>    tilde delimiter.)
> 3. ~~The flanking-rule description is too simplified for the actual
>    CommonMark logic. "If `1` is neither whitespace nor punctuation, encode
>    `1` …" and the corresponding check for `5` describes only a coarse
>    approximation. The actual rules are run-length-sensitive and depend on
>    whether the delimiter reaches a whitespace/punctuation boundary, whether
>    the neighboring code point is a `~` from another run, and whether the run
>    is length 1 vs length >1. The prose is therefore not a faithful summary of
>    the spec and can lead to wrong output in edge cases.~~ (The steps are a
>    sufficient encoding, not a summary: they always place `;` or `&` next to a
>    single tilde; `&#72;~2~&#79;` renders as `H<sub>2</sub>O`.)
> 4. The prose mixes two different notions of "neighboring characters": the
>    original HTML source characters (`1`/`5`) and the emitted Markdown text
>    after escaping and raw-HTML handling. Step 4 says "retain tags whenever the
>    characters outside the element are unknown," but the earlier flanking
>    checks are described as if they can be decided from the original
>    neighboring characters alone. In pulldown-cmark, the actual parse depends
>    on the emitted text that the reader sees, including escapes and adjacent
>    raw HTML, so the section should be clearer about checking the output text,
>    not just original source adjacency.
> 5. ~~The section is missing a concrete note that delimiter runs are not
>    decided only by the immediate character classes but also by word
>    boundaries. The doc correctly says "cannot delimit text within a word,"
>    but it never spells out that this is the same `_`-style intraword
>    restriction as CommonMark, and that this is why `foo~a~bar` stays literal
>    while `~a~` works. Without that explicit tie-back to the `_` rules, the
>    reader may assume "single tilde is just a kind of punctuation rule" rather
>    than the actual flanking behavior.~~ (The introduction already ties `~` to
>    the `_` rules; repeating it would break DRY.)
> 6. ~~The "adjacent subscripts" paragraph is a bit internally fuzzy. It says
>    adjacent subscripts "retain separate wrappers," but the later approach
>    says "Adjacent subscripts are combined under the adjacent-inline rule in
>    the emphasis section." Those are not clearly the same claim. If the
>    intended rule is "keep separate HTML wrappers but treat them as adjacent
>    inline elements," the wording should say that explicitly; otherwise it
>    reads as two different strategies.~~ (Not in my paragraphs; Faithful keeps
>    separate wrappers and Pure combines them, as in the emphasis section.)
> 7. The section is repetitive in a way that makes the key rule harder to
>    follow: it repeats the same point several times that single `~` behaves
>    like `_`, that content may not begin/end with whitespace, and that a
>    subscript with a dangerous neighboring tilde should fall back to HTML. The
>    actual rule set is small, but the repeated examples and warnings make it
>    hard to distinguish the normative algorithm from the conservative
>    fallback. The prose would be clearer if the "actual flanking algorithm"
>    and the "conservative HTML fallback" were separated more sharply.

### Pavel Shegay

For round 4, prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review the updated introduction subsection of “Subscript” in translation_approach.md. Since the previous review, I added a link to “Emphasis and strong emphasis,” clarified that a single ~ follows the rules for _ rather than *, stated that literal tildes must be escaped in ordinary Markdown text outside <sub> too, clarified that ENABLE_STRIKETHROUGH is separate from ENABLE_SUBSCRIPT, and changed “The examples below” to “The rules below.” Check these changes for accuracy and clarity against CommonMark and pulldown-cmark with ENABLE_SUBSCRIPT. Do not edit any files. List any serious issues as numbered findings.` produced:

> No serious issues found. The introduction accurately describes the proposed subscript syntax, its delimiter constraint, literal-tilde escaping, and the separate strikethrough option. No further review round seems necessary.

### Arsen Zhanserkenov

Prompting Codex (GPT-5) with `Review only Arsen Zhanserkenov's current changes
in the Subscript section of translation_approach.md: the empty and
whitespace-only handling, nesting explanation, and Approach rules for
TranslationMode::Pure and TranslationMode::Faithful. Check them for internal
contradictions and against the existing "Emphasis and strong emphasis" rules.
Do not edit any files. List serious problems as numbered findings.` produced:

> No numbered findings.
>
> Pure mode now ignores attributes, moves edge whitespace outside the generated
> single-tilde delimiters, combines adjacent subscripts under the documented
> adjacent-inline rule, and drops an element with no remaining content instead
> of emitting `~~`. Faithful mode retains tags for empty or whitespace-edge
> content, comments, nested subscripts, attributes, adjacent subscripts, and
> unavailable neighboring output. The future delimiter check is explicitly
> separated from the current conservative fallback.
