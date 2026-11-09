Project 2 submission
====================

Team members are Armanzhan Onggarbay, Nurlybek Orynbassar, Miras Shaltayev, and Aidyn Tazhibay. The specification consists of the file [testing_approach.md](testing_approach.md).

Draft submission
----------------

### Contributions

* Armanzhan Onggarbay: (Armanzhan's contributions)
* Nurlybek Orynbassar: Wrote the "Whitespace" section of `testing_approach.md` and revised it over four LLM review rounds.
* Miras Shaltayev: Wrote the initial draft of `testing_approach.md`.
* Aidyn Tazhibay: (Aidyn's contributions)

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

### Last LLM review

#### Whitespace

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