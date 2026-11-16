Testing approach
================

Contributions
-------------

* Miras Shaltayev: wrote the initial draft of this document.
* Armanzhan Onggarbay: 
* Nurlybek Orynbassar: wrote the "Whitespace" section.
* Aidyn Tazhibay: wrote the "Test helpers" section.

Approach
--------

This approach covers `TranslationMode::Faithful` only. Today most tests check only the Markdown which htmd writes, so a test can pass while the Markdown means something else to pulldown-cmark. Each test should also check the round trip: pulldown-cmark renders the Markdown back to HTML.

### Properties

For an HTML input `h`, with one set of htmd and pulldown-cmark options:

1. **Expected Markdown**: `convert_faithful(h)` equals the Markdown in the test.
2. **Round trip**: `h2 = render_markdown(convert_faithful(h))` is equivalent to the HTML which the test expects, as the "Whitespace" section defines.
3. **Idempotency**: `convert_faithful(h2)` equals the Markdown of property 1.

Markdown is compared exactly. Only HTML is compared with the "Whitespace" rules.

### Test helpers

Four helpers in `tests/common/mod.rs` check the three properties of one case. Each converts with `convert_faithful_options` and renders with `render_markdown`. `html` and the expected strings are `&str`, `htmd_options` is `&htmd::options::Options` (used for both conversions; its `translation_mode` is ignored, since the conversion is always Faithful), and `cmark_options` is `pulldown_cmark::Options`.

| Helper | Arguments |
| ------ | --------- |
| `assert_faithful` | `html, expected_markdown, expected_round_trip` |
| `assert_faithful_options` | `html, htmd_options, cmark_options, expected_markdown, expected_round_trip` |
| `assert_faithful_unstable` | `html, expected_markdown, expected_round_trip, expected_second_markdown` |
| `assert_faithful_unstable_options` | `html, htmd_options, cmark_options, expected_markdown, expected_round_trip, expected_second_markdown` |

**Options.** The helpers without `_options` use htmd's `Options::default()` and `CMARK_OPTIONS`, a constant in `tests/common/mod.rs` which names six pulldown-cmark extensions, joined with `union` since `|` is not `const`: `ENABLE_TABLES`, `ENABLE_MATH`, `ENABLE_FOOTNOTES`, `ENABLE_STRIKETHROUGH`, `ENABLE_TASKLISTS` and `ENABLE_GFM`. `translation_approach.md` covers tables and math, and the other four are widely used. htmd's output should trigger none of them, so a round trip which does is a bug (`<p>~~b~~</p>` reads back as `<del>b</del>`). The rest of `Options::all()` is left out, because it changes text which htmd does not escape: smart punctuation gives `don't` a curly apostrophe, and heading attributes, definition lists, superscript, subscript, wikilinks and metadata blocks read ordinary text as markup. Old-style footnotes are left out too, since they change the footnote syntax of `ENABLE_FOOTNOTES`. The constant is spelled out rather than derived from `Options::all()`, so a new pulldown-cmark flag is not added without review. A case uses an `_options` helper only for other htmd options (for example `HeadingStyle::Setex`), or to test an extension outside `CMARK_OPTIONS`. `round_trip` renders with no extensions, so an existing round trip of text which these six extensions read may change when its test moves to these helpers.

**Order of checks.** Each helper stops at the first failure. Its panic message starts with the property's name (`property 2:`, `property 1:`, `property 3:`) and shows the input HTML, the Markdown, and the two values it compared. A property 2 message also shows the diff of the two normalized trees (`normalize_html` in the "Whitespace" section), and a property 3 message also shows the HTML which was converted.

0. The `_unstable` helpers first check, before any conversion, that `expected_second_markdown` differs from `expected_markdown`, and panic with `unstable helper misused:` if it does not: such a case belongs in a stable helper.
1. Convert `html` and render the Markdown. The conversion cannot fail today, so this is only an `expect`.
2. Property 2: the rendered HTML is equivalent to `expected_round_trip` (`html_equivalent`). This comes first, so a broken translation shows the harm it does (a paragraph turned into a heading) rather than a different string.
3. Property 1: the Markdown equals `expected_markdown` exactly.
4. Property 3, by converting the rendered HTML with the same htmd options:
   1. The stable helpers expect `expected_markdown` again.
   2. The `_unstable` helpers expect `expected_second_markdown`.
   3. The `_unstable` helpers then render `expected_second_markdown` with the same pulldown-cmark options, convert it with the same htmd options, and expect `expected_second_markdown` again.

   For an unstable case, property 3 therefore means: the second conversion gives `expected_second_markdown`, and that Markdown is stable.

When the trip loses nothing, the expected round trip is the input HTML with the standard deviations below.

**Tests of the helpers.** `tests/test_helpers_tests.rs` holds, for each helper, one case which passes and one `#[should_panic(expected = ...)]` case for each check which that helper makes: check 0 expects `unstable helper misused:`, check 2 `property 2:`, check 3 `property 1:`, and check 4 `property 3:`. Each case makes only the expected value of its own check wrong. The unstable input is `<p><code>a\nb</code></p>`: its Markdown is `<code>a b</code>` and its second Markdown is `` `a b` ``. The stable helpers' check 4 case passes this input; the check 0 case passes a stable input (`<p>a</p>`, both Markdown strings `a`), so that only check 0 can fail. Check 4.3 gets a `#[should_panic]` case only once an input is known whose second Markdown is unstable. The helpers carry `#[allow(dead_code)]`, since not every test binary calls each one. `render_markdown` stays; `round_trip` stays until the migration removes its last caller.

### Known deviations

**Standard deviations** need no reason in a test: `<b>` and `<i>` come back as `<strong>` and `<em>`; inline content with no block element inside comes back inside `<p>` at the root and in a blockquote; a header row comes back in `<thead>`; a link destination comes back percent-encoded.

**Other deviations.** Some HTML is not converted exactly by design. The test then expects the lossy round trip (and the second Markdown, if it differs), and gives a one-line reason which names the section of `translation_approach.md` that explains the loss. If no section explains it, it is a bug.

**Bugs.** An output which contradicts `translation_approach.md` or CommonMark is an htmd bug. Fix htmd, or put the case in its own `#[ignore]` test with the correct expectation and a comment which names the issue.

A test never skips a property without a reason.

### Migration

First write the HTML comparison helper and its tests, then the test helpers. Then convert the test files one at a time: `basic_tests.rs` and `link_tests.rs` (which already round-trip in part), then `table_tests.rs`, `list_tests.rs`, `code_tests.rs` and `context_tests.rs`, then `translation_approach_tests.rs` and `turndown_cases_tests.rs`. For each test:

1. Find its Faithful assertions on one input: the Markdown and any round trip. Leave other modes unchanged.
2. Replace them with one helper call. Write both expected strings by hand from `translation_approach.md` and the rules above, never by copying htmd's output.
3. Run the test. If it fails, fix the expectation only if it contradicts `translation_approach.md` or CommonMark. Otherwise record a deviation or a bug as above.

Tests with custom handlers (`event_tests.rs`) and tests in another mode (`turndown_online_demo_tests.rs`) are not converted.

### Whitespace

This section defines how a test compares whitespace in two HTML strings: the expected round trip and the HTML which pulldown-cmark renders (property 2). Markdown is always compared exactly, whitespace included (properties 1 and 3).

HTML whitespace is [ASCII whitespace](https://developer.mozilla.org/en-US/docs/Glossary/Whitespace): U+0020 space, tab, line feed, form feed and carriage return ([Infra](https://infra.spec.whatwg.org/#ascii-whitespace)). A browser collapses only space, tab, line feed and carriage return ([CSS Text §4.1](https://www.w3.org/TR/css-text-3/#white-space-rules)), and renders a form feed as a visible character. So "whitespace" below means those four, except in rule 3. Elsewhere a form feed, like U+00A0 and U+2003, is text and is compared exactly, and a trip which turns a form feed into a space is a difference.

Two HTML strings are equivalent when they parse to the same DOM after the normalization below. The normalization removes only differences which a browser with its default style sheet does not render, following [Handling whitespace](https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Text/Whitespace). A few differences which a browser does not render are still kept, as listed under "Where whitespace matters".

#### Comparison

The comparison helper parses both strings with html5ever, as htmd does, and compares the two DOMs after the normalization below. Parsing already makes these equivalent: `\r\n`, `\r` and `\n` (the parser normalizes line endings), a character reference and the character it stands for (`&#10;` and a line feed), and the spelling of tags and attributes. A character reference is decoded after line endings are normalized, so `&#13;` stays a carriage return and is not equivalent to a line feed. Attributes are compared as a set, ignoring their order.

In text rendered with `white-space: normal`, which is everything outside the elements listed under "Where whitespace matters":

1. A run of whitespace is equivalent to one space, which stays where the run starts ([CSS Text §4.1.1](https://www.w3.org/TR/css-text-3/#white-space-phase-1)). This holds across the boundaries of inline elements: `a <em> b</em>` is equivalent to `a <em>b</em>`, and `<em>a </em> b` to `<em>a </em>b`, not to `<em>a</em> b`.
2. Whitespace at the start or end of a line is removed ([CSS Text §4.1.2](https://www.w3.org/TR/css-text-3/#white-space-phase-2)). A line starts and ends at the edges of a block container (`<p>`, `<h1>`–`<h6>`, `<li>`, `<blockquote>`, `<td>`, `<th>` and the other elements which the default style sheet displays as blocks or table cells), before and after a block child (so `<li>a\n<ul>` is equivalent to `<li>a<ul>`), and at a `<br>`. So `<p> a </p>` is equivalent to `<p>a</p>`, and `a <br> b` to `a<br>b`. This also applies inside an inline element at the edge of a line: `<p><em> a</em></p>` is equivalent to `<p><em>a</em></p>`.
3. A text node holding only ASCII whitespace, form feed included, is removed when it lies between two block elements, between two parts of a table (`<table>`, `<caption>`, `<thead>`, `<tbody>`, `<tfoot>`, `<tr>`, `<td>`, `<th>`), or between one of these and the edge of its parent ([CSS 2 §17.2.1](https://www.w3.org/TR/CSS2/tables.html#anonymous-boxes)). So `<p>a</p>\n<p>b</p>\n` is equivalent to `<p>a</p><p>b</p>`, and `<table>\n<tr>\n<td>a</td>\n</tr>\n</table>` to `<table><tr><td>a</td></tr></table>`. A browser does not render a form feed here either. Comments are skipped when deciding what lies between two blocks, so `<p>a</p>\n<!-- x -->\n<p>b</p>` is equivalent to `<p>a</p><!-- x --><p>b</p>`.

Everything else is compared exactly: element names, attribute values, the order of nodes, and all text other than the whitespace above.

Tests use text in which a line feed renders as a space. Between Chinese or Japanese characters a browser may remove it instead ([CSS Text §4.1.3](https://www.w3.org/TR/css-text-3/#line-break-transform)), which these rules do not model.

#### Where whitespace matters

1. **Preformatted elements.** The default style sheet renders `<pre>`, `<listing>`, `<xmp>` and `<plaintext>` with `white-space: pre` ([HTML rendering, flow content](https://html.spec.whatwg.org/multipage/rendering.html#flow-content-3)), and `<textarea>` with `white-space: pre-wrap` ([HTML rendering, form controls](https://html.spec.whatwg.org/multipage/rendering.html#form-controls)). Their text, and the text of any element inside a `<pre>` or `<listing>`, is compared exactly, with two exceptions:
   * The parser drops a single line feed directly after the start tag of `<pre>`, `<listing>` or `<textarea>`, so `<pre>\na</pre>` is equivalent to `<pre>a</pre>`, but `<pre>\n\na</pre>` is not.
   * A single line feed at the end of the content, after a character other than a line feed, is not rendered, since the empty line after it has no height ([CSS 2 §9.4.2](https://www.w3.org/TR/CSS2/visuren.html#inline-formatting)). So `<pre>a\n</pre>` is equivalent to `<pre>a</pre>`, and `<pre><code>a\n</code></pre>` to `<pre><code>a</code></pre>`, but `<pre>a\n\n</pre>` is not, and neither is `<pre><code>\n</code></pre>` to `<pre><code></code></pre>`, whose line feed adds a line.
2. **Whitespace between inline content.** Whether whitespace is present between two pieces of inline content matters, even though its amount does not: `<em>a</em> <em>b</em>` is not `<em>a</em><em>b</em>`, and `a<code>b</code>` is not `a <code>b</code>`.
3. **Which side of an element boundary the whitespace is on.** A single space keeps its place: `a <em>b</em>` is not `a<em> b</em>`. A browser usually renders the two the same, but the space has moved into the element, which an underline, a background or a link target shows. A trip which moves whitespace into or out of an element is a difference, which "Known deviations" classifies as a deviation or a bug.
4. **Attribute values, comments, `<script>` and `<style>`.** These are compared exactly, although a comment is never rendered and some whitespace in an attribute value (such as a leading space in an `href`) does not change rendering: a trip should keep them as they were.

`<code>` is not preformatted: its default style is `white-space: normal`, so the rules under "Comparison" apply to it.

The comparison assumes the default style sheet. Tests do not use a `style` attribute or a `<style>` element which changes `white-space`.

#### Rules for tests

1. Write expected Markdown with the exact whitespace htmd should write. Never trim or normalize it to make a test pass.
2. Write the expected round trip in whichever equivalent form reads best. When the trip loses nothing, this is the input HTML with the standard deviations of "Known deviations" (`<b>a</b>` comes back as `<p><strong>a</strong></p>`), so the test shows that the HTML comes back as it went in.
3. The comparison helper has its own tests. Each rule under "Comparison" gets a pair of strings which must compare equal. Each item under "Where whitespace matters" gets a pair which must not, using the examples above, and item 1 also gets its two exceptions as pairs which must. Item 4 uses `<a href="x">` against `<a href=" x">`, and `<!--a-->` against `<!-- a -->`. A form feed gets `<p>a\u{c}b</p>` against `<p>a b</p>`, which must not compare equal, and `<p>a</p>\u{c}<p>b</p>` against `<p>a</p><p>b</p>`, which must. Each exception in item 1 also gets its counter-example, including `<pre><code>\n</code></pre>` against `<pre><code></code></pre>`, and rule 2 gets `<li>a\n<ul><li>b</li></ul></li>` against `<li>a<ul><li>b</li></ul></li>`.
4. Write invisible characters as escapes (`\t`, `\r\n`, `\u{a0}`, `\u{c}` for a form feed), so a reader sees them.
5. A difference which remains after the normalization is a deviation or a bug, recorded as "Known deviations" describes.
