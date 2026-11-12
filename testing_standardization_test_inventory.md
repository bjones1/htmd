Test migration inventory
========================

This is a planning table for all 227 named integration tests in `tests/`.
Section headings count test functions. A function with multiple input-and-mode
cases has one row per case. Labels show a short input expression or fixture
name. Links point to the source assertion or fixture; generated combinations
show their expanded HTML. The `Faithful round trip` column concerns
**Faithful-mode** HTML → Markdown → HTML checks only. A candidate may need a
compact fixture; this table does not claim that the exact comparison of section
3 of `spec.md` has already passed. Keep Pure-mode assertions in mixed tests.

| Round trip            | Meaning                                                                                          |
| --------------------- | ------------------------------------------------------------------------------------------------ |
| Yes                   | Compare the rendered HTML with the input exactly, as section 3 of `spec.md` requires.            |
| Expected              | Use an explicit expected HTML result for a documented transformation, parser repair, or wrapper. |
| Configured            | Use the test's htmd options or the matching pulldown-cmark extension.                            |
| Configured + expected | Both configuration and explicit expected HTML are needed.                                        |
| No                    | The existing test has no suitable Faithful round-trip claim. Keep its current purpose.           |

`Keep Markdown = Yes` means its Markdown output assertion protects syntax,
escaping, fallback, options, or an intentional loss. `No` means the test
currently checks rendered HTML or its simple Markdown assertion could be
replaced once the standardized comparison covers the same behavior. `N/A` means
the test has no Markdown-output assertion; keep its other checks.

`Existing round trip` records current input-derived HTML → Markdown → HTML
checks, including calls through local helpers and explicit rendering with
pulldown-cmark. In a row for a specific input and mode, `Faithful` or `Pure
only` means that input is rendered back in that mode; `No` means it is not. In a
single-input function row, `Faithful`, `Both`, and `Pure only` retain their
function-level meanings. `N/A` marks unit tests and documentation examples
outside the conversion pipeline. Rendering a fixed Markdown literal alone does
not count. Existing checks may compare HTML strings or only selected HTML
features; this column does not claim that they use the proposed standardized
comparison.

Root-level inline content may gain a `<p>` on rendering. Pretty-printed input
may contain whitespace text nodes that htmd does not preserve. Those cases need
a compact fixture or explicit expected HTML; they do not justify a global rule
that ignores wrappers or whitespace. The known `<br><br>` root failure remains a
failure of preservation, even though an existing test records the HTML it
currently renders.

Integration tests
-----------------

### [tests/basic\_tests.rs](tests/basic_tests.rs) (91)

<table style="width: 98.2342%; height: 7339.6px;">
  <thead>
    <tr style="height: 53.2px;">
      <th style="width: 66.6613%;">Test</th>
      <th style="width: 11.2716%;">Faithful round trip</th>
      <th style="width: 11.8525%;">Keep Markdown</th>
      <th style="width: 10.12%;">Existing round trip</th>
    </tr>
  </thead>
  <tbody>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L18"><code>links_with_spaces</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L29"><code>referenced_links_with_title</code></a>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L49"><code>consecutive_referenced_links_with_title</code></a>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L75"><code>images</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L89"><code>images_with_spaces_in_url</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L100"><code>image_title_stays_outside_an_angle_bracket_destination</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L113"><code>headings</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L133"><code>pure_heading_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L134">input 1, Pure</a> <code>"&lt;em&gt;&lt;h2&gt;b&lt;/h2&gt;&lt;/em&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L133"><code>pure_heading_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L137">input 2, Pure</a> <code>r#"&lt;a href="u"&gt;&lt;h2&gt;b&lt;/h2&gt;&lt;/a&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L133"><code>pure_heading_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L141">input 3, Pure</a> <code>"&lt;table&gt;&lt;thead&gt;&lt;tr&gt;&lt;th&gt;&lt;h2&gt;b&lt;/h2&gt;&lt;/th&gt;&lt;/tr&gt;&lt;/thead&gt;&lt;/table&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L133"><code>pure_heading_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L147">input 4, Pure</a> <code>"&lt;h1&gt;&lt;div&gt;&lt;h2&gt; a # &lt;/h2&gt;&lt;/div&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L154"><code>heading_content_trims_only_document_whitespace</code></a> / <a href="tests/basic_tests.rs#L157">input 1, Faithful</a> <code>"&lt;h1&gt; \t\r\n a \t\r\n &lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L154"><code>heading_content_trims_only_document_whitespace</code></a> / <a href="tests/basic_tests.rs#L159">input 2, Faithful</a> <code>"&lt;h1&gt;&amp;nbsp;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L154"><code>heading_content_trims_only_document_whitespace</code></a> / <a href="tests/basic_tests.rs#L162">input 3, Faithful, setext</a> <code>"&lt;h1&gt;&amp;nbsp;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L170"><code>setext_underline</code></a> / <a href="tests/basic_tests.rs#L171">input 1, Faithful, setext</a> <code>"&lt;h2&gt;a&lt;/h2&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L170"><code>setext_underline</code></a> / <a href="tests/basic_tests.rs#L172">input 2, Faithful, setext</a> <code>"&lt;h1&gt;é&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L170"><code>setext_underline</code></a> / <a href="tests/basic_tests.rs#L175">input 3, Faithful, setext</a> <code>"&lt;h1&gt;a*b&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L170"><code>setext_underline</code></a> / <a href="tests/basic_tests.rs#L179">input 4, Faithful, setext</a> <code>"&lt;h1&gt;&lt;br&gt;&lt;em&gt;b&lt;/em&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L170"><code>setext_underline</code></a> / <a href="tests/basic_tests.rs#L183">input 5, Faithful, setext</a> <code>"&lt;h1&gt;&lt;em&gt;a&lt;/em&gt;&lt;br&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L170"><code>setext_underline</code></a> / <a href="tests/basic_tests.rs#L185">input 6, Faithful, setext</a> <code>"&lt;h3&gt;a&lt;/h3&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L189"><code>paragraphs</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L201"><code>quotes</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L209"><code>br</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L216"><code>hr</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L222"><code>strong_italic</code></a> / <a href="tests/basic_tests.rs#L226">input 1, Faithful</a> <code>r#"&lt;i&gt;Italic&lt;/i&gt;&lt;em&gt;Also italic&lt;/em&gt;&lt;strong&gt;Strong&lt;/strong&gt;&lt;b&gt;Stronger&lt;/b&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L222"><code>strong_italic</code></a> / <a href="tests/basic_tests.rs#L230">input 2, Pure</a> <code>r#"&lt;i&gt;Italic&lt;/i&gt;&lt;em&gt;Also italic&lt;/em&gt;&lt;strong&gt;Strong&lt;/strong&gt;&lt;b&gt;Stronger&lt;/b&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L239"><code>faithful_mode_keeps_adjacent_emphasis_apart</code></a> / <a href="tests/basic_tests.rs#L242">input 1, Faithful</a> <code>"&lt;p&gt;&lt;em&gt;a&lt;/em&gt;&lt;em&gt;b&lt;/em&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L239"><code>faithful_mode_keeps_adjacent_emphasis_apart</code></a> / <a href="tests/basic_tests.rs#L246">input 2, Faithful</a> <code>"&lt;p&gt;&lt;i&gt;a&lt;/i&gt;&lt;em&gt;b&lt;/em&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L239"><code>faithful_mode_keeps_adjacent_emphasis_apart</code></a> / <a href="tests/basic_tests.rs#L250">input 3, Faithful</a> <code>"&lt;p&gt;&lt;b&gt;a&lt;/b&gt;&lt;strong&gt;b&lt;/strong&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L258"><code>faithful_mode_keeps_adjacent_code_spans_apart</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L268"><code>faithful_mode_keeps_adjacent_untranslated_elements_apart</code></a> / <a href="tests/basic_tests.rs#L271">input 1, Faithful</a> <code>"&lt;p&gt;&lt;sub&gt;a&lt;/sub&gt;&lt;sub&gt;b&lt;/sub&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L268"><code>faithful_mode_keeps_adjacent_untranslated_elements_apart</code></a> / <a href="tests/basic_tests.rs#L275">input 2, Faithful</a> <code>r#"&lt;p&gt;&lt;abbr title="t"&gt;a&lt;/abbr&gt;&lt;abbr title="t"&gt;b&lt;/abbr&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L268"><code>faithful_mode_keeps_adjacent_untranslated_elements_apart</code></a> / <a href="tests/basic_tests.rs#L279">input 3, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="x"&gt;a&lt;/span&gt;&lt;span class="x"&gt;b&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L285"><code>pure_mode_merges_adjacent_inline_elements</code></a> / <a href="tests/basic_tests.rs#L286">input 1, Pure</a> <code>"&lt;p&gt;&lt;em&gt;a&lt;/em&gt;&lt;em&gt;b&lt;/em&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L285"><code>pure_mode_merges_adjacent_inline_elements</code></a> / <a href="tests/basic_tests.rs#L289">input 2, Pure</a> <code>"&lt;p&gt;&lt;sub&gt;a&lt;/sub&gt;&lt;sub&gt;b&lt;/sub&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L299">input 1, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;/span&gt;&lt;span class="math math-inline"&gt;b&lt;/span&gt;&lt;…</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L299">input 2, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;/span&gt;&lt;span class="math math-inline"&gt;b&lt;/span&gt;&lt;…</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L303">input 3, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;a&lt;/span&gt;&lt;span class="math math-display"&gt;b&lt;/span…</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L303">input 4, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;a&lt;/span&gt;&lt;span class="math math-display"&gt;b&lt;/span…</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L307">input 5, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;/span&gt;&lt;span class="math math-display"&gt;b&lt;/span&gt;…</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L307">input 6, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;/span&gt;&lt;span class="math math-display"&gt;b&lt;/span&gt;…</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L311">input 7, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;a&lt;/span&gt;&lt;span class="math math-inline"&gt;b&lt;/span&gt;…</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L297"><code>adjacent_math_spans_stay_math</code></a> / <a href="tests/basic_tests.rs#L311">input 8, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;a&lt;/span&gt;&lt;span class="math math-inline"&gt;b&lt;/span&gt;…</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L331"><code>blank_emphasis_leaves_its_whitespace_behind</code></a> / <a href="tests/basic_tests.rs#L332">input 1, Pure</a> <code>"&lt;p&gt;a&lt;em&gt; &lt;/em&gt;b&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L331"><code>blank_emphasis_leaves_its_whitespace_behind</code></a> / <a href="tests/basic_tests.rs#L333">input 2, Pure</a> <code>"&lt;p&gt;a&lt;strong&gt; &lt;/strong&gt;b&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L331"><code>blank_emphasis_leaves_its_whitespace_behind</code></a> / <a href="tests/basic_tests.rs#L336">input 3, Pure</a> <code>"&lt;p&gt;a&lt;em&gt;&amp;#160;&lt;/em&gt;b&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L331"><code>blank_emphasis_leaves_its_whitespace_behind</code></a> / <a href="tests/basic_tests.rs#L338">input 4, Pure</a> <code>"&lt;p&gt;a&lt;em&gt;&lt;/em&gt;b&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L342"><code>italic_inside_word</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L348"><code>italic_with_br</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L357"><code>inline_raw_html_escaping</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L366"><code>multiline_raw_html_escaping</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L389"><code>inline_email_autolink_escaping</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L408"><code>a_sequence_finished_by_a_sibling_stays_text</code></a> / <a href="tests/basic_tests.rs#L410">input 1, Pure</a> <code>"&lt;p&gt;&amp;lt;&lt;span&gt;div&lt;/span&gt; class&amp;gt;x&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L408"><code>a_sequence_finished_by_a_sibling_stays_text</code></a> / <a href="tests/basic_tests.rs#L415">input 2, Pure</a> <code>"&lt;p&gt;&amp;lt;&lt;a&gt;!--&lt;/a&gt; x --&amp;gt; y&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L408"><code>a_sequence_finished_by_a_sibling_stays_text</code></a> / <a href="tests/basic_tests.rs#L420">input 3, Pure</a> <code>"&lt;p&gt;a&amp;lt;&lt;em&gt;&lt;/em&gt;b&amp;gt;c&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L408"><code>a_sequence_finished_by_a_sibling_stays_text</code></a> / <a href="tests/basic_tests.rs#L421">input 4, Pure</a> <code>"&lt;p&gt;&amp;lt;/&lt;span&gt;p&lt;/span&gt; &amp;gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L408"><code>a_sequence_finished_by_a_sibling_stays_text</code></a> / <a href="tests/basic_tests.rs#L422">input 5, Pure</a> <code>"&lt;p&gt;&amp;lt;&lt;span&gt;http://x.y&lt;/span&gt;&amp;gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L408"><code>a_sequence_finished_by_a_sibling_stays_text</code></a> / <a href="tests/basic_tests.rs#L427">input 6, Pure</a> <code>"&lt;p&gt;&amp;lt;1&lt;span&gt;@b.co&amp;gt;&lt;/span&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L444"><code>html_escaping</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L474"><code>faithful_mode_inline</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L494"><code>faithful_mode_hr</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L502"><code>faithful_mode_blockquote</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L522"><code>faithful_mode_h1</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L533"><code>heading_attributes</code></a> / <a href="tests/basic_tests.rs#L536">input 1, Faithful</a> <code>r#"&lt;h1 class=""&gt;a&lt;/h1&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L533"><code>heading_attributes</code></a> / <a href="tests/basic_tests.rs#L540">input 2, Faithful</a> <code>"&lt;h2 id&gt;a&lt;/h2&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L533"><code>heading_attributes</code></a> / <a href="tests/basic_tests.rs#L542">input 3, Pure</a> <code>r#"&lt;h1 class="x"&gt;a&lt;/h1&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L546"><code>faithful_mode_p</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L554"><code>faithful_mode_ol1</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L575"><code>faithful_mode_ol2</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L592"><code>faithful_mode_comment</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L600"><code>faithful_mode_html</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L623"><code>faithful_mode_table</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 53.2px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L666"><code>faithful_mode_serializes_a_table_with_a_caption</code></a> / <a href="tests/basic_tests.rs#L677">input 1, Faithful</a> <code>concat!( r#"&lt;table&gt;&lt;caption&gt;&lt;span class="label"&gt;Caption&lt;/span&gt;&lt;/caption&gt;"#, "&lt;tr&gt;&lt;th&gt;…</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 53.2px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L666"><code>faithful_mode_serializes_a_table_with_a_caption</code></a> / <a href="tests/basic_tests.rs#L682">input 2, Faithful</a> <code>concat!( r#"&lt;table&gt;&lt;caption&gt;&lt;span class="label"&gt;Caption&lt;/span&gt;&lt;/caption&gt;"#, "&lt;tbody&gt;&lt;…</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L688"><code>pure_mode_writes_a_caption_as_a_paragraph</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L705"><code>pure_mode_separates_a_caption_from_the_table_it_sits_above</code></a> / <a href="tests/basic_tests.rs#L706">input 1, Pure</a> <code>"&lt;ul&gt;&lt;li&gt;a&lt;/li&gt;&lt;/ul&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L705"><code>pure_mode_separates_a_caption_from_the_table_it_sits_above</code></a> / <a href="tests/basic_tests.rs#L706">input 2, Pure</a> <code>"&lt;blockquote&gt;a&lt;/blockquote&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Pure only</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L732"><code>faithful_mode_nested_inline_html</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L740"><code>spaces_check</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L749"><code>consecutive_blocks</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L763"><code>raw_text</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L769"><code>nested_divs</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L783"><code>with_head</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L803"><code>with_custom_rules</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L815"><code>with_custom_rules_and_fallback</code></a> / <a href="tests/basic_tests.rs#L833">input 1, Pure</a> <code>r#"&lt;img src="https://example.com"/&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L815"><code>with_custom_rules_and_fallback</code></a> / <a href="tests/basic_tests.rs#L838">input 2, Pure</a> <code>r#"&lt;img src="https://example.com" id="do_not_skip_me"/&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L843"><code>upper_case_tags</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L852"><code>html_entities</code></a> / <a href="tests/basic_tests.rs#L856">input 1, Faithful</a> <code>r#"&lt;p&gt;&lt;a href="/my%20&amp;amp;uri" title="my%20&amp;amp;title"&gt;my%20&amp;amp;link&lt;/a&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L852"><code>html_entities</code></a> / <a href="tests/basic_tests.rs#L862">input 2, Faithful</a> <code>r#"&lt;p&gt;This &amp;amp; that, then &amp;lt; &amp;gt; now.&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L852"><code>html_entities</code></a> / <a href="tests/basic_tests.rs#L866">input 3, Faithful</a> <code>"&lt;p&gt;&amp;amp;copy; &amp;amp;#169; &amp;amp;#xA9;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L870"><code>scripting_option</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L888"><code>multithreading</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L924"><code>unterminated_html</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L930"><code>misnested_formatting_does_not_duplicate_or_lose_text</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L937"><code>math</code></a> / <a href="tests/basic_tests.rs#L940">input 1, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;x^2&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L937"><code>math</code></a> / <a href="tests/basic_tests.rs#L945">input 2, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;x^2&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L937"><code>math</code></a> / <a href="tests/basic_tests.rs#L951">input 3, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;{a}_1, b_{2}, a*1, b*2, [a](b), 3 &amp;lt;a&amp;gt; b, …</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L971"><code>empty_math_span</code></a> / <a href="tests/basic_tests.rs#L972">input 1, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L971"><code>empty_math_span</code></a> / <a href="tests/basic_tests.rs#L973">input 2, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L971"><code>empty_math_span</code></a> / <a href="tests/basic_tests.rs#L979">input 3, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L986"><code>math_span_whitespace</code></a> / <a href="tests/basic_tests.rs#L989">input 1, Faithful</a> <code>"&lt;p&gt;&lt;span class="math math-inline"&gt; \u{b}x\t&lt;/span&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L986"><code>math_span_whitespace</code></a> / <a href="tests/basic_tests.rs#L991">input 2, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt; x &lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L986"><code>math_span_whitespace</code></a> / <a href="tests/basic_tests.rs#L997">input 3, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a\ &lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L986"><code>math_span_whitespace</code></a> / <a href="tests/basic_tests.rs#L1001">input 4, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-display"&gt;a\ &lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1013">input 1, Faithful</a> <code>"a$b"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1015">input 2, Faithful</a> <code>r"a\$b"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1016">input 3, Faithful</a> <code>r"a"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1018">input 4, Faithful</a> <code>"}a{"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1020">input 5, Faithful</a> <code>"a{"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1021">input 6, Faithful</a> <code>r"\{a"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1028">input 7, Faithful</a> <code>r"a$b"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1028">input 8, Faithful</a> <code>r"a\"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1028">input 9, Faithful</a> <code>"{a}"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1028">input 10, Faithful</a> <code>r"{a"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1011"><code>unwritable_math_span</code></a> / <a href="tests/basic_tests.rs#L1028">input 11, Faithful</a> <code>r"a}"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1040"><code>span_not_of_the_math_form</code></a> / <a href="tests/basic_tests.rs#L1042">input 1, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;em&gt;b&lt;/em&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1040"><code>span_not_of_the_math_form</code></a> / <a href="tests/basic_tests.rs#L1043">input 2, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;em&gt;b&lt;/em&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1040"><code>span_not_of_the_math_form</code></a> / <a href="tests/basic_tests.rs#L1045">input 3, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;!--c--&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1040"><code>span_not_of_the_math_form</code></a> / <a href="tests/basic_tests.rs#L1046">input 4, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline"&gt;a&lt;!--c--&gt;&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1040"><code>span_not_of_the_math_form</code></a> / <a href="tests/basic_tests.rs#L1048">input 5, Faithful</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline" id="x"&gt;a&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1040"><code>span_not_of_the_math_form</code></a> / <a href="tests/basic_tests.rs#L1049">input 6, Pure</a> <code>r#"&lt;p&gt;&lt;span class="math math-inline" id="x"&gt;a&lt;/span&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1055"><code>math_span_in_a_table_cell</code></a>
      </td>
      <td style="width: 11.2716%;">Configured + expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1073"><code>dollar_sign_in_text</code></a> / <a href="tests/basic_tests.rs#L1074">input 1, Faithful</a> <code>"&lt;p&gt;$5,$6&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1073"><code>dollar_sign_in_text</code></a> / <a href="tests/basic_tests.rs#L1075">input 2, Pure</a> <code>"&lt;p&gt;$5,$6&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1080"><code>document_whitespace</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1098"><code>multibyte_ordered_list_escape_half</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1104"><code>multibyte_ordered_list_escape_accented</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1111"><code>multibyte_ordered_list_escape_trademark</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1118"><code>ascii_ordered_list_escape</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1124"><code>multibyte_no_dot</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1131"><code>cjk_before_ordered_list</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1138"><code>multibyte_atx_heading_escape</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1144"><code>multibyte_atx_heading_escape_umlaut</code></a>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1179">input 1, Faithful</a> <code>"&lt;p&gt;-&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1180">input 2, Faithful</a> <code>"&lt;p&gt;*&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1181">input 3, Faithful</a> <code>"&lt;p&gt;+&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1182">input 4, Faithful</a> <code>"&lt;p&gt;---&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1183">input 5, Faithful</a> <code>"&lt;p&gt;===&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1184">input 6, Faithful</a> <code>"&lt;p&gt;1.&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1185">input 7, Faithful</a> <code>"&lt;p&gt;1)&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1186">input 8, Faithful</a> <code>"&lt;p&gt;- - -&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1178"><code>a_paragraph_of_leaf_block_syntax_stays_a_paragraph</code></a> / <a href="tests/basic_tests.rs#L1190">input 9, Faithful</a> <code>"&lt;p&gt;--foo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1196"><code>an_atx_marker_alone_in_a_paragraph_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1197">input 1, Faithful</a> <code>"&lt;p&gt;#&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1196"><code>an_atx_marker_alone_in_a_paragraph_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1198">input 2, Faithful</a> <code>"&lt;p&gt;######&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1196"><code>an_atx_marker_alone_in_a_paragraph_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1202">input 3, Faithful</a> <code>"&lt;p&gt;#######&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1196"><code>an_atx_marker_alone_in_a_paragraph_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1206">input 4, Faithful</a> <code>"&lt;p&gt;#x&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1219"><code>a_bullet_marker_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1220">input 1, Faithful</a> <code>"&lt;p&gt;-\nfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1219"><code>a_bullet_marker_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1221">input 2, Faithful</a> <code>"&lt;p&gt;+\nfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1219"><code>a_bullet_marker_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1222">input 3, Faithful</a> <code>"&lt;p&gt;-\tfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1228"><code>an_atx_marker_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1229">input 1, Faithful</a> <code>"&lt;p&gt;#\nfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1228"><code>an_atx_marker_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1230">input 2, Faithful</a> <code>"&lt;p&gt;###\nfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1228"><code>an_atx_marker_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1231">input 3, Faithful</a> <code>"&lt;p&gt;#\tfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1237"><code>an_ordered_item_delimiter_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1238">input 1, Faithful</a> <code>"&lt;p&gt;1.\nfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1237"><code>an_ordered_item_delimiter_closed_by_a_line_ending_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1239">input 2, Faithful</a> <code>"&lt;p&gt;1)\nfoo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1247"><code>a_marker_behind_leading_whitespace_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1248">input 1, Faithful</a> <code>"&lt;p&gt; 1. foo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1247"><code>a_marker_behind_leading_whitespace_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1249">input 2, Faithful</a> <code>"&lt;p&gt; - foo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1247"><code>a_marker_behind_leading_whitespace_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1250">input 3, Faithful</a> <code>"&lt;p&gt;\n# foo&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1247"><code>a_marker_behind_leading_whitespace_is_escaped</code></a> / <a href="tests/basic_tests.rs#L1251">input 4, Faithful</a> <code>"&lt;p&gt;\n 1.\n Item numbering\n&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1261"><code>a_marker_which_starts_no_line_is_not_escaped</code></a> / <a href="tests/basic_tests.rs#L1262">input 1, Faithful</a> <code>"&lt;p&gt;a &lt;em&gt;b&lt;/em&gt; - c&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1261"><code>a_marker_which_starts_no_line_is_not_escaped</code></a> / <a href="tests/basic_tests.rs#L1263">input 2, Faithful</a> <code>"&lt;p&gt;a &lt;em&gt;b&lt;/em&gt; 1. c&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1274"><code>a_run_of_ordered_item_delimiters_opens_no_list</code></a> / <a href="tests/basic_tests.rs#L1275">input 1, Faithful</a> <code>"&lt;p&gt;1)))&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1274"><code>a_run_of_ordered_item_delimiters_opens_no_list</code></a> / <a href="tests/basic_tests.rs#L1276">input 2, Faithful</a> <code>"&lt;p&gt;1.)&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1285"><code>a_paragraph_of_unicode_numerics_needs_no_escape</code></a> / <a href="tests/basic_tests.rs#L1286">input 1, Faithful</a> <code>"&lt;p&gt;2½. text&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 37.2px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1285"><code>a_paragraph_of_unicode_numerics_needs_no_escape</code></a> / <a href="tests/basic_tests.rs#L1287">input 2, Faithful</a> <code>"&lt;p&gt;1١. text&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1301"><code>a_literal_cdata_section_is_lost_unescaped</code></a>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1332"><code>round_trip_in_a_block_context</code></a> / <a href="tests/basic_tests.rs#L1338">input 1, Faithful</a> <code>"&lt;body&gt;&lt;script&gt;a*b\n\nc&lt;/script&gt;&lt;/body&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1332"><code>round_trip_in_a_block_context</code></a> / <a href="tests/basic_tests.rs#L1342">input 2, Faithful</a> <code>"&lt;body&gt;&lt;style&gt;a*b\n\nc&lt;/style&gt;&lt;/body&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1332"><code>round_trip_in_a_block_context</code></a> / <a href="tests/basic_tests.rs#L1347">input 3, Faithful</a> <code>"&lt;div&gt;a*b\n\nc&lt;/div&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1332"><code>round_trip_in_a_block_context</code></a> / <a href="tests/basic_tests.rs#L1356">input 4, Faithful</a> <code>"&lt;del&gt;a*b\n\nc&lt;/del&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1332"><code>round_trip_in_a_block_context</code></a> / <a href="tests/basic_tests.rs#L1360">input 5, Faithful</a> <code>"&lt;br&gt;&lt;br&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1332"><code>round_trip_in_a_block_context</code></a> / <a href="tests/basic_tests.rs#L1363">input 6, Faithful</a> <code>"&lt;br&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1373">input 1, Faithful</a> <code>"&lt;p&gt;&lt;br&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1375">input 2, Faithful</a> <code>"&lt;p&gt;&lt;!--c--&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1376">input 3, Faithful</a> <code>"&lt;p&gt;&lt;!--c--&gt;a&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1379">input 4, Faithful</a> <code>r#"&lt;p&gt;&lt;iframe src="u"&gt;a&lt;/iframe&gt;&lt;/p&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1385">input 5, Faithful</a> <code>"&lt;blockquote&gt;&lt;p&gt;&lt;br&gt;&lt;/p&gt;&lt;/blockquote&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1390">input 6, Faithful</a> <code>"&lt;p&gt;&lt;br&gt;&lt;br&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1391">input 7, Faithful</a> <code>"&lt;p&gt;&lt;br&gt;a&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1392">input 8, Faithful</a> <code>"&lt;p&gt;&lt;del&gt;&lt;br&gt;&lt;/del&gt;&lt;/p&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1396">input 9, Faithful, setext</a> <code>"&lt;h1&gt;&lt;br&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1399">input 10, Faithful, setext</a> <code>"&lt;h1&gt;&lt;div&gt;a&lt;/div&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1371"><code>round_trip_of_a_leaf_block_an_html_block_would_dissolve</code></a> / <a href="tests/basic_tests.rs#L1404">input 11, Faithful, setext</a> <code>"&lt;h1&gt;&lt;br&gt;&lt;br&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1422"><code>round_trip_of_a_setext_heading_of_leaf_block_syntax</code></a> / <a href="tests/basic_tests.rs#L1423">input 1, Faithful, setext</a> <code>"&lt;h1&gt;-&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1422"><code>round_trip_of_a_setext_heading_of_leaf_block_syntax</code></a> / <a href="tests/basic_tests.rs#L1424">input 2, Faithful, setext</a> <code>"&lt;h2&gt;-&lt;/h2&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1422"><code>round_trip_of_a_setext_heading_of_leaf_block_syntax</code></a> / <a href="tests/basic_tests.rs#L1425">input 3, Faithful, setext</a> <code>"&lt;h1&gt;---&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1422"><code>round_trip_of_a_setext_heading_of_leaf_block_syntax</code></a> / <a href="tests/basic_tests.rs#L1426">input 4, Faithful, setext</a> <code>"&lt;h1&gt;1.&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Configured</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1434">input 1, Faithful</a> <code>"&lt;h1&gt;a #&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1435">input 2, Faithful</a> <code>"&lt;h1&gt;a ##&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1436">input 3, Faithful</a> <code>"&lt;h1&gt;&lt;em&gt;a&lt;/em&gt; #&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1437">input 4, Faithful</a> <code>"&lt;h3&gt;#&lt;/h3&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1439">input 5, Faithful</a> <code>"&lt;h1&gt;a\t#&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1440">input 6, Faithful</a> <code>r"&lt;h1&gt;a \#&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1443">input 7, Faithful</a> <code>"&lt;h1&gt;a#&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1433"><code>round_trip_of_an_atx_heading_ending_in_a_hash_run</code></a> / <a href="tests/basic_tests.rs#L1444">input 8, Faithful</a> <code>"&lt;h1&gt;a&amp;nbsp;#&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1452"><code>atx_heading_of_a_hash_run_from_a_handler</code></a> / <a href="tests/basic_tests.rs#L1463">input 1, Faithful, custom handler</a> <code>"&lt;h1&gt;&lt;x-hash&gt;&lt;/x-hash&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1452"><code>atx_heading_of_a_hash_run_from_a_handler</code></a> / <a href="tests/basic_tests.rs#L1472">input 2, Faithful, custom handler</a> <code>"&lt;h1&gt;&lt;x-hash&gt;&lt;/x-hash&gt;&lt;x-hash&gt;&lt;/x-hash&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1488">input 1, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a*b\nc&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1492">input 2, Faithful</a> <code>"&lt;h1&gt;x&lt;style&gt;a*b\nc&lt;/style&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1494">input 3, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a*b*c&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1495">input 4, Faithful</a> <code>"&lt;h1&gt;x&lt;style&gt;a*b*c&lt;/style&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1496">input 5, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;[a](b)&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1503">input 6, Faithful</a> <code>"&lt;h1&gt;x&lt;textarea&gt;a*b\nc&lt;/textarea&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1507">input 7, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a*b\nc&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1510">input 8, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a*b*c&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1513">input 9, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a*b\nc&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1517">input 10, Faithful</a> <code>"&lt;h1&gt;x&lt;del&gt;a*b\nc&lt;/del&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1484"><code>round_trip_in_an_inline_context</code></a> / <a href="tests/basic_tests.rs#L1523">input 11, Faithful</a> <code>"&lt;h1&gt;x&lt;!--a\nb--&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1544">input 1, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a&lt;em&gt;b&lt;/em&gt;c&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1545">input 2, Faithful</a> <code>"&lt;h1&gt;x&lt;del&gt;a&lt;em&gt;b&lt;/em&gt;c&lt;/del&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1546">input 3, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a&lt;em&gt;b&lt;/em&gt;c&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1549">input 4, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a&amp;lt;b&amp;gt;c&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1550">input 5, Faithful</a> <code>"&lt;h1&gt;x&lt;del&gt;a&amp;lt;b&amp;gt;c&lt;/del&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1551">input 6, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a&amp;lt;b&amp;gt;c&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1552">input 7, Faithful</a> <code>"&lt;h1&gt;x&lt;textarea&gt;a&amp;lt;b&amp;gt;c&lt;/textarea&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1555">input 8, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a&amp;amp;b&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1556">input 9, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a&amp;amp;b&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1559">input 10, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a*b_c[d]&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1560">input 11, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a*b_c[d]&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1561">input 12, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a*b_c[d]&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1562">input 13, Faithful</a> <code>"&lt;h1&gt;x&lt;style&gt;a*b_c[d]&lt;/style&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1566">input 14, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a&lt;code&gt;c*d&lt;/code&gt;b&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1567">input 15, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a&lt;code&gt;c*d&lt;/code&gt;b&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1568">input 16, Faithful</a> <code>"&lt;h1&gt;&lt;pre&gt;&lt;code&gt;a&lt;/code&gt;&lt;/pre&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1569">input 17, Faithful</a> <code>r#"&lt;h1&gt;x&lt;div&gt;a&lt;code class="q"&gt;c*d&lt;/code&gt;b&lt;/div&gt;y&lt;/h1&gt;"#</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1542"><code>round_trip_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1573">input 18, Faithful</a> <code>"&lt;pre&gt;a*b\n\nc&lt;/pre&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Yes</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1629">input 1, Faithful</a> <code>"&lt;h1&gt;x&lt;div&gt;a\nb&lt;/div&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1630">input 2, Faithful</a> <code>"&lt;h1&gt;x&lt;pre&gt;a\nb&lt;/pre&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1631">input 3, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a\nb&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1635">input 4, Faithful</a> <code>"&lt;h1&gt;x&lt;textarea&gt;a\nb&lt;/textarea&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1639">input 5, Faithful</a> <code>"&lt;h1&gt;&lt;pre&gt;&lt;code&gt;a*b\nc&lt;/code&gt;&lt;/pre&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1587">input 6, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;if(a&lt;b){}&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1591">input 7, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a&lt;em&gt;b&lt;/em&gt;c&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1598">input 8, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a&amp;amp;b&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1602">input 9, Faithful</a> <code>"&lt;h1&gt;x&lt;script&gt;a&amp;lt;b&amp;gt;c&lt;/script&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1609">input 10, Faithful</a> <code>"&lt;h1&gt;&lt;pre&gt;&lt;b&gt;a&lt;/b&gt;&lt;/pre&gt;&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1615">input 11, Faithful</a> <code>"&lt;h1&gt;x&lt;textarea&gt;a&lt;em&gt;b&lt;/em&gt;c&lt;/textarea&gt;y&lt;/h1&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 36.4px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1579"><code>round_trip_losses_of_a_walked_raw_inline</code></a> / <a href="tests/basic_tests.rs#L1621">input 12, Faithful</a> <code>"&lt;pre&gt;&lt;code&gt;a*b\nc&lt;/code&gt;&lt;/pre&gt;"</code>
      </td>
      <td style="width: 11.2716%;">Expected</td>
      <td style="width: 11.8525%;">No</td>
      <td style="width: 10.12%;">Faithful</td>
    </tr>
    <tr style="height: 19.6px;">
      <td style="width: 66.6613%;">
        <a href="tests/basic_tests.rs#L1649"><code>append_content_when_the_body_is_empty</code></a>
      </td>
      <td style="width: 11.2716%;">No</td>
      <td style="width: 11.8525%;">Yes</td>
      <td style="width: 10.12%;">No</td>
    </tr>
  </tbody>
</table>

### [tests/code\_tests.rs](tests/code_tests.rs) (20)

| Test                                                                                                                                                                                | Faithful round trip | Keep Markdown | Existing round trip |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`code_blocks`](tests/code_tests.rs#L29)                                                                                                                                            | Expected            | Yes           | No                  |
| [`code_blocks_with_lang_class`](tests/code_tests.rs#L40)                                                                                                                            | Expected            | Yes           | No                  |
| [`faithful_mode_preserves_an_empty_language_class`](tests/code_tests.rs#L51)                                                                                                        | Yes                 | Yes           | No                  |
| [`code_blocks_decode_html_entities`](tests/code_tests.rs#L58)                                                                                                                       | Expected            | Yes           | No                  |
| [`code_blocks_with_lang_class_on_pre_tag`](tests/code_tests.rs#L71)                                                                                                                 | No                  | Yes           | No                  |
| [`span_subtree_conversion_preserves_ancestor_preformatted_context`](tests/code_tests.rs#L82)                                                                                        | No                  | Yes           | No                  |
| [`delegated_unhandled_subtree_preserves_ancestor_preformatted_context`](tests/code_tests.rs#L93)                                                                                    | No                  | Yes           | No                  |
| [`faithful_mode_pre`](tests/code_tests.rs#L113)                                                                                                                                     | Yes                 | Yes           | No                  |
| [`faithful_mode_code_block1`](tests/code_tests.rs#L121)                                                                                                                             | Yes                 | Yes           | No                  |
| [`faithful_mode_code_block2`](tests/code_tests.rs#L129)                                                                                                                             | Yes                 | Yes           | No                  |
| [`inline_code_made_only_of_backticks_uses_a_non_colliding_delimiter`](tests/code_tests.rs#L140)                                                                                     | No                  | Yes           | No                  |
| [`inline_code_ending_in_a_backtick_keeps_the_backtick_inside_the_span`](tests/code_tests.rs#L147)                                                                                   | No                  | Yes           | No                  |
| [`preformatted_inline_code_preserves_boundary_spaces`](tests/code_tests.rs#L154)                                                                                                    | No                  | Yes           | No                  |
| [`pure_mode_writes_a_code_span_line_ending_as_the_space_commonmark_reads`](tests/code_tests.rs#L173) / [input 1, Pure](tests/code_tests.rs#L176) `"<h1>a <code>x\ny</code> b</h1>"` | No                  | Yes           | No                  |
| [`pure_mode_writes_a_code_span_line_ending_as_the_space_commonmark_reads`](tests/code_tests.rs#L173) / [input 2, Pure](tests/code_tests.rs#L180) `"<p>a <code>x\ny</code> b</p>"`   | No                  | Yes           | No                  |
| [`fenced_code_uses_a_fence_longer_than_any_run_in_its_content`](tests/code_tests.rs#L185)                                                                                           | No                  | Yes           | No                  |
| [`tilde_fenced_code_uses_a_fence_longer_than_any_run_in_its_content`](tests/code_tests.rs#L193)                                                                                     | No                  | Yes           | No                  |
| [`processing_instruction_nodes_are_ignored`](tests/code_tests.rs#L209)                                                                                                              | No                  | Yes           | No                  |
| [`code_block_with_spans_ending_in_newline`](tests/code_tests.rs#L220)                                                                                                               | No                  | Yes           | No                  |
| [`code_block_with_blank_lines_and_indentation_in_spans`](tests/code_tests.rs#L232)                                                                                                  | No                  | Yes           | No                  |
| [`span_fallback_in_pre_preserves_newlines`](tests/code_tests.rs#L245)                                                                                                               | No                  | Yes           | No                  |

### [tests/context\_tests.rs](tests/context_tests.rs) (26)

| Test                                                                                                                                                                                                                                | Faithful round trip   | Keep Markdown | Existing round trip |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`html_at_the_document_root_is_a_block`](tests/context_tests.rs#L11) / [input 1, Faithful](tests/context_tests.rs#L14) `"<div>a</div><div>b</div>"`                                                                                 | Expected              | Yes           | No                  |
| [`html_at_the_document_root_is_a_block`](tests/context_tests.rs#L11) / [input 2, Faithful](tests/context_tests.rs#L18) `"<div><br><br></div>"`                                                                                      | Expected              | Yes           | No                  |
| [`html_at_the_document_root_is_a_block`](tests/context_tests.rs#L11) / [input 3, Faithful](tests/context_tests.rs#L22) `"<h1><div><p>a</p></div></h1>"`                                                                             | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 1, Faithful](tests/context_tests.rs#L34) `"<br>"`                                                                                     | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 2, Faithful](tests/context_tests.rs#L35) `"<br><br>"`                                                                                 | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 3, Faithful](tests/context_tests.rs#L38) `"This is <em foo>really</em> important."`                                                   | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 4, Faithful](tests/context_tests.rs#L42) `"<ul><li>a<br>b</li></ul>"`                                                                 | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 5, Faithful](tests/context_tests.rs#L46) `"<blockquote>a<br>b</blockquote>"`                                                          | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 6, Faithful](tests/context_tests.rs#L50) `"<br><p>a</p>"`                                                                             | Expected              | Yes           | No                  |
| [`a_type_7_tag_is_a_raw_inline_even_in_a_block_context`](tests/context_tests.rs#L33) / [input 7, Faithful](tests/context_tests.rs#L51) `"<p>a</p><br>"`                                                                             | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 1, Faithful](tests/context_tests.rs#L56) `"<h1><p>a</p></h1>"`                                                                     | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 2, Faithful](tests/context_tests.rs#L59) `"<h1>x<p>a</p>y</h1>"`                                                                   | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 3, Faithful](tests/context_tests.rs#L63) `"<h1><blockquote>a</blockquote></h1>"`                                                   | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 4, Faithful](tests/context_tests.rs#L67) `"<h1><ul><li>a</li></ul></h1>"`                                                          | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 5, Faithful](tests/context_tests.rs#L69) `"<h1><hr></h1>"`                                                                         | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 6, Faithful](tests/context_tests.rs#L74) `"<h1><pre><code>a</code></pre></h1>"`                                                    | Expected              | Yes           | No                  |
| [`a_commonmark_block_in_an_inline_context_is_a_raw_inline`](tests/context_tests.rs#L55) / [input 7, Faithful](tests/context_tests.rs#L78) `"<h1><table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td>a</td></tr></tbody></t…`  | Expected              | Yes           | No                  |
| [`raw_text_at_the_document_root_is_left_alone`](tests/context_tests.rs#L89)                                                                                                                                                         | Expected              | Yes           | No                  |
| [`html_in_a_paragraph_is_a_raw_inline`](tests/context_tests.rs#L94) / [input 1, Faithful](tests/context_tests.rs#L95) `"<p><br><br></p>"`                                                                                           | Yes                   | Yes           | No                  |
| [`html_in_a_paragraph_is_a_raw_inline`](tests/context_tests.rs#L94) / [input 2, Faithful](tests/context_tests.rs#L98) `"<p><br><em>b</em></p>"`                                                                                     | Yes                   | Yes           | No                  |
| [`html_in_a_paragraph_is_a_raw_inline`](tests/context_tests.rs#L94) / [input 3, Faithful](tests/context_tests.rs#L102) `"<p><em>a</em><br></p>"`                                                                                    | Yes                   | Yes           | No                  |
| [`html_in_a_paragraph_is_a_raw_inline`](tests/context_tests.rs#L94) / [input 4, Faithful](tests/context_tests.rs#L106) `r#"<p><br><img src="i" alt=""></p>"#`                                                                       | Yes                   | Yes           | No                  |
| [`html_in_a_heading_is_a_raw_inline`](tests/context_tests.rs#L111) / [input 1, Faithful](tests/context_tests.rs#L112) `"<h1><br><br></h1>"`                                                                                         | Yes                   | Yes           | No                  |
| [`html_in_a_heading_is_a_raw_inline`](tests/context_tests.rs#L111) / [input 2, Faithful](tests/context_tests.rs#L115) `"<h1><br><em>b</em></h1>"`                                                                                   | Yes                   | Yes           | No                  |
| [`html_in_a_heading_is_a_raw_inline`](tests/context_tests.rs#L111) / [input 3, Faithful](tests/context_tests.rs#L119) `"<h6><em>a</em><br></h6>"`                                                                                   | Yes                   | Yes           | No                  |
| [`html_in_a_blockquote_is_a_block`](tests/context_tests.rs#L124) / [input 1, Faithful](tests/context_tests.rs#L127) `"<blockquote><div>a</div><div>b</div></blockquote>"`                                                           | Expected              | Yes           | No                  |
| [`html_in_a_blockquote_is_a_block`](tests/context_tests.rs#L124) / [input 2, Faithful](tests/context_tests.rs#L131) `"<blockquote><br><br></blockquote>"`                                                                           | Expected              | Yes           | No                  |
| [`html_in_a_blockquote_is_a_block`](tests/context_tests.rs#L124) / [input 3, Faithful](tests/context_tests.rs#L135) `"<blockquote><p><br><em>b</em></p></blockquote>"`                                                              | Expected              | Yes           | No                  |
| [`html_in_a_list_item_is_a_block`](tests/context_tests.rs#L140) / [input 1, Faithful](tests/context_tests.rs#L143) `"<ul><li><div>a</div><div>b</div></li></ul>"`                                                                   | Expected              | Yes           | No                  |
| [`html_in_a_list_item_is_a_block`](tests/context_tests.rs#L140) / [input 2, Faithful](tests/context_tests.rs#L147) `"<ul><li><br><p>a</p></li></ul>"`                                                                               | Expected              | Yes           | No                  |
| [`html_in_a_list_item_is_a_block`](tests/context_tests.rs#L140) / [input 3, Faithful](tests/context_tests.rs#L153) `"<ul><li><br><br></li></ul>"`                                                                                   | Expected              | Yes           | No                  |
| [`html_in_a_table_cell_is_a_raw_inline`](tests/context_tests.rs#L160) / [input 1, Faithful](tests/context_tests.rs#L163) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td><br><em>b</em></td></tr></…`                   | Configured            | Yes           | No                  |
| [`html_in_a_table_cell_is_a_raw_inline`](tests/context_tests.rs#L160) / [input 2, Faithful](tests/context_tests.rs#L172) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td><p>a</p></td></tr></tbody>…`                   | Configured            | Yes           | No                  |
| [`html_in_a_table_cell_is_a_raw_inline`](tests/context_tests.rs#L160) / [input 3, Faithful](tests/context_tests.rs#L180) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td><ul><li>a</li></ul></td></…`                   | Configured            | Yes           | No                  |
| [`a_cell_attribute_writes_the_whole_table_as_html`](tests/context_tests.rs#L194) / [input 1, Faithful](tests/context_tests.rs#L197) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td colspan="2">a</td></tr><…`          | Yes                   | Yes           | No                  |
| [`a_cell_attribute_writes_the_whole_table_as_html`](tests/context_tests.rs#L194) / [input 2, Faithful](tests/context_tests.rs#L200) `"<table><thead><tr><th colspan="2">h</th></tr></thead>\ <tbody><tr><td>a</td></tr><…`          | Yes                   | Yes           | No                  |
| [`html_in_an_inline_element_is_a_raw_inline`](tests/context_tests.rs#L204) / [input 1, Faithful](tests/context_tests.rs#L207) `"<p>a<span><br></span>b</p>"`                                                                        | Yes                   | Yes           | No                  |
| [`html_in_an_inline_element_is_a_raw_inline`](tests/context_tests.rs#L204) / [input 2, Faithful](tests/context_tests.rs#L211) `"<p>a<del><br></del>b</p>"`                                                                          | Yes                   | Yes           | No                  |
| [`html_in_an_inline_element_is_a_raw_inline`](tests/context_tests.rs#L204) / [input 3, Faithful](tests/context_tests.rs#L215) `r#"<p>a<a href="u"><br></a>b</p>"#`                                                                  | Yes                   | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222) / [input 1, Faithful](tests/context_tests.rs#L225) `"<p>a<code>x<br>y</code>b</p>"`                                                                                   | Expected              | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222) / [input 2, Faithful](tests/context_tests.rs#L229) `"<pre><code>a<br>b</code></pre>"`                                                                                 | Expected              | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222) / [input 3, Faithful](tests/context_tests.rs#L235) `"<h1><pre><b>a</b></pre></h1>"`                                                                                   | Expected              | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222) / [input 4, Faithful](tests/context_tests.rs#L239) `r#"<h1><pre class="x"><b>a</b></pre></h1>"#`                                                                      | Expected              | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222) / [input 5, Faithful](tests/context_tests.rs#L245) `"<h1><pre><code>a*b\nc</code></pre></h1>"`                                                                        | Expected              | Yes           | No                  |
| [`html_in_code_is_a_raw_inline`](tests/context_tests.rs#L222) / [input 6, Faithful](tests/context_tests.rs#L251) `"<h1><div>a<code>c*d</code>b</div></h1>"`                                                                         | Expected              | Yes           | No                  |
| [`a_raw_text_element_escapes_the_markdown_it_holds`](tests/context_tests.rs#L260) / [input 1, Faithful](tests/context_tests.rs#L263) `"<h1><script>a*b_c[d]</script></h1>"`                                                         | Expected              | Yes           | No                  |
| [`a_raw_text_element_escapes_the_markdown_it_holds`](tests/context_tests.rs#L260) / [input 2, Faithful](tests/context_tests.rs#L271) `"<h1><script>if(a<b){}</script></h1>"`                                                        | Expected              | Yes           | No                  |
| [`a_raw_text_element_escapes_the_markdown_it_holds`](tests/context_tests.rs#L260) / [input 3, Faithful](tests/context_tests.rs#L275) `"<p>x<script>a*b</script>y</p>"`                                                              | Expected              | Yes           | No                  |
| [`a_raw_text_element_escapes_the_markdown_it_holds`](tests/context_tests.rs#L260) / [input 4, Faithful](tests/context_tests.rs#L282) `"<body><script>a*b\nc</script></body>"`                                                       | Expected              | Yes           | No                  |
| [`a_raw_text_element_escapes_the_markdown_it_holds`](tests/context_tests.rs#L260) / [input 5, Faithful](tests/context_tests.rs#L287) `"<h1><script>a\nb</script></h1>"`                                                             | Expected              | Yes           | No                  |
| [`an_rcdata_element_is_translated`](tests/context_tests.rs#L295) / [input 1, Faithful](tests/context_tests.rs#L298) `"<h1><textarea>a*b*c</textarea></h1>"`                                                                         | Expected              | Yes           | No                  |
| [`an_rcdata_element_is_translated`](tests/context_tests.rs#L295) / [input 2, Faithful](tests/context_tests.rs#L302) `"<h1><title>a*b*c</title></h1>"`                                                                               | Expected              | Yes           | No                  |
| [`an_rcdata_element_is_translated`](tests/context_tests.rs#L295) / [input 3, Faithful](tests/context_tests.rs#L306) `"<h1><textarea>a\nb</textarea></h1>"`                                                                          | Expected              | Yes           | No                  |
| [`an_rcdata_element_is_translated`](tests/context_tests.rs#L295) / [input 4, Faithful](tests/context_tests.rs#L310) `"<textarea>a*b\nc</textarea>"`                                                                                 | Expected              | Yes           | No                  |
| [`a_raw_inline_collapses_the_whitespace_of_its_content`](tests/context_tests.rs#L320) / [input 1, Faithful](tests/context_tests.rs#L324) `"<h1>x<div>a\nb</div>y</h1>"`                                                             | Expected              | Yes           | No                  |
| [`a_raw_inline_collapses_the_whitespace_of_its_content`](tests/context_tests.rs#L320) / [input 2, Faithful](tests/context_tests.rs#L329) `"<p>a<del>\n b\n</del>c</p>"`                                                             | Expected              | Yes           | No                  |
| [`a_raw_inline_collapses_the_whitespace_of_its_content`](tests/context_tests.rs#L320) / [input 3, Faithful](tests/context_tests.rs#L335) `"<h1><pre>a\n b</pre></h1>"`                                                              | Expected              | Yes           | No                  |
| [`a_raw_inline_collapses_the_whitespace_of_its_content`](tests/context_tests.rs#L320) / [input 4, Faithful](tests/context_tests.rs#L338) `"<p>a\nb</p>"`                                                                            | Expected              | Yes           | No                  |
| [`a_raw_inline_collapses_the_whitespace_of_its_content`](tests/context_tests.rs#L320) / [input 5, Faithful](tests/context_tests.rs#L339) `"<h1>a\n b</h1>"`                                                                         | Expected              | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347) / [input 1, Faithful](tests/context_tests.rs#L350) `"<p>a<em foo="1\n\n2">y</em>b</p>"`                                                    | Configured + expected | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347) / [input 2, Faithful](tests/context_tests.rs#L355) `"<h1>a<div title="x y">z w</div>b</h1>"`                                               | Configured + expected | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347) / [input 3, Faithful](tests/context_tests.rs#L360) `r#"<h1>a<div title="p q"><em foo="r s">t</em></div>b</h1>"#`                           | Configured + expected | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347) / [input 4, Faithful](tests/context_tests.rs#L364) `"<h1>a<div title="p\nq"><em foo="r\ns">t</em></div>b</h1>"`                            | Configured + expected | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347) / [input 5, Faithful](tests/context_tests.rs#L372) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td><em foo="1\n2">y</em></t…`  | Configured + expected | Yes           | No                  |
| [`a_raw_inline_escapes_the_line_endings_of_its_attributes`](tests/context_tests.rs#L347) / [input 6, Faithful](tests/context_tests.rs#L382) `"<div>a\n\nb</div>"`                                                                   | Configured + expected | Yes           | No                  |
| [`a_raw_inline_encodes_the_pipes_of_its_attributes`](tests/context_tests.rs#L391) / [input 1, Faithful](tests/context_tests.rs#L398) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td><em foo="1&#124;2">y&#124;z</em>…` | Configured            | Yes           | No                  |
| [`a_raw_inline_encodes_the_pipes_of_its_attributes`](tests/context_tests.rs#L391) / [input 2, Faithful](tests/context_tests.rs#L409) `r#"<p>a<em foo="1&#124;2">y&#124;z</em>b</p>"#`                                               | Configured            | Yes           | No                  |
| [`a_type_1_block_keeps_its_blank_lines`](tests/context_tests.rs#L417) / [input 1, Faithful](tests/context_tests.rs#L422) `"<body><script>a\n\nb</script></body>"`                                                                   | Expected              | Yes           | No                  |
| [`a_type_1_block_keeps_its_blank_lines`](tests/context_tests.rs#L417) / [input 2, Faithful](tests/context_tests.rs#L426) `"<body><style>a\n\nb</style></body>"`                                                                     | Expected              | Yes           | No                  |
| [`a_type_1_block_keeps_its_blank_lines`](tests/context_tests.rs#L417) / [input 3, Faithful](tests/context_tests.rs#L430) `"<pre>a\n\nb</pre>"`                                                                                      | Expected              | Yes           | No                  |
| [`a_type_1_block_keeps_its_blank_lines`](tests/context_tests.rs#L417) / [input 4, Faithful](tests/context_tests.rs#L434) `"<textarea>a\n\nb</textarea>"`                                                                            | Expected              | Yes           | No                  |
| [`a_type_1_block_keeps_its_blank_lines`](tests/context_tests.rs#L417) / [input 5, Faithful](tests/context_tests.rs#L440) `"<h1><script>a\n\nb</script></h1>"`                                                                       | Expected              | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450) / [input 1, Faithful](tests/context_tests.rs#L453) `"<h1><a href="u" title="a\nb">t</a></h1>"`                                                                    | Configured + expected | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450) / [input 2, Faithful](tests/context_tests.rs#L457) `"<h1><img src="i" alt="a\nb" title="c\nd"></h1>"`                                                             | Configured + expected | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450) / [input 3, Faithful](tests/context_tests.rs#L462) `"<p><a href="u" title="a\n\nb">t</a></p>"`                                                                    | Configured + expected | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450) / [input 4, Faithful](tests/context_tests.rs#L469) `"<h1><a href="u" title="a&#13;&#10;b">t</a></h1>"`                                                            | Configured + expected | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450) / [input 5, Faithful](tests/context_tests.rs#L473) `"<h1><a href="u" title="a&#13;b">t</a></h1>"`                                                                 | Configured + expected | Yes           | No                  |
| [`a_title_escapes_its_line_endings`](tests/context_tests.rs#L450) / [input 6, Faithful](tests/context_tests.rs#L481) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td><a href="u" title="a\nb…`                          | Configured + expected | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 1, Faithful](tests/context_tests.rs#L495) `"<!--a\n\nb-->"`                                                                                                 | Expected              | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 2, Faithful](tests/context_tests.rs#L498) `"<div>a</div><!--c--><div>b</div>"`                                                                              | Expected              | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 3, Faithful](tests/context_tests.rs#L502) `"<blockquote><!--c--></blockquote>"`                                                                             | Expected              | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 4, Faithful](tests/context_tests.rs#L506) `"<h1>x<!--a\nb-->y</h1>"`                                                                                        | Expected              | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 5, Faithful](tests/context_tests.rs#L510) `"<p>x<!--a\n\nb-->y</p>"`                                                                                        | Expected              | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 6, Faithful](tests/context_tests.rs#L514) `"<h1>x<!--a\r\nb-->y</h1>"`                                                                                      | Expected              | Yes           | No                  |
| [`a_comment_follows_its_context`](tests/context_tests.rs#L494) / [input 7, Faithful](tests/context_tests.rs#L519) `"<h1><div>x<!--a\nb-->y</div></h1>"`                                                                             | Expected              | Yes           | No                  |
| [`a_cdata_section_and_a_processing_instruction_are_comments`](tests/context_tests.rs#L527) / [input 1, Faithful](tests/context_tests.rs#L530) `"<![CDATA[x\n\ny]]>"`                                                                | Expected              | Yes           | No                  |
| [`a_cdata_section_and_a_processing_instruction_are_comments`](tests/context_tests.rs#L527) / [input 2, Faithful](tests/context_tests.rs#L534) `"<p>a<![CDATA[x\ny]]>b</p>"`                                                         | Expected              | Yes           | No                  |
| [`a_cdata_section_and_a_processing_instruction_are_comments`](tests/context_tests.rs#L527) / [input 3, Faithful](tests/context_tests.rs#L538) `"<p>a<?php echo "x\ny"; ?>b</p>"`                                                    | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546) / [input 1, Faithful](tests/context_tests.rs#L549) `"<h1><div><blockquote>a</blockquote></div></h1>"`                                       | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546) / [input 2, Faithful](tests/context_tests.rs#L553) `"<h1><div><ul><li>a</li></ul></div></h1>"`                                              | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546) / [input 3, Faithful](tests/context_tests.rs#L557) `"<h1><div><h2>a</h2></div></h1>"`                                                       | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546) / [input 4, Faithful](tests/context_tests.rs#L561) `"<h1><div><hr></div></h1>"`                                                             | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546) / [input 5, Faithful](tests/context_tests.rs#L566) `"<h1><div>a<em>b</em>c</div></h1>"`                                                     | Expected              | Yes           | No                  |
| [`a_commonmark_block_inside_a_raw_inline_is_a_raw_inline`](tests/context_tests.rs#L546) / [input 6, Faithful](tests/context_tests.rs#L570) `r#"<h1><div>a<a href="u">l*m</a>b</div></h1>"#`                                         | Expected              | Yes           | No                  |
| [`a_serialized_element_follows_its_context`](tests/context_tests.rs#L575) / [input 1, Faithful](tests/context_tests.rs#L578) `"<p>x <em foo>y</em> z</p>"`                                                                          | Yes                   | Yes           | No                  |
| [`a_serialized_element_follows_its_context`](tests/context_tests.rs#L575) / [input 2, Faithful](tests/context_tests.rs#L582) `"<h1><em foo>y</em></h1>"`                                                                            | Yes                   | Yes           | No                  |
| [`a_serialized_element_follows_its_context`](tests/context_tests.rs#L575) / [input 3, Faithful](tests/context_tests.rs#L586) `"<ul><li><em foo>y</em></li></ul>"`                                                                   | Yes                   | Yes           | No                  |
| [`a_math_span_replaces_its_line_endings_with_spaces`](tests/context_tests.rs#L594) / [input 1, Faithful](tests/context_tests.rs#L597) `"<p>x<span class="math math-inline">a\nb</span>y</p>"`                                       | Configured + expected | Yes           | No                  |
| [`a_math_span_replaces_its_line_endings_with_spaces`](tests/context_tests.rs#L594) / [input 2, Faithful](tests/context_tests.rs#L601) `"<p><span class="math math-display">a\nb</span></p>"`                                        | Configured + expected | Yes           | No                  |
| [`a_math_span_replaces_its_line_endings_with_spaces`](tests/context_tests.rs#L594) / [input 3, Faithful](tests/context_tests.rs#L608) `"<p><span class="math math-inline">a&#13;&#10;b</span></p>"`                                 | Configured + expected | Yes           | No                  |
| [`a_math_span_replaces_its_line_endings_with_spaces`](tests/context_tests.rs#L594) / [input 4, Faithful](tests/context_tests.rs#L613) `"<p><span class="math math-inline">a&#13;b</span></p>"`                                      | Configured + expected | Yes           | No                  |
| [`a_math_span_replaces_its_line_endings_with_spaces`](tests/context_tests.rs#L594) / [input 5, Faithful](tests/context_tests.rs#L619) `"<p><span class="math math-inline">a\\nb</span></p>"`                                        | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 1, Faithful](tests/context_tests.rs#L631) `"<p><span class="math math-inline">\alpha%c\n \tx</span></p>"`                                                 | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 2, Faithful](tests/context_tests.rs#L637) `"<p><span class="math math-inline">\text{fo%c\no}</span></p>"`                                                 | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 3, Faithful](tests/context_tests.rs#L643) `"<p><span class="math math-display">a%x&#13;&#10;b&#13;c%y&#13;d</span></p>"`                                  | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 4, Faithful](tests/context_tests.rs#L652) `"<p><span class="math math-display">a%b</span></p>"`                                                           | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 5, Faithful](tests/context_tests.rs#L657) `r#"<p><span class="math math-inline">a%b</span></p>"#`                                                         | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 6, Faithful](tests/context_tests.rs#L662) `r#"<p><span class="math math-inline">a\%b</span></p>"#`                                                        | Configured + expected | Yes           | No                  |
| [`a_math_span_strips_its_comments`](tests/context_tests.rs#L628) / [input 7, Faithful](tests/context_tests.rs#L669) `r#"<p><span class="math math-inline">%a</span></p>"#`                                                          | Configured + expected | Yes           | No                  |
| [`an_unwritable_math_span_strips_its_comments`](tests/context_tests.rs#L677) / [input 1, Faithful](tests/context_tests.rs#L681) `"<p><span class="math math-inline">a$b%c\nd</span></p>"`                                           | Configured + expected | Yes           | No                  |
| [`an_unwritable_math_span_strips_its_comments`](tests/context_tests.rs#L677) / [input 2, Pure](tests/context_tests.rs#L683) `"<p><span class="math math-inline">a$b%c\nd</span></p>"`                                               | No                    | Yes           | No                  |

### [tests/event\_tests.rs](tests/event_tests.rs) (6)

| Test                                                                               | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`test_element_events_skip_on_handler_fallback`](tests/event_tests.rs#L13)         | No                  | Yes           | No                  |
| [`test_passive_tag_filtering`](tests/event_tests.rs#L77)                           | No                  | N/A           | No                  |
| [`test_doc_lifecycle_events_with_nesting`](tests/event_tests.rs#L128)              | No                  | N/A           | No                  |
| [`test_span_events_dispatched_in_pure_mode`](tests/event_tests.rs#L194)            | No                  | Yes           | No                  |
| [`test_unhandled_tag_events_dispatched_in_pure_mode`](tests/event_tests.rs#L248)   | No                  | Yes           | No                  |
| [`test_span_and_unhandled_tag_events_in_faithful_mode`](tests/event_tests.rs#L302) | No                  | N/A           | No                  |

### [tests/link\_tests.rs](tests/link_tests.rs) (26)

| Test                                                                                                                                                                                                                 | Faithful round trip   | Keep Markdown | Existing round trip |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`links`](tests/link_tests.rs#L12)                                                                                                                                                                                   | Expected              | Yes           | No                  |
| [`links_with_spaces_in_destination_and_title`](tests/link_tests.rs#L24)                                                                                                                                              | Expected              | Yes           | No                  |
| [`links_with_spaces_around_text`](tests/link_tests.rs#L33) / [input 1, Faithful](tests/link_tests.rs#L36) `r#"<a href="/"> bla </a>"#`                                                                               | Expected              | Yes           | No                  |
| [`links_with_spaces_around_text`](tests/link_tests.rs#L33) / [input 2, Faithful](tests/link_tests.rs#L40) `r#"Some <a href="/"> random </a> text"#`                                                                  | Expected              | Yes           | No                  |
| [`links_inlined_prefer_autolinks`](tests/link_tests.rs#L45) / [input 1, Faithful, autolinks](tests/link_tests.rs#L55) `r#"<a href="https://example.com">https://example.com</a>"#`                                   | Configured + expected | Yes           | No                  |
| [`links_inlined_prefer_autolinks`](tests/link_tests.rs#L45) / [input 2, Faithful, autolinks](tests/link_tests.rs#L60) `r#"<a href="https://example.com">Link</a>"#`                                                  | Configured + expected | Yes           | No                  |
| [`links_inlined_prefer_autolinks`](tests/link_tests.rs#L45) / [input 3, Faithful, autolinks](tests/link_tests.rs#L66) `r#"<a href="https://example.com" title="https://example.com">Link</a>"#`                      | Configured + expected | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 1, Faithful](tests/link_tests.rs#L77) `"<p>a<a href="u\nv">t</a>b</p>"`                                                            | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 2, Faithful](tests/link_tests.rs#L81) `"<p>a<a href="u\n\nv">t</a>b</p>"`                                                          | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 3, Faithful](tests/link_tests.rs#L85) `"<h1>a<a href="u\nv">t</a>b</h1>"`                                                          | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 4, Faithful](tests/link_tests.rs#L91) `"<p>a<a href="u&#13;v">t</a>b</p>"`                                                         | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 5, Faithful](tests/link_tests.rs#L96) `"<p>a<img src="i\nj" alt="">b</p>"`                                                         | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 6, Faithful](tests/link_tests.rs#L100) `r#"<p><a href="u">t</a></p>"#`                                                             | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_line_endings`](tests/link_tests.rs#L74) / [input 7, Faithful](tests/link_tests.rs#L104) `r#"<p><a href="u(v)">t</a></p>"#`                                                          | Expected              | Yes           | No                  |
| [`nested_convert_with_referenced_links`](tests/link_tests.rs#L109) / [input 1, Pure](tests/link_tests.rs#L129) `r#"<a href="https://inner.com">Inner</a>"#`                                                          | No                    | Yes           | No                  |
| [`nested_convert_with_referenced_links`](tests/link_tests.rs#L109) / [input 2, Pure](tests/link_tests.rs#L143) `indoc!( r#" <p><a href="https://outer1.com">Outer 1</a></p> <widget></widget> <p><a h…`              | No                    | Yes           | No                  |
| [`discard_links_in_table_falling_back_to_raw_html`](tests/link_tests.rs#L164)                                                                                                                                        | Configured + expected | Yes           | No                  |
| [`a_link_destination_escapes_its_backslashes`](tests/link_tests.rs#L212) / [input 1, Faithful](tests/link_tests.rs#L215) `r#"<a href="a">x</a>"#`                                                                    | Expected              | Yes           | Faithful            |
| [`a_link_destination_escapes_its_backslashes`](tests/link_tests.rs#L212) / [input 2, Faithful](tests/link_tests.rs#L219) `r#"<img src="i" alt="q">"#`                                                                | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_backslashes`](tests/link_tests.rs#L212) / [input 3, Faithful](tests/link_tests.rs#L225) `r#"<a href="a b">x</a>"#`                                                                  | Expected              | Yes           | No                  |
| [`a_link_destination_escapes_its_backslashes`](tests/link_tests.rs#L212) / [input 4, Faithful](tests/link_tests.rs#L231) `r#"<a href="a(b">x</a>"#`                                                                  | Expected              | Yes           | No                  |
| [`a_link_title_escapes_its_backslashes`](tests/link_tests.rs#L243) / [input 1, Faithful](tests/link_tests.rs#L246) `r#"<a href="x" title="a">t</a>"#`                                                                | Expected              | Yes           | No                  |
| [`a_link_title_escapes_its_backslashes`](tests/link_tests.rs#L243) / [input 2, Faithful](tests/link_tests.rs#L253) `r#"<a href="x" title="a*b">t</a>"#`                                                              | Expected              | Yes           | No                  |
| [`a_link_title_escapes_its_backslashes`](tests/link_tests.rs#L243) / [input 3, Faithful](tests/link_tests.rs#L257) `r#"<img src="i" alt="q" title="a">"#`                                                            | Expected              | Yes           | No                  |
| [`an_image_alt_escapes_its_markdown_specials`](tests/link_tests.rs#L268) / [input 1, Faithful](tests/link_tests.rs#L271) `r#"<img src="x" alt="a](y)b">"#`                                                           | Expected              | Yes           | Faithful            |
| [`an_image_alt_escapes_its_markdown_specials`](tests/link_tests.rs#L268) / [input 2, Faithful](tests/link_tests.rs#L280) `r#"<img src="x" alt="a[b">"#`                                                              | Expected              | Yes           | No                  |
| [`an_image_alt_escapes_its_markdown_specials`](tests/link_tests.rs#L268) / [input 3, Faithful](tests/link_tests.rs#L284) ``r#"<img src="x" alt="a\b*c_d`e$f">"#``                                                    | Expected              | Yes           | No                  |
| [`an_image_alt_escapes_its_raw_html`](tests/link_tests.rs#L296) / [input 1, Faithful](tests/link_tests.rs#L299) `r#"<img src="x" alt='a<b c="*">d'>"#`                                                               | Expected              | Yes           | Faithful            |
| [`an_image_alt_escapes_its_raw_html`](tests/link_tests.rs#L296) / [input 2, Faithful](tests/link_tests.rs#L310) `r#"<img src="x" alt="a < b">"#`                                                                     | Expected              | Yes           | No                  |
| [`a_bracketed_link_destination_escapes_its_angle_brackets`](tests/link_tests.rs#L320) / [input 1, Faithful](tests/link_tests.rs#L323) `r#"<img src="a b>c" alt="q">"#`                                               | Expected              | Yes           | Faithful            |
| [`a_bracketed_link_destination_escapes_its_angle_brackets`](tests/link_tests.rs#L320) / [input 2, Faithful](tests/link_tests.rs#L327) `r#"<img src="a b<c" alt="q">"#`                                               | Expected              | Yes           | No                  |
| [`an_autolink_spells_its_destination_without_escapes`](tests/link_tests.rs#L342) / [input 1, Faithful, autolinks](tests/link_tests.rs#L345) `r#"<a href="https://example.com/a\b">https://example.com/a\b</a>"#`     | Configured + expected | Yes           | No                  |
| [`an_autolink_spells_its_destination_without_escapes`](tests/link_tests.rs#L342) / [input 2, Faithful, autolinks](tests/link_tests.rs#L350) `r#"<a href="https://example.com/a_b*c">https://example.com/a_b*c</a>"#` | Configured + expected | Yes           | Faithful            |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366) / [input 1, Faithful, autolinks](tests/link_tests.rs#L370) `r#"<a href="https://example.com/a<b">https://example.com/a&lt;b</a>"#`                   | Configured + expected | Yes           | No                  |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366) / [input 2, Faithful, autolinks](tests/link_tests.rs#L375) `r#"<a href="https://example.com/?a&amp;b">https://example.com/?a&amp;b</a>"#`            | Configured + expected | Yes           | No                  |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366) / [input 3, Faithful, autolinks](tests/link_tests.rs#L382) `r#"<a href="https://example.com" title="t">https://example.com</a>"#`                    | Configured + expected | Yes           | No                  |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366) / [input 4, Faithful, autolinks](tests/link_tests.rs#L387) `r#"<a href="https://example.com"><em>https://example.com</em></a>"#`                     | Configured + expected | Yes           | No                  |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366) / [input 5, Faithful, autolinks](tests/link_tests.rs#L391) `r#"<a href="a/b">a/b</a>"#`                                                              | Configured + expected | Yes           | No                  |
| [`an_autolink_needs_every_condition`](tests/link_tests.rs#L366) / [input 6, Faithful, autolinks](tests/link_tests.rs#L393) `r#"<a href="a:b">a:b</a>"#`                                                              | Configured + expected | Yes           | No                  |
| [`a_referenced_link_destination_brackets_its_spaces`](tests/link_tests.rs#L404)                                                                                                                                      | Configured + expected | Yes           | No                  |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413) / [input 1, Faithful](tests/link_tests.rs#L416) `r#"<p>Hi!<a href="x">t</a></p>"#`                                                                     | Configured            | Yes           | Faithful            |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413) / [input 2, Faithful, referenced links](tests/link_tests.rs#L424) `r#"<p>Hi!<a href="x">t</a></p>"#`                                                   | Configured            | Yes           | No                  |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413) / [input 3, Pure](tests/link_tests.rs#L432) `r#"<p>Hi!<span><a href="x">t</a></span></p>"#`                                                            | No                    | Yes           | No                  |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413) / [input 4, Faithful](tests/link_tests.rs#L437) `r#"<p>!<a href="x">t</a></p>"#`                                                                       | Configured            | Yes           | No                  |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413) / [input 5, Faithful](tests/link_tests.rs#L442) `r#"<p>Hi! <a href="x">t</a></p>"#`                                                                    | Configured            | Yes           | No                  |
| [`a_bang_before_a_link_is_escaped`](tests/link_tests.rs#L413) / [input 6, Faithful](tests/link_tests.rs#L446) `r#"<p><img src="i" alt="a"><img src="j" alt="b"></p>"#`                                               | Configured            | Yes           | No                  |
| [`a_link_escapes_its_ampersands`](tests/link_tests.rs#L454) / [input 1, Faithful](tests/link_tests.rs#L457) `r#"<a href="a&amp;copy;b" title="t&amp;copy;">x&amp;copy;</a>"#`                                        | Configured + expected | Yes           | Faithful            |
| [`a_link_escapes_its_ampersands`](tests/link_tests.rs#L454) / [input 2, Faithful](tests/link_tests.rs#L465) `r#"<img src="i&amp;copy;" alt="a&amp;copy;" title="t&amp;copy;">"#`                                     | Configured + expected | Yes           | Faithful            |
| [`a_link_escapes_its_ampersands`](tests/link_tests.rs#L454) / [input 3, Faithful, referenced links](tests/link_tests.rs#L476) `r#"<a href="a&amp;b">t</a>"#`                                                         | Configured + expected | Yes           | No                  |
| [`link_text_escapes_its_math_delimiters`](tests/link_tests.rs#L483) / [input 1, Faithful](tests/link_tests.rs#L486) `r#"<p><a href="u">$a](v)$</a></p>"#`                                                            | Configured            | Yes           | Faithful            |
| [`link_text_escapes_its_math_delimiters`](tests/link_tests.rs#L483) / [input 2, Faithful](tests/link_tests.rs#L492) `r#"<p><a href="u"><span class="math math-inline">a](v)</span></a></p>"#`                        | Configured            | Yes           | Faithful            |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503) / [input 1, Faithful](tests/link_tests.rs#L504) `r#"<a href="">t</a>"#`                                                          | Expected              | Yes           | Faithful            |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503) / [input 2, Faithful, referenced links](tests/link_tests.rs#L512) `r#"<a href="">t</a>"#`                                        | Expected              | Yes           | No                  |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503) / [input 3, Faithful](tests/link_tests.rs#L516) `r#"<a href="&lt;a">t</a>"#`                                                     | Expected              | Yes           | Faithful            |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503) / [input 4, Faithful](tests/link_tests.rs#L524) `"<a href="a\tb">t</a>"`                                                         | Expected              | Yes           | Faithful            |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503) / [input 5, Faithful](tests/link_tests.rs#L532) `r#"<a href="a&lt;b&gt;c">t</a>"#`                                               | Expected              | Yes           | Faithful            |
| [`a_link_destination_takes_pointy_brackets_where_needed`](tests/link_tests.rs#L503) / [input 6, Faithful](tests/link_tests.rs#L540) `r#"<img src="" alt="q">"#`                                                      | Expected              | Yes           | No                  |
| [`link_text_encodes_its_line_endings`](tests/link_tests.rs#L547) / [input 1, Pure](tests/link_tests.rs#L551) `r#"<a href="x"><p>a</p><p>b</p></a>"#`                                                                 | No                    | Yes           | No                  |
| [`link_text_encodes_its_line_endings`](tests/link_tests.rs#L547) / [input 2, Faithful](tests/link_tests.rs#L557) `r#"<a href="x"><p>a</p><p>b</p></a>"#`                                                             | Expected              | Yes           | Faithful            |
| [`link_text_encodes_its_line_endings`](tests/link_tests.rs#L547) / [input 3, Faithful, referenced links](tests/link_tests.rs#L565) `r#"<a href="x"><p>a</p><p>b</p></a>"#`                                           | Expected              | Yes           | No                  |
| [`link_text_encodes_its_line_endings`](tests/link_tests.rs#L547) / [input 4, Pure](tests/link_tests.rs#L570) `r#"<h1><a href="x">a<br>b</a></h1>"#`                                                                  | No                    | Yes           | No                  |
| [`an_image_is_inline_in_the_referenced_style`](tests/link_tests.rs#L576)                                                                                                                                             | Configured + expected | Yes           | No                  |
| [`a_link_keeps_the_whitespace_around_its_text`](tests/link_tests.rs#L586) / [input 1, Faithful](tests/link_tests.rs#L589) `r#"<p>Hi!<a href="x"> t</a></p>"#`                                                        | Expected              | Yes           | Faithful            |
| [`a_link_keeps_the_whitespace_around_its_text`](tests/link_tests.rs#L586) / [input 2, Faithful, referenced links](tests/link_tests.rs#L597) `r#"<p>Hi!<a href="x"> t</a></p>"#`                                      | Expected              | Yes           | No                  |
| [`a_link_keeps_the_whitespace_around_its_text`](tests/link_tests.rs#L586) / [input 3, Faithful](tests/link_tests.rs#L605) `r#"<p>Hi! <a href="x"> t</a></p>"#`                                                       | Expected              | Yes           | No                  |
| [`a_link_keeps_the_whitespace_around_its_text`](tests/link_tests.rs#L586) / [input 4, Faithful](tests/link_tests.rs#L609) `r#"<p><a href="x">t </a>now</p>"#`                                                        | Expected              | Yes           | No                  |
| [`an_image_without_a_source_is_written_as_html`](tests/link_tests.rs#L617)                                                                                                                                           | Yes                   | Yes           | Faithful            |
| [`an_image_without_alt_text_is_written_as_html`](tests/link_tests.rs#L633) / [input 1, Faithful](tests/link_tests.rs#L636) `r#"<p>a<img src="i">b</p>"#`                                                             | Expected              | Yes           | Faithful            |
| [`an_image_without_alt_text_is_written_as_html`](tests/link_tests.rs#L633) / [input 2, Faithful](tests/link_tests.rs#L645) `r#"<img src="i" alt="">"#`                                                               | Expected              | Yes           | Faithful            |
| [`an_image_with_an_empty_title_is_written_as_html`](tests/link_tests.rs#L659)                                                                                                                                        | Yes                   | Yes           | Faithful            |
| [`a_link_escapes_an_email_autolink`](tests/link_tests.rs#L675) / [input 1, Faithful](tests/link_tests.rs#L678) `r#"<a href="x">&lt;1foo@bar.com&gt;</a>"#`                                                           | Expected              | Yes           | Faithful            |
| [`a_link_escapes_an_email_autolink`](tests/link_tests.rs#L675) / [input 2, Faithful](tests/link_tests.rs#L686) `r#"<img src="i" alt="<+a@b.co>">"#`                                                                  | Expected              | Yes           | Faithful            |

### [tests/list\_tests.rs](tests/list_tests.rs) (8)

| Test                                                                                                                                                                                            | Faithful round trip   | Keep Markdown | Existing round trip |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`unordered_lists`](tests/list_tests.rs#L6)                                                                                                                                                     | Yes                   | Yes           | No                  |
| [`unordered_lists_custom_bullet_spacing`](tests/list_tests.rs#L21)                                                                                                                              | Configured            | Yes           | No                  |
| [`ordered_lists`](tests/list_tests.rs#L38)                                                                                                                                                      | Yes                   | Yes           | No                  |
| [`ordered_lists_custom_bullet_spacing`](tests/list_tests.rs#L53)                                                                                                                                | Configured            | Yes           | No                  |
| [`ordered_lists_start_with_zero_or_negative`](tests/list_tests.rs#L70)                                                                                                                          | Configured + expected | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95) / [input 1, Faithful, UL spacing 0](tests/list_tests.rs#L108) `<ul><li>Item 1</li><li>Item 2</li></ul>`                  | Configured            | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95) / [input 2, Faithful, UL spacing 9](tests/list_tests.rs#L109) `<ul><li>Item 1</li><li>Item 2</li></ul>`                  | Configured            | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95) / [input 3, Faithful, OL spacing 0](tests/list_tests.rs#L110) `<ol><li>Item 1</li><li>Item 2</li></ol>`                  | Configured            | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95) / [input 4, Faithful, OL spacing 9](tests/list_tests.rs#L111) `<ol><li>Item 1</li><li>Item 2</li></ol>`                  | Configured            | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95) / [input 5, Faithful, UL spacing 9](tests/list_tests.rs#L121) `<ul><li>Outer<ul><li>Inner</li></ul></li></ul>`           | Configured            | Yes           | No                  |
| [`list_spacing_outside_the_commonmark_range`](tests/list_tests.rs#L95) / [input 6, Faithful, OL spacing 9](tests/list_tests.rs#L125) `<ol><li>Outer<ol><li>Inner</li></ol></li></ol>`           | Configured            | Yes           | No                  |
| [`ordered_list_alignment_across_a_digit_boundary`](tests/list_tests.rs#L130) / [input 1, Faithful](tests/list_tests.rs#L135) `format!(r#"<ol start="998">{}</ol>"#, "<li>Item</li>".repeat(5))` | Yes                   | Yes           | No                  |
| [`ordered_list_alignment_across_a_digit_boundary`](tests/list_tests.rs#L130) / [input 2, Faithful](tests/list_tests.rs#L142) `format!(r#"<ol start="9">{}</ol>"#, "<li>Item</li>".repeat(2))`   | Yes                   | Yes           | No                  |
| [`ordered_list_alignment_yields_to_the_spacing_limit`](tests/list_tests.rs#L146)                                                                                                                | Yes                   | Yes           | No                  |

### [tests/table\_tests.rs](tests/table_tests.rs) (32)

| Test                                                                                                                                                                                                                                 | Faithful round trip   | Keep Markdown | Existing round trip |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------- | ------------- | ------------------- |
| [`test_simple_table`](tests/table_tests.rs#L27)                                                                                                                                                                                      | Configured + expected | Yes           | No                  |
| [`table_rows_do_not_drop_cells_beyond_the_header_width`](tests/table_tests.rs#L59)                                                                                                                                                   | No                    | Yes           | No                  |
| [`malformed_table_foster_parenting_does_not_lose_visible_text`](tests/table_tests.rs#L88)                                                                                                                                            | No                    | Yes           | No                  |
| [`test_table_with_thead_tbody`](tests/table_tests.rs#L107)                                                                                                                                                                           | Configured            | Yes           | No                  |
| [`test_table_with_thead_td_headers`](tests/table_tests.rs#L146)                                                                                                                                                                      | No                    | Yes           | No                  |
| [`test_table_with_caption`](tests/table_tests.rs#L182)                                                                                                                                                                               | Expected              | Yes           | No                  |
| [`test_table_caption_is_commonmark_only_in_pure_mode`](tests/table_tests.rs#L213) / [input 1, Pure](tests/table_tests.rs#L222) `concat!( "<table><caption>Sample Table</caption>", "<thead><tr><th>h</th></tr></thead…`              | No                    | Yes           | No                  |
| [`test_table_caption_is_commonmark_only_in_pure_mode`](tests/table_tests.rs#L213) / [input 2, Faithful](tests/table_tests.rs#L226) `concat!( "<table><caption>Sample Table</caption>", "<thead><tr><th>h</th></tr></thead…`          | Yes                   | Yes           | No                  |
| [`a_table_with_no_header_row_is_written_as_html`](tests/table_tests.rs#L232) / [input 1, Faithful](tests/table_tests.rs#L237) `concat!( "<table><tbody><tr><td>a</td><td>b</td></tr>", "<tr><td>c</td><td>d</td></tr…`               | Yes                   | Yes           | No                  |
| [`a_table_with_no_header_row_is_written_as_html`](tests/table_tests.rs#L232) / [input 2, Faithful](tests/table_tests.rs#L242) `"<table><tbody><tr><td><p>a</p></td></tr></tbody></table>"`                                           | Yes                   | Yes           | No                  |
| [`pure_mode_writes_an_empty_header_row_for_a_headerless_table`](tests/table_tests.rs#L249) / [input 1, Pure](tests/table_tests.rs#L254) `"<table><thead></thead><tbody><tr><td>a</td></tr></tbody></table>"`                         | No                    | Yes           | No                  |
| [`pure_mode_writes_an_empty_header_row_for_a_headerless_table`](tests/table_tests.rs#L249) / [input 2, Faithful](tests/table_tests.rs#L256) `"<table><thead></thead><tbody><tr><td>a</td></tr></tbody></table>"`                     | Yes                   | Yes           | No                  |
| [`extra_header_rows_are_written_as_html`](tests/table_tests.rs#L274)                                                                                                                                                                 | Yes                   | Yes           | No                  |
| [`a_header_cell_in_a_body_row_is_written_as_html`](tests/table_tests.rs#L288)                                                                                                                                                        | Yes                   | Yes           | No                  |
| [`table_content_a_markdown_row_cannot_hold_is_written_as_html`](tests/table_tests.rs#L301) / [input 1, Faithful](tests/table_tests.rs#L303) `concat!( "<table><colgroup><col></colgroup><thead><tr><th>A</th></tr></thead>", "<tbo…` | Yes                   | Yes           | No                  |
| [`table_content_a_markdown_row_cannot_hold_is_written_as_html`](tests/table_tests.rs#L301) / [input 2, Faithful](tests/table_tests.rs#L307) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><script>x</script><tr><…` | Yes                   | Yes           | No                  |
| [`table_content_a_markdown_row_cannot_hold_is_written_as_html`](tests/table_tests.rs#L301) / [input 3, Faithful](tests/table_tests.rs#L311) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><tr><td>c</td></tr><tr>…` | Yes                   | Yes           | No                  |
| [`a_thead_of_data_cells_is_written_as_html`](tests/table_tests.rs#L323)                                                                                                                                                              | Yes                   | Yes           | No                  |
| [`a_second_tbody_is_written_as_html`](tests/table_tests.rs#L336)                                                                                                                                                                     | Yes                   | Yes           | No                  |
| [`a_second_thead_is_written_as_html`](tests/table_tests.rs#L351)                                                                                                                                                                     | Yes                   | Yes           | No                  |
| [`a_thead_after_a_body_row_is_written_as_html`](tests/table_tests.rs#L368) / [input 1, Faithful](tests/table_tests.rs#L370) `concat!( "<table><tbody><tr><td>c</td></tr></tbody>", "<thead><tr><th>B</th></tr></th…`                 | Yes                   | Yes           | No                  |
| [`a_thead_after_a_body_row_is_written_as_html`](tests/table_tests.rs#L368) / [input 2, Faithful](tests/table_tests.rs#L374) `concat!( "<table><tbody><tr><th>A</th></tr><tr><td>c</td></tr></tbody>", "<thead><tr>…`                 | Yes                   | Yes           | No                  |
| [`a_thead_after_an_empty_tbody_is_written_as_html`](tests/table_tests.rs#L388)                                                                                                                                                       | Yes                   | Yes           | No                  |
| [`a_header_row_after_a_body_row_is_written_as_html`](tests/table_tests.rs#L404)                                                                                                                                                      | Yes                   | Yes           | No                  |
| [`a_row_wider_or_narrower_than_the_header_is_written_as_html`](tests/table_tests.rs#L415) / [input 1, Faithful](tests/table_tests.rs#L416) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><tr><td>c</td><td>d</td…`  | Yes                   | Yes           | No                  |
| [`a_row_wider_or_narrower_than_the_header_is_written_as_html`](tests/table_tests.rs#L415) / [input 2, Faithful](tests/table_tests.rs#L423) `concat!( "<table><thead><tr><th>A</th><th>B</th></tr></thead>", "<tbody><tr><td>c</td…`  | Yes                   | Yes           | No                  |
| [`a_comment_in_a_table_is_written_as_html`](tests/table_tests.rs#L436) / [input 1, Faithful](tests/table_tests.rs#L438) `concat!( "<table><!-- c --><thead><tr><th>A</th></tr></thead>", "<tbody><tr><td>c</td…`                     | Yes                   | Yes           | No                  |
| [`a_comment_in_a_table_is_written_as_html`](tests/table_tests.rs#L436) / [input 2, Faithful](tests/table_tests.rs#L442) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><!-- c --><tr><td>c</td…`                     | Yes                   | Yes           | No                  |
| [`a_comment_in_a_table_is_written_as_html`](tests/table_tests.rs#L436) / [input 3, Faithful](tests/table_tests.rs#L446) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><tr><!-- c --><td>c</td…`                     | Yes                   | Yes           | No                  |
| [`a_comment_in_a_table_is_written_as_html`](tests/table_tests.rs#L436) / [input 4, Faithful](tests/table_tests.rs#L460) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody>stray<tr><td>c</td></tr…`                     | Yes                   | Yes           | No                  |
| [`a_pipe_in_an_open_tag_is_encoded`](tests/table_tests.rs#L469) / [input 1, Faithful](tests/table_tests.rs#L480) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><tr><td><span title="a…`                             | Yes                   | Yes           | No                  |
| [`a_pipe_in_an_open_tag_is_encoded`](tests/table_tests.rs#L469) / [input 2, Faithful](tests/table_tests.rs#L493) `concat!( "<table><thead><tr><th><span title="a&#124;b">c</span></th></tr></thead>", "<t…`                          | Yes                   | Yes           | No                  |
| [`a_pipe_in_an_open_tag_is_encoded`](tests/table_tests.rs#L469) / [input 3, Pure](tests/table_tests.rs#L499) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><tr><td><span title="a…`                                 | No                    | Yes           | No                  |
| [`a_comment_holding_a_pipe_is_written_as_html`](tests/table_tests.rs#L507) / [input 1, Faithful](tests/table_tests.rs#L517) `concat!( "<table><tbody><tr><th><!-- a&#124;b --></th></tr>", "<tr><td>c</td></tr></tbody…`             | Yes                   | Yes           | No                  |
| [`a_comment_holding_a_pipe_is_written_as_html`](tests/table_tests.rs#L507) / [input 2, Faithful](tests/table_tests.rs#L521) `concat!( "<table><tbody><tr><th>A</th></tr><tr><td>", "<table><tbody><tr><td><!-- a&#124;…`             | Yes                   | Yes           | No                  |
| [`a_comment_holding_a_pipe_is_written_as_html`](tests/table_tests.rs#L507) / [input 3, Faithful](tests/table_tests.rs#L509) `concat!( "<table><tbody><tr><th>A</th></tr>", "<tr><td><!-- a&#124;b --></td></tr></tbody…`             | Yes                   | Yes           | No                  |
| [`a_comment_holding_a_pipe_is_written_as_html`](tests/table_tests.rs#L507) / [input 4, Faithful](tests/table_tests.rs#L539) `concat!( "<table><thead><tr><th>A</th></tr></thead>", "<tbody><tr><td>a<!-- c -->b</t…`                 | Yes                   | Yes           | No                  |
| [`an_empty_column_is_one_wide`](tests/table_tests.rs#L548) / [input 1, Faithful](tests/table_tests.rs#L555) `concat!( "<table><thead><tr><th></th></tr></thead>", "<tbody><tr><td></td></tr></tbod…`                                 | Configured + expected | Yes           | No                  |
| [`an_empty_column_is_one_wide`](tests/table_tests.rs#L548) / [input 2, Faithful](tests/table_tests.rs#L564) `concat!( "<table><thead><tr><th></th><th>B</th></tr></thead>", "<tbody><tr><td></td><…`                                 | Configured + expected | Yes           | No                  |
| [`an_empty_table_is_written_as_html`](tests/table_tests.rs#L571) / [input 1, Faithful](tests/table_tests.rs#L572) `"<table></table>"`                                                                                                | Yes                   | Yes           | No                  |
| [`an_empty_table_is_written_as_html`](tests/table_tests.rs#L571) / [input 2, Faithful](tests/table_tests.rs#L573) `"<table><tbody></tbody></table>"`                                                                                 | Yes                   | Yes           | No                  |
| [`an_empty_table_is_written_as_html`](tests/table_tests.rs#L571) / [input 3, Faithful](tests/table_tests.rs#L574) `"<table><thead></thead></table>"`                                                                                 | Yes                   | Yes           | No                  |
| [`test_table_with_inlines`](tests/table_tests.rs#L579)                                                                                                                                                                               | Configured + expected | Yes           | No                  |
| [`test_table_block_cells`](tests/table_tests.rs#L643)                                                                                                                                                                                | Configured + expected | Yes           | No                  |
| [`test_hacker_news_layout_table_does_not_turn_into_giant_padded_rows`](tests/table_tests.rs#L675)                                                                                                                                    | No                    | Yes           | No                  |
| [`test_headerless_table`](tests/table_tests.rs#L697)                                                                                                                                                                                 | No                    | Yes           | No                  |
| [`test_headered_with_inner_headerless`](tests/table_tests.rs#L718)                                                                                                                                                                   | No                    | Yes           | No                  |
| [`test_headerless_with_inner_headered`](tests/table_tests.rs#L757)                                                                                                                                                                   | No                    | Yes           | No                  |
| [`a_row_directly_under_a_table_is_read_as_a_body_row`](tests/table_tests.rs#L801)                                                                                                                                                    | No                    | Yes           | No                  |
| [`test_headered_with_inner_headered`](tests/table_tests.rs#L863)                                                                                                                                                                     | No                    | Yes           | No                  |

### [tests/translation\_approach\_tests.rs](tests/translation_approach_tests.rs) (16)

| Test                                                                                                                                                                                                                                              | Faithful round trip   | Keep Markdown | Existing round trip |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | ------------- | ------------------- |
| [`translating_html_nodes`](tests/translation_approach_tests.rs#L17)                                                                                                                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 1, Faithful](tests/translation_approach_tests.rs#L50) `a<a href="u">x<p>b</p>y</a>c`                                                                                  | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 2, Faithful](tests/translation_approach_tests.rs#L50) `a<a href="u"><p>b</p></a>c`                                                                                    | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 3, Faithful](tests/translation_approach_tests.rs#L51) `a<a href="u">x<h2>b</h2>y</a>c`                                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 4, Faithful](tests/translation_approach_tests.rs#L51) `a<a href="u"><h2>b</h2></a>c`                                                                                  | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 5, Faithful](tests/translation_approach_tests.rs#L52) `a<a href="u">x<ul><li>b</li></ul>y</a>c`                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 6, Faithful](tests/translation_approach_tests.rs#L52) `a<a href="u"><ul><li>b</li></ul></a>c`                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 7, Faithful](tests/translation_approach_tests.rs#L53) `a<a href="u">x<ol><li>b</li></ol>y</a>c`                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 8, Faithful](tests/translation_approach_tests.rs#L53) `a<a href="u"><ol><li>b</li></ol></a>c`                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 9, Faithful](tests/translation_approach_tests.rs#L54) `a<a href="u">x<blockquote>b</blockquote>y</a>c`                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 10, Faithful](tests/translation_approach_tests.rs#L54) `a<a href="u"><blockquote>b</blockquote></a>c`                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 11, Faithful](tests/translation_approach_tests.rs#L55) `a<a href="u">x<pre>b</pre>y</a>c`                                                                             | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 12, Faithful](tests/translation_approach_tests.rs#L55) `a<a href="u"><pre>b</pre></a>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 13, Faithful](tests/translation_approach_tests.rs#L56) `a<a href="u">x<hr>y</a>c`                                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 14, Faithful](tests/translation_approach_tests.rs#L56) `a<a href="u"><hr></a>c`                                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 15, Faithful](tests/translation_approach_tests.rs#L57) `a<a href="u">x<table><tbody><tr><td>b</td></tr></tbody></table>y</a>c`                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 16, Faithful](tests/translation_approach_tests.rs#L57) `a<a href="u"><table><tbody><tr><td>b</td></tr></tbody></table></a>c`                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 17, Faithful](tests/translation_approach_tests.rs#L58) `a<a href="u">x<div>b</div>y</a>c`                                                                             | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 18, Faithful](tests/translation_approach_tests.rs#L58) `a<a href="u"><div>b</div></a>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 19, Faithful](tests/translation_approach_tests.rs#L50) `a<em>x<p>b</p>y</em>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 20, Faithful](tests/translation_approach_tests.rs#L50) `a<em><p>b</p></em>c`                                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 21, Faithful](tests/translation_approach_tests.rs#L51) `a<em>x<h2>b</h2>y</em>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 22, Faithful](tests/translation_approach_tests.rs#L51) `a<em><h2>b</h2></em>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 23, Faithful](tests/translation_approach_tests.rs#L52) `a<em>x<ul><li>b</li></ul>y</em>c`                                                                             | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 24, Faithful](tests/translation_approach_tests.rs#L52) `a<em><ul><li>b</li></ul></em>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 25, Faithful](tests/translation_approach_tests.rs#L53) `a<em>x<ol><li>b</li></ol>y</em>c`                                                                             | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 26, Faithful](tests/translation_approach_tests.rs#L53) `a<em><ol><li>b</li></ol></em>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 27, Faithful](tests/translation_approach_tests.rs#L54) `a<em>x<blockquote>b</blockquote>y</em>c`                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 28, Faithful](tests/translation_approach_tests.rs#L54) `a<em><blockquote>b</blockquote></em>c`                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 29, Faithful](tests/translation_approach_tests.rs#L55) `a<em>x<pre>b</pre>y</em>c`                                                                                    | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 30, Faithful](tests/translation_approach_tests.rs#L55) `a<em><pre>b</pre></em>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 31, Faithful](tests/translation_approach_tests.rs#L56) `a<em>x<hr>y</em>c`                                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 32, Faithful](tests/translation_approach_tests.rs#L56) `a<em><hr></em>c`                                                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 33, Faithful](tests/translation_approach_tests.rs#L57) `a<em>x<table><tbody><tr><td>b</td></tr></tbody></table>y</em>c`                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 34, Faithful](tests/translation_approach_tests.rs#L57) `a<em><table><tbody><tr><td>b</td></tr></tbody></table></em>c`                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 35, Faithful](tests/translation_approach_tests.rs#L58) `a<em>x<div>b</div>y</em>c`                                                                                    | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 36, Faithful](tests/translation_approach_tests.rs#L58) `a<em><div>b</div></em>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 37, Faithful](tests/translation_approach_tests.rs#L50) `a<i>x<p>b</p>y</i>c`                                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 38, Faithful](tests/translation_approach_tests.rs#L50) `a<i><p>b</p></i>c`                                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 39, Faithful](tests/translation_approach_tests.rs#L51) `a<i>x<h2>b</h2>y</i>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 40, Faithful](tests/translation_approach_tests.rs#L51) `a<i><h2>b</h2></i>c`                                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 41, Faithful](tests/translation_approach_tests.rs#L52) `a<i>x<ul><li>b</li></ul>y</i>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 42, Faithful](tests/translation_approach_tests.rs#L52) `a<i><ul><li>b</li></ul></i>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 43, Faithful](tests/translation_approach_tests.rs#L53) `a<i>x<ol><li>b</li></ol>y</i>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 44, Faithful](tests/translation_approach_tests.rs#L53) `a<i><ol><li>b</li></ol></i>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 45, Faithful](tests/translation_approach_tests.rs#L54) `a<i>x<blockquote>b</blockquote>y</i>c`                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 46, Faithful](tests/translation_approach_tests.rs#L54) `a<i><blockquote>b</blockquote></i>c`                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 47, Faithful](tests/translation_approach_tests.rs#L55) `a<i>x<pre>b</pre>y</i>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 48, Faithful](tests/translation_approach_tests.rs#L55) `a<i><pre>b</pre></i>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 49, Faithful](tests/translation_approach_tests.rs#L56) `a<i>x<hr>y</i>c`                                                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 50, Faithful](tests/translation_approach_tests.rs#L56) `a<i><hr></i>c`                                                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 51, Faithful](tests/translation_approach_tests.rs#L57) `a<i>x<table><tbody><tr><td>b</td></tr></tbody></table>y</i>c`                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 52, Faithful](tests/translation_approach_tests.rs#L57) `a<i><table><tbody><tr><td>b</td></tr></tbody></table></i>c`                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 53, Faithful](tests/translation_approach_tests.rs#L58) `a<i>x<div>b</div>y</i>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 54, Faithful](tests/translation_approach_tests.rs#L58) `a<i><div>b</div></i>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 55, Faithful](tests/translation_approach_tests.rs#L50) `a<strong>x<p>b</p>y</strong>c`                                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 56, Faithful](tests/translation_approach_tests.rs#L50) `a<strong><p>b</p></strong>c`                                                                                  | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 57, Faithful](tests/translation_approach_tests.rs#L51) `a<strong>x<h2>b</h2>y</strong>c`                                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 58, Faithful](tests/translation_approach_tests.rs#L51) `a<strong><h2>b</h2></strong>c`                                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 59, Faithful](tests/translation_approach_tests.rs#L52) `a<strong>x<ul><li>b</li></ul>y</strong>c`                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 60, Faithful](tests/translation_approach_tests.rs#L52) `a<strong><ul><li>b</li></ul></strong>c`                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 61, Faithful](tests/translation_approach_tests.rs#L53) `a<strong>x<ol><li>b</li></ol>y</strong>c`                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 62, Faithful](tests/translation_approach_tests.rs#L53) `a<strong><ol><li>b</li></ol></strong>c`                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 63, Faithful](tests/translation_approach_tests.rs#L54) `a<strong>x<blockquote>b</blockquote>y</strong>c`                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 64, Faithful](tests/translation_approach_tests.rs#L54) `a<strong><blockquote>b</blockquote></strong>c`                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 65, Faithful](tests/translation_approach_tests.rs#L55) `a<strong>x<pre>b</pre>y</strong>c`                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 66, Faithful](tests/translation_approach_tests.rs#L55) `a<strong><pre>b</pre></strong>c`                                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 67, Faithful](tests/translation_approach_tests.rs#L56) `a<strong>x<hr>y</strong>c`                                                                                    | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 68, Faithful](tests/translation_approach_tests.rs#L56) `a<strong><hr></strong>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 69, Faithful](tests/translation_approach_tests.rs#L57) `a<strong>x<table><tbody><tr><td>b</td></tr></tbody></table>y</strong>c`                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 70, Faithful](tests/translation_approach_tests.rs#L57) `a<strong><table><tbody><tr><td>b</td></tr></tbody></table></strong>c`                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 71, Faithful](tests/translation_approach_tests.rs#L58) `a<strong>x<div>b</div>y</strong>c`                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 72, Faithful](tests/translation_approach_tests.rs#L58) `a<strong><div>b</div></strong>c`                                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 73, Faithful](tests/translation_approach_tests.rs#L50) `a<b>x<p>b</p>y</b>c`                                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 74, Faithful](tests/translation_approach_tests.rs#L50) `a<b><p>b</p></b>c`                                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 75, Faithful](tests/translation_approach_tests.rs#L51) `a<b>x<h2>b</h2>y</b>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 76, Faithful](tests/translation_approach_tests.rs#L51) `a<b><h2>b</h2></b>c`                                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 77, Faithful](tests/translation_approach_tests.rs#L52) `a<b>x<ul><li>b</li></ul>y</b>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 78, Faithful](tests/translation_approach_tests.rs#L52) `a<b><ul><li>b</li></ul></b>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 79, Faithful](tests/translation_approach_tests.rs#L53) `a<b>x<ol><li>b</li></ol>y</b>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 80, Faithful](tests/translation_approach_tests.rs#L53) `a<b><ol><li>b</li></ol></b>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 81, Faithful](tests/translation_approach_tests.rs#L54) `a<b>x<blockquote>b</blockquote>y</b>c`                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 82, Faithful](tests/translation_approach_tests.rs#L54) `a<b><blockquote>b</blockquote></b>c`                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 83, Faithful](tests/translation_approach_tests.rs#L55) `a<b>x<pre>b</pre>y</b>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 84, Faithful](tests/translation_approach_tests.rs#L55) `a<b><pre>b</pre></b>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 85, Faithful](tests/translation_approach_tests.rs#L56) `a<b>x<hr>y</b>c`                                                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 86, Faithful](tests/translation_approach_tests.rs#L56) `a<b><hr></b>c`                                                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 87, Faithful](tests/translation_approach_tests.rs#L57) `a<b>x<table><tbody><tr><td>b</td></tr></tbody></table>y</b>c`                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 88, Faithful](tests/translation_approach_tests.rs#L57) `a<b><table><tbody><tr><td>b</td></tr></tbody></table></b>c`                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 89, Faithful](tests/translation_approach_tests.rs#L58) `a<b>x<div>b</div>y</b>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 90, Faithful](tests/translation_approach_tests.rs#L58) `a<b><div>b</div></b>c`                                                                                        | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 91, Faithful](tests/translation_approach_tests.rs#L50) `a<code>x<p>b</p>y</code>c`                                                                                    | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 92, Faithful](tests/translation_approach_tests.rs#L50) `a<code><p>b</p></code>c`                                                                                      | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 93, Faithful](tests/translation_approach_tests.rs#L51) `a<code>x<h2>b</h2>y</code>c`                                                                                  | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 94, Faithful](tests/translation_approach_tests.rs#L51) `a<code><h2>b</h2></code>c`                                                                                    | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 95, Faithful](tests/translation_approach_tests.rs#L52) `a<code>x<ul><li>b</li></ul>y</code>c`                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 96, Faithful](tests/translation_approach_tests.rs#L52) `a<code><ul><li>b</li></ul></code>c`                                                                           | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 97, Faithful](tests/translation_approach_tests.rs#L53) `a<code>x<ol><li>b</li></ol>y</code>c`                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 98, Faithful](tests/translation_approach_tests.rs#L53) `a<code><ol><li>b</li></ol></code>c`                                                                           | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 99, Faithful](tests/translation_approach_tests.rs#L54) `a<code>x<blockquote>b</blockquote>y</code>c`                                                                  | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 100, Faithful](tests/translation_approach_tests.rs#L54) `a<code><blockquote>b</blockquote></code>c`                                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 101, Faithful](tests/translation_approach_tests.rs#L55) `a<code>x<pre>b</pre>y</code>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 102, Faithful](tests/translation_approach_tests.rs#L55) `a<code><pre>b</pre></code>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 103, Faithful](tests/translation_approach_tests.rs#L56) `a<code>x<hr>y</code>c`                                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 104, Faithful](tests/translation_approach_tests.rs#L56) `a<code><hr></code>c`                                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 105, Faithful](tests/translation_approach_tests.rs#L57) `a<code>x<table><tbody><tr><td>b</td></tr></tbody></table>y</code>c`                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 106, Faithful](tests/translation_approach_tests.rs#L57) `a<code><table><tbody><tr><td>b</td></tr></tbody></table></code>c`                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 107, Faithful](tests/translation_approach_tests.rs#L58) `a<code>x<div>b</div>y</code>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 108, Faithful](tests/translation_approach_tests.rs#L58) `a<code><div>b</div></code>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 109, Faithful](tests/translation_approach_tests.rs#L50) `a<del>x<p>b</p>y</del>c`                                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 110, Faithful](tests/translation_approach_tests.rs#L50) `a<del><p>b</p></del>c`                                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 111, Faithful](tests/translation_approach_tests.rs#L51) `a<del>x<h2>b</h2>y</del>c`                                                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 112, Faithful](tests/translation_approach_tests.rs#L51) `a<del><h2>b</h2></del>c`                                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 113, Faithful](tests/translation_approach_tests.rs#L52) `a<del>x<ul><li>b</li></ul>y</del>c`                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 114, Faithful](tests/translation_approach_tests.rs#L52) `a<del><ul><li>b</li></ul></del>c`                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 115, Faithful](tests/translation_approach_tests.rs#L53) `a<del>x<ol><li>b</li></ol>y</del>c`                                                                          | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 116, Faithful](tests/translation_approach_tests.rs#L53) `a<del><ol><li>b</li></ol></del>c`                                                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 117, Faithful](tests/translation_approach_tests.rs#L54) `a<del>x<blockquote>b</blockquote>y</del>c`                                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 118, Faithful](tests/translation_approach_tests.rs#L54) `a<del><blockquote>b</blockquote></del>c`                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 119, Faithful](tests/translation_approach_tests.rs#L55) `a<del>x<pre>b</pre>y</del>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 120, Faithful](tests/translation_approach_tests.rs#L55) `a<del><pre>b</pre></del>c`                                                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 121, Faithful](tests/translation_approach_tests.rs#L56) `a<del>x<hr>y</del>c`                                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 122, Faithful](tests/translation_approach_tests.rs#L56) `a<del><hr></del>c`                                                                                           | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 123, Faithful](tests/translation_approach_tests.rs#L57) `a<del>x<table><tbody><tr><td>b</td></tr></tbody></table>y</del>c`                                            | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 124, Faithful](tests/translation_approach_tests.rs#L57) `a<del><table><tbody><tr><td>b</td></tr></tbody></table></del>c`                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 125, Faithful](tests/translation_approach_tests.rs#L58) `a<del>x<div>b</div>y</del>c`                                                                                 | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 126, Faithful](tests/translation_approach_tests.rs#L58) `a<del><div>b</div></del>c`                                                                                   | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 127, Faithful](tests/translation_approach_tests.rs#L50) `a<span class="s">x<p>b</p>y</span>c`                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 128, Faithful](tests/translation_approach_tests.rs#L50) `a<span class="s"><p>b</p></span>c`                                                                           | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 129, Faithful](tests/translation_approach_tests.rs#L51) `a<span class="s">x<h2>b</h2>y</span>c`                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 130, Faithful](tests/translation_approach_tests.rs#L51) `a<span class="s"><h2>b</h2></span>c`                                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 131, Faithful](tests/translation_approach_tests.rs#L52) `a<span class="s">x<ul><li>b</li></ul>y</span>c`                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 132, Faithful](tests/translation_approach_tests.rs#L52) `a<span class="s"><ul><li>b</li></ul></span>c`                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 133, Faithful](tests/translation_approach_tests.rs#L53) `a<span class="s">x<ol><li>b</li></ol>y</span>c`                                                              | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 134, Faithful](tests/translation_approach_tests.rs#L53) `a<span class="s"><ol><li>b</li></ol></span>c`                                                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 135, Faithful](tests/translation_approach_tests.rs#L54) `a<span class="s">x<blockquote>b</blockquote>y</span>c`                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 136, Faithful](tests/translation_approach_tests.rs#L54) `a<span class="s"><blockquote>b</blockquote></span>c`                                                         | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 137, Faithful](tests/translation_approach_tests.rs#L55) `a<span class="s">x<pre>b</pre>y</span>c`                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 138, Faithful](tests/translation_approach_tests.rs#L55) `a<span class="s"><pre>b</pre></span>c`                                                                       | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 139, Faithful](tests/translation_approach_tests.rs#L56) `a<span class="s">x<hr>y</span>c`                                                                             | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 140, Faithful](tests/translation_approach_tests.rs#L56) `a<span class="s"><hr></span>c`                                                                               | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 141, Faithful](tests/translation_approach_tests.rs#L57) `a<span class="s">x<table><tbody><tr><td>b</td></tr></tbody></table>y</span>c`                                | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 142, Faithful](tests/translation_approach_tests.rs#L57) `a<span class="s"><table><tbody><tr><td>b</td></tr></tbody></table></span>c`                                  | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 143, Faithful](tests/translation_approach_tests.rs#L58) `a<span class="s">x<div>b</div>y</span>c`                                                                     | Expected              | Yes           | No                  |
| [`blocks_inside_inlines`](tests/translation_approach_tests.rs#L27) / [input 144, Faithful](tests/translation_approach_tests.rs#L58) `a<span class="s"><div>b</div></span>c`                                                                       | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 1, Faithful](tests/translation_approach_tests.rs#L92) `"<p>a<code>x<br>y</code>b</p>"`                                                                           | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 2, Faithful](tests/translation_approach_tests.rs#L96) `"<p>a<code>x<br><br><br>y</code>b</p>"`                                                                   | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 3, Faithful](tests/translation_approach_tests.rs#L101) `"<p>a<code><br></code>b</p>"`                                                                            | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 4, Faithful](tests/translation_approach_tests.rs#L105) `"<p>a<code><br><br><br></code>b</p>"`                                                                    | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 5, Faithful](tests/translation_approach_tests.rs#L110) `"<pre><code>a<br>b</code></pre>"`                                                                        | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 6, Faithful](tests/translation_approach_tests.rs#L114) `"<pre><code>a<br><br><br>b</code></pre>"`                                                                | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 7, Faithful](tests/translation_approach_tests.rs#L119) `"<pre><code><br></code></pre>"`                                                                          | Expected              | Yes           | No                  |
| [`code_blocks_and_code_spans`](tests/translation_approach_tests.rs#L89) / [input 8, Faithful](tests/translation_approach_tests.rs#L123) `"<pre><code><br><br><br></code></pre>"`                                                                  | Expected              | Yes           | No                  |
| [`code_span_holding_a_line_ending`](tests/translation_approach_tests.rs#L135) / [input 1, Faithful](tests/translation_approach_tests.rs#L139) `"<h1>a<code>x\ny</code>b</h1>"`                                                                    | Configured + expected | Yes           | No                  |
| [`code_span_holding_a_line_ending`](tests/translation_approach_tests.rs#L135) / [input 2, Faithful, setext](tests/translation_approach_tests.rs#L144) `"<h1>a<code>x\ny</code>b</h1>"`                                                            | Configured + expected | Yes           | No                  |
| [`code_span_holding_a_line_ending`](tests/translation_approach_tests.rs#L135) / [input 3, Faithful](tests/translation_approach_tests.rs#L151) `"<p>a<code>x\n\ny</code>b</p>"`                                                                    | Configured + expected | Yes           | No                  |
| [`code_span_holding_a_line_ending`](tests/translation_approach_tests.rs#L135) / [input 4, Faithful](tests/translation_approach_tests.rs#L155) `"<blockquote><p>a<code>x\n\ny</code>b</p></blockquote>"`                                           | Configured + expected | Yes           | No                  |
| [`code_span_holding_a_line_ending`](tests/translation_approach_tests.rs#L135) / [input 5, Faithful](tests/translation_approach_tests.rs#L161) `"<blockquote><code>x\n\ny</code></blockquote>"`                                                    | Configured + expected | Yes           | No                  |
| [`empty_code_span`](tests/translation_approach_tests.rs#L170) / [input 1, Faithful](tests/translation_approach_tests.rs#L173) `"<p>a<code></code>b</p>"`                                                                                          | Expected              | Yes           | No                  |
| [`empty_code_span`](tests/translation_approach_tests.rs#L170) / [input 2, Faithful](tests/translation_approach_tests.rs#L177) `"<p>a<code> </code>b</p>"`                                                                                         | Expected              | Yes           | No                  |
| [`empty_code_span`](tests/translation_approach_tests.rs#L170) / [input 3, Faithful](tests/translation_approach_tests.rs#L184) `"<p>a<code>\n</code>b</p>"`                                                                                        | Expected              | Yes           | No                  |
| [`empty_code_span`](tests/translation_approach_tests.rs#L170) / [input 4, Pure](tests/translation_approach_tests.rs#L192) `"<p>a<code></code>b</p>"`                                                                                              | No                    | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 1, Faithful](tests/translation_approach_tests.rs#L206) `"<p><em><br>a</em></p>"`                                                                              | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 2, Faithful](tests/translation_approach_tests.rs#L210) `"<p><em><br><br><br>a</em></p>"`                                                                      | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 3, Faithful](tests/translation_approach_tests.rs#L215) `"<p><em>a<br></em></p>"`                                                                              | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 4, Faithful](tests/translation_approach_tests.rs#L219) `"<p><em>a<br><br><br></em></p>"`                                                                      | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 5, Faithful](tests/translation_approach_tests.rs#L224) `"<p><em>a<br>b</em></p>"`                                                                             | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 6, Faithful](tests/translation_approach_tests.rs#L228) `"<p><em>a<br><br><br>b</em></p>"`                                                                     | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 7, Faithful](tests/translation_approach_tests.rs#L233) `"<p>a<em><br></em>b</p>"`                                                                             | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 8, Faithful](tests/translation_approach_tests.rs#L237) `"<p>a<em><br><br><br></em>b</p>"`                                                                     | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 9, Faithful](tests/translation_approach_tests.rs#L242) `"<p>a<em> </em>b</p>"`                                                                                | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 10, Faithful](tests/translation_approach_tests.rs#L246) `"<p>a<strong> </strong>b</p>"`                                                                       | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 11, Faithful](tests/translation_approach_tests.rs#L252) `"<p>a<em>&#160;</em>b</p>"`                                                                          | Expected              | Yes           | No                  |
| [`emphasis_and_strong_emphasis`](tests/translation_approach_tests.rs#L203) / [input 12, Faithful](tests/translation_approach_tests.rs#L257) `"<p>a<em></em>b</p>"`                                                                                | Expected              | Yes           | No                  |
| [`brs_alone_in_a_raw_html_inline`](tests/translation_approach_tests.rs#L264) / [input 1, Faithful](tests/translation_approach_tests.rs#L267) `"<p>a<del><br></del>b</p>"`                                                                         | Yes                   | Yes           | No                  |
| [`brs_alone_in_a_raw_html_inline`](tests/translation_approach_tests.rs#L264) / [input 2, Faithful](tests/translation_approach_tests.rs#L271) `"<p>a<del><br><br><br></del>b</p>"`                                                                 | Yes                   | Yes           | No                  |
| [`brs_alone_in_a_raw_html_inline`](tests/translation_approach_tests.rs#L264) / [input 3, Faithful](tests/translation_approach_tests.rs#L276) `"<p>a<span><br></span>b</p>"`                                                                       | Yes                   | Yes           | No                  |
| [`brs_alone_in_a_raw_html_inline`](tests/translation_approach_tests.rs#L264) / [input 4, Faithful](tests/translation_approach_tests.rs#L280) `"<p>a<span><br><br><br></span>b</p>"`                                                               | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L287) / [input 1, Faithful](tests/translation_approach_tests.rs#L290) `r#"<p>a<a href="u"><br></a>b</p>"#`                                                                                          | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L287) / [input 2, Faithful](tests/translation_approach_tests.rs#L294) `r#"<p>a<a href="u"><br><br><br></a>b</p>"#`                                                                                  | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L287) / [input 3, Faithful](tests/translation_approach_tests.rs#L299) `r#"<p>a<a href="u"><br>c</a>b</p>"#`                                                                                         | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L287) / [input 4, Faithful](tests/translation_approach_tests.rs#L303) `r#"<p>a<a href="u"><br><br><br>c</a>b</p>"#`                                                                                 | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L287) / [input 5, Faithful](tests/translation_approach_tests.rs#L308) `r#"<p>a<a href="u">c<br></a>b</p>"#`                                                                                         | Yes                   | Yes           | No                  |
| [`links`](tests/translation_approach_tests.rs#L287) / [input 6, Faithful](tests/translation_approach_tests.rs#L312) `r#"<p>a<a href="u">c<br><br><br></a>b</p>"#`                                                                                 | Yes                   | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L318) / [input 1, Faithful](tests/translation_approach_tests.rs#L319) `"<br>"`                                                                                                       | Expected              | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L318) / [input 2, Faithful](tests/translation_approach_tests.rs#L320) `"<br><br><br>"`                                                                                               | Expected              | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L318) / [input 3, Faithful](tests/translation_approach_tests.rs#L322) `"<br><p>a</p>"`                                                                                               | Expected              | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L318) / [input 4, Faithful](tests/translation_approach_tests.rs#L323) `"<p>a</p><br>"`                                                                                               | Expected              | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L318) / [input 5, Faithful](tests/translation_approach_tests.rs#L329) `"<div><br></div>"`                                                                                            | Expected              | Yes           | No                  |
| [`at_the_document_root`](tests/translation_approach_tests.rs#L318) / [input 6, Faithful](tests/translation_approach_tests.rs#L333) `"<div><br><br><br></div>"`                                                                                    | Expected              | Yes           | No                  |
| [`setext_headings`](tests/translation_approach_tests.rs#L341) / [input 1, Faithful, setext](tests/translation_approach_tests.rs#L342) `"<h1><br></h1>"`                                                                                           | Configured            | Yes           | No                  |
| [`setext_headings`](tests/translation_approach_tests.rs#L341) / [input 2, Faithful, setext](tests/translation_approach_tests.rs#L345) `"<h1><br><br><br></h1>"`                                                                                   | Configured            | Yes           | No                  |
| [`setext_headings`](tests/translation_approach_tests.rs#L341) / [input 3, Faithful, setext](tests/translation_approach_tests.rs#L348) `"<h1>a #</h1>"`                                                                                            | Configured            | Yes           | No                  |
| [`setext_headings`](tests/translation_approach_tests.rs#L341) / [input 4, Faithful, setext](tests/translation_approach_tests.rs#L349) `"<h1>#</h1>"`                                                                                              | Configured            | Yes           | No                  |
| [`an_empty_setext_heading_falls_back_to_atx`](tests/translation_approach_tests.rs#L356) / [input 1, Faithful, setext](tests/translation_approach_tests.rs#L357) `"<h1></h1>"`                                                                     | Configured + expected | Yes           | No                  |
| [`an_empty_setext_heading_falls_back_to_atx`](tests/translation_approach_tests.rs#L356) / [input 2, Faithful, setext](tests/translation_approach_tests.rs#L358) `"<h2></h2>"`                                                                     | Configured + expected | Yes           | No                  |
| [`an_empty_setext_heading_falls_back_to_atx`](tests/translation_approach_tests.rs#L356) / [input 3, Faithful, setext](tests/translation_approach_tests.rs#L359) `"<h1> </h1>"`                                                                    | Configured + expected | Yes           | No                  |
| [`an_empty_setext_heading_falls_back_to_atx`](tests/translation_approach_tests.rs#L356) / [input 4, Faithful, setext](tests/translation_approach_tests.rs#L364) `"<h1></h1><p>a</p>"`                                                             | Configured + expected | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 1, Faithful](tests/translation_approach_tests.rs#L377) `"<h1></h1>"`                                                                                                          | Expected              | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 2, Faithful](tests/translation_approach_tests.rs#L378) `"<h1></h1><p>a</p>"`                                                                                                  | Expected              | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 3, Faithful](tests/translation_approach_tests.rs#L379) `"<h3></h3><p>a</p>"`                                                                                                  | Expected              | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 4, Faithful](tests/translation_approach_tests.rs#L381) `"<h1><br></h1>"`                                                                                                      | Expected              | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 5, Faithful](tests/translation_approach_tests.rs#L384) `"<h1><br><br><br></h1>"`                                                                                              | Expected              | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 6, Faithful](tests/translation_approach_tests.rs#L387) `"<h1>a #</h1>"`                                                                                                       | Expected              | Yes           | No                  |
| [`atx_headings`](tests/translation_approach_tests.rs#L374) / [input 7, Faithful](tests/translation_approach_tests.rs#L388) `"<h1>#</h1>"`                                                                                                         | Expected              | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 1, Faithful](tests/translation_approach_tests.rs#L400) `r#"<p><iframe src="u">a</iframe></p>"#`                                                                                 | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 2, Faithful](tests/translation_approach_tests.rs#L402) `"<p><br></p>"`                                                                                                          | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 3, Faithful](tests/translation_approach_tests.rs#L403) `"<p><br><br></p>"`                                                                                                      | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 4, Faithful](tests/translation_approach_tests.rs#L406) `"<p><br><br><br></p>"`                                                                                                  | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 5, Faithful](tests/translation_approach_tests.rs#L411) `"<p><br><em>b</em></p>"`                                                                                                | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 6, Faithful](tests/translation_approach_tests.rs#L415) `"<p><em>a</em><br></p>"`                                                                                                | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 7, Faithful](tests/translation_approach_tests.rs#L420) `r#"<p><br><img src="i" alt=""></p>"#`                                                                                   | Yes                   | Yes           | No                  |
| [`paragraphs`](tests/translation_approach_tests.rs#L397) / [input 8, Faithful](tests/translation_approach_tests.rs#L424) `r#"<p><img src="i" alt=""><br></p>"#`                                                                                   | Yes                   | Yes           | No                  |
| [`blockquotes`](tests/translation_approach_tests.rs#L432) / [input 1, Faithful](tests/translation_approach_tests.rs#L435) `"<blockquote><br></blockquote>"`                                                                                       | Expected              | Yes           | No                  |
| [`blockquotes`](tests/translation_approach_tests.rs#L432) / [input 2, Faithful](tests/translation_approach_tests.rs#L439) `"<blockquote><br><br><br></blockquote>"`                                                                               | Expected              | Yes           | No                  |
| [`blockquotes`](tests/translation_approach_tests.rs#L432) / [input 3, Faithful](tests/translation_approach_tests.rs#L444) `"<blockquote><p><br></p></blockquote>"`                                                                                | Expected              | Yes           | No                  |
| [`blockquotes`](tests/translation_approach_tests.rs#L432) / [input 4, Faithful](tests/translation_approach_tests.rs#L449) `"<blockquote><p><br><em>b</em></p></blockquote>"`                                                                      | Expected              | Yes           | No                  |
| [`blockquotes`](tests/translation_approach_tests.rs#L432) / [input 5, Faithful](tests/translation_approach_tests.rs#L453) `"<blockquote><p><em>a</em><br></p></blockquote>"`                                                                      | Expected              | Yes           | No                  |
| [`tables`](tests/translation_approach_tests.rs#L460) / [input 1, Faithful](tests/translation_approach_tests.rs#L472) `<table><thead><tr><th>h</th></tr></thead><tbody><tr><td><br><em>b</em></td></tr></tbo…`                                     | Configured            | Yes           | No                  |
| [`tables`](tests/translation_approach_tests.rs#L460) / [input 2, Faithful](tests/translation_approach_tests.rs#L473) `<table><thead><tr><th>h</th></tr></thead><tbody><tr><td><em>a</em><br></td></tr></tbo…`                                     | Configured            | Yes           | No                  |
| [`tables`](tests/translation_approach_tests.rs#L460) / [input 3, Faithful](tests/translation_approach_tests.rs#L474) `<table><thead><tr><th>h</th></tr></thead><tbody><tr><td><br></td></tr></tbody></table…`                                     | Configured            | Yes           | No                  |
| [`tables`](tests/translation_approach_tests.rs#L460) / [input 4, Faithful](tests/translation_approach_tests.rs#L475) `<table><thead><tr><th>h</th></tr></thead><tbody><tr><td><br><br><br></td></tr></tbody…`                                     | Configured            | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 1, Faithful](tests/translation_approach_tests.rs#L487) `"<table><thead><tr><th scope="col">h</th></tr></thead>\ <tbody><tr><td>a</td></tr><…`   | Yes                   | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 2, Faithful](tests/translation_approach_tests.rs#L490) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr id="r"><td>a</td></tr></tbod…`   | Yes                   | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 3, Faithful](tests/translation_approach_tests.rs#L493) `"<table><thead><tr id="r"><th>h</th></tr></thead>\ <tbody><tr><td>a</td></tr></tbod…`   | Yes                   | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 4, Faithful](tests/translation_approach_tests.rs#L495) `"<table><thead id="t"><tr><th>h</th></tr></thead>\ <tbody><tr><td>a</td></tr></tbod…`   | Yes                   | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 5, Faithful](tests/translation_approach_tests.rs#L497) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody id="b"><tr><td>a</td></tr></tbod…`   | Yes                   | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 6, Faithful](tests/translation_approach_tests.rs#L499) `"<table><thead><tr><th>h</th></tr></thead>\ <tbody><tr><td>a</td></tr></tbody>\ <tfoo…` | Yes                   | Yes           | No                  |
| [`table_children_which_only_html_can_express`](tests/translation_approach_tests.rs#L485) / [input 7, Faithful](tests/translation_approach_tests.rs#L504) `"<table><colgroup><col span="2"></colgroup>\ <thead><tr><th>h</th></tr></thead>\ <t…`   | Yes                   | Yes           | No                  |

### [tests/turndown\_cases\_tests.rs](tests/turndown_cases_tests.rs) (1)

| Test                                                                                                                                                                            | Faithful round trip | Keep Markdown | Existing round trip |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 1, Faithful](tests/html/turndown_test_index.html#L14) `p`                                                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 2, Faithful](tests/html/turndown_test_index.html#L19) `multiple ps`                                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 3, Faithful](tests/html/turndown_test_index.html#L32) `em`                                                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 4, Faithful](tests/html/turndown_test_index.html#L37) `i`                                                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 5, Faithful](tests/html/turndown_test_index.html#L42) `strong`                                                        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 6, Faithful](tests/html/turndown_test_index.html#L47) `b`                                                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 7, Faithful](tests/html/turndown_test_index.html#L52) `code`                                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 8, Faithful](tests/html/turndown_test_index.html#L57) `code containing a backtick`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 9, Faithful](tests/html/turndown_test_index.html#L62) `code containing three or more backticks`                       | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 10, Faithful](tests/html/turndown_test_index.html#L67) `code containing one or more backticks`                        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 11, Faithful](tests/html/turndown_test_index.html#L72) `code starting with a backtick`                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 12, Faithful](tests/html/turndown_test_index.html#L77) `code containing markdown syntax`                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 13, Pure](tests/html/turndown_test_index.html#L82) `code containing markdown syntax in a span`                        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 14, Faithful](tests/html/turndown_test_index.html#L87) `h1`                                                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 15, Faithful](tests/html/turndown_test_index.html#L93) `escape = when used as heading`                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 16, Faithful](tests/html/turndown_test_index.html#L98) `not escaping = outside of a heading`                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 17, Faithful](tests/html/turndown_test_index.html#L103) `h1 as atx`                                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 18, Faithful](tests/html/turndown_test_index.html#L108) `h2`                                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 19, Faithful](tests/html/turndown_test_index.html#L114) `h2 as atx`                                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 20, Faithful](tests/html/turndown_test_index.html#L119) `h3`                                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 21, Faithful](tests/html/turndown_test_index.html#L124) `heading with child`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 22, Pure](tests/html/turndown_test_index.html#L129) `invalid heading`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 23, Faithful](tests/html/turndown_test_index.html#L134) `hr`                                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 24, Faithful](tests/html/turndown_test_index.html#L139) `hr with closing tag`                                         | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 25, Faithful](tests/html/turndown_test_index.html#L144) `hr with option`                                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 26, Pure](tests/html/turndown_test_index.html#L149) `br`                                                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 27, Pure](tests/html/turndown_test_index.html#L155) `br with visible line-ending`                                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 28, Pure](tests/html/turndown_test_index.html#L161) `img with no alt`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 29, Pure](tests/html/turndown_test_index.html#L166) `img with relative src`                                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 30, Faithful](tests/html/turndown_test_index.html#L171) `img with alt`                                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 31, Pure](tests/html/turndown_test_index.html#L176) `img with no src`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 32, Faithful](tests/html/turndown_test_index.html#L181) `img with a new line in alt`                                  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 33, Faithful](tests/html/turndown_test_index.html#L187) `img with more than one new line in alt`                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 34, Pure](tests/html/turndown_test_index.html#L194) `img with new lines in title`                                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 35, Faithful](tests/html/turndown_test_index.html#L201) `a`                                                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 36, Faithful](tests/html/turndown_test_index.html#L206) `a with title`                                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 37, Faithful](tests/html/turndown_test_index.html#L211) `a with multiline title`                                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 38, Faithful](tests/html/turndown_test_index.html#L218) `a with quotes in title`                                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 39, Faithful](tests/html/turndown_test_index.html#L223) `a with parenthesis in query`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 40, Pure](tests/html/turndown_test_index.html#L228) `a without a src`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 41, Faithful](tests/html/turndown_test_index.html#L233) `a with a child`                                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 42, Faithful](tests/html/turndown_test_index.html#L238) `a reference`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 43, Faithful](tests/html/turndown_test_index.html#L245) `a reference with collapsed style`                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 44, Faithful](tests/html/turndown_test_index.html#L252) `a reference with shortcut style`                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 45, Faithful](tests/html/turndown_test_index.html#L259) `pre/code block`                                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 46, Faithful](tests/html/turndown_test_index.html#L271) `multiple pre/code blocks`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 47, Faithful](tests/html/turndown_test_index.html#L297) `pre/code block with multiple new lines`                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 48, Faithful](tests/html/turndown_test_index.html#L315) `fenced pre/code block`                                       | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 49, Faithful](tests/html/turndown_test_index.html#L324) `pre/code block fenced with ~`                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 50, Faithful](tests/html/turndown_test_index.html#L333) `escaping ~~~`                                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 51, Faithful](tests/html/turndown_test_index.html#L340) `not escaping ~~~`                                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 52, Faithful](tests/html/turndown_test_index.html#L345) `fenced pre/code block with language`                         | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 53, Faithful](tests/html/turndown_test_index.html#L354) `fenced pre/code/br block with language`                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 54, Faithful](tests/html/turndown_test_index.html#L365) `empty pre does not throw error`                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 55, Faithful](tests/html/turndown_test_index.html#L372) `ol`                                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 56, Faithful](tests/html/turndown_test_index.html#L385) `ol with start`                                               | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 57, Faithful](tests/html/turndown_test_index.html#L398) `ol with increasing start`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 58, Faithful](tests/html/turndown_test_index.html#L409) `ol with high start`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 59, Faithful](tests/html/turndown_test_index.html#L420) `list spacing`                                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 60, Faithful](tests/html/turndown_test_index.html#L448) `ul`                                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 61, Faithful](tests/html/turndown_test_index.html#L461) `ul with custom bullet`                                       | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 62, Faithful](tests/html/turndown_test_index.html#L474) `ul with paragraph`                                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 63, Faithful](tests/html/turndown_test_index.html#L486) `ol with paragraphs`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 64, Faithful](tests/html/turndown_test_index.html#L505) `ol with start and paragraphs`                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 65, Faithful](tests/html/turndown_test_index.html#L527) `nested uls`                                                  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 66, Faithful](tests/html/turndown_test_index.html#L558) `nested ols and uls`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 67, Faithful](tests/html/turndown_test_index.html#L589) `ul with blockquote`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 68, Faithful](tests/html/turndown_test_index.html#L605) `blockquote`                                                  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 69, Faithful](tests/html/turndown_test_index.html#L617) `nested blockquotes`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 70, Faithful](tests/html/turndown_test_index.html#L634) `html in blockquote`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 71, Pure](tests/html/turndown_test_index.html#L657) `multiple divs`                                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 72, Pure](tests/html/turndown_test_index.html#L667) `comment`                                                         | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 73, Pure](tests/html/turndown_test_index.html#L672) `pre/code with comment`                                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 74, Faithful](tests/html/turndown_test_index.html#L679) `leading whitespace in heading`                               | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 75, Faithful](tests/html/turndown_test_index.html#L685) `trailing whitespace in li`                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 76, Faithful](tests/html/turndown_test_index.html#L707) `multilined and bizarre formatting`                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 77, Faithful](tests/html/turndown_test_index.html#L727) `whitespace between inline elements`                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 78, Faithful](tests/html/turndown_test_index.html#L734) `whitespace in inline elements`                               | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 79, Pure](tests/html/turndown_test_index.html#L740) `newline at the end of a strong element`                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 80, Pure](tests/html/turndown_test_index.html#L746) `whitespace in nested inline elements`                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 81, Pure](tests/html/turndown_test_index.html#L751) `blank inline elements`                                           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 82, Pure](tests/html/turndown_test_index.html#L758) `blank block elements`                                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 83, Pure](tests/html/turndown_test_index.html#L767) `blank inline element with br`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 84, Pure](tests/html/turndown_test_index.html#L772) `whitespace between blocks`                                       | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 85, Faithful](tests/html/turndown_test_index.html#L780) `escaping backslashes`                                        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 86, Faithful](tests/html/turndown_test_index.html#L785) `escaping headings with #`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 87, Faithful](tests/html/turndown_test_index.html#L790) `not escaping # outside of a heading`                         | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 88, Faithful](tests/html/turndown_test_index.html#L795) `escaping em markdown with *`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 89, Faithful](tests/html/turndown_test_index.html#L800) `escaping em markdown with _`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 90, Faithful](tests/html/turndown_test_index.html#L805) `not escaping within code`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 91, Faithful](tests/html/turndown_test_index.html#L810) `escaping strong markdown with *`                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 92, Faithful](tests/html/turndown_test_index.html#L815) `escaping strong markdown with _`                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 93, Faithful](tests/html/turndown_test_index.html#L820) `escaping hr markdown with *`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 94, Faithful](tests/html/turndown_test_index.html#L825) `escaping hr markdown with -`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 95, Faithful](tests/html/turndown_test_index.html#L830) `escaping hr markdown with _`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 96, Faithful](tests/html/turndown_test_index.html#L835) `escaping hr markdown without spaces`                         | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 97, Faithful](tests/html/turndown_test_index.html#L840) `escaping hr markdown with more than 3 characters`            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 98, Faithful](tests/html/turndown_test_index.html#L845) `escaping ol markdown`                                        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 99, Faithful](tests/html/turndown_test_index.html#L850) `not escaping . outside of an ol`                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 100, Faithful](tests/html/turndown_test_index.html#L855) `escaping ul markdown *`                                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 101, Faithful](tests/html/turndown_test_index.html#L860) `escaping ul markdown -`                                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 102, Faithful](tests/html/turndown_test_index.html#L865) `escaping ul markdown +`                                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 103, Faithful](tests/html/turndown_test_index.html#L870) `not escaping - outside of a ul`                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 104, Faithful](tests/html/turndown_test_index.html#L875) `not escaping + outside of a ul`                             | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 105, Faithful](tests/html/turndown_test_index.html#L880) `escaping *`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 106, Pure](tests/html/turndown_test_index.html#L885) `escaping ** inside strong tags`                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 107, Faithful](tests/html/turndown_test_index.html#L890) `escaping _ inside em tags`                                  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 108, Faithful](tests/html/turndown_test_index.html#L895) `escaping > as blockquote`                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 109, Faithful](tests/html/turndown_test_index.html#L900) `escaping > as blockquote without space`                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 110, Faithful](tests/html/turndown_test_index.html#L905) `not escaping > outside of a blockquote`                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 111, Faithful](tests/html/turndown_test_index.html#L910) `escaping code`                                              | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 112, Faithful](tests/html/turndown_test_index.html#L915) `escaping []`                                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 113, Faithful](tests/html/turndown_test_index.html#L920) `escaping [`                                                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 114, Faithful](tests/html/turndown_test_index.html#L926) `escaping * performance`                                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 115, Faithful](tests/html/turndown_test_index.html#L931) `escaping multiple asterisks`                                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 116, Faithful](tests/html/turndown_test_index.html#L936) `escaping delimiters around short words and numbers`         | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 117, Pure](tests/html/turndown_test_index.html#L941) `non-markdown block elements`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 118, Pure](tests/html/turndown_test_index.html#L954) `non-markdown inline elements`                                   | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 119, Pure](tests/html/turndown_test_index.html#L961) `blank inline elements`                                          | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 120, Pure](tests/html/turndown_test_index.html#L968) `elements with a single void element`                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 121, Pure](tests/html/turndown_test_index.html#L975) `elements with a nested void element`                            | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 122, Pure](tests/html/turndown_test_index.html#L982) `text separated by a space in an element`                        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 123, Pure](tests/html/turndown_test_index.html#L989) `text separated by a non-breaking space in an element`           | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 124, Faithful](tests/html/turndown_test_index.html#L996) `triple tildes inside code`                                  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 125, Faithful](tests/html/turndown_test_index.html#L1010) `triple ticks inside code`                                  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 126, Faithful](tests/html/turndown_test_index.html#L1024) `four ticks inside code`                                    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 127, Faithful](tests/html/turndown_test_index.html#L1038) `empty line in start/end of code block`                     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 128, Pure](tests/html/turndown_test_index.html#L1052) `text separated by ASCII and nonASCII space in an element`      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 129, Pure](tests/html/turndown_test_index.html#L1059) `list-like text with non-breaking spaces`                       | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 130, Faithful](tests/html/turndown_test_index.html#L1065) `element with trailing nonASCII WS followed by nonWS`       | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 131, Faithful](tests/html/turndown_test_index.html#L1070) `element with trailing nonASCII WS followed by nonASCII WS` | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 132, Faithful](tests/html/turndown_test_index.html#L1075) `element with trailing ASCII WS followed by nonASCII WS`    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 133, Faithful](tests/html/turndown_test_index.html#L1080) `element with trailing nonASCII WS followed by ASCII WS`    | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 134, Faithful](tests/html/turndown_test_index.html#L1085) `nonWS followed by element with leading nonASCII WS`        | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 135, Faithful](tests/html/turndown_test_index.html#L1090) `nonASCII WS followed by element with leading nonASCII WS`  | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 136, Faithful](tests/html/turndown_test_index.html#L1095) `nonASCII WS followed by element with leading ASCII WS`     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 137, Faithful](tests/html/turndown_test_index.html#L1100) `ASCII WS followed by element with leading nonASCII WS`     | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 138, Faithful](tests/html/turndown_test_index.html#L1106) `preformatted code with leading whitespace`                 | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 139, Faithful](tests/html/turndown_test_index.html#L1111) `preformatted code with trailing whitespace`                | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 140, Faithful](tests/html/turndown_test_index.html#L1116) `preformatted code tightly surrounded`                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 141, Faithful](tests/html/turndown_test_index.html#L1121) `preformatted code loosely surrounded`                      | No                  | Yes           | No                  |
| [`run_cases`](tests/turndown_cases_tests.rs#L63) / [input 142, Faithful](tests/html/turndown_test_index.html#L1127) `preformatted code with newlines`                           | No                  | Yes           | No                  |

### [tests/turndown\_online\_demo\_tests.rs](tests/turndown_online_demo_tests.rs) (1)

| Test                                                  | Faithful round trip | Keep Markdown | Existing round trip |
| ----------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`run_cases`](tests/turndown_online_demo_tests.rs#L9) | No                  | Yes           | No                  |

Unit tests and documentation examples
-------------------------------------

The 37 unit tests below exercise parsers, escaping helpers, Unicode data, and
other internal behavior. They do not run the complete Faithful conversion
pipeline, so retain their existing assertions. The last unit test is ignored by
the normal test run because it generates a table.

### [src/html\_block.rs](src/html_block.rs) (9)

| Test                                                                  | Faithful round trip | Keep Markdown | Existing round trip |
| --------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`recognizes_type_1_through_6_prefixes`](src/html_block.rs#L169)      | No                  | N/A           | N/A                 |
| [`applies_the_type_1_and_6_name_terminators`](src/html_block.rs#L186) | No                  | N/A           | N/A                 |
| [`ends_a_type_1_or_6_name_at_the_line_end`](src/html_block.rs#L196)   | No                  | N/A           | N/A                 |
| [`recognizes_a_lone_type_7_tag`](src/html_block.rs#L204)              | No                  | N/A           | N/A                 |
| [`recognizes_a_lone_closing_type_1_tag`](src/html_block.rs#L218)      | No                  | N/A           | N/A                 |
| [`rejects_a_type_7_tag_which_is_not_alone`](src/html_block.rs#L224)   | No                  | N/A           | N/A                 |
| [`rejects_an_incomplete_or_invalid_tag`](src/html_block.rs#L233)      | No                  | N/A           | N/A                 |
| [`allows_three_spaces_of_indentation`](src/html_block.rs#L247)        | No                  | N/A           | N/A                 |
| [`tests_only_the_first_line`](src/html_block.rs#L257)                 | No                  | N/A           | N/A                 |

### [src/element\_handler/element\_util.rs](src/element_handler/element_util.rs) (3)

| Test                                                                                            | Faithful round trip | Keep Markdown | Existing round trip |
| ----------------------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`encodes_the_line_endings_and_pipes_of_an_open_tag`](src/element_handler/element_util.rs#L304) | No                  | Yes           | N/A                 |
| [`escapes_blank_lines_for_each_line_ending_style`](src/element_handler/element_util.rs#L316)    | No                  | Yes           | N/A                 |
| [`escapes_only_the_lines_which_are_blank`](src/element_handler/element_util.rs#L326)            | No                  | Yes           | N/A                 |

### [src/element\_handler/list.rs](src/element_handler/list.rs) (1)

| Test                                                    | Faithful round trip | Keep Markdown | Existing round trip |
| ------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`test_count_digits`](src/element_handler/list.rs#L167) | No                  | N/A           | N/A                 |

### [src/util/escape.rs](src/util/escape.rs) (16)

| Test                                                                               | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`escapes_basic_tags`](src/util/escape.rs#L280)                                    | No                  | Yes           | N/A                 |
| [`escapes_misc_sequences`](src/util/escape.rs#L290)                                | No                  | Yes           | N/A                 |
| [`leaves_non_html_sequences`](src/util/escape.rs#L301)                             | No                  | Yes           | N/A                 |
| [`escapes_an_email_autolink`](src/util/escape.rs#L311)                             | No                  | Yes           | N/A                 |
| [`leaves_a_near_miss_email_autolink`](src/util/escape.rs#L319)                     | No                  | Yes           | N/A                 |
| [`escapes_a_sequence_cut_short`](src/util/escape.rs#L346)                          | No                  | Yes           | N/A                 |
| [`escapes_a_literal_cdata_section`](src/util/escape.rs#L365)                       | No                  | Yes           | N/A                 |
| [`leaves_an_escaped_cdata_section`](src/util/escape.rs#L379)                       | No                  | Yes           | N/A                 |
| [`escapes_a_link_destination_backslash`](src/util/escape.rs#L390)                  | No                  | Yes           | N/A                 |
| [`brackets_a_link_destination_the_bare_form_cannot_hold`](src/util/escape.rs#L399) | No                  | Yes           | N/A                 |
| [`escapes_a_link_destination_ampersand`](src/util/escape.rs#L410)                  | No                  | Yes           | N/A                 |
| [`escapes_a_title_backslash`](src/util/escape.rs#L417)                             | No                  | Yes           | N/A                 |
| [`escapes_a_title_ampersand_and_line_endings`](src/util/escape.rs#L423)            | No                  | Yes           | N/A                 |
| [`escapes_an_image_description_as_text`](src/util/escape.rs#L429)                  | No                  | Yes           | N/A                 |
| [`test_index_of_markdown_ordered_item_delimiter`](src/util/escape.rs#L437)         | No                  | N/A           | N/A                 |
| [`a_unicode_numeric_opens_no_ordered_item`](src/util/escape.rs#L457)               | No                  | N/A           | N/A                 |

### [src/util/unicode.rs](src/util/unicode.rs) (8)

| Test                                                                               | Faithful round trip | Keep Markdown | Existing round trip |
| ---------------------------------------------------------------------------------- | ------------------- | ------------- | ------------------- |
| [`table_is_sorted_and_merged`](src/util/unicode.rs#L33)                            | No                  | N/A           | N/A                 |
| [`agrees_with_a_linear_scan`](src/util/unicode.rs#L49)                             | No                  | N/A           | N/A                 |
| [`classifies_punctuation_and_symbols`](src/util/unicode.rs#L66)                    | No                  | N/A           | N/A                 |
| [`rejects_everything_else`](src/util/unicode.rs#L84)                               | No                  | N/A           | N/A                 |
| [`table_matches_unicode_general_categories`](src/util/unicode.rs#L133)             | No                  | N/A           | N/A                 |
| [`ascii_punctuation_matches_unicode_general_categories`](src/util/unicode.rs#L155) | No                  | N/A           | N/A                 |
| [`header_names_the_unicode_version_of_the_data`](src/util/unicode.rs#L168)         | No                  | N/A           | N/A                 |
| [`print_punctuation_or_symbol_table`](src/util/unicode.rs#L182)                    | No                  | N/A           | N/A                 |

### Documentation examples

| Example                                          | Faithful round trip | Keep Markdown | Existing round trip |
| ------------------------------------------------ | ------------------- | ------------- | ------------------- |
| `src/lib.rs: convert`                            | No                  | Yes           | N/A                 |
| `src/lib.rs: HtmlToMarkdown`                     | No                  | Yes           | N/A                 |
| `src/lib.rs: HtmlToMarkdownBuilder::add_handler` | No                  | N/A           | N/A                 |
| `src/element_handler/mod.rs: EventTypes`         | No                  | N/A           | N/A                 |
