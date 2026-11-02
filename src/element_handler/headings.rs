use crate::{
    Context, Element,
    element_handler::element_util::serialize_element_when_faithful,
    element_handler::{HandlerResult, Handlers},
    html_block::starts_html_block,
    options::{HeadingStyle, TranslationMode},
    util::text::TrimDocumentWhitespace,
};
use html5ever::Attribute;

pub(super) fn headings_handler(
    handlers: &dyn Handlers,
    element: &Element,
) -> Option<HandlerResult> {
    // Pure mode writes no attribute block, dropping every attribute.
    let attribute_block = if handlers.options().translation_mode == TranslationMode::Faithful {
        attribute_block(element.attrs)
    } else {
        Some(String::new())
    };
    serialize_element_when_faithful!(
        handlers,
        element,
        element.context.is_inline() || attribute_block.is_none()
    );
    let attribute_block = attribute_block.unwrap_or_default();
    let level = element.tag.chars().nth(1).unwrap() as u32 - '0' as u32;
    // A heading is a leaf block: its children begin an inline context.
    let content = handlers.walk_children_content(element.node, Context::INLINE);
    let content = content.trim_document_whitespace();
    let content = content.trim_matches('\n');

    // A setext heading leaves its content at the start of a line, where an HTML
    // block opening would dissolve the heading; an ATX heading's `#` is leaf
    // block syntax the block scan matches first, so it is safe. Empty content
    // has no setext spelling at all -- its underline is empty too, leaving
    // nothing behind -- so it falls back to ATX as well. See the "Special case
    // for paragraphs" and "Headings" sections of `unsupported_html.md`.
    let use_setext = (level == 1 || level == 2)
        && handlers.options().heading_style == HeadingStyle::Setex
        && !content.is_empty()
        && !starts_html_block(content);

    let mut line = content.to_string();
    if attribute_block.is_empty() {
        // A reader with heading attributes would take a final `}` as the end of
        // an attribute block. See the "Heading attributes" section of
        // `unsupported_html.md`.
        if line.ends_with('}') {
            line.insert(line.len() - 1, '\\');
        }
    } else {
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(&attribute_block);
    }

    let mut result = String::from("\n\n");
    if use_setext {
        result.push_str(&line);
        result.push('\n');
        let ch = if level == 1 { "=" } else { "-" };
        result.push_str(&ch.repeat(line.chars().count()));
        result.push_str("\n\n");
    } else {
        result.push_str(&"#".repeat(level as usize));
        // An empty heading is just the `#`s: a space with nothing after it
        // would leave trailing whitespace on the line.
        if !line.is_empty() {
            result.push(' ');
            result.push_str(&line);
        }
        result.push_str("\n\n");
    }
    Some(result.into())
}

/// Writes `attrs` as a pulldown-cmark
/// [attribute block](https://pulldown-cmark.github.io/pulldown-cmark/specs/heading_attrs.html),
/// or an empty string when there are none. Returns `None` when an attribute
/// cannot be written in one, which forces HTML. See the "Heading attributes"
/// section of `unsupported_html.md`.
fn attribute_block(attrs: &[Attribute]) -> Option<String> {
    if attrs.is_empty() {
        return Some(String::new());
    }
    let mut id = None;
    let mut classes = Vec::new();
    let mut others = Vec::new();
    for attr in attrs {
        let name = &*attr.name.local;
        let value = &*attr.value;
        if name == "id" && !value.is_empty() {
            if !is_attribute_block_text(value) {
                return None;
            }
            id = Some(format!("#{value}"));
        } else if name == "class" && value.split_ascii_whitespace().next().is_some() {
            // HTML splits a `class` on the same ASCII whitespace.
            for token in value.split_ascii_whitespace() {
                if !is_attribute_block_text(token) {
                    return None;
                }
                classes.push(format!(".{token}"));
            }
        } else {
            // A leading `#` or `.` would make the token an ID or a class, and
            // pulldown-cmark splits a token at its first `=`. It escapes `&`,
            // `"`, and `'`, which an HTML parser decodes only in a value.
            if !is_attribute_block_text(name)
                || name.starts_with(['#', '.'])
                || name.contains(['=', '&', '"', '\''])
                || !is_attribute_block_text(value)
            {
                return None;
            }
            // `name=` rather than a bare `name`, since pulldown-cmark ignores a
            // one-byte token.
            others.push(format!("{name}={value}"));
        }
    }
    // pulldown-cmark renders the ID, then the classes, then the rest.
    let tokens: Vec<String> = id.into_iter().chain(classes).chain(others).collect();
    Some(format!("{{{}}}", tokens.join(" ")))
}

/// Whether `text` can stand in an attribute block token, which decodes no
/// escape or character reference. ASCII whitespace separates tokens; the rest
/// break pulldown-cmark's scan for the block.
fn is_attribute_block_text(text: &str) -> bool {
    !text.contains(|ch: char| {
        ch.is_ascii_whitespace() || matches!(ch, '{' | '}' | '<' | '>' | '\\')
    })
}
