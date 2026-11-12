use std::rc::Rc;

use markup5ever_rcdom::Node;

use crate::{
    Context, ContextKind, Element,
    element_handler::{HandlerResult, Handlers, element_util::serialize_element_result},
    options::TranslationMode,
    util::{
        node::{get_node_tag_name, get_parent_node, is_block_element},
        text::concat_strings,
        unicode::is_unicode_punctuation,
    },
};

/// Translates strikethrough using the fallbacks in `translation_approach.md`.
pub(super) fn strikethrough_handler(
    handlers: &dyn Handlers,
    element: &Element,
) -> Option<HandlerResult> {
    let faithful = handlers.options().translation_mode == TranslationMode::Faithful;
    let has_blocks = has_descendant(element.node, is_block_element);
    let has_nested_strike = has_descendant(element.node, is_strikethrough);
    if faithful
        && (element.tag != "del"
            || !element.attrs.is_empty()
            || has_blocks
            || has_nested_strike
            || has_adjacent_del(element.node))
    {
        return Some(serialize_element_result(handlers, element));
    }

    let content = handlers.walk_children_content(
        element.node,
        if has_blocks {
            element.context
        } else {
            Context {
                kind: ContextKind::Inline,
                ..element.context
            }
        },
    );
    if has_blocks || has_nested_strike {
        return Some(content.into());
    }

    // See the edge and punctuation rules in the Strikethrough specification.
    let trimmed = content.trim();
    if faithful
        && (trimmed.is_empty()
            || trimmed.len() != content.len()
            || content.ends_with(is_unicode_punctuation))
    {
        return Some(serialize_element_result(handlers, element));
    }
    if trimmed.is_empty() || trimmed.ends_with(is_unicode_punctuation) {
        return Some(content.into());
    }

    let leading_len = content.len() - content.trim_start().len();
    let trailing_start = content.trim_end().len();
    Some(
        concat_strings!(
            &content[..leading_len],
            "~~",
            trimmed,
            "~~",
            &content[trailing_start..]
        )
        .into(),
    )
}

fn is_strikethrough(tag: &str) -> bool {
    matches!(tag, "del" | "s" | "strike")
}

fn has_descendant(node: &Rc<Node>, matches_tag: fn(&str) -> bool) -> bool {
    node.children.borrow().iter().any(|child| {
        get_node_tag_name(child).is_some_and(matches_tag) || has_descendant(child, matches_tag)
    })
}

fn has_adjacent_del(node: &Rc<Node>) -> bool {
    let Some(parent) = get_parent_node(node) else {
        return false;
    };
    let children = parent.children.borrow();
    let Some(index) = children.iter().position(|child| Rc::ptr_eq(child, node)) else {
        return false;
    };
    index
        .checked_sub(1)
        .and_then(|previous| children.get(previous))
        .into_iter()
        .chain(children.get(index + 1))
        .any(|sibling| get_node_tag_name(sibling) == Some("del"))
}
