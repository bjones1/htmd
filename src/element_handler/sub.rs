use std::rc::Rc;

use markup5ever_rcdom::Node;

use crate::{
    Context, ContextKind, Element,
    element_handler::{HandlerResult, Handlers, element_util::serialize_element_when_faithful},
    util::{
        node::{get_node_tag_name, get_parent_node},
        text::{StripWhitespace, concat_strings},
    },
};

/// Translates `<sub>` to the single-tilde subscript of pulldown-cmark's
/// `Options::ENABLE_SUBSCRIPT`. See the "Subscript" section of
/// `translation_approach.md`.
pub(super) fn sub_handler(handlers: &dyn Handlers, element: &Element) -> Option<HandlerResult> {
    // Approach step 4: whether a delimiter flanks depends on the emitted
    // characters on either side of the element, which a handler cannot see.
    // Faithful mode therefore always keeps the tags, translating the children
    // as a raw HTML inline's content (steps 1-3 then hold trivially).
    serialize_element_when_faithful!(handlers, element, true);

    let content = handlers.walk_children_content(
        element.node,
        Context {
            kind: ContextKind::Inline,
            ..element.context
        },
    );
    let (content, leading_whitespace) = content.strip_leading_whitespace();
    let (content, trailing_whitespace) = content.strip_trailing_whitespace();
    // `~~` is a strikethrough delimiter, not an empty subscript, so pure mode
    // drops the element and keeps only the whitespace it held.
    if content.is_empty() {
        return Some(leading_whitespace.unwrap_or("").into());
    }
    // Wrapping both levels of a nested subscript would write `~~a~~`, which is
    // no subscript at all; pure mode keeps only the outermost level.
    if has_sub_ancestor(element.node) {
        return Some(
            concat_strings!(
                leading_whitespace.unwrap_or(""),
                content,
                trailing_whitespace.unwrap_or("")
            )
            .into(),
        );
    }
    Some(
        concat_strings!(
            leading_whitespace.unwrap_or(""),
            "~",
            content,
            "~",
            trailing_whitespace.unwrap_or("")
        )
        .into(),
    )
}

fn has_sub_ancestor(node: &Rc<Node>) -> bool {
    let mut current = get_parent_node(node);
    while let Some(parent) = current {
        if get_node_tag_name(&parent) == Some("sub") {
            return true;
        }
        current = get_parent_node(&parent);
    }
    false
}
