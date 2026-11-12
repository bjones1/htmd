# Alibek Baisholanov — LLM review and project verification

Reviewed section: [3. HTML comparison and correctness criteria](spec.md#3-html-comparison-and-correctness-criteria).
Model: **GPT-6.1 Sol**, reasoning **Extra High**, as confirmed in the Codex interface. These are successive self-reviews in this Codex session.
Other authors' review histories belong in their own `llm-review-NAME.md` files.

## 1. Team contributions

| Team member | One-sentence contribution |
| --- | --- |
| Yuriy Kim | Defined the scope and common case format, integrated the four sections, and updated the inventory to the instructor's current baseline. |
| Sam English | Developed and refined the migration strategy and per-test inventory, including the handling of kept Pure-mode cases. |
| Alibek Baisholanov | Specified and reviewed the source-comparison, whitespace, parser-normalization, and stability criteria and checked them with 31 measured cases. |
| Kamila Bagdat | Defined stage-specific runner options, expectation-specific assertions and diagnostics, and supplied her Section 4 review and verification record. |

These contributions are supported by the authors' commits: Yuriy
[`8cd913a`](https://github.com/Samboeng76/htmd/commit/8cd913a) and
[`4d5f963`](https://github.com/Samboeng76/htmd/commit/4d5f963), Sam
[`304f22b`](https://github.com/Samboeng76/htmd/commit/304f22b) and
[`00de328`](https://github.com/Samboeng76/htmd/commit/00de328), Alibek
[`f596ee5`](https://github.com/Samboeng76/htmd/commit/f596ee5), and Kamila
[`573b542`](https://github.com/Samboeng76/htmd/commit/573b542).

## 2. Review rounds — Alibek's section only

Rounds 1–2 were performed on 2026-11-08 for the original comparison section.
Rounds 3–4 were performed on 2026-11-09 after merging the scope/migration and
round-trip-verification branches. Earlier zero-finding results concern their
saved earlier inputs, not every later edit to the shared specification.

| Round | Findings | Main changes made |
| --- | --- | --- |
| 1 | 3 | Defined the final-LF predicate and allowed context, distinguished Representation from Loss, and required independent expected results. |
| 2 | 0 | No further changes were requested for that version of Section 3. |
| 3 | 1 | Added parser-defined representation changes, cited HTML preprocessing rules, distinguished literal CR from `&#13;`, and added measured whitespace/stability examples. |
| 4 | 0 | The revised Section 3 had no further relevant specification findings. |

All four findings across rounds 1 and 3 were applied; none were crossed out.
The last review used the actual 31-case observations and the primary
[HTML parsing](https://html.spec.whatwg.org/multipage/parsing.html#preprocessing-the-input-stream)
and [CSS whitespace](https://www.w3.org/TR/css-text-3/#white-space-processing) rules. It reviewed the strategy, not a
completed implementation of the proposed generalized runner.

## 3. Last review: exact prompt, model, and output

**GPT-6.1 Sol, Extra High**, with the following prompt and the text of Section 3:

```text
Review only section 3, "HTML comparison and correctness criteria", in the integrated spec.md. Check the exact source-equality predicate and renderer LF allowance, internal block-separator newlines, CRLF and lone-CR preprocessing versus character references, spaces and tabs in text/code/attributes, non-breaking spaces, and Markdown stability. Use the supplied HTML/CommonMark references and measured converter outputs. Report only actionable correctness findings with short corrections; do not rewrite or review the other authors' sections, and do not target a finding count. If no relevant findings remain, say so.
```

Final output, verbatim:

> No further relevant correctness findings in Section 3. Exact source equality, the explicit terminal-LF allowance, internal separator handling, parser-only Representation changes, and source-level Loss are distinguished. Literal CR preprocessing is not confused with a CR produced by a character reference. Markdown stability is checked separately from HTML stability. The measured losses and two unstable Markdown cases remain failures of the corresponding preservation/Stable claims unless independently justified; the review does not approve them as new exceptions.

Reviewed Section 3 SHA-256, with LF line endings:
`4478784e7f7e5e43f37bd84a0e666ac890f2907d13a812fbd57b39b297dbfd6e`.

The original rounds 1–2 prompt and output are retained in the earlier
[recorded report](https://github.com/Samboeng76/htmd/blob/404273c1e68c0a8be82e303f80823bfe598c42b9/submission.md).

## 4. Compile and run verification

Exact instructor-requested prompt:

```text
Verify that you can compile and run the project in the current git repo.
```

Verification date: **2026-11-09**, Asia/Almaty. The integrated implementation
baseline is merge commit `41d1d44`; subsequent edits in this work affect
Markdown documents only. Rust 1.99.1 / Cargo 1.99.1 on macOS ARM64.

LLM verification output:

> I verified that I can compile and run htmd in the current repository. The fresh build, Clippy run with warnings denied, full test suite, and simple example all exited successfully. The suite reported 263 passing unit/integration tests and 4 passing doctests, with one intentionally ignored table-generator test and zero failures.

| Command | Exit code | Result |
| --- | --- | --- |
| `cargo build` | 0 | Fresh build passed. |
| `cargo clippy -- --deny warnings` | 0 | Fresh Clippy run passed with warnings denied. |
| `cargo test` | 0 | 263 unit/integration tests and 4 doctests passed; 1 intentionally ignored; 0 failures. |
| `cargo run --example simple` | 0 | The example ran and its conversion assertions passed. |

## 5. Whitespace observations

An external probe ran **31 cases** through the real default Faithful converter,
pulldown-cmark with empty extension flags, and a second conversion. It also
compared the parsed trees for these fixtures to distinguish parser
normalization from changed text. This is measurement evidence, not the new
case runner proposed in the specification.

| Case | Measured observation and consequence |
| --- | --- |
| Paragraph without/with final LF | Select suffix `"\n"` or `""` respectively; no trimming is needed. |
| Literal CRLF and lone CR | HTML preprocessing yields LF; record the explicit Representation result. |
| Flow-text double spaces, tab, or LF | Current htmd collapses them; this changes parsed text and fails Preserved. |
| Two compact paragraphs | The renderer inserts an internal separator LF; a final suffix alone does not cover it. |
| Fenced code with tabs, LF, and a blank line | The tested interior characters are retained, plus the renderer's final LF after the block. |
| `&nbsp;` | It becomes U+00A0; replacing that value with an ordinary space would lose content. |
| CR from `&#13;` in code | It is produced after input preprocessing and differs on the next trip; a global CR replacement would hide this. |
| Inline code with LF | HTML is stable, but Markdown changes from `<code>a b</code>` to backtick code; HTML stability alone is insufficient. |

In this probe, **29/31** cases were Markdown-stable. The two counterexamples
were inline code with LF and code containing a referenced CR. They remain
failures of a default Stable claim unless a normative rule independently
justifies a documented exception; recording their output alone does not
approve them. Source changes and stability results are reported separately.

## Submission structure

The strategy is shared in `spec.md`. This file is Alibek's personal record;
Kamila's supplied record is preserved in [llm-review-kamila.md](llm-review-kamila.md).
Sam and Yuriy add their own actual review records under the same naming scheme.
