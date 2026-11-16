//! Tests of the "Subscript" section of `translation_approach.md`. Each
//! translation is read back by pulldown-cmark with `ENABLE_SUBSCRIPT`, the
//! reader the section specifies.

use pretty_assertions::assert_eq;
use pulldown_cmark::Options as CommonMarkOptions;

mod common;
use common::{convert_faithful, convert_pure, render_markdown};

/// Reads `markdown` back as HTML with the subscript extension enabled.
fn render_subscript(markdown: &str) -> String {
    render_markdown(markdown, CommonMarkOptions::ENABLE_SUBSCRIPT)
}

/// Asserts that `html` converts to `markdown` in faithful mode, and that the
/// Markdown reads back as `html_back`.
fn assert_faithful(html: &str, markdown: &str, html_back: &str) {
    let actual = convert_faithful(html).unwrap();
    assert_eq!(markdown, actual, "faithful translation of {html}");
    assert_eq!(html_back, render_subscript(&actual), "round trip of {html}");
}

/// [`assert_faithful`] for pure mode.
fn assert_pure(html: &str, markdown: &str, html_back: &str) {
    let actual = convert_pure(html).unwrap();
    assert_eq!(markdown, actual, "pure translation of {html}");
    assert_eq!(html_back, render_subscript(&actual), "round trip of {html}");
}

/// Approach step 4: faithful mode cannot see the characters around the
/// element, so it keeps the tags even where `~2~` would flank.
#[test]
fn faithful_keeps_tags_whatever_the_neighbors() {
    for (html, markdown) in [
        ("<p>H<sub>2</sub>O</p>", "H<sub>2</sub>O"),
        ("<p>a <sub>b</sub> c</p>", "a <sub>b</sub> c"),
        ("<p><sub>b</sub></p>", "<sub>b</sub>"),
        ("<p>x <sub>2</sub>.</p>", "x <sub>2</sub>."),
        ("<p>x.<sub>2</sub></p>", "x.<sub>2</sub>"),
        ("<p>a <sub>.</sub> b</p>", "a <sub>.</sub> b"),
    ] {
        assert_faithful(html, markdown, &format!("{html}\n"));
    }
}

/// Approach step 2: empty content and content with whitespace at an edge keep
/// the tags and the whitespace.
#[test]
fn faithful_keeps_empty_and_whitespace_edged_subscripts() {
    for (html, markdown) in [
        ("<p>a<sub></sub>b</p>", "a<sub></sub>b"),
        ("<p>a<sub> </sub>b</p>", "a<sub> </sub>b"),
        ("<p>x<sub> a </sub>y</p>", "x<sub> a </sub>y"),
    ] {
        assert_faithful(html, markdown, &format!("{html}\n"));
    }
}

/// Approach step 1: attributes, a comment child, a `<sub>` ancestor, and a
/// run of adjacent subscripts each keep the tags.
#[test]
fn faithful_keeps_tags_of_step_one() {
    for (html, markdown) in [
        (
            r#"<p><sub class="x">a</sub></p>"#,
            r#"<sub class="x">a</sub>"#,
        ),
        ("<p><sub>a<!--c-->b</sub></p>", "<sub>a<!--c-->b</sub>"),
        ("<p><sub><sub>a</sub></sub></p>", "<sub><sub>a</sub></sub>"),
        (
            "<p><sub>a</sub><sub>b</sub></p>",
            "<sub>a</sub><sub>b</sub>",
        ),
    ] {
        assert_faithful(html, markdown, &format!("{html}\n"));
    }
}

/// Children of a retained `<sub>` are still translated in an inline context.
#[test]
fn faithful_translates_children_inside_kept_tags() {
    assert_faithful(
        "<p><sub><em>a</em></sub></p>",
        "<sub>*a*</sub>",
        "<p><sub><em>a</em></sub></p>\n",
    );
}

/// Approach step 3: every literal `~` is escaped, inside a subscript and out.
#[test]
fn literal_tildes_are_escaped() {
    assert_faithful(
        "<p><sub>a~b</sub></p>",
        r"<sub>a\~b</sub>",
        "<p><sub>a~b</sub></p>\n",
    );
    // Unescaped, `a ~b~ c` would read back as a subscript.
    assert_faithful("<p>a ~b~ c</p>", r"a \~b\~ c", "<p>a ~b~ c</p>\n");
    // `~~` is strikethrough under `ENABLE_STRIKETHROUGH`.
    let markdown = convert_faithful("<p>~~a~~</p>").unwrap();
    assert_eq!(r"\~\~a\~\~", markdown);
    assert_eq!(
        "<p>~~a~~</p>\n",
        render_markdown(
            &markdown,
            CommonMarkOptions::ENABLE_SUBSCRIPT | CommonMarkOptions::ENABLE_STRIKETHROUGH
        )
    );
    // A line starting with `~~~` would open a code fence.
    assert_faithful("<p>~~~</p>", r"\~\~\~", "<p>~~~</p>\n");
    assert_faithful(
        r#"<p><a href="u">a~b</a></p>"#,
        r"[a\~b](u)",
        "<p><a href=\"u\">a~b</a></p>\n",
    );
    assert_faithful(
        r#"<img src="u" alt="a~b">"#,
        r"![a\~b](u)",
        "<p><img src=\"u\" alt=\"a~b\" /></p>\n",
    );
}

/// Literal contexts follow their own rules, so a `~` in code is not escaped.
#[test]
fn tildes_in_code_stay_literal() {
    assert_eq!(
        "`a~b`",
        convert_faithful("<p><code>a~b</code></p>").unwrap()
    );
    assert_eq!(
        "```\n~~~\n```",
        convert_faithful("<pre><code>~~~</code></pre>").unwrap()
    );
}

/// Pure mode writes single-tilde delimiters, which read back as subscript
/// where they flank.
#[test]
fn pure_writes_tildes() {
    assert_pure("<p><sub>b</sub></p>", "~b~", "<p><sub>b</sub></p>\n");
    assert_pure(
        "<p>a <sub>b</sub> c</p>",
        "a ~b~ c",
        "<p>a <sub>b</sub> c</p>\n",
    );
    assert_pure(
        "<p>x <sub>2</sub>.</p>",
        "x ~2~.",
        "<p>x <sub>2</sub>.</p>\n",
    );
    assert_pure(
        "<p><sub>a~b</sub></p>",
        r"~a\~b~",
        "<p><sub>a~b</sub></p>\n",
    );
}

/// Pure mode is lossy: a subscript inside a word reads back as literal text,
/// since a single `~` cannot delimit text within a word.
#[test]
fn pure_loses_an_in_word_subscript() {
    assert_pure("<p>H<sub>2</sub>O</p>", "H~2~O", "<p>H~2~O</p>\n");
}

/// Pure mode ignores attributes and comments, and moves edge whitespace
/// outside the delimiters.
#[test]
fn pure_ignores_attributes_and_moves_whitespace() {
    assert_pure(
        r#"<p><sub class="x">a</sub></p>"#,
        "~a~",
        "<p><sub>a</sub></p>\n",
    );
    assert_pure(
        "<p><sub>a<!--c-->b</sub></p>",
        "~ab~",
        "<p><sub>ab</sub></p>\n",
    );
    assert_pure(
        "<p>x<sub> a </sub>y</p>",
        "x ~a~ y",
        "<p>x <sub>a</sub> y</p>\n",
    );
}

/// Pure mode drops an empty subscript rather than writing `~~`, keeping any
/// whitespace it held.
#[test]
fn pure_drops_empty_subscripts() {
    assert_pure("<p>a<sub></sub>b</p>", "ab", "<p>ab</p>\n");
    assert_pure("<p>a<sub> </sub>b</p>", "a b", "<p>a b</p>\n");
}

/// Pure mode combines adjacent subscripts and keeps only the outer level of a
/// nested one, since `~~a~~` would be no subscript at all.
#[test]
fn pure_combines_adjacent_and_flattens_nested() {
    assert_pure(
        "<p><sub>a</sub><sub>b</sub></p>",
        "~ab~",
        "<p><sub>ab</sub></p>\n",
    );
    assert_pure(
        "<p><sub><sub>a</sub></sub></p>",
        "~a~",
        "<p><sub>a</sub></p>\n",
    );
    assert_pure(
        "<p><sub><em>a</em></sub></p>",
        "~*a*~",
        "<p><sub><em>a</em></sub></p>\n",
    );
}

/// A run of adjacent subscripts is combined whatever its members hold; writing
/// each one would join their delimiters into a `~~` run, as in `~a~~*b*~`.
#[test]
fn pure_combines_any_adjacent_subscripts() {
    assert_pure(
        "<p><sub>a</sub><sub><em>b</em></sub></p>",
        "~a*b*~",
        "<p><sub>a<em>b</em></sub></p>\n",
    );
    assert_pure(
        r#"<p><sub>a</sub><sub class="x">b</sub></p>"#,
        "~ab~",
        "<p><sub>ab</sub></p>\n",
    );
    assert_pure(
        "<p><sub>a</sub><!--c--><sub>b</sub></p>",
        "~ab~",
        "<p><sub>ab</sub></p>\n",
    );
    assert_pure(
        "<p><sub>a</sub><sub>b</sub><sub><em>c</em></sub></p>",
        "~ab*c*~",
        "<p><sub>ab<em>c</em></sub></p>\n",
    );
    // Whitespace between them separates two subscripts.
    assert_pure(
        "<p><sub>a</sub> <sub>b</sub></p>",
        "~a~ ~b~",
        "<p><sub>a</sub> <sub>b</sub></p>\n",
    );
    // Faithful mode keeps each element of the run as HTML.
    assert_faithful(
        "<p><sub>a</sub><sub><em>b</em></sub></p>",
        "<sub>a</sub><sub>*b*</sub>",
        "<p><sub>a</sub><sub><em>b</em></sub></p>\n",
    );
}
