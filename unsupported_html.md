CommonMark translation of HTML blocks and raw HTML inlines
==========================================================

This design requires `TranslationMode::Faithful`, which outputs all `<br>`
elements not as a CommonMark break, but as the HTML `<br>`. This choice is
driven by [CommonMark](https://spec.commonmark.org/0.31.2/) rules:

1. A [hard line break](https://spec.commonmark.org/0.31.2/#hard-line-breaks)
   must be followed by content **in the same block**. Neither `␣␣⏎` nor `\⏎` can
   encode a `<br>` that ends a block: a line holding only two spaces is
   [blank](https://spec.commonmark.org/0.31.2/#blank-lines), and a trailing `\`
   is a literal backslash. This is what `TranslationMode::Faithful` sidesteps by
   writing the tag where the break is rather than at a line end.

2. Some HTML sequences cannot be represented in CommonMark; only
   `TranslationMode::Faithful` produces the
   [HTML blocks](https://spec.commonmark.org/0.31.2/#html-blocks) and
   [raw HTML inlines](https://spec.commonmark.org/0.31.2/#raw-html) necessary in
   these cases.

Beyond CommonMark and GFM, the output relies on two pulldown-cmark extensions:
math expressions (see the math section) and, in Faithful mode only, heading
attributes (see the heading attributes section).

The Faithful expected column is what a correct design should produce. Notation
inside a cell: `⏎` is a newline; `␣` is a space; `<br><br>...<br>` stands for
one or more `<br>` elements. Where one `<br>` and a run of them need different
Markdown or produce different translations, they get a row each.

Analysis
--------

The CommonMark rules for HTML blocks and raw HTML inlines drive nearly every
table row below. While this file focuses on `<br>` elements, the same principles
apply to any HTML block (in some cases, over-conservatively) or raw HTML inline.
An implementation ensuring correct `<br>` operation should not employ special
cases for a `<br>` element, but rather generalize to any HTML block or raw HTML
inline.

`br` is not a block-level tag name, so the only HTML block a raw `<br>` can open
is [type 7](https://spec.commonmark.org/0.31.2/#html-blocks): a line that begins
with a complete open or closing tag and holds nothing but whitespace after it
(see the spec for additional conditions). Therefore, `<br><br>` is an ordinary
paragraph where `<br>` alone is not.

Type 7 is also the one HTML block that cannot
[interrupt a paragraph](https://spec.commonmark.org/0.31.2/#paragraphs), and the
block it does open runs to the next blank line, swallowing every following line
in the same container. A lone `<br>` therefore needs a blank line on *both*
sides: without the one before, it joins the paragraph above as an ordinary
inline tag; without the one after, it eats the block below.

For simplicity, a serialized HTML block is given a blank line before and after,
though this is only required for some HTML block types. The container supplies
two exceptions: the marker line of a list item stands in for the blank line
before, and a following sibling item closes the block without one. Inside a
[blockquote](https://spec.commonmark.org/0.31.2/#block-quotes) the separator is
a blank `>` line, since a truly blank line would end the blockquote instead.

Translating HTML nodes
----------------------

CommonMark blocks may only process their content into CommonMark in a block
context; in an inline context, they should emit raw HTML inlines. For example:

| Description                     | HTML in             | Faithful expected |
| ------------------------------- | ------------------- | ----------------- |
| Heading with embedded paragraph | `<h1><p>a</p></h1>` | `#␣<p>a</p>`      |

The following rules determine the context of an HTML node's contents based on
the current context. This context passed down by the DOM walker then enables an
HTML node to examine the current context to be correctly translated.

* The initial (root) context is a block context.
* Container blocks, which are a blockquote (`<blockquote>`) and a list/list item
  (`<ol>`/`<ul>` then `<li>`), can only be translated as CommonMark in a block
  context; they begin a (new) block context.
* Thematic breaks (`<hr>`) must occur in a block context to be translated to
  CommonMark. They have no content.
* Leaf blocks, which are headings (`<h1>`-`<h6>`), paragraphs (`<p>`), and
  tables (`<table>`, `<thead>`, `<tbody>`, `<tr>`), can only be translated as
  CommonMark/GFM when they appear in a block context. Table cells
  (`<td>`/`<th>`) begin an inline context.
* Inlines, consisting of emphasis (`<em>`) and strong emphasis (`<strong>`),
  links (`<a>`), autolinks (`<a>`), and raw HTML, begin an inline context.
* Inlines with no content (`<img>`, `<br>`) aren't affected by context.
* Both code blocks and code spans create a literal context; this isn't
  inheritable and is handled by the `<pre>` and `<code>` translators.
* Given an HTML node which cannot be translated to CommonMark:
  * HTML type 1-6 in a block context is emitted as an HTML block; the
    inline/block classification does not apply to these contents. In contrast,
    HTML type 1-6 in an inline context is emitted as raw HTML inlines; the
    resulting context is inline.
  * HTML type 7 is always emitted as raw HTML inlines; the resulting context is
    inline. This choice is a tradeoff: it provides more natural behavior inside
    tight lists and when encountering text that's a direct child of `<body>`;
    for example, it translates `This is <em>really</em> important.` as `This is
    *really*  important.` instead of (as HTML blocks)
    `This  is⏎⏎<em>⏎⏎really⏎⏎</em>⏎⏎important.` This approach, however, fails to
    preserve a possible faithful encoding of lone inline tags: `<br><br>` stays
    `<br><br>` in CommonMark, which re-translates to `<p><br><br></p>`.
    Translating to `<br>⏎⏎<br>` would preserve the original HTML.

Newline encoding works exactly when the parser reading that position decodes
character references there: the CommonMark parser in the text between raw HTML
inlines, the HTML parser in the data and attribute values of an HTML block. It
works nowhere else — inside a comment, a CDATA section, a declaration, or a raw
text element the reference stays as the literal characters composing it. The
rest of this section follows from this.

Type 6 HTML blocks cannot contain blank lines; these must be encoded by
replacing CR/LF characters in blank lines with `&#13;`/`&#10;`, where blank
lines are defined by
[section 2.1 of the CommonMark spec](https://spec.commonmark.org/0.31.2/#characters-and-lines).
All raw HTML inlines will have whitespace collapsed in their contents (including
newlines, which would otherwise be problematic), a standard part of current htmd
processing.

Type 1 content placed inline is interpreted correctly for `<pre>` and
`<textarea>`: always-encoded characters `"`, `&`, `<`, and `>` are interpreted
by the HTML parser; however, newlines are treated as whitespace and emitted as
spaces. For `<script>` and `<style>`, the always-encoded characters are not
interpreted by the HTML parser inside these tags. This requires a tradeoff: this
type 1 content is walked like any other raw HTML inline, so the always-encoded
characters in `<script>`/`<style>` tags are escaped and come back as character
references, causing them to be mis-translated and newlines in
`<pre>`/`<textarea>` are lost; the alternative of emitting this content as an
HTML block would mis-translate this content placed in CommonMark blocks that
cannot contain a newline (headings and tables). Walking ensures that CommonMark
in the content is escaped, so `<h1>a<script>b*c*d</script>e</h1>` keeps its
literal `*c*`.

Type 2-5 content placed inline faces other difficulties. Newlines cannot be
encoded, since the CommonMark parser doesn't interpret character references
inside this type of raw HTML inline. For simplicity, whitespace including
newlines is collapsed, a lossy translation.

A more complex, higher-fidelity result for type 1-5 content placed inline comes
from translating the containing CommonMark block to HTML while also translating
the type 1-5 content as an HTML block, then appending the remaining HTML
contents to it without an intervening newline. The result is slightly lossy:
`<h1>a<script>"&<⏎⏎>"</script></h1>` becomes
`<h1>a⏎⏎<script>"&<⏎⏎>"</script></h1>`, and `<h1>a<!--b⏎⏎c-->d</h1>` becomes
`<h1>a⏎⏎<!--b⏎⏎c-->d</h1>`. For simplicity, this is not implemented.

**Special case**: If a type 1 `<script>`, `<style>` or a type 2-5 block
containing blank lines is nested inside a type 6 block, the naive translated
result is incorrect. The correct result would be placing a blank line before the
next nested type 1-5 block to start another block. Likewise, nesting a type 1-5
block inside another type 1 block fails when the inner block contains any type 1
termination condition followed by newlines: `<pre><style></style>⏎a</pre>`. Due
to this complexity, the current translation is unspecified for simplicity.

**Special case**: Content in `<iframe>`, `<xmp>`, `<noscript>` (scripting
enabled), `<noembed>`, `<noframes>`, and `<plaintext>` tags comes back from the
html5ever tokenizer as literal characters, meaning encoding cannot be used to
handle newlines. The translation for these unusual cases is unspecified for
simplicity.

**Special case**: A `<!DOCTYPE html>` declaration (which html5ever attaches to
the document node) is dropped when translating.

These rules bind every element handler, the custom handlers registered through
`add_handler` included. In particular, no handler may write a bare newline in an
inline context: every one of them is encoded, replaced or removed by the rules
above, so an inline context holds a single line. A handler which writes one
anyway produces a translation this design does not describe, and the result is
unspecified.

Special case for paragraphs
---------------------------

Classification alone is not enough: a paragraph containing only a lone type 7
raw HTML inline or beginning with a type 1-6 raw HTML inline dissolves the
paragraph containing it. Of the CommonMark leaf blocks above, only a paragraph
and a setext heading are exposed. An ATX heading's `#` is leaf-block syntax the
block scan matches before any HTML block start condition; a table cell's
contents are parsed as inline content, so no block scan runs inside one at all;
and a blockquote's `>` and a list item's marker are stripped before that scan
and so protect nothing. Therefore, where a paragraph's content would be replaced
by an HTML block, serialize the entire paragraph instead of its content; a
setext heading falls back to ATX instead (see the headings section). See row 1
of the paragraphs section and row 2 of the blockquotes section in addition to
the following table.

| Description                          | HTML in                             | Faithful expected                   |
| ------------------------------------ | ----------------------------------- | ----------------------------------- |
| Paragraph starting with a `<iframe>` | `<p><iframe src="u">a</iframe></p>` | `<p><iframe src="u">a</iframe></p>` |

Code
----

| Description                  | HTML in                                     | Faithful expected                           |
| ---------------------------- | ------------------------------------------- | ------------------------------------------- |
| `<br>`s in a code span       | `<p>a<code>x<br><br>...<br>y</code>b</p>`   | `a<code>x<br><br>...<br>y</code>b`          |
| `<br>`s alone in a code span | `<p>a<code><br><br>...<br></code>b</p>`     | `a<code><br><br>...<br></code>b`            |
| `<br>`s in a code block      | `<pre><code>a<br><br>...<br>b</code></pre>` | `<pre><code>a<br><br>...<br>b</code></pre>` |
| `<br>`s-only code block      | `<pre><code><br><br>...<br></code></pre>`   | `<pre><code><br><br>...<br></code></pre>`   |

**Special case:** The content of a
[code span](https://spec.commonmark.org/0.31.2/#code-spans) or a
[fenced code block](https://spec.commonmark.org/0.31.2/#fenced-code-blocks) is
literal text: a `<br>` written inside one is four characters, not a break. No
encoding of a break survives there, which is why all rows of the table use HTML
blocks or raw HTML inlines instead of inline code spans or code blocks. A line
ending in the text of a code span is the same case, and so is an empty one: the
backticks meant to open an empty span close it instead, so CommonMark can spell
no such span. Both go out as raw HTML inlines.

Inline elements
---------------

The principles derived in this section apply to all the following sections,
since they are containers for inline elements. Notation: *text*, represented
below as the characters `a`-`b`, stands for a character that is neither Unicode
whitespace nor Unicode punctuation.

| Description                                     | HTML in                                 | Faithful expected                |
| ----------------------------------------------- | --------------------------------------- | -------------------------------- |
| `<br>`s starting emphasis                       | `<p><em><br><br>...<br>a</em></p>`      | `<em><br><br>...<br>a</em>`      |
| `<br>`s ending emphasis                         | `<p><em>a<br><br>...<br></em></p>`      | `<em>a<br><br>...<br></em>`      |
| `<br>`s inside emphasis                         | `<p><em>a<br><br>...<br>b</em></p>`     | `*a<br><br>...<br>b*`            |
| `<br>`s alone in emphasis with surrounding text | `<p>a<em><br><br>...<br></em>b</p>`     | `a<em><br><br>...<br></em>b`     |
| `<br>`s alone in an untranslated element        | `<p>a<del><br><br>...<br></del>b</p>`   | `a<del><br><br>...<br></del>b`   |
| `<br>`s alone in a `<span>`                     | `<p>a<span><br><br>...<br></span>b</p>` | `a<span><br><br>...<br></span>b` |
| Whitespace-only emphasis                        | `<p>a<em>␣</em>b</p>`                   | `a<em>␣</em>b`                   |
| Empty emphasis                                  | `<p>a<em></em>b</p>`                    | `a<em></em>b`                    |
| Adjacent emphases                               | `<p><em>a</em><em>b</em></p>`           | `<em>a</em><em>b</em>`           |
| Adjacent code spans                             | `<p><code>a</code><code>b</code></p>`   | `<code>a</code><code>b</code>`   |

An emphasis [delimiter run](https://spec.commonmark.org/0.31.2/#delimiter-run)
opens emphasis only if it is
[left-flanking](https://spec.commonmark.org/0.31.2/#left-flanking-delimiter-run)
and closes emphasis only if it is
[right-flanking](https://spec.commonmark.org/0.31.2/#right-flanking-delimiter-run).
`<` and `>` are
[Unicode punctuation characters](https://spec.commonmark.org/0.31.2/#unicode-punctuation-character),
so an emphasis delimiter placed against a raw `<br>` dies whenever the character
on its far side is neither whitespace nor Unicode punctuation: `a*<br>*b` is the
literal text `a*`, a break, and `*b`. Rows 1-2 show the conservative algorithm
below at work; while these do have a CommonMark encoding, it is not used for
simplicity.

Rows 7-8 have no emphasis string at all once the whitespace is removed, and
CommonMark cannot spell an emphasis empty: `**` is a literal pair of asterisks,
not a strong emphasis holding nothing. Writing the whitespace on its own would
drop the element, and row 8 has not even that much to write, so both are
serialized.

Rows 9-10 are a pair of elements which CommonMark, written plainly, reads back
as one. `*a**b*` pairs its delimiter runs outside in, giving a single emphasis
holding `a**b`; the
[backtick strings](https://spec.commonmark.org/0.31.2/#backtick-string) of
``` `a``b` ``` pair the same way, giving a single code span holding `a``b`. Pure
mode writes the one element the pair reads as, combining the text of both, which
is a spelling and not a translation: it loses an element, and loses the
distinction between `<i>` and `<em>` or `<b>` and `<strong>` when the pair mixes
them. Faithful mode serializes each element of such a run instead. This applies
to a run of any inline element, so a pair which is already serialized —
`<sub>a</sub><sub>b</sub>` — stays the two elements it was rather than being
combined into one.

Approach:

1. If the emphasis string is empty once its leading and trailing whitespace is
   removed, serialize the entire element as HTML. This comes before step 3,
   which would otherwise write a bare pair of delimiters around nothing.
2. If the first or last character of the emphasis string, after removing leading
   and trailing whitespace, is a Unicode punctuation character, serialize the
   entire element as HTML. (This is conservative; if the emphasis was preceded
   or followed by Unicode whitespace, serialization isn't needed. However, the
   current implementation makes this difficult to determine.)
3. Move leading and trailing whitespace outside the emphasis string; surround
   the string with the appropriate delimiters.

Links
-----

| Description                   | HTML in                                     | Faithful expected         |
| ----------------------------- | ------------------------------------------- | ------------------------- |
| `<br>`s alone in a link       | `<p>a<a href="u"><br><br>...<br></a>b</p>`  | `a[<br><br>...<br>](u)b`  |
| `<br>`s starting a link label | `<p>a<a href="u"><br><br>...<br>c</a>b</p>` | `a[<br><br>...<br>c](u)b` |
| `<br>`s ending a link label   | `<p>a<a href="u">c<br><br>...<br></a>b</p>` | `a[c<br><br>...<br>](u)b` |

Unlike emphasis (see the inline elements section), link text has no flanking
rule: `[` and `]` delimit the label whatever characters sit next to them, which
makes this encoding straightforward.

At the document root
--------------------

| Description           | HTML in                      | Faithful expected            |
| --------------------- | ---------------------------- | ---------------------------- |
| Lone `<br>`s          | `<br><br>...<br>`            | `<br><br>...<br>`            |
| `<br>` before a block | `<br><p>a</p>`               | `<br>⏎⏎a`                    |
| `<br>` after a block  | `<p>a</p><br>`               | `a⏎⏎<br>`                    |
| `<br>`s-only `<div>`  | `<div><br><br>...<br></div>` | `<div><br><br>...<br></div>` |

Note that the row 1 translation is lossy: a `<p>` will be inserted when the
CommonMark is translated back to HTML when there are two or more `<br>`s. The
`<div>` row needs no special case at all: `div` *is* a block-level tag name, so
the whole element is an HTML block of
[type 6](https://spec.commonmark.org/0.31.2/#html-blocks) and round-trips
verbatim, however many `<br>`s it holds.

Headings
--------

The rows use `<h1>`. In the setext table, an `<h2>` underlines with `---`
instead of `===`, or writes `##` in a row which falls back to ATX; levels 3-6
have no setext form. In the ATX table, each level writes one more `#`.

The special case for paragraphs section's criteria also apply to
[Setext headings](https://spec.commonmark.org/0.31.2/#setext-headings); if a
level 1 or level 2 heading (the only headings expressible using setext) meets
these criteria, it must instead be encoded as an ATX heading, as the `<br>`-only
heading row below shows. A heading holding nothing falls back to ATX for a
different reason: its underline is empty as well, so the setext spelling is two
blank lines and the heading is lost.

| Description                        | HTML in                    | Faithful expected                 |
| ---------------------------------- | -------------------------- | --------------------------------- |
| Empty heading                      | `<h1></h1>`                | `#`                               |
| `<br>`-only heading (one `<br>`)   | `<h1><br></h1>`            | `#␣<br>`                          |
| `<br>`s-only heading (two or more) | `<h1><br><br>...<br></h1>` | `<br><br>...<br>⏎===============` |
| `<br>` starting a heading          | `<h1><br><em>b</em></h1>`  | `<br>*b*⏎=======`                 |
| `<br>` ending a heading            | `<h1><em>a</em><br></h1>`  | `*a*<br>⏎=======`                 |

An [ATX heading](https://spec.commonmark.org/0.31.2/#atx-headings) is a single
line. The `#` has already opened the line, so a raw `<br>` is safe anywhere in
the heading (see the analysis section). A heading with no content is the `#`s
alone: a space with nothing after it would leave trailing whitespace on the
line.

| Description               | HTML in                    | Faithful expected   |
| ------------------------- | -------------------------- | ------------------- |
| Empty heading             | `<h1></h1>`                | `#`                 |
| `<br>`s-only heading      | `<h1><br><br>...<br></h1>` | `#␣<br><br>...<br>` |
| `<br>` starting a heading | `<h1><br><em>b</em></h1>`  | `#␣<br>*b*`         |
| `<br>` ending a heading   | `<h1><em>a</em><br></h1>`  | `#␣*a*<br>`         |

### Heading attributes

pulldown-cmark's
[heading attributes](https://pulldown-cmark.github.io/pulldown-cmark/specs/heading_attrs.html)
extension ends a heading with an attribute block: `# a {#x .y z=1}` reads back
as `<h1 id="x" class="y" z="1">a</h1>`. Faithful mode writes attribute blocks,
so it assumes a pulldown-cmark reader with
`pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES`; a GFM reader such as
GitHub's shows them as literal text. `TranslationMode::Pure`, the default,
writes no attribute block and drops every attribute.

The attribute block is not inline content: its tokens are separated by
[ASCII whitespace](https://doc.rust-lang.org/std/primitive.str.html#method.split_ascii_whitespace),
and no escape or character reference is decoded in it. In Faithful mode, an
attribute which cannot be written in it *forces HTML*: the whole heading is
serialized as HTML instead of CommonMark, just as a table with a cell attribute
is serialized whole. Since the attribute block decodes nothing, it must bypass
the text escaping and normalization applied to the heading's content.

Values cannot be quoted: `{title="a b"}` is two tokens, `title="a` and `b"`.
A value holding whitespace therefore forces HTML, so common attributes such as
`style="color: red"` or a multi-word `title` send the whole heading to HTML.

* pulldown-cmark
  [ignores a one-byte token](https://github.com/pulldown-cmark/pulldown-cmark/blob/v0.13.4/pulldown-cmark/src/firstpass.rs#L2548),
  so a bare `#` or `.` is lost on read-back, as is a one-byte name such as `x`
  written alone.
* A non-empty `id` is written `#value` and each `class` token `.token`; only
  the first byte of such a token is stripped, so `id="#a"` is written `{##a}`
  and `class=".a"` is written `{..a}`. Any other attribute is written
  `name=value`, or `name=` when its value is empty: a bare `{hidden}` works too,
  but `name=` is the one form that also keeps a one-byte name. An empty `id` or
  a `class` with no tokens is written like any other attribute, as `id=` or
  `class=`.
* A name or value holding `{`, `}`, `<`, `>`, or `\` forces HTML, since each
  breaks the attribute block's scan described below. So does ASCII
  whitespace in an `id` or other value. A `class` is split into tokens instead,
  which loses nothing meaningful, since HTML splits `class` on the same five
  characters (space, tab, line feed, form feed, and carriage return). A
  vertical tab or a no-break space is not ASCII whitespace and is written as
  is.
* A name beginning with `#`, `.`, or `=`, or holding `&`, `"`, or `'`, forces
  HTML. A leading `#` or `.` makes the token an ID or a class, and
  pulldown-cmark splits a token at its first `=`. (The HTML tokenizer admits
  `=` in a name only as its first character.) pulldown-cmark escapes `&`, `"`,
  and `'`, and an HTML parser decodes character references only in values, so
  `a&b` in a name would read back as `a&amp;b`. In a value, `=`, `&`, `"`, and
  `'` are written as is.

pulldown-cmark renders the ID, then the classes joined by single spaces, then
the other attributes, so attribute order and `class` whitespace, which carry no
meaning in HTML, are lost. Faithful mode writes the attribute block in the same
order, with the other attributes in DOM order. An `id=` or `class=` is one of
the other attributes, so `<h1 id="" class="a">` is written `{.a id=}`.

A heading written as CommonMark with no attribute block (every Pure heading, and
each Faithful heading without attributes), ATX or setext, escapes a final `}`
as `\}`; otherwise `<h1>Price {USD}</h1>` would read back as
`<h1 USD="">Price</h1>`. When an attribute block is written, only the last one
on the line is read, so no escape is needed.

pulldown-cmark finds an attribute block by
[scanning back](https://github.com/pulldown-cmark/pulldown-cmark/blob/v0.13.4/pulldown-cmark/src/firstpass.rs#L2491-L2516)
from the last `}` for a `{`, and a `\` stops the scan. The scan reads the raw
line before any inline parsing, so a `{` inside a code span counts too: without
the escape, `<h1><code>{</code>b}</h1>` would lose its code span. A final `}`
is always plain text — every other inline ends in a different character, such
as a backtick, `*`, `)`, or `>` — so the escape is always available.

Without heading attributes, `\}` is an ordinary CommonMark escape, so Pure
writes it too: it costs a GFM reader nothing and protects a reader with heading
attributes. For example, Pure writes `<h1 id="x">Price {USD}</h1>` as
`# Price {USD\}`.

The headings section's three setext-to-ATX fallbacks (empty content, a lone type
7 tag, and content starting with a type 1-6 tag) are kept for simplicity, though
an attribute block makes the first two unnecessary: `{#x}` is not blank, and
`<br> {#x}` opens no HTML block. `<div>a</div> {#x}⏎===` still opens one.

**Special case**: ATX
[closing sequences](https://spec.commonmark.org/0.31.2/#atx-headings) belong to
the headings section: `# a #` reads back as `<h1>a</h1>`. An attribute block
does not help, since pulldown-cmark removes it first; `# a # {#x}` reads back
as `<h1 id="x">a</h1>`. A `#` with no space before it is kept: `# a# {#x}` reads
back as `<h1 id="x">a#</h1>`. Setext headings have no closing sequence.
Until the headings section handles closing sequences, the translation of an ATX
heading whose content ends in a space followed by `#`s, or consists only of
`#`s, is unspecified. Escaping the first `#` of such a run, as in `# a \#`,
would fix this the same way `\}` fixes a final `}`.

The rows use `<h1>`. A row whose description begins with *Setext* uses
`HeadingStyle::Setex`, and the rest use ATX; an underline spans the whole line,
attribute block included. A row whose description begins with *Pure* uses
`TranslationMode::Pure`, and the rest use Faithful. `⍽` is a no-break space
(U+00A0), `␋` a vertical tab (U+000B), and `␌` a form feed (U+000C). Round-trip
tests must read the Markdown back with `ENABLE_HEADING_ATTRIBUTES`, for example
through `render_markdown` in `tests/common/mod.rs`, since `round_trip` enables
no options.

Each heading below is written with an attribute block:

| Description                    | HTML in                                  | Faithful expected      |
| ------------------------------ | ---------------------------------------- | ---------------------- |
| ID                             | `<h1 id="x">a</h1>`                      | `#␣a␣{#x}`             |
| ID starting with `#`           | `<h1 id="#a">a</h1>`                     | `#␣a␣{##a}`            |
| Class starting with `.`        | `<h1 class=".a">a</h1>`                  | `#␣a␣{..a}`            |
| Classes with extra whitespace  | `<h1 class="␣b␣␣a␣">a</h1>`              | `#␣a␣{.b␣.a}`          |
| Attributes reordered           | `<h1 data-x="1" class="b" id="i">a</h1>` | `#␣a␣{#i␣.b␣data-x=1}` |
| Valueless attribute            | `<h1 hidden>a</h1>`                      | `#␣a␣{hidden=}`        |
| One-byte name                  | `<h1 x>a</h1>`                           | `#␣a␣{x=}`             |
| `&` in a value                 | `<h1 title="a&amp;b">a</h1>`             | `#␣a␣{title=a&b}`      |
| Character reference in a value | `<h1 title="&amp;amp;">a</h1>`           | `#␣a␣{title=&amp;}`    |
| `=` in a value                 | `<h1 title="a=b">a</h1>`                 | `#␣a␣{title=a=b}`      |
| `"` in a value                 | `<h1 title="a&quot;b">a</h1>`            | `#␣a␣{title=a"b}`      |
| `'` in a value                 | `<h1 title="a'b">a</h1>`                 | `#␣a␣{title=a'b}`      |
| No-break space in a value      | `<h1 id="a&nbsp;b">a</h1>`               | `#␣a␣{#a⍽b}`           |
| Vertical tab in a value        | `<h1 id="a&#11;b">a</h1>`                | `#␣a␣{#a␋b}`           |
| Empty ID                       | `<h1 id="">a</h1>`                       | `#␣a␣{id=}`            |
| Empty ID with a class          | `<h1 id="" class="a">a</h1>`             | `#␣a␣{.a␣id=}`         |
| Class with no tokens           | `<h1 class="">a</h1>`                    | `#␣a␣{class=}`         |
| Whitespace-only class          | `<h1 class="␣">a</h1>`                   | `#␣a␣{class=}`         |
| Empty heading                  | `<h1 id="x"></h1>`                       | `#␣{#x}`               |
| Setext heading                 | `<h1 id="x">a</h1>`                      | `a␣{#x}⏎======`        |
| Setext fallback, empty heading | `<h1 id="x"></h1>`                       | `#␣{#x}`               |
| Setext fallback, lone `<br>`   | `<h1 id="x"><br></h1>`                   | `#␣<br>␣{#x}`          |
| Setext fallback, type 6 tag    | `<h1 id="x"><div>a</div></h1>`           | `#␣<div>a</div>␣{#x}`  |

In an inline context every heading is already raw HTML inlines, so its
attributes are written as is. Each attribute below forces HTML in a block
context, where the heading becomes a type 6 HTML block, so a blank line inside
it is encoded as the translating HTML nodes section describes:

| Description            | HTML in                   | Faithful expected             |
| ---------------------- | ------------------------- | ----------------------------- |
| Whitespace in a value  | `<h1 title="a␣b">a</h1>`  | `<h1 title="a␣b">a</h1>`      |
| Blank line in a value  | `<h1 title="a⏎⏎b">a</h1>` | `<h1 title="a⏎&#10;b">a</h1>` |
| Form feed in an ID     | `<h1 id="a&#12;b">a</h1>` | `<h1 id="a␌b">a</h1>`         |
| `}` in a value         | `<h1 title="a}">a</h1>`   | `<h1 title="a}">a</h1>`       |
| `{` in a value         | `<h1 title="a{b">a</h1>`  | `<h1 title="a{b">a</h1>`      |
| `}` in a class token   | `<h1 class="a b}">a</h1>` | `<h1 class="a b}">a</h1>`     |
| Backslash in a value   | `<h1 title="a\b">a</h1>`  | `<h1 title="a\b">a</h1>`      |
| `<` in a value         | `<h1 title="a<b">a</h1>`  | `<h1 title="a&lt;b">a</h1>`   |
| `>` in a value         | `<h1 title="a>b">a</h1>`  | `<h1 title="a&gt;b">a</h1>`   |
| Name starting with `#` | `<h1 #a="1">a</h1>`       | `<h1 #a="1">a</h1>`           |
| Name starting with `.` | `<h1 .a="1">a</h1>`       | `<h1 .a="1">a</h1>`           |
| Name starting with `=` | `<h1 =a="1">a</h1>`       | `<h1 =a="1">a</h1>`           |
| Name holding `&`       | `<h1 a&b="1">a</h1>`      | `<h1 a&b="1">a</h1>`          |
| Name holding `"`       | `<h1 a"b="1">a</h1>`      | `<h1 a"b="1">a</h1>`          |
| Name holding `'`       | `<h1 a'b="1">a</h1>`      | `<h1 a'b="1">a</h1>`          |
| `{` in a name          | `<h1 a{b="1">a</h1>`      | `<h1 a{b="1">a</h1>`          |

A final `}` is escaped only where no attribute block is written:

| Description                                | HTML in                       | Expected                    |
| ------------------------------------------ | ----------------------------- | --------------------------- |
| Text ending in `}`                         | `<h1>Price {USD}</h1>`        | `#␣Price␣{USD\}`            |
| Pure, attribute block dropped              | `<h1 id="x">Price {USD}</h1>` | `#␣Price␣{USD\}`            |
| Setext, text ending in `}`                 | `<h1>Price {USD}</h1>`        | `Price␣{USD\}⏎============` |
| Code span holding `{`                      | `<h1><code>{</code>b}</h1>`   | `` #␣`{`b\} ``              |
| Text ending in `}` with an attribute block | `<h1 id="x">Price {USD}</h1>` | `#␣Price␣{USD}␣{#x}`        |

Paragraphs
----------

| Description                          | HTML in                    | Faithful expected |
| ------------------------------------ | -------------------------- | ----------------- |
| `<br>`-only paragraph (one `<br>`)   | `<p><br></p>`              | `<p><br></p>`     |
| `<br>`s-only paragraph (two or more) | `<p><br><br>...<br></p>`   | `<br><br>...<br>` |
| `<br>` starting a paragraph          | `<p><br><em>b</em></p>`    | `<br>*b*`         |
| `<br>` ending a paragraph            | `<p><em>a</em><br></p>`    | `*a*<br>`         |
| `<br>` before an image               | `<p><br><img src="i"></p>` | `<br>![](i)`      |
| `<br>` after an image                | `<p><img src="i"><br></p>` | `![](i)<br>`      |

The first row is the special case for paragraph section's rule that a paragraph
writing out as a single complete tag must be serialized: a bare `<br>` re-opens
as an HTML block, and the `<p>` around it would be lost.

Two or more `<br>`s need nothing of the sort. The second tag keeps the line from
being type 7, so the plain encoding is already an ordinary paragraph and
round-trips as it stands.

Blockquotes
-----------

| Description                      | HTML in                                          | Faithful expected   |
| -------------------------------- | ------------------------------------------------ | ------------------- |
| `<br>`s-only blockquote          | `<blockquote><br><br>...<br></blockquote>`       | `>␣<br><br>...<br>` |
| `<br>`-only blockquote paragraph | `<blockquote><p><br></p></blockquote>`           | `>␣<p><br></p>`     |
| `<br>` starting a blockquote     | `<blockquote><p><br><em>b</em></p></blockquote>` | `>␣<br>*b*`         |
| `<br>` ending a blockquote       | `<blockquote><p><em>a</em><br></p></blockquote>` | `>␣*a*<br>`         |

Note that the row 1 translation is lossy: a `<p>` will be inserted when the
CommonMark is translated back to HTML when there are two or more `<br>`s.

**Special case**: the blank line rule from the analysis section that puts a
blank line on either side of an HTML block, such as a `<div></div>`, is written
as a blank `>` line here. The first row is the root-level row one container
down, and holds for the same reason. The second row is the paragraph row one
container down: the `>` is stripped before the line is scanned, so the bare tag
would re-open as an HTML block exactly as it does at the root.

Math
----

Like an inline code span, the contents of
[math expressions](https://pulldown-cmark.github.io/pulldown-cmark/specs/math.html)
are literal text and are not parsed by CommonMark. However, newlines in
single-line CommonMark blocks (such as a heading or a table cell) or blank lines
in other blocks end the math span and begin another block. Since LaTeX
mathematics ignores newlines and other forms of whitespace, all newlines are
replaced with spaces in math expressions to prevent these mis-translations.
However, this tradeoff causes LaTeX `%` comments to be mis-translated. In
addition, leading and trailing whitespace is trimmed, since inline math may not
begin or end with whitespace per the
[pulldown-cmark spec](https://pulldown-cmark.github.io/pulldown-cmark/specs/math.html).

Table cells
-----------

The body-cell rows below are a one-column, one-row table —
`<table><thead><tr><th>h</th></tr></thead><tbody><tr><td>…</td></tr></tbody></table>`.
The columns show only the cell that holds the `<br>`, not the rest of the table.
Column padding is normalized here; the real output pads every cell to the column
width. Table heading behavior is identical to body-cell behavior. In the
*Faithful expected* column below, `\|` is
[GFM](https://github.github.com/gfm/#tables-extension-)'s escape for the literal
`|` which appears in the rendered output.

A cell's contents are parsed as inline content, so no HTML block can open inside
one; also, any instance of a `|` must be escaped as `\|`. That escape is read
only in CommonMark text, so a `|` sitting elsewhere in the cell is handled by
where it sits. In the open tag of a raw HTML inline it is encoded as `&#124;`,
which the HTML parser reading the result decodes back — the same trick the
"Translating HTML nodes" section uses for a line ending there. Inside a comment
neither the escape nor the encoding is read, so a comment holding a `|` leaves
the table no CommonMark spelling at all. The entire table must be serialized as
HTML if the table contains a child which could only be written as HTML (a cell
with an attribute, a `<caption>`, a comment between cells, a comment inside one
holding a `|`, etc.).

The table is also serialized as HTML where its shape is not one a
[GFM table](https://github.github.com/gfm/#tables-extension-) has. A GFM table
is a header row of header cells, then one body; every row holds the columns the
header declares. So the table goes out as HTML when it has no header row or more
than one, when a row of `<th>` follows a body row, when a `<thead>` follows a
`<tbody>` (one holding no row included), when its header row is made of `<td>`,
when it holds a `<tfoot>` or a second `<tbody>`, or when a row is wider or
narrower than the header.

| Description                       | HTML in                    | Faithful expected       |
| --------------------------------- | -------------------------- | ----------------------- |
| `<br>` before text in a body cell | `<td><br><em>b</em></td>`  | `\|␣<br>*b*␣\|`         |
| `<br>` after text in a body cell  | `<td><em>a</em><br></td>`  | `\|␣*a*<br>␣\|`         |
| `<br>`s-only body cell            | `<td><br><br>...<br></td>` | `\|␣<br><br>...<br>␣\|` |

Lists
-----

**Important**: This section is under development. Ignore it.

| Description                          | HTML in                                    | Faithful expected     |
| ------------------------------------ | ------------------------------------------ | --------------------- |
| Lone `<br>`s                         | `<ul><li><br><br>...<br></li></ul>`        | `*␣␣␣<br><br>...<br>` |
| Lone `<br>` before a block           | `<ul><li><br><p>a</p></li></ul>`           | `*␣␣␣<br>a`           |
| Lone `<br>` after a block            | `<ul><li><p>a</p><br></li></ul>`           | `*␣␣␣a<br>`           |
| `<br>`-only paragraph (one `<br>`)   | `<ul><li><p><br></p></li></ul>`            | `*␣␣␣<p><br></p>`     |
| `<br>`s-only paragraph (two or more) | `<ul><li><p><br><br>...<br></p></li></ul>` | `*␣␣␣<br><br>...<br>` |

Row 4 results from the special case for paragraphs section above: like a
blockquote, a list item's marker is stripped like a `>`, so a bare tag re-opens
as an HTML block.

Note that the last row of this table produces the desired HTML only in a loose
list; in a tight list, it produces the HTML for row 1. The algorithm below uses
this row for a loose list, using an HTML block for a tight list.

TODO: the table above only tests HTML blocks and raw HTML inlines in a list. The
following algorithm needs a much larger set of tests to cover all its cases.

Lists are difficult to get right. Some lists can only be represented as HTML;
some are more compactly represented as a loose list; others are more compactly
represented as a tight list. The goal of this algorithm is to categorize this
list, then produce the resulting CommonMark. However, it requires knowledge of
the CommonMark type of each list item's direct children (meaning a likely
re-parse of the current CommonMark, or an approach that stores the CommonMark as
a CommonMark AST) and the ability to access the DOM backing each of these
CommonMark list item direct children. Therefore, the following approach is
impractical to implement.

Notes:

* A CommonMark list item that directly contains two block-level elements with a
  blank line between them forces the list to be
  [loose](https://spec.commonmark.org/0.31.2/#loose). To avoid this forcing, if
  the list item contains only a paragraph followed by a block that can
  [interrupt a paragraph](https://spec.commonmark.org/0.31.2/#paragraphs) — a
  [bulleted or ordered list](https://spec.commonmark.org/0.31.2/#list-items)
  starting at 1 with a non-empty first element, a
  [blockquote](https://spec.commonmark.org/0.31.2/#block-quotes), an
  [ATX heading](https://spec.commonmark.org/0.31.2/#atx-headings), a
  [fenced code block](https://spec.commonmark.org/0.31.2/#fenced-code-blocks), a
  [thematic break](https://spec.commonmark.org/0.31.2/#thematic-breaks), or an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks) of types 1-6 —
  then omit the blank line which separates them except in approach 1.2.1 below.
* A CommonMark list of one item holding one block cannot contain a blank line,
  forcing the list to be [tight](https://spec.commonmark.org/0.31.2/#tight).

Approach:

1. If the list consists of one list item:
   1. If that list item consists of one paragraph, set the list type to tight
      (per the notes). Skip to step 3.
   2. If that list item consists of a paragraph followed by another block which
      can interrupt a paragraph:
      1. Determine if the DOM for the paragraph starts with a `<p>`. If so, this
         is more easily represented by a loose list; place a blank line between
         the paragraph and the next block to make this list loose. Set the list
         type to loose, then skip to step 3.
      2. Otherwise, this is more easily represented as a tight list. Write the
         paragraph followed by the block with no intervening blank line; since
         the block interrupts the paragraph, this forms a tight list. Set the
         list type to tight then skip to step 3.
2. Walk each CommonMark list item to determine how to more compactly represent
   the list. Default to an unknown (not loose or tight) list, with 0 votes for a
   loose list.
   1. If this list item directly contains two block-level elements with a blank
      line between them, this forces the list to be loose. If the list type is
      already tight, this list cannot be represented in CommonMark. End the
      walk, serializing the entire list as an HTML block. Otherwise, set the
      list representation to a loose list.
   2. Walk the children of this list item which are paragraphs:
      1. If the DOM for that paragraph begins with a `<p>`, then this paragraph
         is more compactly represented as a loose list, but can also be
         represented as the equivalent HTML block. Add 1 to the loose list vote.
      2. If the DOM for that paragraph begins with an element whose tag name is
         in the type 1 or 6 list, this paragraph is more compactly represented
         as a tight list, but can also be represented as the equivalent HTML
         block. Subtract 1 from the loose list vote.
      3. If the DOM for that paragraph begins with a text node or an element
         whose tag name is not in the type 1 or 6 list, the paragraph can only
         be represented as a tight list; there's no way to remove the beginning
         `<p>` element from a loose list. If the list type is loose, this list
         cannot be represented in CommonMark. End the walk, serializing the
         entire list as an HTML block. Otherwise, set the list representation to
         a tight list.
3. Examine the resulting computed list representation:
   1. If it is loose or unknown with a positive vote, then re-walk the direct
      children of each list item. If the child is a paragraph whose DOM does not
      begin with `<p>`, serialize it as HTML. Otherwise, leave it as CommonMark.
      Combine each list item with an intervening blank line.
   2. If it is tight or unknown with a non-positive vote, then re-walk the
      direct children of each list item. If the child is a paragraph whose DOM
      begins with `<p>`, serialize it as HTML. Otherwise, leave it as
      CommonMark. Combine each list item with an intervening newline.
