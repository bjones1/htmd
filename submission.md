Project 2 submission
====================

Team name: htmd subscript 2

Team: Pavel Shegay, Viacheslav Sokolov, Arsen Zhanserkenov. Arsen Sailau did
not take part in this submission.

The specification is the "Subscript" section of
[translation_approach.md](translation_approach.md).

Draft submission
----------------

### Viacheslav Sokolov

GitHub userid: SokolovVD-IT

#### Contributions

The table rows and explanation of when `<sub>` becomes `~a~` and when it must
stay HTML (in-word positions, punctuation, whitespace at the edges).

#### Build and run test

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

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 4 (1 accepted, 3 rejected) | Reworded the sentence on why in-word subscripts need HTML so it no longer claims that nothing can fix them. |

#### Last specification review

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review only lines
745-774 of translation_approach.md (the table rows I added to the Subscript
section and the three explanation paragraphs below the table). Check them
against the pulldown-cmark subscript extension (Options::ENABLE_SUBSCRIPT) and
CommonMark left/right-flanking delimiter rules. Do not edit any files. List any
factual errors, unclear wording, or missing cases as numbered findings.`
produced the following output; strikethrough indicates LLM output that I
deemed incorrect or irrelevant.

1. ~~The negative example `a ~b~c` is not a valid failure case under
   CommonMark/pulldown-cmark subscript rules. The opener is left-flanking
   because it is preceded by a space, and the closer is right-flanking even
   when followed by a letter, so this should be treated as a valid subscript,
   not as a literal-text case. The prose currently says it "fails for the same
   reason on one side," which contradicts the delimiter-flanking model.~~
   (pulldown-cmark 0.13.4 renders `a ~b~c` as literal text.)
2. ~~The rule sentence is oversimplified and not the actual algorithm: "A
   single `~` opens a subscript only if the character before it is whitespace,
   punctuation, or the start of the line, and closes it only if the character
   after it is whitespace, punctuation, or the end of the line" is not the
   CommonMark left/right-flanking rule. The real test is both sides of the
   delimiter run together: opener/closer validity depends on
   left-flanking/right-flanking, not on a one-sided check of just the adjacent
   character. This matters because cases like `x.~2~` are valid even though
   the closer is not followed by whitespace/punctuation.~~ (In `x.~2~` the
   closer is followed by the end of the line, which the rule covers; for a
   single `~` the stated rule gives the same result as the flanking rules.)
3. The explanation "Nothing written next to the tildes can fix this" is too
   absolute and inconsistent with the surrounding point that the parser looks
   at the characters outside the element. In fact, changing the neighboring
   characters can make the same content valid: `H ~2~ O` is a valid subscript,
   while `H~2~O` is not. The document is describing the outside-neighbor test
   correctly, but then states the opposite too strongly.
4. ~~The section is missing a key positive case that shows the real boundary
   behavior: examples such as `a ~b~c` or `x.~2~` are exactly the cases
   CommonMark uses to demonstrate that the check is on the characters outside
   the delimiters, not on the content inside. Right now the doc only presents
   the blocking cases and omits the corresponding valid "one-side letter,
   other-side space/punctuation" cases that explain the rule.~~ (Based on
   finding 1; `x.~2~` is already a row of the table.)

### Pavel Shegay

GitHub userid: pavelshegay3 (personal GitHub account - used at first mistakenly); pashegay (correct @kbtu GitHub account - switched later in latest commits)

#### Contributions

Subscript section framing and introduction, including the example showing how
to escape a literal `~` in text.

#### Build and run test

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Verify that you can compile and run the project in the current git repo.` produced:
> Verified successfully: `cargo build --locked` compiled the project, and `cargo run --locked --example simple` ran the simple example and exited with code 0. No source files were changed.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 2 | Updated the Subscript introduction in `translation_approach.md` to clarify that tilde syntax is a proposed pulldown-cmark extension encoding, not behavior htmd currently emits, and to qualify strikethrough by its required option. |
| 2 | 2 | Updated the Subscript introduction in `translation_approach.md` to explain single-tilde delimiter constraints and clarify that the examples are proposed behavior, not current htmd output. |
| 3 | 0 | No changes, confirmed round 2 fixes. |

#### Last specification review

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review the “introduction” subsection of “Subscript” in translation_approach.md for round 3. Check it against the CommonMark spec, pulldown-cmark with ENABLE_SUBSCRIPT, and the existing “Emphasis and strong emphasis” section. Write any numbered findings to review_round_3.md. Do not edit translation_approach.md. If there are no findings, record that and briefly say whether another review round seems necessary.`
produced the following output:

> The introduction now distinguishes CommonMark from pulldown-cmark's
> `ENABLE_SUBSCRIPT` extension, notes the single-tilde flanking and intraword
> constraint, clarifies that the examples are proposed rather than current
> htmd output, and qualifies strikethrough by `ENABLE_STRIKETHROUGH`. No
> further changes to `translation_approach.md` are indicated by this review.

### Arsen Zhanserkenov

GitHub userid: TODO (Arsen: add your GitHub userid)

#### Contributions

Empty and whitespace-only `<sub>`, nesting, and the Approach steps.

#### Build and run test

Verified in this Codex session on 2026-11-10 using Rust/Cargo 1.99.1
on Windows (MSVC toolchain):

- `cargo build` — passed.
- `cargo clippy -- --deny warnings` — passed.
- `cargo test` — passed: 267 tests, 1 ignored, 0 failures (including doctests).
- `cargo run --example simple` — completed successfully with exit code 0.

`git diff --check` also passed. These checks validate the current project;
they do not establish that the proposed Subscript encoding is implemented.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 (Codex self-review) | 1 accepted | Clarified that selecting HTML in an earlier step prevents later delimiter selection; escaping still applies to the content. |
| 2 (Codex self-review) | 0 additional static findings | Checked the added table rows against the final algorithm and the existing inline/adjacency rules. Runtime validation was not performed. |

#### Last specification review

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
through pulldown-cmark in this environment, and the subsequent project build
checks passed as recorded above.

Final submission
----------------

### Viacheslav Sokolov

GitHub userid: SokolovVD-IT

#### Contributions

Rewrote the `~a~` versus HTML rules as prose after the instructor review
(character references, neighboring tildes), and implemented `<sub>` in htmd
with its tests.

#### Specification review rounds

Only rounds after the instructor's draft review; round numbers continue from
the draft submission.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 2 | 2 (2 accepted) | After the instructor review, rewrote the rows and explanation as prose rules; encoded edge whitespace as a reference to the original character and kept HTML next to another element's tilde delimiter. |
| 3 | 7 (2 accepted, 5 rejected) | Defined the neighboring characters as those of the emitted Markdown and removed a repeated statement of the `_` rule. |

#### Last specification review

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

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 0 | No changes. The reviewer ran `cargo test`, `cargo clippy -- --deny warnings` and a CRLF check, and found no contradiction with the spec or pulldown-cmark 0.13.4. |
| 2 | 0 | No changes. Re-reviewed the code after the fix for Pavel's round 1 finding (`cd0860b`); `cargo build`, `cargo clippy -- --deny warnings`, `cargo test` and `cargo fmt -- --check` passed. |

#### Last implementation review

Prompting Copilot Fusion, Standard, with
`Review round 2 of the <sub> implementation on branch subscript-spec (commit
cd0860b). Check src/element_handler/sub.rs, the ~ escaping in
src/dom_walker.rs and src/util/escape.rs, and tests/subscript_tests.rs against
the "Subscript" section of translation_approach.md (TranslationMode::Pure and
TranslationMode::Faithful, Approach steps 1-4) and against pulldown-cmark
0.13.4 with Options::ENABLE_SUBSCRIPT. Look for: behavior that contradicts the
spec, input HTML that produces Markdown which does not round-trip, missing test
cases, and violations of AGENTS.md (comments, clippy, CRLF). For every finding
give the input HTML, the actual output, and the expected output. Do not edit
any files. List serious problems as numbered findings. If there are none, say
so and whether another review round seems necessary.` produced:

> **Findings:** None. The reviewed implementation in `sub.rs`, `dom_walker.rs`,
> `escape.rs` and `subscript_tests.rs` is consistent with the documented
> Pure/Faithful behavior and the tested `ENABLE_SUBSCRIPT` round trips. The
> documented Pure-mode loss for in-word subscripts is intentional.
>
> No comment, clippy, or CRLF issues found. `cargo build`,
> `cargo clippy -- --deny warnings`, `cargo test`, and `cargo fmt -- --check`
> passed. No files were edited. **Another review round does not seem
> necessary.**

### Pavel Shegay

GitHub userid: pavelshegay3 (personal GitHub account - used at first mistakenly); pashegay (correct @kbtu GitHub account - switched later in latest commits)

#### Contributions

Revised the introduction after the instructor review: a link to the emphasis
rules, `~` following `_` rather than `*`, escaping literal tildes outside
`<sub>`, and the separate strikethrough option.

#### Specification review rounds

Only rounds after the instructor's draft review; round numbers continue from
the draft submission.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 4 | 0 | Added a link to the emphasis rules, clarified that `~` follows `_` rather than `*`, noted that literal tildes must be escaped outside `<sub>`, distinguished `ENABLE_STRIKETHROUGH` from `ENABLE_SUBSCRIPT`, and changed “examples” to “rules.” |

#### Last specification review

For round 4, prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review the updated introduction subsection of “Subscript” in translation_approach.md. Since the previous review, I added a link to “Emphasis and strong emphasis,” clarified that a single ~ follows the rules for _ rather than *, stated that literal tildes must be escaped in ordinary Markdown text outside <sub> too, clarified that ENABLE_STRIKETHROUGH is separate from ENABLE_SUBSCRIPT, and changed “The examples below” to “The rules below.” Check these changes for accuracy and clarity against CommonMark and pulldown-cmark with ENABLE_SUBSCRIPT. Do not edit any files. List any serious issues as numbered findings.` produced:

> No serious issues found. The introduction accurately describes the proposed subscript syntax, its delimiter constraint, literal-tilde escaping, and the separate strikethrough option. No further review round seems necessary.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 1 serious finding | Fixed in `cd0860b`: Pure now combines any adjacent `<sub>` run, including members with element children, attributes, or intervening comments; added round-trip tests. |
| 2 | 0 serious findings | Verified the round 1 fix against pulldown-cmark 0.13.4 and the documented Pure/Faithful behavior; no further changes needed. |

#### Last implementation review

Prompting Copilot in VS Code (model not exposed in this interface) with `Review the <sub> implementation on branch subscript-spec. Check src/element_handler/sub.rs, the ~ escaping in src/dom_walker.rs and src/util/escape.rs, and tests/subscript_tests.rs against the "Subscript" section of translation_approach.md (TranslationMode::Pure and TranslationMode::Faithful, Approach steps 1-4) and against pulldown-cmark 0.13.4 with Options::ENABLE_SUBSCRIPT. For every finding give the input HTML, the actual output, and the expected output. Do not edit any files. List serious problems as numbered findings. If there are none, say so and whether another review round seems necessary.` produced:

> No serious findings. The round 1 issue is fixed: for
> `<p><sub>a</sub><sub><em>b</em></sub></p>`, Pure mode now outputs
> `~a*b*~`, which pulldown-cmark 0.13.4 with `ENABLE_SUBSCRIPT` renders as
> `<p><sub>a<em>b</em></sub></p>`. I also checked adjacent runs with
> attributes and comments, multiple subscripts with element children, and
> whitespace-separated subscripts; their outputs match the documented Pure
> behavior. Faithful mode preserves adjacent tags as specified. The full
> `cargo test` suite passed (279 passed, 1 ignored). No further review round
> seems necessary.

### Arsen Zhanserkenov

GitHub userid: TODO (Arsen: add your GitHub userid)

#### Contributions

Documented Pure mode, empty-subscript handling, and the conservative delimiter
fallback, and aligned the Approach with review feedback.

#### Specification review rounds

Only rounds after the instructor's draft review; round numbers continue from
the draft submission.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 3 (Codex review) | 1 accepted | Specified that Pure drops an empty subscript after moving its edge whitespace, avoiding the non-subscript `~~` form. |
| 4 (Codex review) | 0 serious findings | Confirmed the final Pure and Faithful rules, including the conservative fallback for unavailable neighboring output. |

#### Last specification review

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

#### Implementation review rounds

TODO (Arsen): table of implementation review rounds.

#### Last implementation review

TODO (Arsen): LLM, prompt, and output of the last implementation review.

Implementation
--------------

Implemented by Viacheslav Sokolov with Claude Code (Claude Opus 5.5) in VS Code.
Prompt: `Implement the Subscript section of translation_approach.md in htmd,
with tests that round-trip through pulldown-cmark with ENABLE_SUBSCRIPT; make
cargo build, cargo clippy -- --deny warnings and cargo test pass.`

Changed files:

- `src/element_handler/sub.rs` (new): the `<sub>` handler. Faithful mode always
  keeps the tags (Approach step 4) and translates the children inline. Pure
  mode moves edge whitespace outside, writes `~content~`, drops an empty
  subscript, keeps only the outer level of a nested one, and combines a run of
  adjacent subscripts into one, whatever their children or attributes.
- `src/element_handler/mod.rs`: registers the handler for `sub`.
- `src/dom_walker.rs`: escapes every literal `~` in text as `\~` (Approach
  step 3), not only a `~` at the start of a line; leaves combining `<sub>`
  runs to the handler.
- `src/util/escape.rs`: the same escaping for image `alt` text.
- `tests/subscript_tests.rs` (new): 12 tests; each converts HTML and reads the
  Markdown back with pulldown-cmark `ENABLE_SUBSCRIPT`.
- `tests/basic_tests.rs`, `tests/turndown_online_demo_tests.rs`,
  `tests/html/turndown_test_index.html`: expected output updated for the
  `\~` escaping and for Pure `~ab~`.
- `translation_approach.md`: the introduction now says what htmd emits; Pure
  mode's handling of a nested subscript is stated.

Results: `cargo build` and `cargo clippy -- --deny warnings` passed;
`cargo test` passed 279 tests (267 before plus 12 new), 1 ignored.

Pavel's implementation review round 1 found that Pure mode wrote `~a~~*b*~`
for `<p><sub>a</sub><sub><em>b</em></sub></p>`, because `can_combine` merges
only elements holding a single text node. The `<sub>` handler now combines
the run itself, giving `~a*b*~`; the same fix covers adjacent subscripts with
different attributes (`~a~~b~` before, `~ab~` now).

Limitations:

- Faithful mode never emits `~x~`: a handler cannot see the emitted
  characters around the element, so the conservative fallback of step 4
  applies to every `<sub>`. The encoding steps 1-2 (`&#72;~2~&#79;`) are not
  implemented.
- Pure mode is lossy: `H<sub>2</sub>O` becomes `H~2~O`, which reads back as
  literal text; attributes and comments are dropped.
