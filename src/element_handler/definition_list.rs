use std::rc::Rc;

use html5ever::ns;
use markup5ever_rcdom::{Node, NodeData};

use crate::{
    Context, Element,
    element_handler::{HandlerResult, Handlers, element_util::serialize_element_result},
    html_block::starts_html_block,
    options::TranslationMode,
    util::{
        node::{get_node_tag_name, get_parent_node, is_block_element},
        text::{
            TrimDocumentWhitespace, append_block, frame_as_block, indent_text_except_first_line,
        },
    },
};

/// Convert a description list to Markdown definition-list syntax.
pub(super) fn definition_list_handler(
    handlers: &dyn Handlers,
    element: &Element,
) -> Option<HandlerResult> {
    let faithful = handlers.options().translation_mode == TranslationMode::Faithful;
    let Some(loose) = list_style(element.node, faithful) else {
        return handlers.fallback(element);
    };
    if element.context.is_inline()
        || element.context.literal
        || (faithful && has_sibling_list(element.node))
    {
        return handlers.fallback(element);
    }

    let mut output = String::new();
    let mut previous_empty = false;
    let mut term_rendered = false;
    for child in element.node.children.borrow().iter() {
        let Some(tag) = get_node_tag_name(child) else {
            continue;
        };
        if tag == "dt" {
            term_rendered = false;
        }
        let Some(result) = handlers.handle(child, Context::BLOCK) else {
            continue;
        };
        if result.content.is_empty() {
            continue;
        }
        if tag == "dd" && !term_rendered {
            append_block(&mut output, &frame_as_block(&result.content));
            previous_empty = false;
            continue;
        }
        if faithful && !result.markdown_translated {
            return Some(serialize_element_result(handlers, element));
        }
        let content = result.content.trim_document_whitespace();
        if faithful && (tag == "dt" || !loose) && content.lines().any(|line| line.trim().is_empty())
        {
            return Some(serialize_element_result(handlers, element));
        }
        if tag == "dt" {
            if faithful && (content.is_empty() || starts_html_block(content)) {
                return Some(serialize_element_result(handlers, element));
            }
            if !output.is_empty() {
                output.push_str(if previous_empty { "\n" } else { "\n\n" });
            }
            output.push_str(content);
            term_rendered = true;
            previous_empty = false;
        } else {
            output.push_str(if loose && !previous_empty {
                "\n\n"
            } else {
                "\n"
            });
            output.push(':');
            if !content.is_empty() {
                output.push(' ');
                output.push_str(&indent_text_except_first_line(content, 2, false));
            }
            previous_empty = content.is_empty();
        }
    }
    Some(frame_as_block(&output).into())
}

/// Translate the contents of a term or definition.
pub(super) fn definition_item_handler(
    handlers: &dyn Handlers,
    element: &Element,
) -> Option<HandlerResult> {
    if element.context.is_inline()
        || element.context.literal
        || !get_parent_node(element.node)
            .is_some_and(|parent| get_node_tag_name(&parent) == Some("dl"))
    {
        return handlers.fallback(element);
    }
    let context = if element.tag == "dt" {
        Context::INLINE
    } else {
        Context::BLOCK
    };
    let result = handlers.walk_children(element.node, context);
    Some(HandlerResult {
        content: frame_as_block(&result.content),
        markdown_translated: result.markdown_translated,
    })
}

fn is_structural_whitespace(node: &Node) -> bool {
    matches!(&node.data, NodeData::Text { contents }
        if contents.borrow().chars().all(|c| matches!(c, ' ' | '\t' | '\n' | '\r' | '\u{c}')))
}

fn plain_element(node: &Node, faithful: bool) -> bool {
    matches!(&node.data, NodeData::Element { name, attrs, .. }
        if name.ns == ns!(html) && (!faithful || attrs.borrow().is_empty()))
}

fn inline_children(node: &Node) -> bool {
    node.children
        .borrow()
        .iter()
        .all(|child| match &child.data {
            NodeData::Text { .. } => true,
            NodeData::Element { name, .. } => !is_block_element(&name.local),
            _ => false,
        })
}

fn list_style(node: &Node, faithful: bool) -> Option<bool> {
    if !plain_element(node, faithful) || (faithful && has_trailing_line_whitespace(node)) {
        return None;
    }
    let mut needs_definition = false;
    let mut has_term = false;
    let mut loose = false;
    let mut tight = false;
    for child in node.children.borrow().iter() {
        if is_structural_whitespace(child) {
            continue;
        }
        if !plain_element(child, faithful) {
            return None;
        }
        match get_node_tag_name(child) {
            Some("dt") if !needs_definition && inline_children(child) => {
                if child.children.borrow().is_empty() {
                    return None;
                }
                has_term = true;
                needs_definition = true;
            }
            Some("dd") if has_term => {
                needs_definition = false;
                let children = child.children.borrow();
                let meaningful: Vec<_> = children
                    .iter()
                    .filter(|node| !is_structural_whitespace(node))
                    .collect();
                if meaningful.is_empty() {
                    if faithful && !children.is_empty() {
                        return None;
                    }
                    continue;
                }
                if meaningful.iter().all(|node| {
                    get_node_tag_name(node) == Some("p")
                        && plain_element(node, faithful)
                        && inline_children(node)
                        && node
                            .children
                            .borrow()
                            .iter()
                            .any(|child| !is_structural_whitespace(child))
                }) {
                    loose = true;
                } else if inline_children(child) {
                    tight = true;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }
    (has_term && !needs_definition && !(loose && tight)).then_some(loose)
}

// Intervening siblings may emit nothing, including when a separator is skipped.
fn has_sibling_list(node: &Rc<Node>) -> bool {
    get_parent_node(node).is_some_and(|parent| {
        parent
            .children
            .borrow()
            .iter()
            .any(|sibling| !Rc::ptr_eq(sibling, node) && get_node_tag_name(sibling) == Some("dl"))
    })
}

fn has_trailing_line_whitespace(node: &Node) -> bool {
    match &node.data {
        NodeData::Text { contents } => {
            contents.borrow().split_inclusive(['\r', '\n']).any(|line| {
                line.strip_suffix(['\r', '\n'])
                    .is_some_and(|line| line.ends_with([' ', '\t']))
            })
        }
        NodeData::Element { name, .. } if matches!(name.local.as_ref(), "code" | "pre") => false,
        _ => node
            .children
            .borrow()
            .iter()
            .any(|child| has_trailing_line_whitespace(child)),
    }
}

/// Preserve soft line breaks only in supported definition-list contents.
pub(crate) fn preserves_definition_whitespace(node: &Rc<Node>, mode: TranslationMode) -> bool {
    let mut parent = get_parent_node(node);
    while let Some(node) = parent {
        parent = get_parent_node(&node);
        match get_node_tag_name(&node) {
            Some("dt" | "dd") => {
                return parent.as_ref().is_some_and(|list| {
                    let faithful = mode == TranslationMode::Faithful;
                    get_node_tag_name(list) == Some("dl")
                        && list_style(list, faithful).is_some()
                        && !(faithful && has_sibling_list(list))
                });
            }
            Some("p")
                if parent
                    .as_ref()
                    .is_some_and(|node| get_node_tag_name(node) == Some("dd")) => {}
            Some(tag) if is_block_element(tag) => return false,
            _ => {}
        }
    }
    false
}
