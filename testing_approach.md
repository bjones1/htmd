Testing approach
================

Contributions
-------------

* Miras Shaltayev: wrote the initial draft of this document.
* Armanzhan Onggarbay: 
* Nurlybek Orynbassar: wrote the "Whitespace" section.
* Aidyn Tazhibay: wrote the "Escaping" section.

Every test of a translation checks two things: the Markdown which htmd writes, and that this Markdown renders as the HTML it came from. The second check is a round trip, and `round_trip` in `tests/common/mod.rs` performs it. Today most tests make only the first check, so a test can pass while the Markdown means something else to pulldown-cmark.

### Properties

For an HTML input `h` and a translation mode:

1. **Expected Markdown**: `convert(h)` equals the Markdown in the test.
2. **Round trip**: in `TranslationMode::Faithful`, `render_markdown(convert(h), options)` is the HTML which `h` is expected to produce. Where the two strings differ but a browser parses them to the same DOM (for example `<p>a</p>` comes back as `<p>a</p>\n`, since pulldown-cmark ends a block with a line ending), the test states the expected string, not a normalized one.
3. **Idempotency**: let `m = convert(h)` and `h2 = render_markdown(m)`. Then `convert(h2)` equals `m`.

This approach covers `TranslationMode::Faithful` only. `TranslationMode::Pure` is out of scope.

### Standard test helper

One helper in `tests/common/mod.rs` checks all three properties. A test calls it instead of repeating `assert_eq!` pairs. It checks the round trip first, so an escape which stops working reports the harm it does, such as a paragraph dissolved into a heading, rather than a mismatched Markdown string.

`assert_faithful(html: &str, expected_markdown: &str, expected_round_trip: &str)` converts in Faithful mode with the default htmd options, and reads the Markdown back with `CommonMarkOptions::empty()`, as `round_trip` does. It panics with a diff on a mismatch, as `assert_eq!` does.

Tests also need other options, of two kinds. `assert_faithful_options(html, htmd_options, commonmark_options, expected_markdown, expected_round_trip)` takes both: the htmd options (for example `HeadingStyle::Setex`, which `translation_approach_tests.rs` uses) and the pulldown-cmark options (for example `ENABLE_MATH | ENABLE_TABLES`). `assert_faithful` calls it with the defaults.

The expected round-trip HTML is always given, never omitted: pulldown-cmark wraps inline content in `<p>` and ends each block with a line ending, so the output seldom equals the input, and an explicit string shows what the trip produces.

### Known failures

A case whose trip is lossy by design (property 2) still uses `assert_faithful`, with the lossy string as the expected round trip. The test adds a one-line reason and names the section of `translation_approach.md` which explains the loss. A case which fails idempotency (property 3) is a bug to fix, or has its own test which asserts the unstable second Markdown, with the same reason. A test never hides a failure by skipping a property without that reason.

### Migration order

1. Add the helpers and convert `basic_tests.rs` and `link_tests.rs`, which already use `round_trip` in part. The existing helpers in `basic_tests.rs` become thin callers of the new ones, or are deleted where `assert_faithful` replaces them: `assert_round_trips`, `assert_math_round_trips` (needs `assert_faithful_options`) and `assert_paragraph_survives`. `round_trip_pure_table` tests Pure mode, which is out of scope, so it stays as it is.
2. Convert `table_tests.rs`, `list_tests.rs`, `code_tests.rs`, `context_tests.rs` and `event_tests.rs`, which never call it.
3. Convert `translation_approach_tests.rs`: each table row gets the three properties.
4. Record each failure found as a known failure, or as a bug to fix.



### Whitespace

A test states whether whitespace is significant. This section applies to `TranslationMode::Faithful`.

HTML whitespace is U+0020, tab, line feed, form feed and carriage return. Other spaces, such as U+00A0, are text to the HTML parser, which never collapses them. htmd trims the edges of emphasis more widely (item 7 below).

#### Where whitespace matters

1. **Preformatted text.** Raw `<pre>` is written as an HTML block and keeps its spaces, tabs and blank lines. The HTML parser drops one line feed directly after `<pre>` and htmd does not write one back, so `<pre>\n\na</pre>` loses a line ending (a known failure). `<pre><code>` is written as a fenced code block, which keeps spaces and tabs but ends its content with a line ending, so `<pre><code>a</code></pre>` comes back as `<pre><code>a\n</code></pre>\n` (lossy). A `<pre>` in an inline context, such as a table cell, is written as a raw inline and its line endings become spaces (see `translation_approach.md`).
2. **Code spans.** `<code>` is not preformatted. Runs of spaces and tabs are kept, a line ending collapses to a space (htmd then writes a raw `<code>` inline, and converting that again gives a backtick span, so the case is not idempotent), and edge spaces are dropped (lossy).
3. **Indentation.** Four leading spaces start an indented code block, and indentation inside list items decides nesting. Tests cover a list item with several blocks, a nested list, and a code block inside a list item, each with the exact indentation. A leading run of U+00A0 is not indentation.
4. **Line breaks.** In Faithful mode `<br>` is written as the raw tag `<br>`, and the round trip returns `<br>`. Spaces around it survive (`a <br> b`). htmd writes no CommonMark hard break, so a line ending in its output is never a hard break (see `translation_approach.md`).
5. **Line endings in text.** Inside a block, a line ending in HTML text becomes one space. htmd writes neither a soft break nor a blank line there. A blank line appears only between blocks.
6. **Delimiters.** Emphasis and strong emphasis cannot touch whitespace inside their delimiters, so htmd moves edge whitespace outside them, and the round-trip DOM differs from the input (lossy). Links can hold edge whitespace, and htmd keeps it inside the brackets. An emphasis holding only whitespace stays raw HTML.
7. **Unicode spaces.** U+00A0 is kept as text. CommonMark counts Unicode whitespace for flanking, and htmd trims with Rust's `White_Space`, so an edge U+00A0 or U+2003 moves out of emphasis like an ASCII space (lossy). In a link it stays inside the brackets.
8. **Table cells.** A cell cannot hold a line ending. Tests use `assert_faithful_options` with `ENABLE_TABLES`, because the default options do not parse tables. A line ending in a cell becomes a space, `<br>` stays raw, leading and trailing cell whitespace is dropped, and several blocks in a cell are written as raw HTML.

#### Where whitespace does not matter

In text content, outside `<pre>`, `<textarea>`, `<code>`, attribute values and content written as an HTML block:

1. A run of HTML whitespace, form feed included, is equivalent to one space.
2. Whitespace between block elements is ignored.
3. Leading and trailing whitespace of a block (`<p>`, `<h1>`, `<li>`, `<td>`) is dropped.
4. `\r\n`, `\r` and `\n` are the same, inside `<pre>` too.

Whitespace between inline elements does matter (`<em>a</em> <em>b</em>` is not `<em>a</em><em>b</em>`), so a pair of inputs under these rules never varies it.

#### Rules for tests

1. Write expected Markdown and expected round-trip HTML with the exact whitespace the code produces. Never trim or normalize an expected string to make a test pass.
2. For each item under "does not matter", add two inputs which differ only in that whitespace. Both call `assert_faithful` with the same expected Markdown and the same expected round-trip HTML.
3. Write invisible characters as escapes (`\t`, `\r\n`, `\u{a0}`), so a reader sees them.
4. A whitespace case which fails a property is recorded as described in "Known failures".

### Escaping

htmd writes a backslash, or a character reference, where text from the HTML would otherwise read as Markdown syntax. An escape test checks that the text comes back as the same text in the same block. This section applies to `TranslationMode::Faithful`.

#### What htmd escapes

1. **Inline specials.** `\`, `&`, `*`, `_`, `` ` ``, `[`, `]` and `$` are escaped wherever they appear in a text node. `$` has a meaning only to pulldown-cmark's math extension.
2. **Block markers.** At the start of a text node, `=`, `~` and `>` are always escaped. `-`, `+`, `#` and an ordered item's `.` or `)` are escaped when a space or the end of the text node closes them (`\# a`, `\-`, `1\. a`). A `-` run is escaped when it could be a thematic break or a setext underline (`\---`, `\- - -`), but `--foo` is not. A line ending or a tab after a marker is first compressed to a space, so `#\nfoo` is escaped like `# foo`.
3. **HTML-like text.** A `<` which could open raw HTML or an autolink is escaped (`\<div`), including one which an inline sibling completes (`&lt;<span>div</span>`).
4. **Table cells.** A `|` in a cell is written as `\|`.
5. **Code.** Text in `<code>` and `<pre><code>` is not escaped. Only a fence-like `` ` `` or `~` at the start of `<pre>` text is.

#### Over- and under-escaping

htmd decides on block markers one text node at a time, and a text node cannot see what follows it on the line. So htmd escapes some text which opens nothing: `#######` (seven `#` open no heading) and the `#` in `<p>a <em>b</em>#</p>`. A backslash reads back as the character it escapes, so the round trip still holds.

* An **under-escape** is a missing escape which lets text open a heading, list, blockquote, emphasis, link, math span or raw HTML. It changes the round-trip HTML, and it is a bug.
* An **over-escape** changes only the Markdown. It is not a failure: the test asserts the escaped Markdown and gives a one-line reason, as "Known failures" describes.

#### Rules for tests

1. Each escape case calls `assert_faithful`, which checks the round trip first, so a broken escape reports the block it opened rather than a different string. The expected round trip is the text unchanged in its block, for example `<p>#</p>\n` for `<p>#</p>`.
2. Pair each escaped case with a near miss which needs no escape: `#x` beside `#`, `--foo` beside `---`, `1.)` beside `1.`, `2½.` beside `2.`, and `a <em>b</em> - c` beside `- c`.
3. For each block marker, cover each way to close it: a space, a tab, a line ending and the end of the text. Also cover the marker behind leading whitespace (`<p> 1. foo</p>`).
4. Cover each place where a text node can start a line: a paragraph, a list item, a blockquote and a setext heading (`HeadingStyle::Setex`). Also cover a table cell, where a block marker opens nothing, so its escape is an over-escape.
5. Read the round trip with the pulldown-cmark option which gives a character its meaning, or the trip cannot catch a missing escape: `ENABLE_MATH` for `$`, and `ENABLE_TABLES` for `|`. These tests call `assert_faithful_options`.
6. An escape case which fails idempotency is a bug, since an escaped character comes back as plain text, which htmd escapes again in the same way.
