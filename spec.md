# Round-trip testing strategy

This specification describes how htmd's integration tests are standardized around two properties: the HTML → Markdown → HTML round trip, and the stability of htmd's Markdown across a second trip. Conversion behavior remains governed by [translation_approach.md](https://github.com/Samboeng76/htmd/blob/spec/scope/translation_approach.md); conversion differences cite its rules; parser-only representation changes cite the HTML rules in section 3. Sam's [initial note](https://github.com/Samboeng76/htmd/blob/main/testing_standardization_ideas.md) provides the starting goal: test Faithful conversion through HTML → Markdown → HTML.

| **Section** | **Owner** | **Responsibility**                            |
| ----------- | --------- | --------------------------------------------- |
| 1           | Yuriy     | Scope and common test-case format             |
| 2           | Sam       | Migration of existing tests                   |
| 3           | Alibek    | HTML comparison and correctness criteria      |
| 4           | Kamila    | Round-trip execution and project verification |

This submission specifies the strategy. Implementing the helper changes and migrating the complete test suite are subsequent work.

## Terms

These terms mean the same thing in every section.

| **Term**               | **Meaning**                                                                                                                                       |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Case                   | One input with its options and expectations. A test function holds one or more cases.                                                             |
| `H`                    | A case's literal input HTML.                                                                                                                      |
| `M(H)`                 | The Markdown htmd writes for `H` under the case's htmd options.                                                                                   |
| `R(H)`                 | One round trip: `M(H)` rendered back to HTML by pulldown-cmark under the case's pulldown-cmark options.                                           |
| Preserved              | `R(H)` equals `H`, apart from the renderer suffix section 3 allows.                                                                               |
| Documented difference  | A case's statement that `R(H)` differs from `H`: its kind (`Representation` or `Loss`), the expected HTML, a reason, and a reference (section 3). |
| Stable                 | Converting `R(H)` again returns the same Markdown: `M(R(H)) == M(H)` (section 3). Also called idempotency.                                        |
| Documented instability | A case's statement that its Markdown is not stable: the second Markdown, a reason, and a reference (section 3).                                   |
| Kept test              | A test outside the case format, which keeps its existing assertions (1.2).                                                                        |
| Unspecified            | A construct `translation_approach.md` marks as unspecified. No test asserts its output.                                                           |

## 1. Scope and common test-case format

### 1.1 Goal

The current tests check the right things but do not agree on how:

- **Six helpers check translations differently.** `round_trip` and `render_markdown` (`tests/common/mod.rs`), `assert_round_trips`, `assert_math_round_trips` and `assert_paragraph_survives` (`basic_tests.rs`), and `assert_html_fallback` (`table_tests.rs`). Markdown is read back with different pulldown-cmark options (none, math, tables, or both); `assert_html_fallback` does not read it back at all. Callers handle the trailing newline and `&#10;` differently: appending `\n`, trimming, or asserting `&#10;` literally.
- **Nothing checks stability.** No test converts htmd's Markdown back to HTML and then to Markdown again.
- **Differences are not documented.** Where a trip loses something, the test asserts the lossy string, usually without a reason or a reference to the rule that explains it.
- **Styles are mixed.** 227 tests in 10 files; `table_tests.rs` uses nested modules, and `table_tests.rs` and `event_tests.rs` use `test_` names.

With one case format and one runner (section 4), every case states what it expects and why, and a reader can tell a documented difference from a bug.

### 1.2 Scope

| **In / kept / out** | **Item**                                                                                                  | **Checked for**                                                                         | **Reason**                                                                                                                                                                                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| In                  | Integration tests in `tests/` which translate HTML in `TranslationMode::Faithful`                         | Stability and any required Markdown assertion; HTML assertion for preservation or an approved difference (sections 2–3)                                                    | Faithful mode exists to preserve the original HTML.                                                                                                                                                                     |
| In                  | Faithful cases with non-default options (`HeadingStyle::Setex`, link styles, ...)                         | Exact Markdown and stability; HTML assertion for preservation or an approved difference | Options change the Markdown spelling, not the HTML it means.                                                                                                                                                            |
| Kept                | Pure mode                                                                                                 | Existing assertions                                                                     | Pure mode drops HTML by design, so most round trips would be documented losses. Checking the stability of Pure-mode Markdown is follow-up work.                                                                         |
| Kept                | Extension API tests: handlers registered with `add_handler`, and element events (`event_tests.rs`)        | Existing assertions                                                                     | The handler or listener decides the output; `translation_approach.md` only constrains it ([Translating HTML nodes](https://github.com/Samboeng76/htmd/blob/spec/scope/translation_approach.md#translating-html-nodes)). |
| Kept                | Shared-converter concurrency (`multithreading` in `basic_tests.rs`)                                       | Existing assertions                                                                     | It checks simultaneous use of one converter, which the common case runner does not exercise.                                                                                                                            |
| Kept                | Turndown cases (`turndown_cases_tests.rs`, `turndown_online_demo_tests.rs`)                               | Existing assertions                                                                     | They compare htmd's Markdown with the Markdown each Turndown case expects.                                                                                                                                              |
| Out                 | Unit tests in `src/`                                                                                      | —                                                                                       | They test private helpers, not translations.                                                                                                                                                                            |
| Out                 | Output of a custom handler which breaks the rules of "Translating HTML nodes", e.g. a bare inline newline | —                                                                                       | `translation_approach.md` leaves it unspecified.                                                                                                                                                                        |
| Out                 | Constructs marked unspecified (the "Special case" notes)                                                  | —                                                                                       | There is no expected output to check against.                                                                                                                                                                           |
| Out                 | Fuzzing                                                                                                   | —                                                                                       | A separate project.                                                                                                                                                                                                     |
| Out                 | `benches/`, `examples/`                                                                                   | —                                                                                       | Not tests; section 4 only runs `simple` as a check.                                                                                                                                                                     |

Kept tests stay as they are: they do not take the case format below.

### 1.3 Common test-case format

Each case records:

| **Field**           | **Required**                                     | **Meaning**                                                                                                                    |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| `id`                | Yes                                              | Unique descriptive name, retained from the original test where possible.                                                       |
| `html`              | Yes                                              | Literal input HTML, with significant whitespace made visible in the source.                                                    |
| `htmd_options`      | Yes                                              | Faithful mode plus all overrides of the pinned baseline defaults.                                                              |
| `markdown_options`  | When not empty                                   | Exact pulldown-cmark extension flags; empty unless explicitly overridden.                                                      |
| `expectation`       | Unless the case checks only Markdown (section 2) | `Preserved { renderer_suffix }` or `DocumentedDifference { kind, expected_html, reason, reference }`, as defined in section 3. |
| `expected_markdown` | When the rule specifies the Markdown text        | An exact assertion of `M(H)`, e.g. a row of a "Faithful expected" column in `translation_approach.md`.                         |
| `stability`         | Yes; `Stable` unless stated                      | `Stable`, or `DocumentedInstability { second_markdown, reason, reference }`, as defined in section 3.                          |

A case states at least one of `expectation` and `expected_markdown`.

Rules for test files and functions:

1. Keep cases in the existing feature test files, as `AGENTS.md` describes, and reuse `tests/common/mod.rs`.
2. No `test_` prefix and no nested modules. The name says the behavior, e.g. `extra_header_rows_are_written_as_html`.
3. The doc comment's first line links the `translation_approach.md` rule the test checks; any further lines say only what the rule does not.
4. Related cases go in one table-driven test. The test runs every case, collects the failures and reports them all at once, as `blocks_inside_inlines` in `translation_approach_tests.rs` does.
5. Changing the format must preserve the original input, option coverage, and purpose of each migrated test.

An illustrative case (not normative; section 4 fixes the real names). The loss is the one section 3's last example and the "At the document root" table describe. The case leaves `stability` at `Stable`: `M(R(H))` is `<br><br>` again.

```
/// [At the document root](../translation_approach.md#at-the-document-root).
#[test]
fn lone_breaks_at_the_document_root() {
    check_cases(&[Case {
        id: "lone_breaks_at_the_document_root",
        html: "<br><br>",
        expected_markdown: Some("<br><br>"),
        expectation: DocumentedDifference {
            kind: Loss,
            expected_html: "<p><br><br></p>\n",
            reason: "two or more root <br>s read back inside a paragraph",
            reference: "translation_approach.md#at-the-document-root",
        },
        ..Case::default()
    }]);
}
```

## 2. Migration of existing tests

Review each input and mode case for the behavior it checks, including cases grouped inside one test function. Classify it as preservation, a documented representation difference or loss, or a test of another contract. The [per-test inventory](https://github.com/Samboeng76/htmd/blob/spec/scope/testing_standardization_test_inventory.md) lists all 227 integration tests on the instructor's current `main` and records proposed Faithful treatment, Markdown assertions to retain, and existing round-trip coverage. For selected Faithful cases, `No` in the `Existing round trip` column identifies those still lacking a round trip. Keep the inventory in its own file.

Migrate every in-scope Faithful-mode case to the common runner, including cases whose only claim is exact Markdown output. Assign an HTML expectation only when a case claims preservation or an approved documented difference. A test function may also contain Pure-mode assertions; leave those assertions in place. Do not add a duplicate round trip for an input already checked, though an existing check may still need the case format and correctness criteria in sections 1 and 3.

Keep `multithreading` in `basic_tests.rs` unchanged as a kept test (1.2): it checks concurrent use of one converter.

| **Case purpose**                                           | **Selection**                                                                                                           |
| ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Preserve ordinary HTML structure                           | Add a `Preserved` round trip, using a compact input when formatting whitespace would change the claim.                  |
| Check Markdown syntax, escaping, or style options          | Keep exact Markdown and check stability; add an HTML expectation only for preservation or an approved difference.       |
| Check raw-HTML fallback or an intentional transformation   | Keep the Markdown assertion and select `Preserved` or `DocumentedDifference` according to the approved conversion rule. |
| Check Pure mode, handler events, or Turndown compatibility | Keep the existing assertion; these are kept tests (1.2), not Faithful round-trip candidates.                            |

Every migrated case also gets the stability check of section 3. An unstable case is investigated against the translation specification, then either fixed or given an approved `DocumentedInstability`.

Pilot compact Faithful paragraph and heading cases in `basic_tests.rs`, including `paragraphs`, `consecutive_blocks`, and `headings`. Preserve the original indented fixtures and Markdown checks when adding compact variants. Then review simple lists in `list_tests.rs`, paragraph-wrapped links in `link_tests.rs`, and raw-HTML fallback such as `faithful_mode_p` in `basic_tests.rs`. The main link input in `a_bang_before_a_link_is_escaped` already has a round trip; its other Faithful inputs remain partial candidates.

After the base cases, keep the existing `with_head` full-document test as a
Pure-mode test. Adding a new Faithful full-document case is outside this
migration. Then review tables and math. Use the case's exact htmd options and
matching pulldown-cmark extensions. For a selected intentional transformation,
record its approved rule and expected HTML rather than treating the current
output as the expectation.
For example, flow text `<p>a\nb</p>` becoming `<p>a b</p>\n` changes text, so
it requires an approved `DocumentedDifference` of kind `Loss`.

Record each selected case's destination, classification, mode, options, and reference HTML. An unexpected mismatch needs investigation against the translation specification; migration must not silently replace an expected result with current converter output.

Once no test calls a local helper which the runner replaces (`assert_round_trips`, `assert_math_round_trips`, `assert_paragraph_survives`, `assert_html_fallback`), remove it. `round_trip` and `render_markdown` remain as the runner's building blocks (section 4).

## 3. HTML comparison and correctness criteria

`H`, `M(H)` and `R(H)` are defined in Terms, under the case's fixed options.

For `Preserved`, require exactly `R(H) == H + renderer_suffix`, where the suffix
is `""` or one LF (`"\n"`). The nonempty suffix is allowed only after a completed
top-level paragraph, heading, list, blockquote, `pre`, table, or `hr`, possibly
followed by LF characters, and outside all other open constructs. It adds one
character; it never trims or replaces existing whitespace. If `H` already has
the renderer's final LF, select the empty suffix.

Compare UTF-8 strings exactly: no trimming, whitespace collapsing, entity
unescaping, attribute sorting, Unicode normalization, or element removal.
Spaces, tabs and line endings in text, code and attribute values remain part of
the assertion. A non-breaking space (`U+00A0`) is not an ordinary space.
This is source fidelity, not CSS rendering equivalence.

The suffix permits no internal separator change. Compact `<p>a</p><p>b</p>`
does not preserve source as `<p>a</p>\n<p>b</p>\n`: a text node was added between
the paragraphs. A separately named renderer-layout fixture
`<p>a</p>\n<p>b</p>` can use suffix `"\n"`; keep the original fixture and its
coverage. Do not strip indentation or internal newlines to make a test pass.

`DocumentedDifference` of kind `Representation` covers different source
spellings that preserve parsed node kinds, names/namespaces, child order,
attribute expanded names/values, text/comment values and other node data
under the same HTML parser and options, apart from the explicitly recorded final-LF convention.
Examples are attribute quoting/order, HTML void-element slash spelling,
equivalent character references, and HTML-parser normalization. Literal CRLF
or CR is normalized to LF [before HTML tokenization](https://html.spec.whatwg.org/multipage/parsing.html#preprocessing-the-input-stream);
that is different from a CR produced later by `&#13;`. The parser also ignores
one initial LF in [`pre` and `textarea`](https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody).
These rules justify specific expectations, not global replacement in the
comparison. References in raw-text elements and comments are not decoded like
ordinary HTML text. A changed parsed structure or value is kind `Loss`, even
when a browser would display the same spacing.

Both kinds require literal `expected_html`, an explanation of each difference,
and an approved conversion rule/limitation in `translation_approach.md`, or a
cited normative HTML parsing rule for a representation-only change. Specify
and review the expected result independently of actual converter output;
copying a failed result into a fixture does not approve it. Require exactly
`R(H) == expected_html`, with no extra suffix or normalization. Report both
kinds separately from preservation; passing a Loss regression is not a
preservation success.

For `Stable`, require exactly `M(R(H)) == M(H)`. An approved
`DocumentedInstability` instead requires exactly
`M(R(H)) == second_markdown`, with that literal, a reason and a reference
specified independently like a documented difference. Equal Markdown renders
to equal HTML, so Markdown stability implies `R(R(H)) == R(H)`; the converse
is false. Neither stability property proves preservation on the first trip.

Comparator examples (`\n`, `\r` and `\t` denote actual characters):

| Input `H` | Candidate result | Decision |
| --- | --- | --- |
| `<p>Hello</p>` | `<p>Hello</p>\n` | Preserved with suffix `"\n"`. |
| `<p>Hello</p>\n` | `<p>Hello</p>\n` | Preserved with the empty suffix. |
| `<p>Hello</p>\r\n` | `<p>Hello</p>\n` | Documented Representation under HTML input preprocessing. |
| `<p>a b</p>` | `<p>ab</p>\n` | Fails preservation: text space lost. |
| `<pre><code>a\nb\n</code></pre>` | `<pre><code>a b\n</code></pre>\n` | Fails preservation: an internal code LF changed. |
| `<p>&#65;</p>` | `<p>A</p>\n` | Requires documented Representation, including the final LF. |
| `<br><br>` | `<p><br><br></p>\n` | Documented Loss: extra paragraph; see "At the document root". |

Measured stability examples from the current default Faithful profile:

| Input `H` | `M(H)` | `M(R(H))` | Decision |
| --- | --- | --- | --- |
| `<br><br>` | `<br><br>` | `<br><br>` | Stable, despite documented HTML loss. |
| `<h1>a\t#</h1>` | `# a \#` | `# a \#` | Stable, despite loss of the original tab. |
| `<p><code>a\nb</code></p>` | `<code>a b</code>` | `` `a b` `` | Fails Stable even though its HTML is stable; investigate or justify an approved instability. |

## 4. Round-trip execution and project verification

The generalized runner converts each case's input `html` to `M(H)` using the case's htmd options, then renders `M(H)` to `R(H)` using the existing `render_markdown` helper with the case's exact pulldown-cmark extension flags. Apply htmd options only to the HTML-to-Markdown conversion and pulldown-cmark flags only to the Markdown-to-HTML rendering; use the same htmd options again when converting `R(H)`. The existing `round_trip` is a reference and remains useful for its default-profile checks, but the generalized runner must retain the intermediate Markdown and handle per-case options and errors rather than relying solely on that helper's final HTML result. pulldown-cmark writes raw HTML through unchanged; the runner must not filter it. Enable extensions such as tables or math only for cases that need them.

For every in-scope case, the runner checks `expected_markdown` exactly when provided, independently of whether the case has an HTML expectation. Check `R(H)` against the case's `Preserved` or `DocumentedDifference` expectation when one is present; a Markdown-only case does not require `expected_html` or an HTML preservation assertion. For stability, compare `M(R(H))` with `M(H)` for a `Stable` case, or with the approved literal `second_markdown` for a `DocumentedInstability` case (section 3).

Conversion errors fail the named case. For every failed assertion, report the case ID, applicable htmd and pulldown-cmark options, input HTML and intermediate Markdown. For an HTML expectation mismatch, show expected and actual HTML; for an exact Markdown mismatch, show expected and actual Markdown; for a stability mismatch, show `M(H)`, `R(H)`, `M(R(H))` and the applicable expected Markdown. Make whitespace visible in diffs (for example, by escaping tabs and line feeds). Report preservation, representation differences, and known losses separately; stability is an additional result. Do not report a missing HTML expectation as a failed HTML comparison.

Migrate in reviewable feature batches. A batch is complete when its inventory is accounted for, previous coverage is retained, and these commands pass:

```
cargo build
cargo clippy -- --deny warnings
cargo test
cargo run --example simple
```

The current helper and existing tests are implementation references, not a claim that this proposed format has already been implemented.
