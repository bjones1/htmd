// Tests of the HTML comparison helper, one pair per rule of the "Whitespace"
// section of `testing_approach.md` ("Rules for tests", item 3).
mod common;

use common::html_compare::{assert_html_equivalent, html_equivalent};

fn assert_equivalent(a: &str, b: &str) {
    assert!(
        html_equivalent(a, b),
        "expected equivalent: {a:?} and {b:?}"
    );
}

fn assert_different(a: &str, b: &str) {
    assert!(
        !html_equivalent(a, b),
        "expected different: {a:?} and {b:?}"
    );
}

// "Comparison": what parsing already makes equivalent.

#[test]
fn line_endings_are_normalized_by_the_parser() {
    assert_equivalent("<pre>a\r\nb\rc</pre>", "<pre>a\nb\nc</pre>");
}

#[test]
fn character_reference_equals_its_character() {
    assert_equivalent("<pre>a&#10;b</pre>", "<pre>a\nb</pre>");
}

#[test]
fn carriage_return_reference_is_not_a_line_feed() {
    assert_different("<pre>a&#13;b</pre>", "<pre>a\nb</pre>");
}

#[test]
fn spelling_of_tags_and_attributes_is_ignored() {
    assert_equivalent("<P CLASS=x>a<BR/>b</P>", "<p class=\"x\">a<br>b</p>");
}

#[test]
fn attribute_order_is_ignored() {
    assert_equivalent(
        "<p><a href=\"u\" title=\"t\">a</a></p>",
        "<p><a title=\"t\" href=\"u\">a</a></p>",
    );
}

#[test]
fn pulldown_cmark_line_ending_after_a_block() {
    assert_equivalent("<p>a</p>\n", "<p>a</p>");
}

// "Comparison", rule 1.

#[test]
fn run_of_whitespace_is_one_space() {
    assert_equivalent("<p>a \t\n\r b</p>", "<p>a b</p>");
}

#[test]
fn run_collapses_across_inline_boundaries() {
    assert_equivalent("<p>a <em> b</em></p>", "<p>a <em>b</em></p>");
}

#[test]
fn collapsed_run_keeps_its_first_space() {
    assert_equivalent("<p><em>a </em> b</p>", "<p><em>a </em>b</p>");
    assert_different("<p><em>a </em> b</p>", "<p><em>a</em> b</p>");
}

// "Comparison", rule 2.

#[test]
fn whitespace_at_block_edges_is_removed() {
    assert_equivalent("<p> a </p>", "<p>a</p>");
}

#[test]
fn whitespace_around_br_is_removed() {
    assert_equivalent("<p>a <br> b</p>", "<p>a<br>b</p>");
}

#[test]
fn line_edge_inside_an_inline_element() {
    assert_equivalent("<p><em> a</em></p>", "<p><em>a</em></p>");
    assert_equivalent("<p>a<em>b </em></p>", "<p>a<em>b</em></p>");
}

#[test]
fn line_ends_before_a_block_child() {
    assert_equivalent(
        "<ul><li>a\n<ul><li>b</li></ul></li></ul>",
        "<ul><li>a<ul><li>b</li></ul></li></ul>",
    );
}

// "Comparison", rule 3.

#[test]
fn whitespace_between_blocks_is_removed() {
    assert_equivalent("<p>a</p>\n<p>b</p>\n", "<p>a</p><p>b</p>");
}

#[test]
fn whitespace_between_table_parts_is_removed() {
    assert_equivalent(
        "<table>\n<tr>\n<td>a</td>\n</tr>\n</table>",
        "<table><tr><td>a</td></tr></table>",
    );
}

#[test]
fn comments_are_skipped_between_blocks() {
    assert_equivalent(
        "<p>a</p>\n<!-- x -->\n<p>b</p>",
        "<p>a</p><!-- x --><p>b</p>",
    );
}

#[test]
fn form_feed_between_blocks_is_removed() {
    assert_equivalent("<p>a</p>\u{c}<p>b</p>", "<p>a</p><p>b</p>");
}

#[test]
fn form_feed_between_table_parts_is_removed() {
    // A form feed is not collapsible, so only rule 3 can make these equal.
    assert_equivalent(
        "<table>\u{c}<tr>\u{c}<td>a</td>\u{c}</tr>\u{c}</table>",
        "<table><tr><td>a</td></tr></table>",
    );
}

#[test]
fn comment_between_blocks_with_whitespace_on_both_sides() {
    assert_equivalent(
        "<p>a</p>\u{c}<!-- x -->\u{c}<p>b</p>",
        "<p>a</p><!-- x --><p>b</p>",
    );
}

#[test]
fn form_feed_between_a_block_and_inline_content_is_text() {
    assert_different("<p>a</p>\u{c}b", "<p>a</p>b");
}

#[test]
fn comments_between_blocks_are_kept() {
    assert_different("<p>a</p><!-- x --><p>b</p>", "<p>a</p><p>b</p>");
}

#[test]
fn form_feed_alone_in_its_parent_is_text() {
    assert_different("<div>\u{c}</div>", "<div></div>");
}

// `<head>`, which the parser fills with a leading `<style>` or `<script>` and
// the line feed after it.

#[test]
fn whitespace_in_head_is_removed() {
    assert_equivalent("<style>p{}</style>\n<p>a</p>", "<style>p{}</style><p>a</p>");
    assert_equivalent(
        "<script>x()</script>\u{c}<p>a</p>",
        "<script>x()</script><p>a</p>",
    );
}

// "Where whitespace matters", item 1.

#[test]
fn preformatted_text_is_compared_exactly() {
    assert_different("<pre>a  b</pre>", "<pre>a b</pre>");
    assert_different("<listing>a  b</listing>", "<listing>a b</listing>");
    assert_different("<xmp>a  b</xmp>", "<xmp>a b</xmp>");
    assert_different("<textarea>a  b</textarea>", "<textarea>a b</textarea>");
    assert_different("<pre><em>a  b</em></pre>", "<pre><em>a b</em></pre>");
}

#[test]
fn parser_drops_line_feed_after_pre_start_tag() {
    assert_equivalent("<pre>\na</pre>", "<pre>a</pre>");
    assert_equivalent("<listing>\na</listing>", "<listing>a</listing>");
    assert_equivalent("<textarea>\na</textarea>", "<textarea>a</textarea>");
    assert_different("<pre>\n\na</pre>", "<pre>a</pre>");
}

#[test]
fn final_line_feed_in_pre_is_not_rendered() {
    assert_equivalent("<pre>a\n</pre>", "<pre>a</pre>");
    assert_equivalent("<pre><code>a\n</code></pre>", "<pre><code>a</code></pre>");
    assert_different("<pre>a\n\n</pre>", "<pre>a</pre>");
    assert_different("<pre><code>\n</code></pre>", "<pre><code></code></pre>");
}

// "Where whitespace matters", items 2 to 4.

#[test]
fn presence_of_whitespace_between_inline_content() {
    assert_different(
        "<p><em>a</em> <em>b</em></p>",
        "<p><em>a</em><em>b</em></p>",
    );
    assert_different("<p>a<code>b</code></p>", "<p>a <code>b</code></p>");
}

#[test]
fn side_of_an_element_boundary() {
    assert_different("<p>a <em>b</em></p>", "<p>a<em> b</em></p>");
}

#[test]
fn attribute_values_are_compared_exactly() {
    assert_different("<p><a href=\"x\">a</a></p>", "<p><a href=\" x\">a</a></p>");
}

#[test]
fn comments_are_compared_exactly() {
    assert_different("<p>a<!--a--></p>", "<p>a<!-- a --></p>");
}

// Characters which are not collapsible whitespace.

#[test]
fn form_feed_in_text_is_compared_exactly() {
    assert_different("<p>a\u{c}b</p>", "<p>a b</p>");
}

#[test]
fn no_break_space_is_text() {
    assert_different("<p>a\u{a0}b</p>", "<p>a b</p>");
}

#[test]
fn code_is_not_preformatted() {
    assert_equivalent("<p><code>a  b</code></p>", "<p><code>a b</code></p>");
}

// The assertion itself.

#[test]
fn assertion_passes_on_equivalent_html() {
    assert_html_equivalent("<p>a</p>\n", "<p> a </p>");
}

#[test]
#[should_panic(expected = "HTML differs after whitespace normalization")]
fn assertion_panics_on_a_difference() {
    assert_html_equivalent("<p>a b</p>", "<p>ab</p>");
}

// Real round trips through htmd and pulldown-cmark.

#[test]
fn round_trip_of_a_paragraph() {
    assert_html_equivalent(&common::round_trip("<p>a  b</p>"), "<p>a b</p>");
}

#[test]
fn round_trip_of_a_code_block() {
    assert_html_equivalent(
        &common::round_trip("<pre><code>a\n  b</code></pre>"),
        "<pre><code>a\n  b</code></pre>",
    );
}

#[test]
fn round_trip_of_a_nested_list() {
    assert_html_equivalent(
        &common::round_trip("<ul><li>a<ul><li>b</li></ul></li></ul>"),
        "<ul><li>a<ul><li>b</li></ul></li></ul>",
    );
}
