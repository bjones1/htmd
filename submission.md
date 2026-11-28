Project 2 submission
====================

Team name: (team name)

Team members are Armanzhan Onggarbay, Nurlybek Orynbassar, Miras Shaltayev, and Aidyn Tazhibay. The specification consists of the file [testing_approach.md](testing_approach.md).

Draft submission
----------------

Each "Build and run test" summarizes LLM output on that team member's computer, based on the prompt "Verify that you can compile and run the project in the current git repo."

### Miras Shaltayev

GitHub userid: miras-kbtu

#### Contributions

Wrote the initial draft of `testing_approach.md`.

#### Build and run test

Claude Sonnet 5.5 found that `cargo` was installed in `~/.cargo/bin` but not on the `PATH`, and fixed this with `export PATH="$HOME/.cargo/bin:$PATH"`. After that, `cargo build` finished with no errors, `cargo run --example simple` exited with code 0 (it prints nothing, since the example only runs `assert_eq!` checks that passed), and `cargo test` passed in every test group, with one test ignored in the source. It did not run `cargo clippy -- --deny warnings`, which the CI also requires.

#### Specification review rounds

(Table showing spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

### Armanzhan Onggarbay

GitHub userid: aronggarbay

#### Contributions

(Armanzhan's contributions)

#### Build and run test

(Summarized LLM output)

#### Specification review rounds

(Table showing spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

### Nurlybek Orynbassar

GitHub userid: noxfurry

#### Contributions

Wrote the "Whitespace" section of `testing_approach.md` and revised it over four LLM review rounds.

#### Build and run test

Claude Opus 5.5 (Claude Code 2.1.319) found that the project builds and runs on this machine with no changes: `cargo build` finished with no errors or warnings, and `cargo clippy -- --deny warnings` exited with status 0 (it touched `src/lib.rs` first to force a fresh check instead of a cached one). Since `htmd` is a library, it ran the bundled examples: `cargo run --example simple` exited with code 0 and prints nothing, because it only runs `assert_eq!` checks which held, and `cargo run --example table` printed the expected Markdown table. `cargo test` passed 267 tests across all suites with 0 failures and one test ignored in the source.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 14 | `<br>` is raw HTML in Faithful mode; links may hold edge whitespace; `<code>` is not preformatted; added `<pre>` and fenced-code loss, NBSP moved out of emphasis, tables need `ENABLE_TABLES` |
| 2 | 11 | Leading whitespace in a leading emphasis is not idempotent; code span with a line ending collapses all whitespace; `<pre>` in a cell collapses only in pipe tables; emphasis next to punctuation stays raw HTML; raw inline edges excluded from "does not matter"; rule 2 allows `ENABLE_TABLES` |
| 3 | 6 | Code spans: trim before the line-ending test; headerless-table `<pre>` writes blank lines as `&#10;` and has the leading-LF bug; item 5 limited to Markdown blocks; lossy-case exception stated against "Known failures" |
| 4 | 8 | Leading-emphasis bug also at the document root; raw `<code>` keeps collapsed edge whitespace; `<textarea>` and `<pre>` in a list have the leading-LF bug; htmd and pulldown-cmark share one whitespace set |

#### Last specification review

Claude Opus 5.5 (Claude Code), with the prompt: "Review the section named "Whitespace" in testing_approach.md, writing findings to review.md. Verify each claim about htmd and pulldown-cmark behavior by running throwaway code, and do not commit it. Do not edit testing_approach.md."

Findings which were not applied are struck through.

> **Review of "Whitespace" in testing_approach.md (round 4)**
>
> This reviews the section as of 91e886c ("Whitespace: apply review round 3").
> Every claim was checked with a throwaway integration test (since deleted). Each
> input was converted with `convert_faithful`, rendered with `render_markdown(md,
> CommonMarkOptions::empty())` (or `ENABLE_TABLES` for pipe tables), and the
> rendered HTML was converted again to check idempotency (property 3). Outputs
> are quoted as Rust string literals.
>
> **Summary**
>
> Round 3 is applied fully. E1 (trim first, then test for a line ending), O1
> (headerless-table `<pre>`), O2 ("written as Markdown") and O3 (the exception to
> "Known failures") all hold now. Two claims are wrong. Item 6 places the
> leading-emphasis bug too narrowly: it also happens at the document root, which
> is where htmd usually starts, since it is often given a fragment (E1). Item 2
> says the trim comes first, but the raw `<code>` fallback is written from the
> untrimmed content, so its edge whitespace comes back (E2). There are three
> omissions. `<textarea>` has the same leading line feed bug as `<pre>` (O1). ~~A
> vertical tab at the end of a paragraph is a new idempotency failure (O2).~~
> Item 7 suggests that htmd and pulldown-cmark use different whitespace sets,
> but they use the same one (O3).
>
> **Errors**
>
> **E1. The leading-emphasis bug is not limited to `<p>` and `<blockquote>`**
>
> Item 6 says "Leading whitespace in an emphasis which starts a `<p>` or a
> `<blockquote>` stays as a leading space". The trigger is wider: an emphasis
> which starts a Markdown paragraph, wherever that paragraph comes from. The
> document root (a fragment with no `<p>`, or `<body>`) fails in the same way:
>
> | HTML                              | Markdown     | Round trip                    | Second pass  |
> | --------------------------------- | ------------ | ----------------------------- | ------------ |
> | `<em> b</em>`                     | `" *b*"`     | `"<p><em>b</em></p>\n"`       | `"*b*"`      |
> | `<strong> b</strong>`             | `" **b**"`   | `"<p><strong>b</strong></p>\n"` | `"**b**"`  |
> | `<body>  <em> b</em></body>`      | `" *b*"`     | `"<p><em>b</em></p>\n"`       | `"*b*"`      |
> | `<p>a</p><p><em> b</em></p>`      | `"a\n\n *b*"` | `"<p>a</p>\n<p><em>b</em></p>\n"` | `"a\n\n*b*"` |
> | `<ul><li><blockquote><em> b</em></blockquote></li></ul>` | `"*   >  *b*"` | `"<ul>\n<li>\n<blockquote>\n<p><em>b</em></p>\n</blockquote>\n</li>\n</ul>\n"` | `"*   > *b*"` |
>
> `<em>\tb</em>`, `<em>\nb</em>` and `<em>    b</em>` at the start of a `<p>` all
> give `" *b*"`. (Four spaces collapse to one, so they never start an indented
> code block.) As before, `<li>`, `<li><p>`, `<h1>` and a pipe-table cell trim
> the space and are idempotent. `<div>`, `<section>` and `<dd>` are written as
> HTML blocks, so the case does not arise. A link at the root keeps its space
> inside the brackets (`"[ b](u)"`) and is idempotent.
>
> Most tests convert fragments, so the root case is the one a test writer is
> most likely to hit. Suggested change: "Leading whitespace in an emphasis which
> starts a paragraph, whether from a `<p>`, a `<blockquote>` or text at the
> document root, stays as a leading space ...".
>
> **E2. The raw `<code>` fallback keeps the edges which the trim dropped**
>
> Item 2 now says "htmd first drops edge spaces, tabs and line endings", then "If
> what remains has a line ending, or is empty, the span is written as a raw
> `<code>` inline and all its whitespace collapses to single spaces". A reader
> takes the raw inline to hold the trimmed content. It does not. The trim only
> chooses the form. The raw inline is serialized from the element's original
> children, so the edge whitespace comes back as one space
> (`src/element_handler/code.rs:145-160`: `content` is trimmed and tested, and
> `serialize_element_when_faithful!` then serializes `element`):
>
> | HTML                          | Markdown               | Round trip                          | Second pass   |
> | ----------------------------- | ---------------------- | ----------------------------------- | ------------- |
> | `<p>x<code> a\nb </code>y</p>` | `"x<code> a b </code>y"` | `"<p>x<code> a b </code>y</p>\n"` | ``"x`a b`y"`` |
> | `<p>x<code>  </code>y</p>`    | `"x<code> </code>y"`   | `"<p>x<code> </code>y</p>\n"`       | idempotent    |
> | `<p>x<code>\n</code>y</p>`    | `"x<code> </code>y"`   | `"<p>x<code> </code>y</p>\n"`       | idempotent    |
>
> The second and third rows are what "or is empty" covers: a whitespace-only span
> goes raw with one space, not as an empty `<code></code>`. A test which expects
> `"x<code></code>y"` for `<code>  </code>` (as the item now reads) fails. In the
> first row the second pass loses the edges as well, so the item's
> non-idempotency note understates the drift.
>
> Suggested rewrite: "htmd drops edge spaces, tabs and line endings to decide the
> form (lossy, reason in the test). ... If what remains has a line ending, or is
> empty, the span is instead written as a raw `<code>` inline from its original
> content, with every run of whitespace, edges included, collapsed to one space."
>
> **Omissions and inaccuracies**
>
> **O1. `<textarea>` has the leading line feed bug too**
>
> The HTML parser drops a line feed directly after `<textarea>`, as it does after
> `<pre>` ([HTML § 13.2.6.4.7](https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody),
> "textarea" start tag). htmd copies `<textarea>` verbatim and, again, does not
> write the line feed back:
>
> * `<textarea>\n\na</textarea>` gives `"<textarea>\na</textarea>"`, then
>   `"<textarea>a</textarea>"` (not idempotent).
> * `<textarea>\na</textarea>` gives `"<textarea>a</textarea>"` (idempotent, same
>   DOM).
>
> The section names `<textarea>` only in the "does not matter" scope list, so
> this case would be missed. Item 1 could add: "The same applies to
> `<textarea>`." The bug also reproduces for a `<pre>` in a list item
> (`<ul><li><pre>\n\na</pre></li></ul>` gives `"*   <pre>\n    a</pre>"`, then
> `"*   <pre>a</pre>"`), which "the leading line feed bug applies there too" covers
> only for tables.
>
> ~~`<listing>`, which the parser treats like `<pre>`, is not handled as
> preformatted at all: `<listing>a  b</listing>` gives `"<listing>a b</listing>"`.
> It is obsolete, so a sentence would do, if anything.~~
>
> ~~**O2. A trailing vertical tab is a new idempotency failure**~~
>
> ~~U+000B is not HTML whitespace (the intro's list is right) and is not in
> htmd's block-edge trim (`is_document_whitespace` in `src/util/text.rs:23` is
> tab, LF, CR and space). pulldown-cmark, however, counts it as ASCII whitespace
> (`scanners.rs`: `0x09..=0x0d`) and strips it from the end of a paragraph:~~
>
> | HTML                  | Markdown        | Round trip              | Second pass |
> | --------------------- | --------------- | ----------------------- | ----------- |
> | ~~`<p>a\u{b}</p>`~~       | ~~`"a\u{b}"`~~      | ~~`"<p>a</p>\n"`~~          | ~~`"a"`~~       |
> | ~~`<p>\u{b}a\u{b}</p>`~~  | ~~`"\u{b}a\u{b}"`~~ | ~~`"<p>\u{b}a</p>\n"`~~     | ~~`"\u{b}a"`~~  |
> | ~~`<h1>a\u{b}</h1>`~~     | ~~`"# a\u{b}"`~~    | ~~`"<h1>a\u{b}</h1>\n"`~~   | ~~idempotent~~  |
>
> ~~A leading vertical tab, and one before `<br>`, survive. No other non-HTML space
> tried does this: U+0085, U+00A0, U+1680, U+2003, U+2028, U+202F and U+3000 are
> kept at both edges of `<p>` and `<h1>` and are idempotent. Item 7 should say
> that U+000B is the exception, and the case is a known failure to record.~~
>
> **O3. Item 7: htmd and pulldown-cmark use the same whitespace set**
>
> Item 7 says "pulldown-cmark counts Unicode whitespace for flanking, and htmd
> trims with Rust's `White_Space` (which also has U+0085, U+2028 and U+2029)".
> Read with CommonMark's definition of Unicode whitespace (Zs plus tab, LF, FF and
> CR), "also" suggests that htmd moves those three out of emphasis when
> pulldown-cmark would not need it. In fact pulldown-cmark tests flanking with
> `char::is_whitespace` (`firstpass.rs:2266`, `2293`, `2311`, `2336`), which is the
> same `White_Space` property, and so it departs from the spec in the same way:
> `"*\u{85}a*"`, `"*\u{2028}a*"` and `"*\u{b}a*"` all stay literal. The two
> sets agree, which is why every case round 3 tried is idempotent. U+200B is in
> neither: htmd keeps it inside (`"a*\u{200b}b*c"`), and it renders as emphasis.
>
> Suggested change: "htmd trims emphasis with Rust's `White_Space`, which is wider
> than CommonMark's Unicode whitespace (it adds U+000B, U+0085, U+2028 and
> U+2029). pulldown-cmark tests flanking with the same property, so an edge
> U+00A0 or U+2003 moves out of emphasis like an ASCII space (lossy) and the
> result is stable."
>
> **O4. Smaller points**
>
> * **Item 2, "Known failures".** A code span with a remaining line ending fails
>   idempotency. "Known failures" requires such a case to be either "a bug to
>   fix" or given its own test of the unstable second Markdown. Item 1 calls its
>   case "a bug". Item 2 does not say which this is. The same failure occurs in a
>   pipe-table cell (`<code>a\nb</code>` gives `"| <code>a b</code> |"`, then
>   ``"| `a b` |"``).
> * **The inline-element sentence.** "a pair of inputs under these rules never
>   varies it" can be read as forbidding `<em>a</em> <em>b</em>` versus
>   `<em>a</em>\n<em>b</em>`. That pair is a valid test of rule 1 and gives
>   `"*a* *b*"` for both. The intent seems to be "never adds or removes it".
> * ~~**Item 6, raw-inline edges.** `<p><span> a</span></p>` keeps its leading space
>   (`"<span> a</span>"`) and is idempotent. This is consistent with the scope
>   sentence, but "does not matter" rule 3 says a block's leading whitespace is
>   dropped "except for the leading emphasis case". A raw inline at the start of
>   a block is a second exception.~~
>
> **Verified as stated**
>
> * **Intro.** NBSP is never collapsed. The exception paragraph is accurate:
>   "Code blocks and code spans" in `translation_approach.md` explains neither
>   the fenced block's trailing line feed nor the code span's edge trim. Item 6's
>   loss is explained by step 3 of "Emphasis and strong emphasis".
> * **Item 1.** `<pre>a  b\n\tc\n\n\nd</pre>` and `<pre>a\u{c}b</pre>` round-trip
>   exactly. `<pre>a\r\nb\rc\nd</pre>` gives `"<pre>a\nb\nc\nd</pre>"`.
>   `<pre>\n\na</pre>` gives `"<pre>\na</pre>"`, then `"<pre>a</pre>"`.
>   `<pre><code>a</code></pre>` comes back as `"<pre><code>a\n</code></pre>\n"`.
>   The fenced block keeps leading blank lines, spaces and tabs (also in a list
>   item and a blockquote, where htmd indents or prefixes each line and
>   pulldown-cmark strips that again). Raw `<pre>` in a list item, after text in a
>   list item, in a blockquote and in a blockquote list round-trips exactly. In
>   `<h1>` and in a pipe-table cell, `<pre>` and `<pre><code>` collapse to
>   `"<pre>a b c</pre>"` and are idempotent. In a headerless table,
>   `<pre>a\n\nb</pre>` gives `"...<pre>a\n&#10;b</pre>..."` (idempotent), and the
>   leading line feed bug reproduces.
> * **Item 2.** `x<code>a  \tb</code>y` keeps its runs. Edge spaces, tabs and line
>   endings are dropped (``"x`a`y"``). Edge form feed, NBSP and U+000B are kept.
>   Form-feed-only and NBSP-only spans are backtick spans. `<code></code>` goes raw.
>   An interior line ending goes raw and is not idempotent, and form feeds in
>   that raw inline collapse while NBSP is kept. A backtick span in a pipe table
>   keeps its runs (``"| `a  b` |"``).
> * **Item 3.** `"*   a\n\n    b"`, `"*   a\n    *   b"` and
>   `"1.  a\n\n    ```\n    x\n      y\n    ```"` round-trip. Four leading NBSP stay
>   a paragraph. Four leading spaces in a `<p>` are trimmed.
> * **Item 4.** `a<br>b`, `a <br> b`, `a<br/>b` round-trip as raw `<br>`.
>   `a<br>\nb` gives `"a<br> b"`. No case tried produced a line ending in a
>   paragraph: trailing spaces in `<li>` are trimmed, NBSP before `<br>` stays
>   before the raw tag, and a backslash before a line ending is escaped
>   (`"a\\\\ b"`).
> * **Item 5.** Line endings and blank lines in `<p>`, `<h1>`, `<li>` and `<a>`
>   give one space. In attributes they are encoded instead (`"![a&#10;b](u)"`,
>   `"[t](u \"a&#10;b\")"`), which is outside "HTML text". A headerless table keeps
>   them.
> * **Item 6.** `"a *b*c"`, `"a**b** c"`, and `"a<em> !b</em>c"` for punctuation.
>   Nested `<p><strong><em> b</em></strong></p>` gives `"<strong> *b*</strong>"`
>   and is idempotent. Trailing space at the end of `<p>` and a space after `<br>`
>   are fine. Links keep edge whitespace, including at the start of a paragraph
>   (`"[ b ](u)"`, idempotent). Whitespace-only `<em>` and `<strong>`, and empty
>   `<em>`, stay raw.
> * **Item 7.** Edge U+00A0, U+2003, U+1680, U+202F and U+000B move out of
>   emphasis. pulldown-cmark leaves `"*\u{a0}a*"`, `"*\u{1680}a*"`, `"*\u{85}a*"`
>   and `"*\u{b}a*"` literal. NBSP stays inside a link (`"x[\u{a0}t\u{a0}](u)y"`).
>   An NBSP-led emphasis at the start of a paragraph is idempotent
>   (`"\u{a0}*b*"`), unlike the ASCII case in E1.
> * **Item 8** (with `ENABLE_TABLES`). `"| a b |"`, `"| a<br>b |"`, `"| a |"`,
>   `"| <p>a</p><p>b</p> |"`, an edge NBSP kept (`"| \u{a0}a |"`) and a leading
>   emphasis space trimmed (`"| *b* |"`) all round-trip and are idempotent. A
>   headerless table is copied as it is (`&nbsp;` for NBSP). With empty options
>   the pipe table is a `<p>` and fails idempotency, as the item warns.
> * **"Does not matter", scope and 1-4.** `<div>`, `<textarea>` and raw inline
>   edges behave as stated (`"a <span> b</span>"` and `"a <span>b</span>"`
>   differ). Runs with tab, form feed, `\r\n` and `\r` give `"a b"`. Whitespace
>   between blocks, form feed included, is ignored. Edges of `<p>`, `<h1>`, `<li>`
>   and `<blockquote>` are trimmed, form feed included.
> * **Inline-element sentence.** `<em>a</em><em>b</em>` gives
>   `"<em>a</em><em>b</em>"`, and one space, two spaces or a line ending between
>   them give `"*a* *b*"`.
>
> **Known failures found by this review**
>
> * `<textarea>\n\na</textarea>` loses a line feed on each pass (O1).
> * ~~`<p>a\u{b}</p>`: pulldown-cmark strips a trailing vertical tab (O2).~~
> * The leading-emphasis bug at the document root, which the section does not
>   list in that form (E1).
>
> The failures the section already lists all reproduce.

### Aidyn Tazhibay

GitHub userid: aidyntazhibai

#### Contributions

Wrote the "Escaping" section of `testing_approach.md` and revised it over six LLM review rounds.

#### Build and run test

Claude Opus 5.5 (Claude Code, medium effort), in a new session given the prompt above plus a short note that this was a course assignment and that it should not edit, commit or push anything, found that the project compiles and runs on this machine with Rust 1.99.1, which I had installed with `rustup-init`: `cargo build` finished with no errors, and `cargo clippy -- --deny warnings` reported no warnings. Since `htmd` is a library, it ran the bundled examples: `cargo run --example simple` printed nothing, because it only runs `assert_eq!` checks which held, and `cargo run --example table` printed the expected Markdown table. `cargo test` passed 267 tests across all suites with 0 failures and one test ignored in the source.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 11 | Code rule limited to Faithful mode; `&lt;<span>div</span>` moved to over-escapes; round trip compared after the Whitespace equivalences and pulldown-cmark's spelling; scope narrowed to text nodes; ATX closing `#` and `!` before a link added |
| 2 | 11 | Over-escape redefined as an escape the position does not need, with both causes; `<` rule stated; `\|` escaped even inside code spans in pipe-table cells; text in an HTML block is not escaped, text in a raw HTML inline is; idempotency reason fixed |
| 3 | 9 | Over-escape after the opening markup of an ATX heading or raw inline; `<` rule points to `escape.rs`; every `\|` in a cell's Markdown is escaped; NBSP and `</ a` near misses; `ENABLE_TABLES` for every pipe-table case |
| 4 | 10 | Code-span delimiter rule corrected; `-` run needs `---` and `- - -` beside `--foo`; list and blockquote markup start a line; form feed added as a closer; near misses for item 5; block-separation failures filed as list bugs |
| 5 | 5 | Text nodes after a list or `<pre>` sibling added; leading whitespace is dropped only at the start of a block; any HTML whitespace closes a marker; over- and under-escape defined once, first; only conditional markers in a cell need a reason |
| 6 | 5 | Expectations come from the rules, not from htmd's output; a `<` cut short by a sibling listed as an over-escape; code-span padding and the backtick fence stated; two htmd bugs recorded as known failures (a backtick in a fence's language, whitespace moved next to an escaped `<`) |

#### Last specification review

Claude Opus 5.5 (Claude Code), with the prompt: "Review the section named "Escaping" in testing_approach.md, writing findings to review.md." Each round ran in a new session, which also received a short note that it was reviewing one team member's section for a course assignment and should write only `review.md`. This is the output of round 6; all its findings were applied.

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

GitHub userid: aronggarbay

#### Contributions

(Armanzhan Onggarbay's contributions)

#### Specification review rounds

(Table showing spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

#### Implementation review rounds

(Table showing implementation review rounds)

#### Last implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

### Nurlybek Orynbassar

GitHub userid: noxfurry

#### Contributions

Rewrote the "Whitespace" section of `testing_approach.md` after the instructor's comments as rules for comparing whitespace in two HTML strings, revised it over three LLM review rounds, and implemented it as the HTML comparison helper in `tests/common/html_compare.rs` with its tests, revised over two LLM review rounds.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 5 | 8 | Form feed is text, not collapsible whitespace; a final line feed in `<pre>` is not rendered; whitespace between table parts is removed; a collapsed run keeps its first space; `&#13;` stays a carriage return; attribute order ignored; citations fixed |
| 6 | 6 | An empty code block is not equal to one holding a blank line; form feed written as `\u{c}`; a line also ends beside a block child (`<li>a\n<ul>`); comments and form feeds between blocks are skipped; lossless round trip includes the standard deviations; a remaining difference is a deviation or a bug |
| 7 | 4 | No errors; the table example in rule 3 is wrapped in `<table>`, which the parser needs to keep its rows and cells; the nested-list helper test names both strings; two trivial findings left as they are |

#### Last specification review

Claude Opus 5.5 (Claude Code), with the prompt: "Review the section named "Whitespace" in testing_approach.md, writing findings to review-round7.md in the repo root. Verify each claim about HTML parsing (html5ever, which the crate already depends on) and browser rendering by running throwaway code or citing the WHATWG HTML, Infra, CSS Text and CSS 2 specifications, and do not commit it. Do not edit testing_approach.md." The prompt went on to say that this is round 7, to check that the round-6 findings were applied, to report only findings which would mislead a test writer or the implementer of the comparison helper and to label smaller ones trivial, and to give the format of the review. This is the output of round 7. O1 and T3 were applied. Strikethrough marks T1 and T2, which I deemed irrelevant: the review itself finds that both statements are correct as written.

> Review of "Whitespace" in testing_approach.md (round 7)
> ======================================================
>
> This reviews the section as of the working tree on `standardize-testing-2`
> (after 6f1360b; the round-6 changes are in 7b9918f), lines 53-94 of
> `testing_approach.md`.
>
> Method. Parsing claims were checked with a throwaway crate outside the repo
> (since deleted) which parsed each string with `html5ever::parse_document`
> (0.39, into `markup5ever_rcdom`, as `src/lib.rs` does) and printed the whole
> DOM, text nodes as Rust string literals. Round trips used htmd in
> `TranslationMode::Faithful` and pulldown-cmark 0.13.4 (`push_html`, with
> `ENABLE_TABLES`). No browser was used this round: every rendering claim which
> changed in 7b9918f was measured in round 6 (Chromium of the Claude desktop
> app's browser pane, Chrome 156), and those measurements are cited below.
> Citations were checked against the WHATWG HTML and Infra specifications, CSS
> Text 3 and CSS 2.
>
> Summary
> -------
>
> All six round-6 findings were applied correctly: the trailing-line-feed
> exception now needs a character before the line feed and has the empty code
> block as a counter-example (E1), the form feed is written `\u{c}` (E2),
> rule 2 ends a line before and after a block child (O1), comments are skipped
> between blocks (O2), rule 3 removes a whitespace-only node with a form feed
> and line 57 says so (O3), and "Rules for tests" items 2 and 5 match "Test
> helpers" and "Known deviations" (O4). None of these changes introduced an
> error.
>
> No errors were found. One inaccuracy remains, older than round 6: rule 3's
> table example parses to plain text, because the parser drops `<tr>` and
> `<td>` outside a `<table>`, so a helper test built from it does not test
> tables (O1). Three trivial points follow.
>
> By the stopping criterion, the section is done once O1 is fixed.
>
> Conflicts in other sections are listed at the end and are not counted.
>
> Errors
> ------
>
> None.
>
> Omissions and inaccuracies
> --------------------------
>
> ### O1. Rule 3's table example loses its table tags when parsed
>
> Rule 3 (line 69): "`<tr>\n<td>a</td>\n</tr>` [is equivalent] to
> `<tr><td>a</td></tr>`." The comparison parses with `parse_document`, and in
> the "in body" insertion mode a `<tr>` or `<td>` start tag is a parse error and
> is ignored (WHATWG HTML,
> [in body](https://html.spec.whatwg.org/multipage/parsing.html#parsing-main-inbody):
> "A start tag whose tag name is one of: ... "tbody", "td", "tfoot", "th",
> "thead", "tr"" - "Parse error. Ignore the token."). html5ever does this:
>
> | HTML                                     | Parses to (`<body>`) |
> | ---------------------------------------- | -------------------- |
> | `<tr>\n<td>a</td>\n</tr>`                | `"\na\n"`            |
> | `<tr><td>a</td></tr>`                    | `"a"`                |
> | `<table><tr>\n<td>a</td>\n</tr></table>` | `<table><tbody><tr>"\n"<td>"a"</td>"\n"</tr></tbody></table>` |
>
> So the stated equivalence holds, but only through rule 2 (whitespace at the
> edges of `<body>`); no table part is involved. "Rules for tests" item 3 asks
> for a pair per rule, and the obvious pair for the table half of rule 3 is this
> example. That test passes with a helper which has no table handling at all,
> and the helper then fails on every pulldown-cmark table, which has
> `<tbody>\n<tr>` and `</tr>\n</tbody>`. (`<li>a\n<ul>` in rule 2 is fine: `<li>`
> is allowed in `<body>` and parses as written.)
>
> Suggested change. Rule 3: "... and `<table>\n<tr>\n<td>a</td>\n</tr>\n</table>`
> to `<table><tr><td>a</td></tr></table>`." Round 6 measured this pair in Chrome
> (13.10 x 24.50 both), and it parses with the line feeds inside `<tbody>` and
> `<tr>`, where only rule 3 can remove them.
>
> Trivial
> -------
>
> ### ~~T1. `&#13;` is "not equivalent to a line feed" only in preformatted text~~
>
> ~~Line 63: "`&#13;` stays a carriage return and is not equivalent to a line~~
> ~~feed." That is a statement about parsing, and it is true (`"a&#13;b"` parses~~
> ~~to `"a\rb"`). In `white-space: normal` text, rule 1 still makes the carriage~~
> ~~return equivalent to a space or line feed, since line 57 counts it as~~
> ~~whitespace; htmd's trip of `<p>a&#13;b</p>` is `<p>a b</p>\n`, which compares~~
> ~~equal. A helper-test writer could read line 63 as "`<p>a&#13;b</p>` must not~~
> ~~equal `<p>a\nb</p>`", but no test asks for that pair. Suggested change, if~~
> ~~any: "... so `&#13;` stays a carriage return: in a `<pre>` it is not~~
> ~~equivalent to a line feed."~~
>
> ### ~~T2. Rule 2 at a line edge can move whitespace across an element boundary~~
>
> ~~Rule 2 makes `<p><em> a</em></p>` and `<p> <em>a</em></p>` both equivalent to~~
> ~~`<p><em>a</em></p>`, and so to each other, though "Where whitespace matters"~~
> ~~item 3 says "A trip which moves whitespace into or out of an element is a~~
> ~~difference". Both are right (rule 2 removes the space before item 3 applies,~~
> ~~and a removed space is not rendered under any decoration), and the helper~~
> ~~follows the order "normalize, then compare". Suggested change, if any: begin~~
> ~~item 3 with "Where the whitespace is kept, ...".~~
>
> ### T3. Rule 2's helper test names one string
>
> "Rules for tests" item 3: "rule 2 gets `<li>a\n<ul>`". The other string,
> `<li>a<ul>`, is in rule 2 itself, so a reader will find it. Suggested change,
> if any: "rule 2 gets `<li>a\n<ul>` against `<li>a<ul>`".
>
> Verified as stated
> ------------------
>
> 1. **Round-6 findings applied.** E1 (line 79, and the counter-example in line
>    92), E2 (lines 92 and 93), O1 (line 68, and the pair in line 92), O2 (line
>    69), O3 (lines 57 and 69, and the pair in line 92) and O4 (lines 81, 91 and
>    94) are in the text as suggested.
> 2. **The trailing-line-feed exception (line 79).** With the new condition,
>    `<pre><code>a\n</code></pre>` normalizes to `<pre><code>a</code></pre>` and
>    `<pre><code>\n</code></pre>` does not normalize to `<pre><code></code></pre>`,
>    matching the round-6 Chrome sizes (7.83 x 41 for both of the first pair;
>    0 x 41 against 0 x 13) and CSS 2
>    [§9.4.2](https://www.w3.org/TR/CSS2/visuren.html#inline-formatting). htmd's
>    trip of an empty code block (`<pre><code>\n</code></pre>\n`) is now
>    reported as a difference, as it should be. `<pre>\n\n</pre>` parses to
>    `<pre>"\n"</pre>` and is correctly not equivalent to `<pre></pre>`.
> 3. **Comments between blocks (line 69).** Re-checked with htmd's trips:
>    `<p>a</p><!-- x --><p>b</p>` gives
>    `<p>a</p>\n<!-- x -->\n<p>b</p>\n`, and two adjacent comments give
>    `<p>a</p>\n<!-- x -->\n<!-- y -->\n<p>b</p>\n`; skipping comments, every
>    `"\n"` lies between two blocks. A comment at the end of a block container
>    (`<p>a</p><!-- x -->` gives `<p>a</p>\n<!-- x -->`; in a blockquote,
>    `<blockquote>\n<p>a</p>\n<!-- x --></blockquote>`) leaves a line feed
>    directly after a block, which rule 2 removes as whitespace at the start of
>    a line. A comment at the very start goes before `<html>` on both sides
>    (initial insertion mode), and the line feed after it is dropped by the
>    parser. Chrome renders the comment cases as without the line feeds (round
>    6: 8 x 85).
> 4. **Form feed (lines 57 and 69, pair in line 92).** `<p>a</p>\u{c}<p>b</p>`
>    parses to a text node `"\u{c}"` between the paragraphs, which rule 3 now
>    removes; Chrome renders it as `<p>a</p><p>b</p>` (round 6: 8 x 85), and
>    `a\u{c}b` wider than `a b` (round 6), so the two pairs in line 92 are right.
>    `\u{c}` is a valid Rust escape.
> 5. **Rule 2 and nested lists (line 68).** `<li>a\n<ul>` parses to
>    `<li>"a\n"<ul></ul></li>`, and the line feed ends a line before the block
>    child, as stated; Chrome renders the two the same (round 6: 88 x 69).
>    pulldown-cmark's other block whitespace (`<ul>\n<li>`, `<li>\n<p>`,
>    `<p>a</p>\n<table>`, `<tbody>\n<tr>`, trailing `\n` in `<body>`) is removed
>    by rules 2 and 3.
> 6. **Parser equivalences (line 63).** `"a\r\nb\rc"` parses to `"a\nb\nc"`,
>    `&#10;` to a line feed and `&#13;` to `"\r"`; the leading line feed after
>    `<pre>`, `<listing>` and `<textarea>` is dropped by html5ever (line 78), as
>    the WHATWG parser specifies.
> 7. **Rules for tests item 2 (line 91).** htmd's trip of `<b>a</b>` is
>    `<p><strong>a</strong></p>\n`, which compares equal to the stated form.
> 8. **Unchanged claims.** Lines 57 (Infra, CSS Text §4.1), 67 (rule 1), 73
>    (CSS Text §4.1.3), 77 (HTML rendering: `pre, listing, plaintext, xmp` with
>    `white-space: pre`, `textarea` with `pre-wrap`), 80-86 are as verified in
>    round 6 and were not changed.
>
> Conflicts in other sections (not counted)
> -----------------------------------------
>
> * **Contributions, line 10** still credits the "Escaping" section, which the
>   document does not have, and line 8 is still an empty entry (both reported
>   in round 6). The other sections (Properties, Test helpers, Known
>   deviations, Migration) agree with the Whitespace section: property 2 and
>   line 25 refer to it, line 31 matches "Rules for tests" item 2, the
>   deviation/bug split matches line 81 and item 5, and Migration starts with
>   the helper and its tests, as item 3 requires.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 6 | Whitespace-only text in `<head>` removed; rule 3 judges a run of whitespace and comments by what surrounds it; form-feed tests added so rule 3 is actually exercised; a redundant check removed and a doc comment fixed; doctype handling left as it is |
| 2 | 3 | No errors; tests added for comments kept between blocks and a form feed alone in its parent; the `<head>` test made independent of htmd; a doc comment updated |

#### Last implementation review

Claude Opus 5.5 (Claude Code), with the prompt: "Review the implementation of the "Whitespace" section of testing_approach.md in tests/common/html_compare.rs and tests/html_compare_tests.rs, writing findings to impl-review-round2.md in the repo root. Check that the code does what the section specifies, and verify each finding by running throwaway tests, and do not commit them. Do not edit any file in the repo except impl-review-round2.md." The prompt went on to say that this is round 2, to check the fixes for round 1, to report only findings which would make a test pass or fail wrongly or mislead a reader and to label smaller ones trivial, and to give the format of the review. This is the output of round 2; all its findings were applied.

> Implementation review, round 2: HTML comparison helper
> ======================================================
>
> Scope: `tests/common/html_compare.rs` and `tests/html_compare_tests.rs` against the "Whitespace" section of `testing_approach.md`, after round 1's items 1-5 were applied. Item 6 (ProcessingInstruction / doctype name) was left on purpose and is not raised again.
>
> Summary
> -------
>
> The round 1 fixes are correct and introduced no new errors. I found no errors and no omissions or inaccuracies which would make a test pass or fail wrongly or mislead a reader. Three trivial items remain: two branches of rule 3 have no test, the `<head>` test sits under the rule 3 heading and depends on htmd, and the `Kind::Hidden` doc comment no longer covers `<head>`.
>
> `cargo build`, `cargo clippy -- --deny warnings`, `cargo clippy --all-targets -- --deny warnings`, `rustfmt --check` on the two files and `cargo test` all pass. All 36 helper tests pass. The files use LF like the other files in `tests/`, and `tests/common/mod.rs` only gains `pub mod html_compare;`.
>
> I checked every finding with throwaway code in a scratch crate outside the repo. The crate loaded copies of `html_compare.rs` through `#[path]`: the current file and five mutants, each with one piece of rule 3 or the `<head>` fix switched off. It ran the repo's test bodies against each copy, plus about 45 extra pairs and 12 real round trips (htmd Faithful, pulldown-cmark 0.13). Nothing in the repo was changed except this file.
>
> Errors
> ------
>
> None.
>
> Omissions and inaccuracies
> --------------------------
>
> None.
>
> Trivial
> -------
>
> ### 1. Two branches of rule 3 have no test
>
> Two mutants of `remove_whitespace_between_blocks` pass all 36 tests:
>
> | Mutant | Wrong result it gives | Current result |
> |---|---|---|
> | `remove[i] = true` instead of `remove[i] = is_whitespace_text(&nodes[i])`, so a run loses its comments too | `<p>a</p>\n<!--x-->\n<p>b</p>` ≡ `<p>a</p><p>b</p>` | ≢ (correct, item 4) |
> | `(before, after) != (Side::Edge, Side::Edge)` removed, so whitespace alone in its parent is dropped | `<div>\u{c}</div>` ≡ `<div></div>`, and `<p>\u{c}</p>` ≡ `<p></p>` | ≢ (correct: no block or table part is next to the form feed) |
>
> The code is correct today. The tests just wouldn't catch either regression. The first matters more: a helper that dropped comments between blocks would let a round trip that loses a comment pass.
>
> Suggested change: add `assert_different("<p>a</p>\n<!--x-->\n<p>b</p>", "<p>a</p><p>b</p>")` and `assert_different("<div>\u{c}</div>", "<div></div>")`.
>
> ### 2. The `<head>` test sits under the rule 3 heading and depends on htmd
>
> `whitespace_in_head_is_removed` is under `// "Comparison", rule 3.`, but removing whitespace in `<head>` is not rule 3. It follows from the section's general statement that only unrendered differences are removed. The test also builds its input with `common::round_trip`, so a change in how htmd writes `<style>` would fail it even if the helper is correct. The other round-trip tests have their own heading at the end of the file.
>
> Suggested change: test the helper with a fixed pair, `assert_equivalent("<style>p{}</style>\n<p>a</p>", "<style>p{}</style><p>a</p>")`, under its own short heading. If you keep the round trip, move it to "Real round trips through htmd and pulldown-cmark".
>
> ### 3. The `Kind::Hidden` doc comment no longer covers `<head>`
>
> `Kind::Hidden` is documented as "`display: none`; its content is compared exactly", and `"head"` is one of its elements. Whitespace-only text in `<head>` is now removed (lines 256-259), so that sentence is not quite true for `<head>`. The comment at line 257 explains the exception where it happens, so a reader who reads on is not misled.
>
> Suggested change: "`display: none`; its content is compared exactly, except whitespace-only text in `<head>`." Or leave it as it is.
>
> Verified as stated
> ------------------
>
> * Round 1 item 1 (`<head>`):
>   * `<script>x</script>\n<p>a</p>`, `<link rel=x>\n<p>a</p>` and `<title>a</title>\n<p>a</p>` ≡ the same strings without the `\n`.
>   * `<style>p{}</style>\u{c}<p>a</p>` ≡ `<style>p{}</style><p>a</p>`.
>   * `<style>p{}</style>\n<!--c-->\n<p>a</p>` ≡ `<style>p{}</style><!--c--><p>a</p>`. The comment stays in `<head>` and is still compared.
>   * Real round trips of `<script>let x = 1;</script><p>a</p>`, `<style>p{}</style><p>a</p>` and `<script>x</script><!-- c --><p>a</p>` compare equal to their input.
>   * Mutation: with the `head` branch disabled, only `whitespace_in_head_is_removed` fails, so the fix is tested.
> * Round 1 item 2 (comment runs):
>   * `<p>a</p>\u{c}<!--x-->\u{c}<!--y-->\u{c}<p>b</p>` ≡ `<p>a</p><!--x--><!--y--><p>b</p>`.
>   * `<div>\u{c}<!--x-->\u{c}<p>a</p></div>` and `<div><p>a</p>\u{c}<!--x-->\u{c}</div>` ≡ their forms without the form feeds (edge on one side, block on the other).
>   * `<p>a</p>\u{c}<!--x-->\u{c}b` ≢ `<p>a</p><!--x-->b`, because the run is next to inline content.
>   * Comments in a run are never removed: `<p>a</p>\n<!--x-->\n<p>b</p>` ≢ `<p>a</p><p>b</p>`.
> * Round 1 item 3 (tests exercise rule 3):
>   * With the call to `remove_whitespace_between_blocks` removed, four tests fail: `form_feed_between_blocks_is_removed`, `form_feed_between_table_parts_is_removed`, `comment_between_blocks_with_whitespace_on_both_sides` and `whitespace_in_head_is_removed`.
>   * With comments no longer skipped, `comment_between_blocks_with_whitespace_on_both_sides` fails.
> * Round 1 items 4 and 5:
>   * `Kind::Preformatted` now appears only in `kind` and `collapse_inline_whitespace`.
>   * The `write_nodes` doc comment ("empty text nodes dropped and adjacent text nodes merged") is accurate.
> * No regressions in the rules round 1 verified:
>   * Rule 1: `<p>a <!--x--> b</p>` ≡ `<p>a <!--x-->b</p>` and ≢ `<p>a<!--x--> b</p>`. `<p>a <em> </em>b</p>` ≡ `<p>a <em></em>b</p>`.
>   * Rule 2: `<p>a <br> b</p>` ≡ `<p>a<br>b</p>`. `<li>a\n<ul><li>b</li></ul></li>` ≡ `<li>a<ul><li>b</li></ul></li>`. `<blockquote>\n<p>a</p>\n</blockquote>\n` ≡ `<blockquote><p>a</p></blockquote>`.
>   * Rule 3: `<ul>`/`<li>` and `<dl>`/`<dt>`/`<dd>` with form feeds between them compare equal to their forms without. `<p>a</p>\u{c}b` and `<p>a</p>\u{c}<img src=x>` stay different, because the form feed is next to inline content.
>   * Item 1: `<pre>a\n</pre>`, `<pre>a\r\n</pre>`, `<pre><code>a\n</code></pre>\n` and `<textarea>a\n</textarea>` ≡ their forms without the final line feed. `<pre><code>\n</code></pre>` ≢ `<pre><code></code></pre>`. `<pre>\u{c}<div>a</div>\u{c}</pre>` keeps its form feeds.
>   * Item 3: `<p>a<em> </em>b</p>` ≢ `<p>a <em></em>b</p>`. A space after `<img>` or before `<textarea>` is kept.
> * Real round trips compare equal to their input, with the standard deviations: a comment before or between paragraphs, `<div>` then `<p>`, a script between paragraphs, a pipe table with `ENABLE_TABLES`, a loose `<ol start="3">`, a list in a blockquote, `<br>`, and a heading followed by a fenced code block with a language.
> * Within the letter of the section:
>   * A form feed next to a `<script>` in the body (`<p>a</p>\u{c}<script>x</script>\u{c}<p>b</p>`) is kept, since `<script>` is not a block or table part.
>   * A form feed between an explicit `</head>` and `<body>` is also kept.
>   * Neither occurs in a pulldown-cmark round trip, so I don't report them.

### Aidyn Tazhibay

GitHub userid: (Aidyn Tazhibay's GitHub userid)

#### Contributions

(Aidyn Tazhibay's contributions)

#### Specification review rounds

(Table showing spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

#### Implementation review rounds

(Table showing implementation review rounds)

#### Last implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)