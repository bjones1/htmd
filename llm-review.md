Team LLM review and project verification
========================================

Shared review record for the strategy in [spec.md](spec.md). The four section
owners' contributions are listed below; the review rounds identify the section
being checked. This report concerns the written strategy, not an implementation
of the proposed generalized runner or completion of the test migration.

1\. Team contributions
----------------------

| Team member        | One-sentence contribution                                                                                                                          |
| ------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Yuriy Kim          | Defined the scope and common case format, integrated the four sections, and updated the inventory to the instructor's current baseline.            |
| Sam English        | Developed and refined the migration strategy and per-test inventory, including the handling of kept Pure-mode cases.                               |
| Alibek Baisholanov | Specified and reviewed the source-comparison, whitespace, parser-normalization, and stability criteria and checked them with 31 measured cases.    |
| Kamila Bagdat      | Defined stage-specific runner options, expectation-specific assertions and diagnostics, and supplied her Section 4 review and verification record. |

These contributions are supported by the authors' commits: Yuriy
[`8cd913a`](https://github.com/Samboeng76/htmd/commit/8cd913a) and
[`4d5f963`](https://github.com/Samboeng76/htmd/commit/4d5f963), Sam
[`304f22b`](https://github.com/Samboeng76/htmd/commit/304f22b) and
[`00de328`](https://github.com/Samboeng76/htmd/commit/00de328), Alibek
[`f596ee5`](https://github.com/Samboeng76/htmd/commit/f596ee5), and Kamila
[`573b542`](https://github.com/Samboeng76/htmd/commit/573b542).

2\. Review rounds
-----------------

Each row is a section-specific review, with the other sections used as context.
Section 1 and Section 2 rounds 8–9 were checked in this Codex session on
2026-11-09 using **GPT-6.1 Sol, Extra High**. Sam supplied the history for
Section 2 rounds 1–7. Alibek's Section 3 history uses the same model and effort
as the Codex checks. These are successive reviews, not claims about which
teammate personally operated the LLM.

Kamila's Section 4 history and final response are reproduced from her committed
record; its model is reported there as **OpenAI GPT-6**. Her earlier rounds
included cross-section consistency checks, so their scope is preserved here.
Finding counts belong to their respective outputs and can include repeated
observations across rounds; they are not a count of unique defects. Each zero
applies to that review's version and scope.

| Section / owner | Round | Findings | Main changes made                                                                                                                                                                                                 |
| --------------- | ----- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1 / Yuriy       | 1     | 1        | Aligned the two Faithful scope rows with Markdown-only cases and HTML assertions for preservation or approved documented differences.                                                                             |
| 1 / Yuriy       | 2     | 0        | Rechecked the corrected scope and case contract; no further relevant changes were requested.                                                                                                                      |
| 2 / Sam         | 1     | 3        | Added implementation status to the inventory and merged Section 2 into the group specification.                                                                                                                   |
| 2 / Sam         | 2     | 3        | Removed footnotes from the inventory.                                                                                                                                                                             |
| 2 / Sam         | 3     | 2        | Corrected the inventory entries for functions with multiple inputs.                                                                                                                                               |
| 2 / Sam         | 4     | 3        | Updated the specification to match the inventory and added the stability check to Section 2.                                                                                                                      |
| 2 / Sam         | 5     | 3        | Added the concurrency exception and made minor inventory corrections.                                                                                                                                             |
| 2 / Sam         | 6     | 2        | Made minor clarifications and inventory edits.                                                                                                                                                                    |
| 2 / Sam         | 7     | 0        | No further changes were requested.                                                                                                                                                                                |
| 3 / Alibek      | 1     | 3        | Defined the final-LF predicate and allowed context, distinguished Representation from Loss, and required independent expected results.                                                                            |
| 3 / Alibek      | 2     | 0        | No further changes were requested for that version of Section 3.                                                                                                                                                  |
| 3 / Alibek      | 3     | 1        | Added parser-defined representation changes and distinguished literal CR preprocessing from character-reference CR, with measured whitespace/stability examples.                                                  |
| 3 / Alibek      | 4     | 0        | The revised Section 3 had no further relevant specification findings.                                                                                                                                             |
| 4 / Kamila      | 1     | 2        | Identified missing intermediate-Markdown, per-case option, instability, and Markdown-only handling; no edits were applied in that round.                                                                          |
| 4 / Kamila      | 2     | 2        | Rechecked per-stage options and stability ambiguity; no edits were applied in that round.                                                                                                                         |
| 4 / Kamila      | 3     | 3        | Checked consistency across the specification, including instability exceptions, Markdown-only reporting, and the limits of the default helper; no edits were applied in that round.                               |
| 4 / Kamila      | 4     | 0        | Applied concise revisions for per-stage options, independent assertions, stability expectations, and expectation-specific diagnostics; the recorded final consistency review found no additional relevant issues. |

Section 3 rounds 1–2 took place on 2026-11-08; rounds 3–4 reviewed the
integrated version on 2026-11-09. The Section 1 clarification above changes no
conversion rule, comparator, or migration requirement. None of the recorded
findings were struck out as incorrect or irrelevant.

3\. Last reviews: prompts, models, and outputs
----------------------------------------------

### Section 1 — Scope and common test-case format

<strong>Model:</strong> GPT-6.1 Sol, Extra High.

<strong>Exact prompt:</strong>

```text
Review only Section 1, "Scope and common test-case format", in spec.md. Check the in/kept/out boundaries, consistency of the scope table with the case fields, Markdown-only and documented-difference cases, option handling, and retention of existing coverage. Use the other sections and current tests only as context. Report only actionable correctness findings with short corrections; do not rewrite the section or target a finding count. If no relevant findings remain, say so.
```

<strong>Final output, verbatim:</strong>

> No further relevant correctness findings in Section 1. The scope table now
> matches the optional HTML expectation in the case format: Markdown-only cases
> retain their exact Markdown assertion and stability check, while preservation
> and approved differences use their corresponding HTML assertion. Kept
> Pure-mode, extension API, concurrency, and Turndown tests retain their
> existing purpose and coverage.

### Section 2 — Migration of existing tests

<strong>Model:</strong> GPT-6.1 Sol, Extra High.

<strong>Exact prompt:</strong>

<pre>review the migration of existing tests section of spec.md</pre>

<strong>Final output, verbatim:</strong>

> <div>
>   <div>
>     <div>
>       <div>
>         <p>
>           I found no actionable correctness issues in&nbsp;“Migration of existing tests” (line 107). It covers individual input and mode cases, retained assertions, exact options, stability checks, and removal of helpers only after their callers are migrated.
>         </p>
>         <p>
>           I checked the inventory and the versioned test source. Several test files are currently deleted in the working tree, so I did not run tests.
>         </p>
>       </div>
>     </div>
>   </div>
> </div>

### Section 3 — HTML comparison and correctness criteria

**GPT-6.1 Sol, Extra High**, with the following prompt and the text of Section
3:

```text
Review only section 3, "HTML comparison and correctness criteria", in the integrated spec.md. Check the exact source-equality predicate and renderer LF allowance, internal block-separator newlines, CRLF and lone-CR preprocessing versus character references, spaces and tabs in text/code/attributes, non-breaking spaces, and Markdown stability. Use the supplied HTML/CommonMark references and measured converter outputs. Report only actionable correctness findings with short corrections; do not rewrite or review the other authors' sections, and do not target a finding count. If no relevant findings remain, say so.
```

Final output, verbatim:

> No further relevant correctness findings in Section 3. Exact source equality,
> the explicit terminal-LF allowance, internal separator handling, parser-only
> Representation changes, and source-level Loss are distinguished. Literal CR
> preprocessing is not confused with a CR produced by a character reference.
> Markdown stability is checked separately from HTML stability. The measured
> losses and two unstable Markdown cases remain failures of the corresponding
> preservation/Stable claims unless independently justified; the review does not
> approve them as new exceptions.

Reviewed Section 3 SHA-256, with LF line endings:
`4478784e7f7e5e43f37bd84a0e666ac890f2907d13a812fbd57b39b297dbfd6e`.

### Section 4 — Round-trip execution and project verification

<strong>Model:</strong> OpenAI GPT-6.

<strong>Exact review prompt used for the final written consistency
check:</strong>

> Review the updated `spec.md`, concentrating on Section 4 and its consistency
> with Sections 1–3. Check the handling of default versus generalized round-trip
> helpers, htmd and pulldown-cmark options, `Stable` versus
> `DocumentedInstability` expectations, Markdown-only cases, failure reporting,
> and the verification command list. Report only concrete outstanding problems
> in the specification; do not infer that the proposed runner has been
> implemented.

<strong>Final review output:</strong>

> No additional relevant findings in the supplied specification. Section 4 now
> distinguishes the stage-specific converter options; preserves the intermediate
> Markdown; specifies `Stable` and approved `DocumentedInstability` comparisons;
> allows exact-Markdown cases without an HTML expectation; and reports failures
> according to which assertion failed. Section 3 now states the
> documented-instability exception explicitly. This is a consistency review of
> the written strategy, not a source-code execution or validation of the
> unimplemented generalized runner.

4\. Compile and run verification
--------------------------------

Exact instructor-requested prompt:

```text
Verify that you can compile and run the project in the current git repo.
```

Verification date: **2026-11-09**, Asia/Almaty. The integrated implementation
baseline is merge commit `41d1d44`; subsequent edits in this work affect
Markdown documents only. Rust 1.99.1 / Cargo 1.99.1 on macOS ARM64.

LLM verification output:

> I verified that I can compile and run htmd in the current repository. The
> fresh build, Clippy run with warnings denied, full test suite, and simple
> example all exited successfully. The suite reported 263 passing
> unit/integration tests and 4 passing doctests, with one intentionally ignored
> table-generator test and zero failures.

| Command                           | Exit code | Result                                                                                 |
| --------------------------------- | --------- | -------------------------------------------------------------------------------------- |
| `cargo build`                     | 0         | Fresh build passed.                                                                    |
| `cargo clippy -- --deny warnings` | 0         | Fresh Clippy run passed with warnings denied.                                          |
| `cargo test`                      | 0         | 263 unit/integration tests and 4 doctests passed; 1 intentionally ignored; 0 failures. |
| `cargo run --example simple`      | 0         | The example ran and its conversion assertions passed.                                  |

These checks ran before the shared-report assembly. The current Rust source,
tests, examples, Cargo.toml, and Cargo.lock are unchanged from that verified
baseline; subsequent work changes Markdown documents only.

5\. Whitespace and stability evidence
-------------------------------------

An external probe ran **31 cases** through the real default Faithful converter,
pulldown-cmark with empty extension flags, and a second conversion. It also
compared the parsed trees for these fixtures to distinguish parser normalization
from changed text. This is measurement evidence, not the new case runner
proposed in the specification.

| Case                                        | Measured observation and consequence                                                                                 |
| ------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| Paragraph without/with final LF             | Select suffix `"\n"` or `""` respectively; no trimming is needed.                                                    |
| Literal CRLF and lone CR                    | HTML preprocessing yields LF; record the explicit Representation result.                                             |
| Flow-text double spaces, tab, or LF         | Current htmd collapses them; this changes parsed text and fails Preserved.                                           |
| Two compact paragraphs                      | The renderer inserts an internal separator LF; a final suffix alone does not cover it.                               |
| Fenced code with tabs, LF, and a blank line | The tested interior characters are retained, plus the renderer's final LF after the block.                           |
| `&nbsp;`                                    | It becomes U+00A0; replacing that value with an ordinary space would lose content.                                   |
| CR from `&#13;` in code                     | It is produced after input preprocessing and differs on the next trip; a global CR replacement would hide this.      |
| Inline code with LF                         | HTML is stable, but Markdown changes from `<code>a b</code>` to backtick code; HTML stability alone is insufficient. |

In this probe, **29/31** cases were Markdown-stable. The two counterexamples
were inline code with LF and code containing a referenced CR. They remain
failures of a default Stable claim unless a normative rule independently
justifies a documented exception; recording their output alone does not approve
them. Source changes and stability results are reported separately.

6\. Review provenance
---------------------

The common report preserves these committed source records:

* [Alibek's Section 3 review and verification](https://github.com/Samboeng76/htmd/blob/9617409661f2e184181766d1f7883a1f66dd51d9/llm-review-alibek.md).
* [Kamila's supplied Section 4 review](https://github.com/Samboeng76/htmd/blob/602dff476e967c22e290b34a6d8322e10e2d7ef8/llm-review-kamila.md).
* [Original Section 3 rounds 1–2 record](https://github.com/Samboeng76/htmd/blob/404273c1e68c0a8be82e303f80823bfe598c42b9/submission.md).

The submission documents are the shared `spec.md` and this `llm-review.md`;
retain [the inventory](testing_standardization_test_inventory.md) referenced by
the strategy. The personal source records remain available for provenance.
