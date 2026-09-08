//! Every table row of `unsupported_html.md`, checked against the translation
//! the *Faithful expected* column asks for.
//!
//! The "Inline elements" and "Lists" sections are marked "under development.
//! Ignore it." in that document and are left out here.
//!
//! Where a row writes `<br><br>...<br>` — "one or more `<br>` elements" — it is
//! checked with one, two and three of them, unless the section gives a lone
//! `<br>` and a run their own rows.

use pretty_assertions::assert_eq;

mod common;
use common::{convert_faithful, convert_faithful_setext, one_cell_table};

/// Checks `template` with a run of two and of three `<br>`s substituted for
/// `{brs}`.
fn check_runs(template: &str, expected: &str) {
    for count in 2..=3 {
        let brs = "<br>".repeat(count);
        assert_eq!(
            expected.replace("{brs}", &brs),
            convert_faithful(&template.replace("{brs}", &brs)).unwrap(),
            "{count} <br>s in {template}"
        );
    }
}

/// [`check_runs`] for a row which covers a lone `<br>` as well.
fn check_one_or_more(template: &str, expected: &str) {
    assert_eq!(
        expected.replace("{brs}", "<br>"),
        convert_faithful(&template.replace("{brs}", "<br>")).unwrap(),
        "one <br> in {template}"
    );
    check_runs(template, expected);
}

/// The "Translating HTML nodes" section's classification table, one node of
/// each type in each context. An inline context is written as the content of an
/// `<h1>`.
#[test]
fn translating_html_nodes() {
    // Type 1 in a block context is an HTML block. `<body>` keeps html5ever from
    // filing a leading `<script>` or `<style>` under the `<head>`.
    assert_eq!(
        "<script>a</script>",
        convert_faithful("<body><script>a</script></body>").unwrap()
    );
    assert_eq!("<pre>a</pre>", convert_faithful("<pre>a</pre>").unwrap());
    // Type 2-5 in a block context. html5ever hands a CDATA section and a
    // processing instruction back as comments, so all four arrive as type 2.
    assert_eq!("<!--a-->", convert_faithful("<!--a-->").unwrap());
    assert_eq!(
        "<!--[CDATA[a]]-->",
        convert_faithful("<![CDATA[a]]>").unwrap()
    );
    // Type 6 in a block context is an HTML block.
    assert_eq!("<div>a</div>", convert_faithful("<div>a</div>").unwrap());
    // Type 7 in a block context is a raw HTML inline all the same.
    assert_eq!("<br>", convert_faithful("<br>").unwrap());

    // Every type in an inline context is a raw HTML inline.
    assert_eq!(
        "# x<script>a</script>y",
        convert_faithful("<h1>x<script>a</script>y</h1>").unwrap()
    );
    assert_eq!(
        "# x<!--a-->y",
        convert_faithful("<h1>x<!--a-->y</h1>").unwrap()
    );
    assert_eq!(
        "# x<div>a</div>y",
        convert_faithful("<h1>x<div>a</div>y</h1>").unwrap()
    );
    assert_eq!("# x<br>y", convert_faithful("<h1>x<br>y</h1>").unwrap());

    // A type 6 HTML block cannot hold a blank line, so one is encoded; a type 1
    // block ends at its closing tag instead and keeps its blank lines.
    assert_eq!(
        "<div>a\n&#10;b</div>",
        convert_faithful("<div>a\n\nb</div>").unwrap()
    );
    assert_eq!(
        "<pre>a\n\nb</pre>",
        convert_faithful("<pre>a\n\nb</pre>").unwrap()
    );
    // Every raw HTML inline has its whitespace collapsed.
    for (html, expected) in [
        (
            "<h1>x<script>a\n\nb</script>y</h1>",
            "# x<script>a b</script>y",
        ),
        ("<h1>x<!--a\n\nb-->y</h1>", "# x<!--a b-->y"),
        ("<h1>x<div>a\n\nb</div>y</h1>", "# x<div>a b</div>y"),
        ("<h1>x<pre>a\n\nb</pre>y</h1>", "# x<pre>a b</pre>y"),
    ] {
        assert_eq!(expected, convert_faithful(html).unwrap(), "{html}");
    }

    // The type 1 tradeoff the same section names: a `<script>`'s content is
    // walked like any other raw HTML inline, so its Markdown is escaped.
    assert_eq!(
        r"# a<script>b\*c\*d</script>e",
        convert_faithful("<h1>a<script>b*c*d</script>e</h1>").unwrap()
    );

    // A `<!DOCTYPE html>` declaration is dropped.
    assert_eq!(
        "a",
        convert_faithful("<!DOCTYPE html><html><body><p>a</p></body></html>").unwrap()
    );
}

/// "CommonMark blocks may only process their content into CommonMark in a block
/// context; in an inline context, they should emit raw HTML inlines."
#[test]
fn a_commonmark_block_in_an_inline_context() {
    assert_eq!("# <p>a</p>", convert_faithful("<h1><p>a</p></h1>").unwrap());
}

/// The "Special case for paragraphs" section's table.
#[test]
fn special_case_for_paragraphs() {
    assert_eq!(
        r#"<p><iframe src="u">a</iframe></p>"#,
        convert_faithful(r#"<p><iframe src="u">a</iframe></p>"#).unwrap()
    );
}

/// The Code section's table.
#[test]
fn code() {
    check_one_or_more("<p>a<code>x{brs}y</code>b</p>", "a<code>x{brs}y</code>b");
    check_one_or_more("<p>a<code>{brs}</code>b</p>", "a<code>{brs}</code>b");
    check_one_or_more(
        "<pre><code>a{brs}b</code></pre>",
        "<pre><code>a{brs}b</code></pre>",
    );
    check_one_or_more(
        "<pre><code>{brs}</code></pre>",
        "<pre><code>{brs}</code></pre>",
    );
}

/// The Links section's table.
#[test]
fn links() {
    check_one_or_more(r#"<p>a<a href="u">{brs}</a>b</p>"#, "a[{brs}](u)b");
    check_one_or_more(r#"<p>a<a href="u">{brs}c</a>b</p>"#, "a[{brs}c](u)b");
    check_one_or_more(r#"<p>a<a href="u">c{brs}</a>b</p>"#, "a[c{brs}](u)b");
}

/// The "At the document root" section's table.
#[test]
fn at_the_document_root() {
    assert_eq!("<br>", convert_faithful("<br>").unwrap());
    check_runs("{brs}", "{brs}");
    assert_eq!("<br>\n\na", convert_faithful("<br><p>a</p>").unwrap());
    assert_eq!("a\n\n<br>", convert_faithful("<p>a</p><br>").unwrap());
    check_one_or_more("<div>{brs}</div>", "<div>{brs}</div>");
}

/// The Headings section's setext table.
#[test]
fn setext_headings() {
    assert_eq!("# <br>", convert_faithful_setext("<h1><br></h1>"));
    assert_eq!("## <br>", convert_faithful_setext("<h2><br></h2>"));
    for count in 2..=3 {
        let brs = "<br>".repeat(count);
        assert_eq!(
            format!("{brs}\n{}", "=".repeat(brs.len())),
            convert_faithful_setext(&format!("<h1>{brs}</h1>")),
            "{count} <br>s in an h1"
        );
    }
    assert_eq!(
        "<br>*b*\n=======",
        convert_faithful_setext("<h1><br><em>b</em></h1>")
    );
    assert_eq!(
        "*a*<br>\n=======",
        convert_faithful_setext("<h1><em>a</em><br></h1>")
    );
    assert_eq!(
        "<br>*b*\n-------",
        convert_faithful_setext("<h2><br><em>b</em></h2>")
    );
    assert_eq!("### <br>", convert_faithful_setext("<h3><br></h3>"));
}

/// The Headings section's ATX table.
#[test]
fn atx_headings() {
    check_one_or_more("<h1>{brs}</h1>", "# {brs}");
    assert_eq!(
        "# <br>*b*",
        convert_faithful("<h1><br><em>b</em></h1>").unwrap()
    );
    assert_eq!(
        "# *a*<br>",
        convert_faithful("<h1><em>a</em><br></h1>").unwrap()
    );
    for level in 1..=6 {
        assert_eq!(
            format!("{} <br>", "#".repeat(level)),
            convert_faithful(&format!("<h{level}><br></h{level}>")).unwrap(),
            "level {level}"
        );
    }
}

/// The Paragraphs section's table.
#[test]
fn paragraphs() {
    assert_eq!("<p><br></p>", convert_faithful("<p><br></p>").unwrap());
    check_runs("<p>{brs}</p>", "{brs}");
    assert_eq!(
        "<br>*b*",
        convert_faithful("<p><br><em>b</em></p>").unwrap()
    );
    assert_eq!(
        "*a*<br>",
        convert_faithful("<p><em>a</em><br></p>").unwrap()
    );
    assert_eq!(
        "<br>![](i)",
        convert_faithful(r#"<p><br><img src="i"></p>"#).unwrap()
    );
    assert_eq!(
        "![](i)<br>",
        convert_faithful(r#"<p><img src="i"><br></p>"#).unwrap()
    );
}

/// The Blockquotes section's table.
#[test]
fn blockquotes() {
    check_one_or_more("<blockquote>{brs}</blockquote>", "> {brs}");
    assert_eq!(
        "> <p><br></p>",
        convert_faithful("<blockquote><p><br></p></blockquote>").unwrap()
    );
    assert_eq!(
        "> <br>*b*",
        convert_faithful("<blockquote><p><br><em>b</em></p></blockquote>").unwrap()
    );
    assert_eq!(
        "> *a*<br>",
        convert_faithful("<blockquote><p><em>a</em><br></p></blockquote>").unwrap()
    );
}

/// The Math section: "all newlines are removed from math expressions".
#[test]
fn math() {
    assert_eq!(
        "x$ab$y",
        convert_faithful("<p>x<span class=\"math math-inline\">a\nb</span>y</p>").unwrap()
    );
    assert_eq!(
        "$$ab$$",
        convert_faithful("<p><span class=\"math math-display\">a\nb</span></p>").unwrap()
    );
    assert_eq!(
        "# x$ab$y",
        convert_faithful("<h1>x<span class=\"math math-inline\">a\r\nb</span>y</h1>").unwrap()
    );
}

/// The "Table cells" section's table, plus its rule that a `|` in a cell is
/// escaped and that a child which could only be written as HTML sends the whole
/// table out as HTML.
#[test]
fn table_cells() {
    assert_eq!(
        "| h       |\n| ------- |\n| <br>*b* |",
        convert_faithful(&one_cell_table("<br><em>b</em>")).unwrap()
    );
    assert_eq!(
        "| h       |\n| ------- |\n| *a*<br> |",
        convert_faithful(&one_cell_table("<em>a</em><br>")).unwrap()
    );
    for count in 1..=3 {
        let brs = "<br>".repeat(count);
        let width = brs.len().max(1);
        assert_eq!(
            format!(
                "| h{} |\n| {} |\n| {brs} |",
                " ".repeat(width - 1),
                "-".repeat(width)
            ),
            convert_faithful(&one_cell_table(&brs)).unwrap(),
            "{count} <br>s in a cell"
        );
    }
    assert_eq!(
        "| h    |\n| ---- |\n| a\\|b |",
        convert_faithful(&one_cell_table("a|b")).unwrap()
    );
    // A `<caption>` can only be written as HTML, so the table goes with it.
    let with_caption = "<table><caption>c</caption><tbody><tr><th>h</th></tr>\
                        <tr><td>a</td></tr></tbody></table>";
    assert_eq!(with_caption, convert_faithful(with_caption).unwrap());
    // So does a cell carrying an attribute.
    let with_colspan = "<table><tbody><tr><th>h</th></tr>\
                        <tr><td colspan=\"2\">a</td></tr></tbody></table>";
    assert_eq!(with_colspan, convert_faithful(with_colspan).unwrap());
}
