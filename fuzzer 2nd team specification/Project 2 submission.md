Project 2 submission
====================

Team members are Altynay Kishkenebayeva, Aiym Saparkha,
<span id="author-1794145613557" class="fui-StyledText ___s8pl3v0 fz5stix f1a3p1vp f1cmbuwj" dir="auto">Daulet
Bakytkhanov</span>,
<span id="author-1794145678874" class="fui-StyledText ___s8pl3v0 fz5stix f1a3p1vp f1cmbuwj" dir="auto">Ali
Zhumatayev</span>. The specification consists of the file
[fuzzer-spec.md](fuzzer-spec.md).

Draft submission
----------------

### Contributions

* Altynay: Run the first 4 rounds of LLM reviews with constant updates
* Aiym: 
* Daulet: Generated initial specification
* Ali: Specified the input generator — grammar, element tiers, attribute and text domains, bounds, the malformed class, and the case-count breakdown — and fixed the list snapshot, `code` opacity, and `Out of Scope` classification

### Compiler test

The following statements summarize LLM output on each team member's computer,
based on the prompt "Verify that you can compile and run the project in the
current git repo."

#### Altynay

The project compiled and passed `cargo clippy -- --deny warnings` and `cargo
test` (36 unit tests passed, 1 ignored; integration/doc tests passed). The
specification is not yet fully executable: generator rules and CI case counts
are underspecified, normalization and replay need precise contracts, and
out-of-scope inputs, comments/raw content, and soft memory limits need clearer
verdicts. These issues can be resolved by specifying the generator,
deterministic normalization/replay behavior, and platform-specific test
outcomes; implementing the fuzzer itself will require adding a harness and
worker isolation.

#### Aiym

(Summarized LLM output)

#### Daulet

(Summarized LLM output)

#### Ali

The project compiled and passed all three CI commands from `AGENTS.md`: `cargo build` with no errors, `cargo clippy -- --deny warnings` with no warnings, and `cargo test` with 262 passed, 0 failed, 1 ignored across 13 test groups including doctests. The examples also ran: `simple` exited 0, `table` printed a correct Markdown table, and `page-to-markdown` converted the Hacker News page in 15 ms and the Wikipedia page in 282 ms. Toolchain was cargo and rustc 1.99.1; the working tree stayed clean, since the two output files the last example writes are git-ignored.

### Spec review rounds

| Round | Findings (issues) | Summary and round-4 status                                                                                                                                                                                            |
| ----- | ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 10                | Identified missing round-trip checks, configuration, failure rules, generation/replay, and malformed-input coverage. Round 4 adds the oracle, modes, failure classes, and seeds; generator details remain incomplete. |
| 2     | 10                | Requested a precise oracle, normalization, parser profiles, error/crash handling, and replay. Round 4 addresses most structure; normalization and generator/replay details remain unclear.                            |
| 3     | 6                 | Flagged profile/scope conflicts, seed/resource rules, normalization, and a broken section reference. Round 4 fixes most; normalization and replay/resource guarantees still need clarification.                       |
| 4     | 7                 | Remaining issues: generator coverage, case-count breakdown, reproducible replay, normalization, comments/raw content, soft memory limits, and section numbering. Not yet resolved.                                    |
| 5 | 5 | Restored the generator contract (tiers, attribute and text domains, bounds, malformed class, seed derivation); named the three class/profile combinations behind the 15,000 total; fixed the list snapshot, `code` opacity vs `preformatted_code`, and `Out of Scope` classification. One finding rejected as incorrect. |

### Last LLM review

```
prompt: Review the updated specification in specification, point out what has improved and what are issues. Write your feedback in LLM review.md appending from the last text line (do not overwrite old review).
```

### What has improved

* The guarantee is now explicitly limited to HTML-namespace body fragments in
  `Faithful` mode, and out-of-scope generated content is no longer counted as a
  successful preservation case.
* `core` and `gfm` inputs are separated, with GFM-only constructs defined and no
  ambiguous cross-profile round-trip verdict.
* The seed list, cases-per-run target, failure artifact fields, and platform
  resource-limit approaches are more concrete than in the previous revision.
* The missing revision-mapping claim has been removed. The options example
  remains compilable through `Options::default()`, and the non-lossless
  `Faithful` guarantee is appropriately qualified.

### Issues to resolve

1. <strong>The generator is no longer specified sufficiently to implement or
   assess coverage.</strong> The latest version omits the prior grammar, element
   tiers, attribute/text domains, size/depth limits, and malformed-input
   construction rules. “Seeded fuzzing” alone does not determine what inputs are
   generated. Restore a bounded generation contract, including which supported
   constructs and combinations are covered and how malformed inputs are
   produced.
2. <strong>The stated 15,000-case CI total is not fully auditable.</strong> Five
   master seeds times 1,000 cases yields 5,000 cases per class/profile
   combination, so the total assumes exactly three combinations. Name those
   combinations explicitly. In particular, the malformed-HTML run is not
   assigned a Markdown parser profile or a case count, although the execution
   order includes it.
3. <strong>Seed replay is still underspecified.</strong> Fixed master seeds and
   `rand_chacha` version logging are progress, but the specification does not
   pin the PRNG algorithm/version as a requirement or define how a seed and case
   index deterministically produce a case. Failure artifacts should record the
   generator version (and any derived seed/state needed), so a failure remains
   reproducible after generator changes; storing the original input should
   remain the definitive replay path.
4. <strong>The normalization rules do not yet define a deterministic
   algorithm.</strong> “Trim spaces around `br` boundaries,” “collapse spaces
   across adjacent inline text leaves,” and “trim leading and trailing
   whitespace at inline-run edges” can erase meaningful whitespace depending on
   traversal and node boundaries. Define precisely what is trimmed and in what
   order, how inline descendants participate, and how opaque subtrees skip
   normalization. Add required self-tests for inline boundary spaces, spaces
   adjacent to `<br>`, and whitespace around inline wrappers at block edges. The
   previous revision's useful normalizer self-test table should be retained or
   replaced by an equivalent explicit test contract.
5. <strong>Comments and raw-content elements are treated as preserved without
   establishing that they are representable.</strong> The comparison requires
   comments and `<script>`/`<style>` nodes to match unless excluded, but the
   current specification does not say how the Markdown pipeline can preserve
   them or classify their known losses. Clarify whether these constructs are
   outside the supported guarantee, are expected to fail until implemented, or
   may enter a narrow, evidenced allowlist. Do not have generated cases silently
   turn such mismatches into unexpected baseline failures.
6. <strong>The resource-limit result needs to distinguish hard from soft
   enforcement.</strong> On macOS/other, resident-size polling is explicitly a
   soft limit, yet the result categories call exceeding the memory limit a
   `Resource Limit` failure and the parent is said to kill the worker when it
   exceeds that limit. State how delayed polling/overshoot is reported and
   whether runs on those platforms can claim the same resource-safety property
   as hard-limited platforms.
7. <strong>Section numbering is inconsistent.</strong> The document labels
   “Limitations” as Section 6 after Section 12, then jumps to Section 14 for
   “Conclusion.” Renumber these headings sequentially and update any references
   to them.

#### Ali review (round 5)

Prompting Claude Opus 5 with `Review fuzzer-spec.md against the actual htmd source and list concrete findings; do not rewrite the specification` produced the following output. Strikethrough indicates output I deemed incorrect.

1. **§9, the unordered-list snapshot is wrong.** The table expects `* One`, but `ul_bullet_spacing` defaults to 3 in `src/options.rs` and is applied in `src/element_handler/li.rs`, so the output is `*   One`. §9 itself requires these snapshots to be verified against actual output before commit.
2. **§5 counts `del`/`s` as GFM-only constructs that drive the `gfm` profile, but `src/element_handler/` has no handler for either.** In `Faithful` mode such an element is emitted as raw HTML and round-trips verbatim, so the case always passes without exercising any htmd translation path, while inflating the `gfm` case count.
3. **§6 contradicts §8 on `code`.** §6 declares `code` opaque and compares it exactly, while §8 inherits `preformatted_code: false` from `Options::default()`, under which whitespace inside an inline `<code>` is not preserved.
4. **§3 classifies `Out of Scope` as a failure.** §6 requires the generator not to produce foreign-content elements, so a non-HTML namespace is a generator defect, not a conversion failure; the current table reports a harness bug as a library failure.
5. ~~`Project 2 submission.md` links to `fuzzer-spec.md`, which does not exist at that path.~~ Incorrect: both files are in the same directory, so the relative link resolves.

Findings 1 through 4 are applied in this commit. The round also restores the generation contract that round 4 recorded as missing (issue 1), names the three class/profile combinations behind the 15,000-case total (issue 2), and pins the PRNG and case-derivation rule (issue 3).

---

Final submission
----------------

### Contributions

* Altynay:
* Aiym:
* Daulet:
* Ali:

### Spec review rounds

(Table showing spec review rounds)

### Implementation review rounds

(Table showing implementation review rounds)

### Last LLM spec review

### Last LLM implementation review
