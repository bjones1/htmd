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

This section defines how a test compares whitespace in two HTML strings: the expected round trip and the HTML which pulldown-cmark renders (property 2). Markdown is always compared exactly, whitespace included (properties 1 and 3).

Whitespace here is [ASCII whitespace](https://developer.mozilla.org/en-US/docs/Glossary/Whitespace): U+0020 space, tab, line feed, form feed and carriage return ([Infra](https://infra.spec.whatwg.org/#ascii-whitespace)). Other spaces, such as U+00A0 and U+2003, are text and are compared exactly.

Two HTML strings are equivalent when a browser with its default style sheet renders them the same. [How whitespace is handled by HTML, CSS, and in the DOM](https://developer.mozilla.org/en-US/docs/Web/API/Document_Object_Model/Whitespace) describes how a browser does this, and the rules below follow it.

#### Comparison

The comparison helper parses both strings with html5ever, as htmd does, and compares the two DOMs after the normalization below. Parsing already makes these equivalent: `\r\n`, `\r` and `\n` (the parser normalizes line endings), a character reference and the character it stands for (`&#10;` and a line feed), and the spelling of tags and attributes.

In text rendered with `white-space: normal`, which is everything outside the elements listed under "Where whitespace matters":

1. A run of whitespace is equivalent to one space. This holds across the boundaries of inline elements: `a <em> b</em>` is equivalent to `a <em>b</em>`, since the browser collapses the second space.
2. Whitespace at the start or end of a line is removed. A line starts and ends at the edges of a block element (`<p>`, `<h1>`–`<h6>`, `<li>`, `<blockquote>`, `<td>`, `<th>` and the other elements which the default style sheet displays as blocks) and at a `<br>`. So `<p> a </p>` is equivalent to `<p>a</p>`, and `a <br> b` to `a<br>b`. This also applies inside an inline element at the edge of a line: `<p><em> a</em></p>` is equivalent to `<p><em>a</em></p>`.
3. A text node holding only whitespace between two block elements, or between a block element and the edge of its parent, is removed. So `<p>a</p>\n<p>b</p>\n` is equivalent to `<p>a</p><p>b</p>`.

Everything else is compared exactly: element names, attributes and their values, the order of nodes, and all text other than the whitespace above.

#### Where whitespace matters

1. **Preformatted elements.** The default style sheet renders `<pre>`, `<listing>`, `<xmp>` and `<plaintext>` with `white-space: pre`, and `<textarea>` with `white-space: pre-wrap` ([HTML rendering](https://html.spec.whatwg.org/multipage/rendering.html#flow-content-3)). Their text, and the text of any element inside them, is compared exactly. The one exception comes from parsing: the parser drops a single line feed directly after the start tag of `<pre>`, `<listing>` or `<textarea>`, so `<pre>\na</pre>` is equivalent to `<pre>a</pre>`, but `<pre>\n\na</pre>` is not.
2. **Whitespace between inline content.** Whether whitespace is present between two pieces of inline content matters, even though its amount does not: `<em>a</em> <em>b</em>` is not `<em>a</em><em>b</em>`, and `a<code>b</code>` is not `a <code>b</code>`.
3. **Which side of an element boundary the whitespace is on.** Rule 1 collapses a run which spans a boundary, but a single space keeps its place: `a <em>b</em>` is not `a<em> b</em>`. The space renders the same in most fonts, but an underline, a background or a link target shows the difference. A trip which moves whitespace into or out of an element is a difference, recorded as described in "Known failures".
4. **Attribute values, comments, `<script>` and `<style>`.** These are compared exactly.

`<code>` is not preformatted: its default style is `white-space: normal`, so the rules under "Comparison" apply to it.

The comparison assumes the default style sheet. Tests do not use a `style` attribute or a `<style>` element which changes `white-space`.

#### Rules for tests

1. Write expected Markdown with the exact whitespace htmd should write. Never trim or normalize it to make a test pass.
2. Write the expected round trip in whichever equivalent form reads best. When the trip loses nothing, this is the input HTML itself, so the test shows that the HTML comes back as it went in.
3. The comparison helper has its own tests. Each rule under "Comparison" gets a pair of strings which must compare equal, and each item under "Where whitespace matters" gets a pair which must not, using the examples above.
4. Write invisible characters as escapes (`\t`, `\r\n`, `\u{a0}`), so a reader sees them.
5. A difference which remains after the normalization is a failure, recorded as described in "Known failures".

### Escaping

htmd writes a backslash where the text of an HTML text node would otherwise read as Markdown syntax. An escape test checks that the text comes back as the same text in the same block. This section applies to text, and to the Markdown htmd writes around it, in `TranslationMode::Faithful`. Text which htmd writes as an HTML block (for example `<div>`, raw `<pre>`, a table without a header row, or a `<p>` holding a comment) is not escaped, since a backslash would be literal there. Escapes in link destinations, link titles and image descriptions, and line endings written as character references, follow the sections of `translation_approach.md` for those constructs.

#### What htmd escapes

1. **Inline specials.** `\`, `&`, `*`, `_`, `` ` ``, `[`, `]` and `$` are escaped wherever they appear in a text node, including text inside a raw HTML inline (`<p>a <u>*b*</u></p>` gives `a <u>\*b\*</u>`). `$` has a meaning only to pulldown-cmark's math extension. Through this rule `*` and `_` are also kept from opening a list or a thematic break (`\* a`, `\_\_\_`).
2. **Block markers.** At the start of a text node, `=`, `~` and `>` are always escaped. `-`, `+`, `#` and an ordered item's `.` or `)` are escaped when a space or the end of the text node closes them (`\# a`, `\-`, `1\. a`). A text node of only `-` and spaces is always escaped (`\--`, `\---`, `\- - -`), but `--foo` is not. Any HTML whitespace after a marker (see the Whitespace section) is first compressed to a space, so `#\nfoo` is escaped like `# foo`.
3. **HTML-like text.** A `<` which could open raw HTML or an autolink, or a prefix of one which the end of the text node cuts short, is escaped, whether or not a `>` follows (`<p>&lt;pre</p>` gives `\<pre`). `should_escape_html_like_sequence` in `src/util/escape.rs` and its unit tests define the exact test.
4. **Pipe-table cells.** Every `|` in the Markdown of a cell of a table with a header row is written as `\|`, even inside a code span or a raw HTML inline (`` `a\|b` ``), since GFM splits the cell at any unescaped `|`.
5. **Context.** A `#` run which ends the text of an ATX heading is escaped (`<h1>a #</h1>` gives `# a \#`), and so is a `!` which a link follows (`\![a](x)`).
6. **Code.** Apart from item 4, text in `<code>` and `<pre><code>` is not escaped. Instead, a code span uses a backtick run whose length occurs nowhere in its content (`<code>a`b</code>` gives ``` ``a`b`` ```), padded with a space at each edge when the content starts or ends with a backtick (`<code>`a</code>` gives ``` `` `a `` ```). A code fence uses backticks, at least three and more than any backtick run in the content. A backtick in the language (`class="language-a`b"`) breaks the fence's info string: this under-escape is a known failure.

#### Over- and under-escaping

* An **under-escape** is a missing escape which lets text open any Markdown construct: a heading, a list, emphasis, a link, an entity reference, a math span, raw HTML and so on. It changes the round-trip HTML, and it is a bug.
* An **over-escape** is an escape which the rules above produce but which CommonMark does not need at that position: removing the backslash leaves the round trip unchanged, since a backslash reads back as the character it escapes. It is not a failure: the test asserts the escaped Markdown and gives a one-line reason, as "Known failures" describes.

htmd decides on block markers one text node at a time, treating the start of a text node as the start of a line and its end as the end of a line. Leading whitespace is dropped only at the start of a block, so mid-line a text node which starts with a space escapes nothing (`<p>a <em>b</em> - c</p>` gives `a *b* - c`). That gives the over-escapes which need a reason in their tests:

* a marker which is not at the start of its line, because an inline sibling, or the opening markup of an ATX heading or of an inline construct (a link, an emphasis, a raw HTML inline), comes before it (after a list marker or a `>`, a new block can start, so the escape there is needed): `<p>a <em>b</em>- c</p>` gives `a *b*\- c`, and `<h1># a</h1>` gives `# \# a`;
* a marker which an inline sibling follows, so no space closes it: `<p>#<em>b</em></p>` gives `\#*b*`;
* a `#` run longer than six, or an ordered marker of ten or more digits (`\#######`, `1234567890\.`);
* a `-`, `+`, `#` or ordered marker in a pipe-table cell, where no block can start;
* a `<` which ends a text node when the sibling's Markdown does not continue raw HTML or an autolink: `<p>&lt;<em>1</em></p>` gives `\<*1*`.

The always-escaped characters of items 1 and 2 (such as `a \& b`, `\=foo`, `\--`, or a `\>` in a cell) are expected even where the position does not need them, and their tests need no reason.

#### Rules for tests

1. Take each expected Markdown and round trip from the rules of this section, never from htmd's current output: a test which copies the output asserts whatever htmd does, including a missing escape. Each escape case calls `assert_faithful`, which checks the round trip first, so a broken escape reports the block it opened rather than a different string. The expected round trip is the same text in the same block, after the equivalences of the Whitespace section, spelled as pulldown-cmark writes it: `<p>#</p>` gives `<p>#</p>\n`, `<p>-\tfoo</p>` gives `<p>- foo</p>\n`, and `<p>a & b</p>` gives `<p>a &amp; b</p>\n`.
2. Pair each escaped case with a near miss which needs no escape: `#x` beside `#`, `--foo` beside `---`, `1.)` beside `1.`, `2½.` beside `2.`, `-&nbsp;a` beside `- a`, and `</ a` or `<123>` beside `<a`. Pair each over-escape with the case where the escape is needed: `a <em>b</em>- c` with `- c` (escape needed) and with `a <em>b</em> - c` (no escape), `#<em>b</em>` with `# <em>b</em>`, and `<h2>- a</h2>` in an ATX heading with the same heading in `HeadingStyle::Setex`. For item 5, pair `<h2>a #</h2>` with `<h2>a#</h2>`, and `!<a href="x">a</a>` with `!<em>a</em>`.
3. For `-`, `+`, `#` and an ordered item's delimiter, cover each way to close the marker: each kind of HTML whitespace (space, tab, line ending, form feed) and the end of the text. Also cover the marker behind leading whitespace (`<p> 1. foo</p>`). `=`, `~` and `>` need one case each, because what follows them does not change the escape. A `-` run needs `--`, `---` and `- - -` beside the near miss `--foo`.
4. Cover each place where a text node can start a line: a paragraph, a list item, a blockquote, a setext heading (`HeadingStyle::Setex`), and a text node after a `<p>`, `<blockquote>`, list or `<pre>` sibling (`<li><p>a</p> - b</li>`, `<li><ul><li>a</li></ul>- b</li>`). A failure there whose cause is block separation, not a missing escape, is a list bug and goes under "Known failures". Also cover a raw HTML inline (item 1) and a pipe-table cell, including a code span in a cell (item 4).
5. Read the round trip with plain CommonMark, plus the pulldown-cmark option which gives a character its meaning, or the trip cannot catch a missing escape: `ENABLE_MATH` for `$`, and `ENABLE_TABLES` for every pipe-table case. These tests call `assert_faithful_options`. Other extensions, such as strikethrough, are out of scope: htmd does not escape `~~` after the start of a text node, so a strikethrough found in such a trip is not an under-escape.
6. An escape case which fails idempotency is a bug: a backslash reads back as the character it escapes, in the same context, so the second conversion meets the same text and should write the same escape. The text changes only where the trip moves whitespace into it (Whitespace item 6): `<p>a &lt;<em> b</em></p>` gives `a \< *b*`, then `a < *b*`. Record such a case as a known failure.
