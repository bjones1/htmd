Project 2 submission
====================

Team members are Armanzhan Onggarbay, Nurlybek Orynbassar, Miras Shaltayev, and Aidyn Tazhibay. The specification consists of the file [testing_approach.md](testing_approach.md).

Draft submission
----------------

### Contributions

* Armanzhan Onggarbay: (Armanzhan's contributions)
* Nurlybek Orynbassar: Wrote the "Whitespace" section of `testing_approach.md` and revised it over four LLM review rounds. After the instructor's review, rewrote it as rules for comparing whitespace in two HTML strings, and revised the rewrite over a fifth LLM review round.
* Miras Shaltayev: Wrote the initial draft of `testing_approach.md`.
* Aidyn Tazhibay: Wrote the "Escaping" section of `testing_approach.md` and revised it over six LLM review rounds.

### Compiler test

The following statements summarize LLM output on each team member's computer, based on the prompt "Verify that you can compile and run the project in the current git repo."

#### Armanzhan Onggarbay

(Summarized LLM output)

#### Nurlybek Orynbassar

Claude Opus 5.5 (Claude Code 2.1.319) found that the project builds and runs on this machine with no changes: `cargo build` finished with no errors or warnings, and `cargo clippy -- --deny warnings` exited with status 0 (it touched `src/lib.rs` first to force a fresh check instead of a cached one). Since `htmd` is a library, it ran the bundled examples: `cargo run --example simple` exited with code 0 and prints nothing, because it only runs `assert_eq!` checks which held, and `cargo run --example table` printed the expected Markdown table. `cargo test` passed 267 tests across all suites with 0 failures and one test ignored in the source.

#### Miras Shaltayev

Claude Sonnet 5.5 found that `cargo` was installed in `~/.cargo/bin` but not on the `PATH`, and fixed this with `export PATH="$HOME/.cargo/bin:$PATH"`. After that, `cargo build` finished with no errors, `cargo run --example simple` exited with code 0 (it prints nothing, since the example only runs `assert_eq!` checks that passed), and `cargo test` passed in every test group, with one test ignored in the source. It did not run `cargo clippy -- --deny warnings`, which the CI also requires.

#### Aidyn Tazhibay

Claude Opus 5.5 (Claude Code, medium effort) found that the project compiles and runs on this machine with Rust 1.99.1, which I had installed with `rustup-init`: `cargo build` finished with no errors, and `cargo clippy -- --deny warnings` reported no warnings. Since `htmd` is a library, it ran the bundled examples: `cargo run --example simple` printed nothing, because it only runs `assert_eq!` checks which held, and `cargo run --example table` printed the expected Markdown table. `cargo test` passed 267 tests across all suites with 0 failures and one test ignored in the source.

### Spec review rounds

| Section | Round | Findings | Main changes made |
| ------- | ----- | -------- | ----------------- |
| Whitespace | 1 | 14 | `<br>` is raw HTML in Faithful mode; links may hold edge whitespace; `<code>` is not preformatted; added `<pre>` and fenced-code loss, NBSP moved out of emphasis, tables need `ENABLE_TABLES` |
| Whitespace | 2 | 11 | Leading whitespace in a leading emphasis is not idempotent; code span with a line ending collapses all whitespace; `<pre>` in a cell collapses only in pipe tables; emphasis next to punctuation stays raw HTML; raw inline edges excluded from "does not matter"; rule 2 allows `ENABLE_TABLES` |
| Whitespace | 3 | 6 | Code spans: trim before the line-ending test; headerless-table `<pre>` writes blank lines as `&#10;` and has the leading-LF bug; item 5 limited to Markdown blocks; lossy-case exception stated against "Known failures" |
| Whitespace | 4 | 8 | Leading-emphasis bug also at the document root; raw `<code>` keeps collapsed edge whitespace; `<textarea>` and `<pre>` in a list have the leading-LF bug; htmd and pulldown-cmark share one whitespace set |
| Whitespace | Instructor | 4 | Rewritten to cover only how two HTML strings are compared; Markdown compared exactly; whitespace defined as ASCII whitespace with MDN and Infra links; descriptions of htmd output (code spans, indentation, `<br>`, emphasis, tables) removed |
| Whitespace | 5 | 8 | Form feed is text, not collapsible whitespace; a final line feed in `<pre>` is not rendered; whitespace between table parts removed; a collapsed run keeps its first space; `&#13;` stays a carriage return; normalization stated as stricter than rendering where it is; attribute order ignored; citations fixed; helper test pairs for every item |
| Escaping | 1 | 11 | Code rule limited to Faithful mode; `&lt;<span>div</span>` moved to over-escapes; round trip compared after the Whitespace equivalences and pulldown-cmark's spelling; scope narrowed to text nodes; ATX closing `#` and `!` before a link added |
| Escaping | 2 | 11 | Over-escape redefined as an escape the position does not need, with both causes; `<` rule stated; `\|` escaped even inside code spans in pipe-table cells; text in an HTML block is not escaped, text in a raw HTML inline is; idempotency reason fixed |
| Escaping | 3 | 9 | Over-escape after the opening markup of an ATX heading or raw inline; `<` rule points to `escape.rs`; every `\|` in a cell's Markdown is escaped; NBSP and `</ a` near misses; `ENABLE_TABLES` for every pipe-table case |
| Escaping | 4 | 10 | Code-span delimiter rule corrected; `-` run needs `---` and `- - -` beside `--foo`; list and blockquote markup start a line; form feed added as a closer; near misses for item 5; block-separation failures filed as list bugs |
| Escaping | 5 | 5 | Text nodes after a list or `<pre>` sibling added; leading whitespace is dropped only at the start of a block; any HTML whitespace closes a marker; over- and under-escape defined once, first; only conditional markers in a cell need a reason |
| Escaping | 6 | 5 | Expectations come from the rules, not from htmd's output; a `<` cut short by a sibling listed as an over-escape; code-span padding and the backtick fence stated; two htmd bugs recorded as known failures (a backtick in a fence's language, whitespace moved next to an escaped `<`) |

### Last LLM review

#### Whitespace

Claude Opus 5.5 (Claude Code), with the prompt: "Review the section named "Whitespace" in testing_approach.md, writing findings to review-round5.md in the repo root. Verify each claim about HTML parsing (html5ever, which the crate already depends on) and browser rendering by running throwaway code or citing the WHATWG HTML, Infra and CSS Text specifications, and do not commit it. Do not edit testing_approach.md." The prompt went on to give the instructor's comments as context and to ask for the review's format. This is the output of round 5, the first review of the rewritten section; all its findings were applied.

> Review of "Whitespace" in testing_approach.md (round 5)
> ======================================================
>
> This reviews the section as of 25df8d1 ("Whitespace: rewrite as HTML
> comparison rules per review"), lines 47-84 of `testing_approach.md`.
>
> Method. Parsing claims were checked with a throwaway crate (since deleted)
> which parsed each string with `html5ever::parse_document` (0.39, into
> `markup5ever_rcdom`, as `src/lib.rs` does) and printed the `<body>` subtree,
> text nodes as Rust string literals. Round trips used htmd in
> `TranslationMode::Faithful` and pulldown-cmark 0.13 (`push_html`). Rendering
> claims were checked in the Chromium of the Claude desktop app (Chrome 156) by
> inserting each fragment into a page and measuring it with
> `getBoundingClientRect()` (an absolutely positioned element, `white-space:
> nowrap` for normal text so that width shows collapsing), and against the
> WHATWG HTML, Infra, CSS Text 3 and CSS 2 specifications.
>
> Summary
> -------
>
> The rewrite meets the instructor's goal. The section now covers only how two
> HTML strings are compared, says that Markdown is compared exactly, and defines
> whitespace as ASCII whitespace with the MDN link. Most claims about html5ever
> are correct, and so are the collapsing rules for spaces, tabs and line feeds.
>
> There are two errors. Form feed is ASCII whitespace, but CSS does not collapse
> it: a browser renders it as a glyph, so rule 1 makes strings equal which render
> differently (E1). And every fenced code block will fail: pulldown-cmark ends
> the code with a line feed which the input lacks, and "compared exactly" makes
> `<pre><code>a</code></pre>` differ from `<pre><code>a\n</code></pre>`, though a
> browser renders the two the same (E2).
>
> There are six omissions or inaccuracies. Whitespace between table rows and
> cells is not covered (O1). Rule 1 does not say which side of an element a
> collapsed run keeps (O2). `&#13;` is not equivalent to `\r` (O3). The section
> calls "renders the same" the test but then is stricter in some places, which
> it does not say (O4). The `textarea` citation points to the wrong section of
> the HTML spec, and the second MDN link is a redirect with a new title (O5).
> Rule for tests 3 does not fit two of the items it names (O6).
>
> Conflicts in other sections are listed at the end and are not counted.
>
> Errors
> ------
>
> ### E1. Form feed is ASCII whitespace but CSS does not collapse it
>
> Lines 51 and 59-62 make every ASCII whitespace character, U+000C FORM FEED
> included, collapsible: "A run of whitespace is equivalent to one space", and
> whitespace at the edge of a line is removed. That is true of the HTML parser's
> idea of whitespace, but rendering follows CSS, and CSS's set is smaller.
>
> CSS Text 3, [§4.1](https://www.w3.org/TR/css-text-3/#white-space-rules):
> "white space processing in CSS affects only the document white space
> characters: spaces (U+0020), tabs (U+0009), and segment breaks." Under
> [§4](https://www.w3.org/TR/css-text-3/#white-space-processing), control
> characters other than tab, line feed and carriage return "must be rendered as a
> visible glyph". U+000C is a control character. The MDN page which the section
> cites (line 53) agrees: its list of document white space characters is space,
> tab, LF and CR, with no form feed.
>
> Measured in Chrome 156 (width x height in CSS px, `<p>` with `white-space:
> nowrap`):
>
> | Text       | Width  |
> | ---------- | ------ |
> | `"ab"`     | 18.03  |
> | `"a b"`    | 22.22  |
> | `"a  b"`   | 22.22  |
> | `"a\fb"`   | 22.95  |
> | `"a\f\fb"` | 27.88  |
> | `"a \fb"`  | 27.14  |
> | `"\fab"`   | 22.95  |
> | `" ab"`    | 18.03  |
>
> A form feed is not one space wide, two form feeds are not collapsed into one,
> and a leading form feed is not removed. Without `nowrap`, `"a b"` wrapped at
> the space in a narrow box and `"a\fb"` did not, so a form feed is not a
> break opportunity either. Rules 1 and 2 therefore make `<p>a\fb</p>`
> equal to `<p>a b</p>`, and `<p>\fa</p>` equal to `<p>a</p>`, though each pair
> renders differently.
>
> This matters to the trip: htmd treats a form feed as whitespace and writes it
> as a space (`<p>a\fb</p>` gives `a b`, which comes back as `<p>a b</p>\n`), and
> the Escaping section asks for form-feed cases (line 118). Under the current
> rules those trips pass, yet they change the rendering.
>
> Suggested change. Keep ASCII whitespace as the definition of HTML whitespace,
> as the instructor asked, and say which part of it collapses: "A browser
> collapses only space, tab, line feed and carriage return
> ([CSS Text §4.1](https://www.w3.org/TR/css-text-3/#white-space-rules)); it
> renders U+000C as a visible character. So in the rules below a form feed is
> text, compared exactly, and a trip which turns a form feed into a space is a
> difference." If the team prefers to treat a form feed as a space anyway, say
> so as a deliberate choice, with the reason.
>
> ### E2. A trailing line feed in `<pre>` is compared exactly, so every code block fails
>
> "Where whitespace matters" item 1 (line 69) compares the text of `<pre>`
> exactly, with one exception for the leading line feed. pulldown-cmark ends the
> content of every code block with a line feed, and htmd writes the same fence
> whether or not the input has one:
>
> | HTML                          | Markdown        | Round trip                            |
> | ----------------------------- | --------------- | ------------------------------------- |
> | `<pre><code>a</code></pre>`   | ``"```\na\n```"`` | `"<pre><code>a\n</code></pre>\n"`   |
> | `<pre><code>a\n</code></pre>` | ``"```\na\n```"`` | `"<pre><code>a\n</code></pre>\n"`   |
>
> html5ever keeps the line feed (`<pre><code>"a\n"</code></pre>`), so the DOMs
> differ and, by line 84, the first trip is a failure. Since htmd's input HTML
> seldom ends code with a line feed, nearly every code-block test in
> `code_tests.rs` would become a known failure.
>
> A browser renders the two the same. Chrome 156, `<pre>` measured
> (width x height):
>
> | HTML                          | Size          |
> | ----------------------------- | ------------- |
> | `<pre>a</pre>`                | 9.63 x 25.59  |
> | `<pre>a\n</pre>`              | 9.63 x 25.59  |
> | `<pre>a\n\n</pre>`            | 9.63 x 51.19  |
> | `<pre><code>a</code></pre>`   | 9.63 x 25.59  |
> | `<pre><code>a\n</code></pre>` | 9.63 x 25.59  |
>
> The line after a final line feed holds nothing, and an empty line box has no
> height ([CSS 2 §9.4.2](https://www.w3.org/TR/CSS2/visuren.html#inline-formatting):
> line boxes which contain no text, no preserved white space, no inline
> elements with margins, padding or borders and no other in-flow content, "and do
> not end with a preserved newline must be treated as zero-height line boxes").
> The line after the last line feed is such a line box. A second line feed ends
> the line before it with a preserved newline, so it does add a line.
>
> Suggested change. Add a second exception to item 1: "A single line feed at the
> end of the content of a preformatted element is not rendered, so `<pre>a\n</pre>`
> is equivalent to `<pre>a</pre>`, and `<pre><code>a\n</code></pre>` to
> `<pre><code>a</code></pre>`, but `<pre>a\n\n</pre>` is not." Add the pair to
> the helper's tests (Rules for tests, item 3). This holds for `white-space: pre`
> and `pre-wrap` alike. (If the team wants the trip to report this, say so
> explicitly and record it once in "Known failures" rather than in every test.)
>
> Omissions and inaccuracies
> --------------------------
>
> ### O1. Whitespace between table parts is not covered
>
> Comparison rule 3 removes whitespace "between two block elements", and rule 2
> defines block elements as those "which the default style sheet displays as
> blocks", listing `<td>` and `<th>`. But the default style sheet displays `tr`
> as `table-row` and `td, th` as `table-cell`
> ([HTML §15.3.8](https://html.spec.whatwg.org/multipage/rendering.html#tables-2)),
> and `tbody`/`thead` as row groups, none of them `block`. pulldown-cmark puts line
> feeds between them (with `ENABLE_TABLES`):
>
> ```
> "| a |\n|---|\n| b |\n"
> => "<table><thead><tr><th>a</th></tr></thead><tbody>\n<tr><td>b</td></tr>\n</tbody></table>\n"
> ```
>
> html5ever keeps such whitespace as text nodes inside the table
> (`<table>\n<tr>\n<td>a</td>\n</tr>\n</table>` parses to
> `<table>"\n"<tbody><tr>"\n"<td>"a"</td>"\n"</tr>"\n"</tbody></table>`). A browser
> does not render it ([CSS 2 §17.2.1](https://www.w3.org/TR/CSS2/tables.html#anonymous-boxes),
> rule 1: white space between table-internal boxes is treated as if it had
> `display: none`). As written, a reader cannot tell whether the helper removes
> it, so every pipe-table trip may fail on it.
>
> Suggested change. In rule 3, add "or between two parts of a table (`<table>`,
> `<thead>`, `<tbody>`, `<tfoot>`, `<tr>`, `<td>`, `<th>`, `<caption>`)". In
> rule 2, say "the edges of a block container" (which includes `<td>`, `<th>` and
> `<li>`) rather than "block element".
>
> ### O2. Rule 1 does not say where the collapsed space stays
>
> Rule 1 collapses a run across inline boundaries, and "Where whitespace matters"
> item 3 says the side of a boundary matters. Together they need to say which
> space of a run survives, or the helper cannot decide `<em>a </em> b` against
> `<em>a</em> b`. CSS keeps the first: "Any collapsible space immediately
> following another collapsible space ... is collapsed to have zero advance
> width" ([CSS Text §4.1.1](https://www.w3.org/TR/css-text-3/#white-space-phase-1)).
> The section's own example agrees (`a <em> b</em>` keeps the space outside the
> `<em>`), but the rule does not state it.
>
> Suggested change. Rule 1: "A run of whitespace is equivalent to its first
> character, as a space. This holds across the boundaries of inline elements, so
> the space stays in the element where the run starts: `a <em> b</em>` is
> equivalent to `a <em>b</em>`, and `<em>a </em> b` to `<em>a </em>b`, not to
> `<em>a</em> b`."
>
> ### O3. `&#13;` is not equivalent to `\r`
>
> Line 57 lists `\r\n`, `\r` and `\n` as equivalent, and a character reference as
> equivalent to the character it stands for. Read together, that suggests
> `&#13;` is equivalent to `\r` and so to `\n`. It is not: line-ending
> normalization happens in input preprocessing
> ([HTML §13.2.3.5](https://html.spec.whatwg.org/multipage/parsing.html#preprocessing-the-input-stream)),
> before character references are decoded, so a referenced CR survives.
>
> ```
> "x\r\ny\rz" => "x\ny\nz"
> "a&#10;b"   => "a\nb"
> "a&#13;b"   => "a\rb"
> ```
>
> In normal text it does not matter (a CR collapses like a space; CSS Text §4:
> "Carriage returns (U+000D) are treated identically to spaces (U+0020) in all
> respects"; Chrome measured `"a\rb"` and `"a b"` both at 22.22). Inside `<pre>`
> it does: the DOM text is `"a\rb"`, not `"a\nb"`, so the comparison rejects
> `<pre>a&#13;b</pre>` against `<pre>a\nb</pre>`. (Chrome 156 renders the CR in
> `<pre>` as a line break, 9.63 x 51.19, though the CSS sentence above says it
> should render as a space; the specs and the browser disagree, so this case is
> best kept exact.) htmd itself writes a raw `\r` for it
> (`<pre>a&#13;b</pre>` gives `<pre>a\rb</pre>`, which parses to `"a\nb"`).
>
> Suggested change. Add after the list: "A character reference is decoded after
> line endings are normalized, so `&#13;` stays a carriage return and is not
> equivalent to a line feed."
>
> ### O4. "Renders the same" is the stated test, but the comparison is stricter, and the section does not say so
>
> Line 53 says two strings are equivalent when a browser renders them the same.
> Three rules then call strings different which render the same with the default
> style sheet:
>
> * item 3: `a <em>b</em>` and `a<em> b</em>` measured the same in Chrome (9.52
>   wide, wrapped the same); the text admits this ("renders the same in most
>   fonts");
> * item 4: comments are never rendered (`<p>a<!-- x -->b</p>` keeps a comment
>   node in the DOM), and leading or trailing spaces in an `href`, or extra spaces
>   in a `class`, do not change rendering either;
> * "the order of nodes" (line 65) is listed, but attribute order is not
>   mentioned at all, though it never affects rendering and pulldown-cmark's
>   order need not match the test author's.
>
> Each stricter rule is a defensible choice (the trip should not move whitespace
> into a link), but the section presents them as consequences of "renders the
> same".
>
> Suggested change. Line 53: "Two HTML strings are equivalent when they parse to
> the same DOM after the normalization below. The normalization removes only
> differences which a browser with its default style sheet does not render; a
> few which it does not render are still kept, as listed under 'Where whitespace
> matters'." And state attribute order explicitly ("attributes are compared as a
> set; their order is ignored"), or say that it is compared.
>
> ### O5. Two citations are off
>
> * Item 1 (line 69) cites
>   [HTML rendering, flow content](https://html.spec.whatwg.org/multipage/rendering.html#flow-content-3)
>   for both rules. That section has `listing, plaintext, pre, xmp { ...
>   white-space: pre; }`; `textarea { white-space: pre-wrap; }` is in
>   [§15.3.10 Form controls](https://html.spec.whatwg.org/multipage/rendering.html#form-controls).
> * Line 53's link, `.../Web/API/Document_Object_Model/Whitespace`, now answers
>   `301` to `/en-US/docs/Web/CSS/Guides/Text/Whitespace`, titled "Handling
>   whitespace". The link text gives the old title.
>
> Also, "the text of any element inside them" cannot apply to `<textarea>`,
> `<xmp>` or `<plaintext>`, whose content the parser reads as text only
> (`<xmp>\na</xmp>` parses to `<xmp>"\na"</xmp>`, with no elements possible).
>
> Suggested change. Cite both rendering sections; update the MDN link and title;
> say "and of any element inside a `<pre>` or `<listing>`".
>
> ### O6. Rule for tests 3 does not fit two items
>
> Rule 3 asks for "a pair which must not [compare equal]" for each item under
> "Where whitespace matters", "using the examples above". Item 1's exception gives
> a pair which must compare equal (`<pre>\na</pre>` and `<pre>a</pre>`), and
> item 4 has no example.
>
> Suggested change. "Each item ... gets a pair which must not compare equal, and
> item 1 also gets its exceptions as pairs which must. Item 4 uses
> `<a href="x">` against `<a href=" x">`, and `<!--a-->` against `<!-- a -->`."
>
> Verified as stated
> ------------------
>
> 1. **Scope and goal.** The section covers only the comparison of two HTML
>    strings for property 2 and says Markdown is compared exactly (properties 1
>    and 3), as the instructor asked. "Rules for tests" items 1 and 5 restate
>    this; item 4 (write invisible characters as escapes) is test style, not
>    comparison, but harmless.
> 2. **ASCII whitespace.** [Infra](https://infra.spec.whatwg.org/#ascii-whitespace)
>    defines it as U+0009, U+000A, U+000C, U+000D and U+0020; the
>    [MDN glossary](https://developer.mozilla.org/en-US/docs/Glossary/Whitespace)
>    gives the same five, citing Infra. U+00A0 and U+2003 are not whitespace for
>    CSS either (MDN: non-breaking spaces "don't trigger any collapsing").
> 3. **Parser equivalences (line 57).** `"x\r\ny\rz"` parses to `"x\ny\nz"`;
>    `"a&#10;b"` to `"a\nb"`; `<P CLASS=x>a<BR/>b` to
>    `<p class="x">"a"<br></br>"b"</p>`.
> 4. **Leading line feed (item 1).** `<pre>\na</pre>`, `<pre>\r\na</pre>` and
>    `<pre>&#10;a</pre>` all parse to `<pre>"a"</pre>`; `<listing>\na</listing>`
>    to `<listing>"a"</listing>`; `<textarea>\na</textarea>` to
>    `<textarea>"a"</textarea>`; `<pre>\n\na</pre>` to `<pre>"\na"</pre>`.
>    `<xmp>` and `<plaintext>` keep it (`"\na"`), as the section implies by
>    leaving them out. (HTML spec: "in body" insertion mode, start tags `pre`,
>    `listing`, and `textarea`: "If the next token is a U+000A LINE FEED (LF)
>    character token, then ignore that token".)
> 5. **Collapsing across inline boundaries (rule 1).** CSS Text §4.1.1, quoted in
>    O2. Tabs become spaces (same section).
> 6. **Line edges (rule 2).** CSS Text
>    [§4.1.2](https://www.w3.org/TR/css-text-3/#white-space-phase-2): "A sequence
>    of collapsible spaces at the beginning of a line is removed", and likewise
>    at the end. `<p>a <br> b</p>` parses to `"a "<br>" b"`, so the helper must
>    strip both, as the rule says.
> 7. **Whitespace between blocks (rule 3).** CSS Text §4: "any such white space
>    surrounding a block-level element is collapsed away". `<p>a</p>\n<p>b</p>\n`
>    parses to `<p>"a"</p>"\n"<p>"b"</p>"\n"`, so the rule is needed.
> 8. **Default `white-space` of preformatted elements and `<code>`.** As quoted
>    in O5; the default style sheet sets no `white-space` on `code`.
> 9. **Item 2 and item 3 examples.** The pairs differ in the DOM as stated, and
>    item 2's pairs also render differently.
> 10. **Line feed as a space.** Chrome renders `"中\n文"` as `"中 文"` (both 34.87
>     wide, `"中文"` 30.68). CSS Text 3 leaves the segment-break transformation
>     to the UA ([§4.1.3](https://www.w3.org/TR/css-text-3/#line-break-transform)),
>     and the MDN page cited on line 53 says line breaks are removed in languages
> without word separators such as Chinese, so the rule
>     holds in Chrome but not in every browser. Tests here use Latin text, so no
>     change is needed; a sentence could say so.
>
> Conflicts in other sections (not counted)
> -----------------------------------------
>
> * **Properties, line 19** says that where the strings differ but "a browser
>   parses them to the same DOM", the test states pulldown-cmark's string, "not a
>   normalized one". Two problems: `<p>a</p>` and `<p>a</p>\n` do not parse to
>   the same DOM (the second has a `"\n"` text node in `<body>`), and the
>   Whitespace section's rule for tests 2 says the opposite: write the expected
>   trip "in whichever equivalent form reads best", such as the input HTML.
>   Line 32 ("an explicit string shows what the trip produces") and Escaping rule
>   for tests 1, line 116 ("spelled as pulldown-cmark writes it") side with line
>   19. One of the two needs to change.
> * **Escaping, line 121** refers to "Whitespace item 6", which no longer exists
>   after the rewrite. The case it describes (`<p>a &lt;<em> b</em></p>`, a
>   space moved out of an element) is now "Where whitespace matters" item 3.
> * **Escaping, lines 93 and 118** treat a form feed as whitespace which closes a
>   block marker. That is right for CommonMark and for htmd, but see E1: the trip
>   then turns the form feed into a space, which changes rendering.

#### Escaping

Claude Opus 5.5 (Claude Code), with the prompt: "Review the section named "Escaping" in testing_approach.md, writing findings to review.md." Each round ran in a new session. This is the output of round 6; all its findings were applied.

> Review: "Escaping" section of testing_approach.md
> ===============================================
>
> Scope: `testing_approach.md` lines 84–118. Claims were checked against
> `src/dom_walker.rs` (`walk_text`, `walk_children`, `escape_if_needed`,
> `escape_image_marker`), `src/util/escape.rs`, `src/element_handler/headings.rs`
> (`push_escaping_closing_sequence`) and `normalize_cell_content` in
> `src/element_handler/table.rs`. Cases were run through Faithful mode and
> pulldown-cmark 0.13 (`ENABLE_TABLES | ENABLE_MATH`) in a scratch crate outside
> the repo, checking Markdown, round trip and idempotency.
>
> Findings
> --------
>
> ### 1. A `<` cut short by a sibling is an unlisted over-escape, and one form of it is not idempotent (medium)
>
> Item 3 escapes a `<` which the end of its text node cuts short. When the
> sibling's Markdown cannot continue a tag or an autolink, that escape is an
> over-escape, but it is neither in the over-escape list nor one of the
> "always-escaped characters" (line 109 lists `a \<b` there, yet item 3 is
> conditional: `a < b` is not escaped). A tester following rule 1 cannot tell
> whether the test needs a reason.
>
> ```
> <p>&lt;<em>1</em></p>       ->  \<*1*        round trip ok, idempotent
> <p>a &lt;<code> b</code></p> -> a \<`b`      round trip ok, idempotent
> ```
>
> Worse, when the sibling is an emphasis with leading whitespace, Whitespace
> item 6 moves that space out into the text, so the second conversion meets
> `< ` and does not escape:
>
> ```
> <p>a &lt;<em> b</em></p>    ->  a \< *b*     2nd pass: a < *b*      (not idempotent)
> <p>&lt;/<em> b</em></p>     ->  \</ *b*      2nd pass: </ *b*       (not idempotent)
> same in a pipe-table cell   ->  | \< *b* |   2nd pass: | < *b* |
> ```
>
> Rule 6 calls this a bug, which is right, but its reason ("the second
> conversion meets the same text") does not hold here: the text node changed
> between passes. Nothing in rules 2–4 would produce these inputs.
>
> Suggestion: add an over-escape bullet for a `<` which ends a text node when
> the sibling's Markdown does not continue raw HTML or an autolink (`\<*1*`);
> drop `a \<b` from the "always-escaped" sentence or say `<` is conditional; and
> add `<p>a &lt;<em> b</em></p>` as a case which rule 6 records as a bug. Qualify
> rule 6's reason: the second conversion meets the same text unless the trip
> moved whitespace into it (Whitespace item 6).
>
> ### 2. Item 6 omits the space padding of a code span which starts or ends with a backtick (medium-low)
>
> Rule 1 says to take expectations from the rules, but item 6 gives only the
> length of the backtick run. htmd also pads the content with a space when it
> starts or ends with a backtick, as CommonMark requires:
>
> ```
> <code>`a</code>  ->  `` `a ``
> <code>a`</code>  ->  `` a` ``
> <code>``</code>  ->  ``` `` ```
> ```
>
> From item 6 alone a tester would write ``` ```a`` ```, which opens a run of
> three. Suggestion: add "and pads the content with a space at each edge when it
> starts or ends with a backtick" with `<code>`a</code>` as the example and
> cases for both edges.
>
> ### 3. A backtick in a fence's info string breaks the fence; the section is silent (medium-low)
>
> Item 6 says a fence uses "a run of at least three, longer than any run of its
> character in the content", without naming the character or the info string.
> htmd always writes backticks and copies the language as it is, and
> [CommonMark](https://spec.commonmark.org/0.31.2/#fenced-code-blocks) forbids a
> backtick in the info string of a backtick fence:
>
> ```
> <pre><code class="language-a`b">x</code></pre>
>   Markdown:   "```a`b\nx\n```"
>   round trip: <p>```a`b\nx</p>\n<pre><code></code></pre>\n
>   idempotent: no
> ```
>
> (`language-a~b` is fine.) This is an under-escape in the "Markdown htmd writes
> around" the text, which the scope sentence includes, and
> `translation_approach.md` says nothing about info strings. Suggestion: name the
> fence character (backtick), state that an info string holding a backtick is a
> bug to record (or that htmd should switch to a tilde fence), and add the case
> above.
>
> ### 4. A lone `--` is escaped, and the rules do not decide whether it should be (low)
>
> `<p>--</p>` gives `\--` (`escape_if_needed` escapes any text of only `-` and
> spaces). Item 2 says a `-` run is escaped "when it could be a thematic break or
> a setext underline". `--` is too short for a break, and at the start of a block
> htmd writes it can never be an underline (htmd writes no soft break, Whitespace
> item 5; even the lazy continuation `<li><ul><li>a</li></ul>--</li>` cannot
> form one). So it is an over-escape under the definition, yet it is not in the
> over-escape list, and rule 3 covers `---`, `- - -` and `--foo` but not `--`.
> Suggestion: state that a text node of only `-` and spaces is always escaped,
> like `=`, so it needs no reason, and add `--` to rule 3.
>
> ### 5. The first over-escape bullet names too few kinds of opening markup (low)
>
> The bullet lists "an inline sibling, or the opening markup of an ATX heading or
> a raw HTML inline". The opening markup of a link or an emphasis has the same
> effect:
>
> ```
> <p><a href="x">- b</a></p>      ->  [\- b](x)
> <p><a href="x">1. b</a></p>     ->  [1\. b](x)
> <p><strong>1. b</strong></p>    ->  **1\. b**
> ```
>
> Suggestion: "the opening markup of an ATX heading or of any inline construct
> (a link, an emphasis, a raw HTML inline)", with `[\- b](x)` as an example.
>
> Not findings
> ------------
>
> * Section examples rechecked and correct: `\![a](x)` (also `a\![a](x)` and
>   `\\\![a](x)` after a backslash), `# a \#`, `# a *b* \#` (the closing-run
>   escape reads the heading's Markdown, not one text node), `<h1>a&nbsp;#</h1>`
>   correctly unescaped, the Setex `\- a\n----`, `> \> a`, `1\)`, `1.a`,
>   `\~`, `\==`, `a \$b\$ c`, `<u>\- b</u>`, `<em>\- b</em>` (punctuation keeps it
>   raw), `\|` in text, code spans and raw inlines of a cell, and `| \- a |` in
>   both header and body cells.
> * Form feed and tab close a marker (`<p>-\fa</p>` gives `\- a`, `<p>1.\ta</p>`
>   gives `1\. a`), and mid-line whitespace before a marker escapes nothing
>   (`a *b* - c`), as items 2 and the "one text node at a time" paragraph say.
> * `[x](a\|b)` in a cell comes back as `a%7Cb` and is not idempotent, but
>   pulldown-cmark percent-encodes `|` in any href; that belongs to the link
>   section, which the scope sentence already defers to.
> * A `<p>` holding a comment, `<div>` and `<dl>` are HTML blocks with no
>   escapes, as the scope sentence says.

Final submission
----------------

### Miras Shaltayev

GitHub userid: miras-kbtu

#### Contributions

Applied the instructor's pull request comments to `testing_approach.md` outside the "Whitespace" section (Faithful-only scope, removal of the "Escaping" section, known deviations, and a procedure to convert the existing tests), revised that part over six LLM review rounds, and cut it back to remove overspecification before the last round.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 20 | Pure mode removed; "Approach" heading added; comparison helper named and written first; property 3 uses the same options; lossy cases which also fail idempotency; conversion steps made explicit for each test file |
| 2 | 20 | Idempotency classified by Markdown, not HTML; `assert_faithful_unstable` added; standard deviations listed; root `<p>` around a block element recorded as a loss; rule for a wrong expectation; `event_tests.rs` and turndown files decided |
| 3 | 15 | Adjacent-list merging removed from the standard deviations; `<thead>` limited to a first row of `<th>`; the second Markdown must be stable; the migration finds every kind of Faithful call; Pure turndown cases kept |
| 4 | 16 | Wrong `<code>` example and wrong cause of the standard deviations fixed; root `<p>` around a block uses `assert_faithful_unstable`; one bug rule for all properties |
| 5 | 18 | Whole file reviewed; `#[ignore]` limited to the failing case; tests with custom handlers not converted; stale references to `round_trip` removed; nine "Whitespace" findings passed to its author |
| 6 | 15 | Spec cut back from about 2,450 to 1,490 words before this round; standard deviations which fail property 3 use `assert_faithful_unstable`; the `<p>` deviation limited to content with no block element; `ENABLE_TABLES` and `ENABLE_MATH` required; an unexplained loss is a bug |

#### Last specification review

Claude Opus 5.5 (Claude Code), in a new session, with the prompt: "Review /Users/iomiras/uni/htmd/testing_approach.md. Do not edit any file; give your findings in your final message." This is the output of round 6. Strikethrough marks findings which I deemed irrelevant, since they ask for implementation detail which the spec leaves to the implementer. Findings 1 and 8 concern the "Contributions" list and the "Whitespace" section, which their authors own; finding 14 needs no change.

> Review: testing_approach.md (working-tree version, uncommitted diff vs HEAD f4ee316). Read-only; no repo file edited. Read the spec, tests/common/mod.rs, tests/*.rs, translation_approach.md, git diff, and submission.md (to check the claimed round-2 fixes). Built a throwaway probe crate (htmd path dep + pulldown-cmark 0.13.4). It runs convert → render → convert.
>
> HIGH
>
> 1. L10: Contributions still credits Aidyn Tazhibay for the "Escaping" section. The working-tree diff deletes that whole section (old L90-125). submission.md says the removal was intended (instructor comment). Fix: change Aidyn's line, or note that the section was removed. L8 (Armanzhan Onggarbay) is still empty.
>
> 2. L35 vs L29/L41: a standard deviation can fail property 3, so "needs no reason" contradicts the procedure. Probe: `<a href="a b">x</a>` → MD `[x](<a b>)` → RT `<p><a href="a%20b">x</a></p>` → MD2 `[x](a%20b)`. L35 says percent-encoding needs no reason. But only `assert_faithful_unstable` passes this case, and L29 limits that helper to "a case which fails property 3 by design" (here the cause is pulldown-cmark, not htmd's design). L41 also says no property is skipped without a reason. Fix: say that a standard deviation which changes the second Markdown uses `assert_faithful_unstable` with a fixed reason, or with no reason.
>
> 3. L35: the claim that round-2 finding 3 was applied is false. submission.md says "root `<p>` around a block element recorded as a loss" was applied. The text only says "inline content comes back inside `<p>`", with no exception. When the wrapped content holds a block element (`a<del>x<hr>y</del>c`), the `<p>` changes the DOM. The hand-written expected string parses the same way, so property 2 passes and the loss stays hidden. Property 3 then fails, and L35 gives no guidance. The same broad wording also covers loose `<li><p>`. Probe: `<ul><li>a</li></ul><ul><li>b</li></ul>` → MD `*   a\n\n*   b` → RT is one loose list `<ul><li><p>a</p></li><li><p>b</p></li></ul>`. That is a real loss (two lists merge, which looks like an htmd bug against CommonMark). But a reader can file the added `<p>` as "standard". Fix: limit the `<p>` deviation to where it is added (root, blockquote) and to content with no block element. State that list merging is not covered.
>
> 4. ~~L45/L47-49: the migration of `turndown_cases_tests.rs` cannot be done as written. All ~140 cases are data in tests/html/turndown_test_index.html, and one `#[test] run_cases` runs them. A case has `html` and `md` fields only, with no field for an expected round trip. Step 2 ("write both expected strings by hand") has no place to put the round trip. The `#[ignore]` rule (L39) cannot isolate one data case. 28 cases are Pure and the defaults are Setex, so these cases need `_options`. Fix: give the storage for the round trip (for example a new element per case, which `load_test_cases` reads). Or state that turndown cases check properties 1 and 3 only, and give the reason. Round-2 finding 9 raised this. The new text adds the file to the migration but does not solve it.~~
>
> MEDIUM
>
> 5. L29: the spec does not say which pulldown-cmark options a test must use. It gives `ENABLE_TABLES`/`ENABLE_MATH` only "for example". The rule was in the deleted Escaping section (old rule 5). Probe: `<table><tr><th>h</th></tr><tr><td>a</td></tr></table>` with `CommonMarkOptions::empty()` → RT `<p>| h |\n| - |\n| a |</p>` → MD2 `| h | | - | | a |` (unstable). So every pipe-table test fails under plain `assert_faithful`, and a writer can record that as a deviation. htmd always escapes `$`, so without `ENABLE_MATH` the trip cannot catch a missing `$` escape. Fix: add one rule. A test which emits a pipe table uses `ENABLE_TABLES`. A test with `$` uses `ENABLE_MATH`. Other extensions are out of scope.
>
> 6. L37 vs L39: the boundary between "other deviation" and "bug" depends on who writes the design text. L37 says "If no section explains it, add one first". So a test writer can turn any bug into "by design" by writing a section in translation_approach.md. Fix: say who approves a new section, or say that an unexplained loss is a bug by default.
>
> 7. ~~L45-51: the procedure removes all guidance about the existing helpers. Old text named `assert_round_trips`, `assert_math_round_trips`, `assert_paragraph_survives`, and `round_trip_pure_table` (Pure, must stay). `round_trip_setext`, `round_trip` and `render_markdown` in tests/common/mod.rs also exist. Now the spec does not say if they are deleted, kept, or rewritten as wrappers. `round_trip` is used in basic_tests.rs and link_tests.rs. `round_trip_pure_table` (basic_tests.rs:724) is Pure, and "Leave other modes unchanged" covers it only by implication.~~
>
> 8. Whitespace "Rules for tests" item 2 (L91) says that a lossless expected round trip "is the input HTML itself". L31 says it is "the input HTML with the standard deviations". These contradict. submission.md says this cross-reference (round-2 finding 15) was skipped on purpose. It is still a contradiction for a reader.
>
> LOW
>
> 9. ~~L29: "`assert_faithful_unstable` and its `_options` variant take a fourth argument". The `_options` variant takes a sixth argument. No signature or argument order is given for any helper. The old text had one.~~
>
> 10. ~~L29: the spec does not say what `assert_faithful_unstable` checks after MD2. For example, does MD2 have to be stable on a third pass?~~
>
> 11. ~~L29/L45: the spec does not say where the comparison helper and its own tests live. A `#[test]` in tests/common/mod.rs runs once in each test binary that has `mod common;` (10 files). New helpers there also need `#[allow(dead_code)]`, as the existing helpers have. The spec also does not say what the helper prints on a mismatch.~~
>
> 12. ~~L47: "Find its Faithful assertions on one input" does not cover loop tests. `blocks_inside_inlines` (translation_approach_tests.rs:27) collects failures over many combinations. A panic-on-first helper changes that behavior.~~
>
> 13. ~~L39: "a comment which names the issue" names no tracker or format.~~
>
> 14. L21-23: the properties name `convert_faithful` and `render_markdown(x)` with one argument. The options variants call `convert_faithful_options`, and `render_markdown` takes an options argument. This is acceptable as shorthand.
>
> 15. ~~L29: the spec says that the round trip is checked first. It does not give the order of properties 1 and 3.~~
>
> Verified OK: event_tests.rs has 6 tests, all with `add_handler` (L51 correct). turndown_online_demo_tests.rs uses `Options` without Faithful, so it is Pure (L51 correct). The migration list covers all 10 test files. `<b>`/`<i>`→`<strong>`/`<em>`, the `<thead>`, and the percent-encoding deviations are real. `<pre>a</pre>` round-trips as an unchanged HTML block. `<pre><code>a</code></pre>` → `<pre><code>a\n</code></pre>` is covered by Whitespace item 1, exception 2. Blockquote `<p>` wrap, `<br>`, headings, `$` (with and without math), `<del>` and `<ol start>` are stable under property 3. No text still points to the removed Escaping section, except Contributions L10.

#### Implementation review rounds

(Table showing implementation review rounds)

#### Last implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

### Armanzhan Onggarbay

(Same sections as Miras Shaltayev)

### Nurlybek Orynbassar

(Same sections as Miras Shaltayev)

### Aidyn Tazhibay

(Same sections as Miras Shaltayev)
