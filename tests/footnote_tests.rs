mod common;

use common::{convert_faithful, convert_faithful_options, convert_pure, render_markdown};
use htmd::options::{CodeBlockStyle, Options as ConversionOptions};
use pulldown_cmark::Options;

const ONE_NOTE: &str = r##"<p>a statement<sup class="footnote-reference"><a href="#1">1</a></sup></p>
<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
<p>this is true</p>
</div>"##;

fn render_footnotes(markdown: &str) -> String {
    render_markdown(markdown, Options::ENABLE_FOOTNOTES)
}

#[test]
fn translates_the_pulldown_cmark_example() {
    let markdown = convert_faithful(ONE_NOTE).unwrap();
    assert_eq!("a statement[^1]\n\n[^1]: this is true", markdown);
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<sup class=\"footnote-reference\"><a href=\"#1\">1</a></sup>"));
    assert!(rendered.contains("<div class=\"footnote-definition\" id=\"1\">"));
}

#[test]
fn keeps_a_note_as_html_when_an_empty_block_would_disappear() {
    let empty_blockquote = render_footnotes("x[^1]\n\n[^1]: >");
    let empty_blockquote_before_text = render_footnotes("x[^1]\n\n[^1]: >\n    text");
    let empty_paragraph = ONE_NOTE.replace("<p>this is true</p>", "<p></p>");
    for (html, block) in [
        (empty_blockquote.as_str(), "<blockquote>"),
        (empty_blockquote_before_text.as_str(), "<blockquote>"),
        (empty_paragraph.as_str(), "<p></p>"),
    ] {
        let markdown = convert_faithful(html).unwrap();
        assert!(!markdown.contains("[^1]"), "{markdown}");
        assert!(
            markdown.contains("<div class=\"footnote-definition\""),
            "{markdown}"
        );
        assert!(render_footnotes(&markdown).contains(block), "{markdown}");
    }
}

#[test]
fn a_reference_followed_by_a_colon_remains_a_reference() {
    let html = r##"<p><sup class="footnote-reference"><a href="#1">1</a></sup>: at the start</p><p>middle<sup class="footnote-reference"><a href="#1">1</a></sup>: inline</p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>the note</p></div>"##;
    for markdown in [convert_faithful(html).unwrap(), convert_pure(html).unwrap()] {
        assert_eq!(
            "[^1]\\: at the start\n\nmiddle[^1]\\: inline\n\n[^1]: the note",
            markdown
        );
        let rendered = render_footnotes(&markdown);
        assert_eq!(2, rendered.matches("class=\"footnote-reference\"").count());
        assert!(rendered.contains("</sup>: at the start</p>"));
        assert!(rendered.contains("</sup>: inline</p>"));
        assert!(rendered.contains("<p>the note</p>"));
    }
}

#[test]
fn a_pure_reference_inside_spans_followed_by_a_colon_remains_a_reference() {
    let html = r##"<p><span><span><sup class="footnote-reference"><a href="#1">1</a></sup></span></span>: at the start</p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>the note</p></div>"##;
    let markdown = convert_pure(html).unwrap();
    assert_eq!("[^1]\\: at the start\n\n[^1]: the note", markdown);
    let rendered = render_footnotes(&markdown);
    assert_eq!(1, rendered.matches("class=\"footnote-reference\"").count());
    assert_eq!(1, rendered.matches("class=\"footnote-definition\"").count());
    assert!(rendered.contains("</sup>: at the start</p>"));
}

#[test]
fn a_pure_reference_skips_empty_spans_before_a_colon() {
    for gap in [
        "<span></span>",
        "<span class=\"empty\"><span></span></span>",
    ] {
        let html = format!(
            r##"<p><sup class="footnote-reference"><a href="#1">1</a></sup>{gap}: text</p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div>"##
        );
        let markdown = convert_pure(&html).unwrap();
        assert_eq!("[^1]\\: text\n\n[^1]: note", markdown);
        let rendered = render_footnotes(&markdown);
        assert_eq!(1, rendered.matches("class=\"footnote-reference\"").count());
        assert_eq!(1, rendered.matches("class=\"footnote-definition\"").count());
    }
}

#[test]
fn pure_mode_escapes_colons_across_transparent_wrappers() {
    let reference = r##"<sup class="footnote-reference"><a href="#1">1</a></sup>"##;
    let definition = r#"<div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div>"#;
    let cases = [
        format!("<p>{reference}<span>: text</span></p>{definition}"),
        format!("<p>{reference}<span><span>: text</span></span></p>{definition}"),
        format!("<p><foo>{reference}</foo>: text</p>{definition}"),
        format!("<p>{reference}<foo></foo>: text</p>{definition}"),
    ];
    for html in cases {
        let markdown = convert_pure(&html).unwrap();
        assert_eq!("[^1]\\: text\n\n[^1]: note", markdown);
        let rendered = render_footnotes(&markdown);
        assert_eq!(1, rendered.matches("class=\"footnote-reference\"").count());
        assert_eq!(1, rendered.matches("class=\"footnote-definition\"").count());
    }
}

#[test]
fn keeps_multiple_references_and_the_definition_in_source_order() {
    let html = r##"<p>first<sup class="footnote-reference"><a href="#1">1</a></sup> and second<sup class="footnote-reference"><a href="#1">1</a></sup>.</p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div><p>after</p>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!("first[^1] and second[^1].\n\n[^1]: note\n\nafter", markdown);
    let rendered = render_footnotes(&markdown);
    assert_eq!(2, rendered.matches("class=\"footnote-reference\"").count());
    assert!(rendered.find("</div>").unwrap() < rendered.find("<p>after</p>").unwrap());
}

#[test]
fn keeps_a_note_local_when_another_reference_is_inside_raw_html() {
    let html = r##"<div class="box"><p>raw<sup class="footnote-reference"><a href="#1">1</a></sup></p></div><p>converted<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("<div class=\"box\">"));
    assert!(markdown.contains("converted[^1]"));
    assert!(markdown.contains("[^1]: note"));
}

#[test]
fn indents_following_paragraphs_inside_the_definition() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>first</p><p>second</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!("x[^1]\n\n[^1]: first\n\n    second", markdown);
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<p>first</p>"));
    assert!(rendered.contains("<p>second</p>"));
}

#[test]
fn leaves_an_ordinary_link_to_the_target_ordinary() {
    let html = format!("{ONE_NOTE}<p><a href=\"#1\">see note</a></p>");
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("a statement[^1]"));
    assert!(markdown.contains("[^1]: this is true"));
    assert!(markdown.contains("[see note](#1)"));
}

#[test]
fn ignores_unrelated_ids_and_base_href() {
    let duplicate = format!("{ONE_NOTE}<span id=\"1\"></span>");
    let based = format!("<base href=\"https://example.com/other\">{ONE_NOTE}");
    for html in [duplicate, based] {
        let markdown = convert_faithful(&html).unwrap();
        assert!(markdown.contains("[^1]: this is true"));
    }
}

#[test]
fn faithful_mode_rejects_extra_attributes_and_a_changed_fragment() {
    let extra = ONE_NOTE.replace(
        "class=\"footnote-reference\"",
        "class=\"footnote-reference\" title=\"source\"",
    );
    let encoded = ONE_NOTE.replace("href=\"#1\"", "href=\"#%31\"");
    for html in [extra, encoded] {
        let markdown = convert_faithful(&html).unwrap();
        assert!(markdown.contains("<sup class=\"footnote-reference\""));
        assert!(!markdown.contains("[^1]:"));
    }
}

#[test]
fn preserves_a_rejected_reference_inside_a_heading_with_a_multiline_attribute() {
    let html = r##"<h1>heading<sup class="footnote-reference" title="first&#10;second"><a href="#1">1</a></sup></h1><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>note</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(
        markdown.contains("title=\"first&#10;second\""),
        "{markdown}"
    );
    assert!(!markdown.contains("[^1]"));
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<h1>heading<sup"), "{rendered}");
    assert!(
        rendered.contains("<a href=\"#1\">1</a></sup></h1>"),
        "{rendered}"
    );
}

#[test]
fn pure_mode_uses_the_id_as_a_label() {
    let html = r##"<p>x<sup class="footnote-reference" title="source"><a href="#note" data-x="y">1</a></sup></p><div class="footnote-definition" id="note" data-x="y"><p>text</p></div>"##;
    let markdown = convert_pure(html).unwrap();
    assert_eq!("x[^note]\n\n[^note]: text", markdown);
    assert!(render_footnotes(&markdown).contains("href=\"#note\""));
}

#[test]
fn leaves_mismatched_display_numbers_as_html() {
    let html = ONE_NOTE.replace(
        "footnote-definition-label\">1",
        "footnote-definition-label\">2",
    );
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]:"));
}

#[test]
fn rejects_only_the_note_that_cannot_convert() {
    let second = ONE_NOTE
        .replace("#1", "#2")
        .replace("id=\"1\"", "id=\"2\"")
        .replace(">1<", ">2<");
    let second = second.replace(
        "footnote-reference\"",
        "footnote-reference\" title=\"extra\"",
    );
    let html = format!("{ONE_NOTE}{second}");
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("[^1]: this is true"));
    assert!(!markdown.contains("[^2]:"));
    assert_eq!(
        1,
        markdown
            .matches("<div class=\"footnote-definition\"")
            .count()
    );
}

#[test]
fn rejects_a_definition_nested_inside_another() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>first</p><div class="footnote-definition" id="2"><sup class="footnote-definition-label">2</sup><p>second</p></div></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]:"));
}

#[test]
fn pure_mode_accepts_empty_wrappers_around_the_structural_elements() {
    let html = r##"<p>x<sup class="footnote-reference"><em> <a href="#note">1</a> </em></sup></p><div class="footnote-definition" id="note"><span> <sup class="footnote-definition-label">1</sup> </span><p>text</p></div>"##;
    assert_eq!("x[^note]\n\n[^note]: text", convert_pure(html).unwrap());
}

#[test]
fn a_full_url_does_not_count_as_a_same_document_link() {
    let html = format!("{ONE_NOTE}<p><a href=\"https://example.com/page#1\">elsewhere</a></p>");
    let markdown = convert_faithful(&html).unwrap();
    assert!(markdown.contains("[^1]: this is true"));
}

#[test]
fn faithful_mode_keeps_a_nonnumeric_id_when_the_display_number_survives() {
    let html = ONE_NOTE
        .replace("#1", "#note")
        .replace("id=\"1\"", "id=\"note\"");
    let markdown = convert_faithful(&html).unwrap();
    assert_eq!("a statement[^note]\n\n[^note]: this is true", markdown);
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("href=\"#note\">1</a>"));
    assert!(rendered.contains("id=\"note\"><sup class=\"footnote-definition-label\">1</sup>"));
}

#[test]
fn places_a_first_nonparagraph_block_below_the_marker() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><ul><li>item</li></ul></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("[^1]:\n    *"), "{markdown}");
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<ul>"));
    assert!(rendered.contains("<li>item</li>"));
}

#[test]
fn checks_display_numbers_per_note_without_reordering_references() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#2">2</a></sup> y<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>one</p></div><div class="footnote-definition" id="2"><sup class="footnote-definition-label">2</sup><p>two</p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("x[^2] y[^1]"));
    assert!(markdown.contains("[^1]: one"));
    assert!(markdown.contains("[^2]: two"));
}

#[test]
fn retains_inline_formatting_in_a_note_body() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><p>some <em>emphasis</em></p></div>"##;
    let markdown = convert_faithful(html).unwrap();
    assert_eq!("x[^1]\n\n[^1]: some *emphasis*", markdown);
    assert!(render_footnotes(&markdown).contains("<em>emphasis</em>"));
}

#[test]
fn does_not_percent_decode_fragments_in_either_mode() {
    let html = ONE_NOTE.replace("href=\"#1\"", "href=\"#%31\"");
    let pure = convert_pure(&html).unwrap();
    assert!(pure.contains("a statement[^%31]"));
    assert!(pure.contains("[^1]: this is true"));
    let faithful = convert_faithful(&html).unwrap();
    assert!(faithful.contains("href=\"#%31\""));
    assert!(!faithful.contains("[^1]:"));
}

#[test]
fn pure_mode_translates_references_and_definitions_independently() {
    let reference = r##"<p>x<sup class="footnote-reference"><a href="#missing">1</a></sup></p>"##;
    assert_eq!("x[^missing]", convert_pure(reference).unwrap());
    assert!(
        convert_faithful(reference)
            .unwrap()
            .contains("<sup class=\"footnote-reference\">")
    );

    let definition = r#"<div class="footnote-definition" id="unused"><p>text</p></div>"#;
    assert_eq!("[^unused]: text", convert_pure(definition).unwrap());
    assert!(
        convert_faithful(definition)
            .unwrap()
            .contains("<div class=\"footnote-definition\" id=\"unused\">")
    );
}

#[test]
fn rejects_duplicate_definitions_for_an_id() {
    let html = format!(
        "{ONE_NOTE}<div class=\"footnote-definition\" id=\"1\"><sup class=\"footnote-definition-label\">1</sup><p>other</p></div>"
    );
    let markdown = convert_faithful(&html).unwrap();
    assert!(!markdown.contains("[^1]:"));
    assert_eq!(
        2,
        markdown
            .matches("<div class=\"footnote-definition\"")
            .count()
    );
}

#[test]
fn rejects_invalid_ids_and_zero_display_numbers() {
    let long_id = "0".repeat(1000);
    for id in ["bad id", "bad]id", "bad|id", long_id.as_str()] {
        let html = ONE_NOTE
            .replace("href=\"#1\"", &format!("href=\"#{id}\""))
            .replace("id=\"1\"", &format!("id=\"{id}\""));
        assert!(!convert_faithful(&html).unwrap().contains("[^1]:"));
    }
    let zero = ONE_NOTE.replace(">1<", ">0<");
    assert!(!convert_faithful(&zero).unwrap().contains("[^1]:"));
}

#[test]
fn rejects_mismatched_references_for_one_id() {
    let html = ONE_NOTE.replacen(
        "</p>",
        r##"<sup class="footnote-reference"><a href="#1">2</a></sup></p>"##,
        1,
    );
    let markdown = convert_faithful(&html).unwrap();
    assert!(!markdown.contains("[^1]:"));
    assert_eq!(
        2,
        markdown
            .matches("<sup class=\"footnote-reference\">")
            .count()
    );
}

#[test]
fn faithful_mode_rejects_extra_attributes_on_each_structural_element() {
    let cases = [
        ONE_NOTE.replace("href=\"#1\"", "href=\"#1\" title=\"source\""),
        ONE_NOTE.replace("id=\"1\"", "id=\"1\" data-x=\"source\""),
        ONE_NOTE.replace(
            "class=\"footnote-definition-label\"",
            "class=\"footnote-definition-label\" data-x=\"source\"",
        ),
    ];
    for html in cases {
        let markdown = convert_faithful(&html).unwrap();
        assert!(!markdown.contains("[^1]"), "{markdown}");
        assert!(
            markdown.contains("<div class=\"footnote-definition\""),
            "{markdown}"
        );
    }
}

#[test]
fn keeps_a_following_block_outside_the_definition() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup><h2>Heading</h2><blockquote><p>quoted</p></blockquote></div><p>after</p>"##;
    let markdown = convert_faithful(html).unwrap();
    assert!(markdown.contains("[^1]:\n    ## Heading"), "{markdown}");
    assert!(markdown.contains("\n\nafter"), "{markdown}");
    let rendered = render_footnotes(&markdown);
    assert!(rendered.contains("<h2>Heading</h2>"), "{rendered}");
    assert!(rendered.contains("<blockquote>"), "{rendered}");
    assert!(rendered.find("</div>").unwrap() < rendered.find("<p>after</p>").unwrap());
}

#[test]
fn keeps_a_first_indented_code_block_inside_the_definition() {
    let html = r##"<p>x<sup class="footnote-reference"><a href="#1">1</a></sup></p><div class="footnote-definition" id="1"><sup class="footnote-definition-label">1</sup>
<pre><code>code</code></pre></div>"##;
    let markdown = convert_faithful_options(
        html,
        ConversionOptions {
            code_block_style: CodeBlockStyle::Indented,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!("x[^1]\n\n[^1]:\n        code", markdown);
    assert!(render_footnotes(&markdown).contains("<pre><code>code</code></pre>"));
}

#[test]
fn rejects_a_definition_before_an_indented_code_block() {
    let html = format!("{ONE_NOTE}<pre><code>after</code></pre>");
    let markdown = convert_faithful_options(
        &html,
        ConversionOptions {
            code_block_style: CodeBlockStyle::Indented,
            ..Default::default()
        },
    )
    .unwrap();
    assert!(markdown.contains("<div class=\"footnote-definition\" id=\"1\">"));
    assert!(!markdown.contains("[^1]:"));
}
