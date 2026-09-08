use crate::{
    Context, Element,
    element_handler::element_util::serialize_if_extra_attrs_or_inline,
    element_handler::{HandlerResult, Handlers},
    html_block::opens_html_block,
    options::HeadingStyle,
    text_util::TrimDocumentWhitespace,
};

pub(super) fn headings_handler(handlers: &dyn Handlers, element: Element) -> Option<HandlerResult> {
    serialize_if_extra_attrs_or_inline!(handlers, element, 0);
    let level = element.tag.chars().nth(1).unwrap() as u32 - '0' as u32;
    // A heading is a leaf block: its children begin an inline context.
    let content = handlers.walk_children_content(element.node, Context::Inline);
    let content = content.trim_document_whitespace();
    let content = content.trim_matches('\n');

    // A setext heading's content is a paragraph, so an HTML block opening in it
    // dissolves the heading. An ATX heading's `#` is leaf block syntax the
    // block scan matches first. See the headings section of
    // `unsupported_html.md`.
    let mut result = String::from("\n\n");
    if (level == 1 || level == 2)
        && handlers.options().heading_style == HeadingStyle::Setex
        && !opens_html_block(content)
    {
        // Use the Setext heading style for h1 and h2
        result.push_str(content);
        result.push('\n');
        let ch = if level == 1 { "=" } else { "-" };
        result.push_str(&ch.repeat(content.chars().count()));
        result.push_str("\n\n");
    } else {
        result.push_str(&"#".repeat(level as usize));
        result.push(' ');
        result.push_str(content);
        result.push_str("\n\n");
    }
    Some(result.into())
}
