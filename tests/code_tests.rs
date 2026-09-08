use std::rc::Rc;

use htmd::{
    Element, HtmlToMarkdown, Node,
    element_handler::Handlers,
    options::{CodeBlockFence, CodeBlockStyle, Options},
};
use indoc::indoc;
use markup5ever_rcdom::NodeData;
use pretty_assertions::assert_eq;

mod common;
use common::{convert_faithful, convert_with, faithful_options};

fn find_element(node: &Rc<Node>, tag: &str) -> Option<Rc<Node>> {
    if let NodeData::Element { name, .. } = &node.data
        && name.local.as_ref() == tag
    {
        return Some(node.clone());
    }

    node.children
        .borrow()
        .iter()
        .find_map(|child| find_element(child, tag))
}

#[test]
fn code_blocks() {
    let html = r#"
        <pre><code>println!("Hello");</code></pre>
        "#;
    assert_eq!(
        "```\nprintln!(\"Hello\");\n```",
        convert_faithful(html).unwrap()
    );
}

#[test]
fn code_blocks_with_lang_class() {
    let html = r#"
        <pre><code class="language-rust">println!("Hello");</code></pre>
        "#;
    assert_eq!(
        "```rust\nprintln!(\"Hello\");\n```",
        convert_faithful(html).unwrap()
    );
}

#[test]
fn faithful_mode_preserves_an_empty_language_class() {
    let html = r#"<pre><code class="language-">Test</code></pre>"#;

    assert_eq!(html, convert_faithful(html).unwrap());
}

#[test]
fn code_blocks_decode_html_entities() {
    let html = r#"<pre><code>let x = 5 &amp;&amp; y &lt; 10;</code></pre>"#;

    assert_eq!(
        "```\nlet x = 5 && y < 10;\n```",
        convert_faithful(html).unwrap()
    );
}

// See https://github.com/letmutex/htmd/issues/14 for background on this test --
// the `class` attribute is deliberately misplaced to support Markdown renderers
// which don't follow the CommonMark spec.
#[test]
fn code_blocks_with_lang_class_on_pre_tag() {
    let html = r#"
        <pre class="language-rust"><code>println!("Hello");</code></pre>
        "#;
    assert_eq!(
        "```rust\nprintln!(\"Hello\");\n```",
        htmd::convert(html).unwrap()
    );
}

#[test]
fn span_subtree_conversion_preserves_ancestor_preformatted_context() {
    let converter = HtmlToMarkdown::new();
    let tree = converter
        .html_to_tree("<pre><span>  *literal*  \nsecond</span></pre>")
        .unwrap();
    let span = find_element(&tree, "span").unwrap();

    assert_eq!("  *literal*  \nsecond", converter.tree_to_markdown(&span));
}

#[test]
fn delegated_unhandled_subtree_preserves_ancestor_preformatted_context() {
    let converter = HtmlToMarkdown::builder()
        .add_handler(
            vec!["delegate"],
            |handlers: &dyn Handlers, element: Element| {
                let child = element.node.children.borrow().first()?.clone();
                handlers.handle(&child, element.context)
            },
        )
        .build();

    assert_eq!(
        "  *literal*  \nsecond",
        converter
            .convert("<pre><delegate><mark>  *literal*  \nsecond</mark></delegate></pre>")
            .unwrap()
    );
}

#[test]
fn faithful_mode_pre() {
    assert_eq!(
        convert_faithful(indoc!(r#"<pre>Test</pre>"#)).unwrap(),
        indoc!(r#"<pre>Test</pre>"#)
    );
}

#[test]
fn faithful_mode_code_block1() {
    assert_eq!(
        convert_faithful(indoc!(r#"<pre><code accesskey="f">Test</code></pre>"#)).unwrap(),
        indoc!(r#"<pre><code accesskey="f">Test</code></pre>"#)
    );
}

#[test]
fn faithful_mode_code_block2() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<pre><code class="language-ruby"><i>Test</i></code></pre>"#
        ))
        .unwrap(),
        indoc!(r#"<pre><code class="language-ruby"><i>Test</i></code></pre>"#)
    );
}

#[test]
fn inline_code_made_only_of_backticks_uses_a_non_colliding_delimiter() {
    let markdown = htmd::convert("<code>``</code>").unwrap();

    assert_eq!("``` `` ```", markdown);
}

#[test]
fn inline_code_ending_in_a_backtick_keeps_the_backtick_inside_the_span() {
    let markdown = htmd::convert("<code>code`</code>").unwrap();

    assert_eq!("`` code` ``", markdown);
}

#[test]
fn preformatted_inline_code_preserves_boundary_spaces() {
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            preformatted_code: true,
            ..Default::default()
        })
        .build();

    let markdown = converter.convert("<code> foo </code>").unwrap();

    assert_eq!("`  foo  `", markdown);
}

#[test]
fn fenced_code_uses_a_fence_longer_than_any_run_in_its_content() {
    let markdown =
        htmd::convert("<pre><code>`````\nlet parsed = true;\n`````</code></pre>").unwrap();

    assert_eq!("``````\n`````\nlet parsed = true;\n`````\n``````", markdown);
}

#[test]
fn tilde_fenced_code_uses_a_fence_longer_than_any_run_in_its_content() {
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            code_block_fence: CodeBlockFence::Tildes,
            ..Default::default()
        })
        .build();

    let markdown = converter
        .convert("<pre><code>~~~~~\nlet parsed = true;\n~~~~~</code></pre>")
        .unwrap();

    assert_eq!("~~~~~~\n~~~~~\nlet parsed = true;\n~~~~~\n~~~~~~", markdown);
}

/// A line ending in a code span cannot be encoded or escaped away, and a blank
/// one ends the paragraph holding the span. Hence the HTML in every row of the
/// code section of `unsupported_html.md`.
#[test]
fn a_code_span_holding_a_line_ending_is_written_as_html() {
    assert_eq!(
        "a<code>x y</code>b",
        convert_faithful("<p>a<code>x\ny</code>b</p>").unwrap()
    );
    assert_eq!(
        "<code>a b</code>",
        convert_faithful("<p><code>a\n\nb</code></p>").unwrap()
    );
    assert_eq!(
        "<code>a === b</code>",
        convert_faithful("<p><code>a\n===\nb</code></p>").unwrap()
    );
    assert_eq!(
        "# <code>a b</code>",
        convert_faithful("<h1><code>a\n\nb</code></h1>").unwrap()
    );
    assert_eq!(
        "> <code>a b</code>",
        convert_faithful("<blockquote><p><code>a\n\nb</code></p></blockquote>").unwrap()
    );
    assert_eq!(
        "a`xy`b",
        convert_faithful("<p>a<code>xy</code>b</p>").unwrap()
    );
    // A `<code>` inside a `<pre>` is a code block, which keeps its line
    // endings.
    assert_eq!(
        "```\na\nb\n```",
        convert_faithful("<pre><code>a\nb</code></pre>").unwrap()
    );
}

/// CommonMark cannot spell an empty code span: with nothing between them the
/// delimiters meet, and a backtick string closed by no other is literal text.
#[test]
fn an_empty_code_span_is_written_as_html() {
    assert_eq!(
        "a<code></code>b",
        convert_faithful("<p>a<code></code>b</p>").unwrap()
    );
    assert_eq!("<code></code>", convert_faithful("<code></code>").unwrap());
    assert_eq!(
        "# a<code></code>b",
        convert_faithful("<h1>a<code></code>b</h1>").unwrap()
    );
    // Whitespace-only content is trimmed away, leaving the same empty span.
    assert_eq!(
        "a<code> </code>b",
        convert_faithful("<p>a<code>  </code>b</p>").unwrap()
    );
    // Pure mode has no HTML to fall back on, so the span writes nothing.
    assert_eq!("ab", htmd::convert("<p>a<code></code>b</p>").unwrap());
}

/// A content line in an empty code block would give it a blank line the
/// `<code>` never held.
#[test]
fn an_empty_code_block_has_no_content_line() {
    assert_eq!(
        "```\n```",
        convert_faithful("<pre><code></code></pre>").unwrap()
    );
    assert_eq!(
        "```rust\n```",
        convert_faithful(r#"<pre><code class="language-rust"></code></pre>"#).unwrap()
    );
}

/// A `<code>` holding one line ending is not empty: that line ending is the
/// whole of its content, and the block needs a content line to carry it.
#[test]
fn a_code_block_holding_only_a_line_ending_keeps_its_blank_line() {
    assert_eq!(
        "```\n\n```",
        convert_faithful("<pre><code>\n</code></pre>").unwrap()
    );
    assert_eq!(
        "```\na\n\n```",
        convert_faithful("<pre><code>a\n\n</code></pre>").unwrap()
    );
}

/// Indented code cannot spell an empty block — a line of nothing but the four
/// spaces is blank — so without the HTML fallback the `<pre>` would vanish.
#[test]
fn an_empty_indented_code_block_is_written_as_html() {
    let indented = || Options {
        code_block_style: CodeBlockStyle::Indented,
        ..faithful_options()
    };
    assert_eq!(
        "x\n\n<pre><code></code></pre>\n\ny",
        convert_with(indented(), "<p>x</p><pre><code></code></pre><p>y</p>").unwrap()
    );
    // A `<code>` holding one line ending has no content line either.
    assert_eq!(
        "x\n\n<pre><code>\n</code></pre>\n\ny",
        convert_with(indented(), "<p>x</p><pre><code>\n</code></pre><p>y</p>").unwrap()
    );
}

#[test]
fn processing_instruction_nodes_are_ignored() {
    let node = Node::new(NodeData::ProcessingInstruction {
        target: "xml".into(),
        contents: "version=\"1.0\"".into(),
    });

    assert_eq!("", HtmlToMarkdown::new().tree_to_markdown(&node));
}
