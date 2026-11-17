Benchmark corpus
================

The compute-budget corpus of `demo-spec.md` §6 and §8.3: one full conversion of
each document below, including canonicalization and diff generation, completes
in under 100 ms.

| File               | Mode            | Bytes  | Contents |
| ------------------ | --------------- | ------ | -------- |
| `basics-scaled.md` | Markdown → HTML | 19,859 | The FR-6 CommonMark basics Markdown sample (`demo/src/samples.rs`), repeated 60 times, separated by blank lines. |
| `math-scaled.md`   | Markdown → HTML | 20,019 | The FR-6 Math Markdown sample, repeated 182 times, separated by blank lines. |
| `inline-html.html` | HTML → Markdown | 20,060 | The FR-6 Inline HTML sample, repeated 85 times. |
| `real-page.html`   | HTML → Markdown | 70,545 | A complete documentation page: doctype, head with style, header nav and search form, sidebar contents, and 16 sections of headings, paragraphs, nested lists, tables, code blocks, block quotes, and definition lists. Maximum element depth is 9 by FR-7's measure (children of `<body>` at depth 1), well under the 512 limit. |

Sizes are exact UTF-8 byte counts (`wc -c`) of the files with LF line endings.
The repository's `.gitattributes` marks `demo/bench/corpus/*` as `-text`, so
Git never converts their line endings and the sizes hold in every checkout,
including Windows with `core.autocrlf=true`.

`real-page.html` is a synthetic page, not one saved from the web. Its markup was
written to resemble a typical documentation site.
