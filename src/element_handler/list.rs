use markup5ever_rcdom::{Node, NodeData};
use std::rc::Rc;

use crate::{
    Element,
    dom_walker::is_block_element,
    element_handler::{HandlerResult, Handlers, serialize_element},
    node_util::{get_node_tag_name, get_parent_node},
    options::{Options, TranslationMode},
    serialize_if_faithful,
    text_util::{append_block, concat_strings, indent_text_except_first_line},
};

pub(super) fn list_handler(handlers: &dyn Handlers, element: Element) -> Option<HandlerResult> {
    // In faithful mode, ...
    if handlers.options().translation_mode == TranslationMode::Faithful {
        // ...make sure this element's attributes can be translated as markdown.
        let has_start = element
            .attrs
            .first()
            .is_some_and(|attr| &attr.name.local == "start");
        serialize_if_faithful!(handlers, element, if has_start { 1 } else { 0 });

        // ...all children must be translated as Markdown, and all children must
        // be li elements.
        if !element.markdown_translated
            || !element.node.children.borrow().iter().all(|node| {
                let tag_name = get_node_tag_name(node);
                // In addition to elements, there will be text nodes, generally
                // with whitespace; these should be ignored.
                tag_name == Some("li") || tag_name.is_none()
            })
        {
            return Some(HandlerResult {
                content: serialize_element(handlers, &element),
                markdown_translated: false,
            });
        }

        // ...and a loose list needs somewhere to put the blank line that spells
        // it. Where there is nowhere, the list survives only as HTML: the tight
        // list is all the syntax has, and it means something else.
        if !is_tight_list(element.node) && !loose_list_is_writable(element.node) {
            return Some(HandlerResult {
                content: serialize_element(handlers, &element),
                markdown_translated: false,
            });
        }
    }
    let parent = get_parent_node(element.node);
    let is_parent_li = parent
        .map(|p| get_node_tag_name(&p).is_some_and(|tag| tag == "li"))
        .unwrap_or(false);

    let result = if element.tag == "ol" {
        let (content, translated) = get_ol_content(handlers, &element);
        HandlerResult {
            content,
            markdown_translated: translated,
        }
    } else {
        handlers.walk_children(element.node)
    };

    if handlers.options().translation_mode == TranslationMode::Faithful
        && !result.markdown_translated
    {
        return Some(HandlerResult {
            content: serialize_element(handlers, &element),
            markdown_translated: false,
        });
    }

    let trimmed = result.content.trim_matches(|ch| ch == '\n');
    if trimmed.is_empty() {
        return None;
    }

    if is_parent_li {
        Some(concat_strings!("\n", trimmed, "\n").into())
    } else {
        Some(concat_strings!("\n\n", trimmed, "\n\n").into())
    }
}

/// Whether blocks written directly inside `node` have to land on consecutive
/// lines, with no blank line between them: `node` is a tight list, or an item of
/// one.
///
/// A blank line between two items, or between two blocks of one item, is exactly
/// what spells a *loose* list, so a tight one can hold neither. This is what the
/// document walker asks before joining two blocks.
pub(crate) fn holds_tight_blocks(node: &Rc<Node>) -> bool {
    match get_node_tag_name(node) {
        Some("ul" | "ol") => is_tight_list(node),
        Some("li") => enclosing_list(node).is_some_and(|list| is_tight_list(&list)),
        _ => false,
    }
}

/// Whether this `<ul>` or `<ol>` is *tight*, which CommonMark renders with each
/// item's text bare rather than wrapped in a `<p>`.
///
/// The choice belongs to the list rather than to any one item — one blank line
/// anywhere in a list makes the whole list loose — so a single `<p>` under any
/// item settles it for all of them. An item with no paragraph of its own, one
/// holding only a nested block, renders the same either way and so says nothing.
fn is_tight_list(list: &Rc<Node>) -> bool {
    !items(list).any(|item| {
        item.children
            .borrow()
            .iter()
            .any(|child| get_node_tag_name(child) == Some("p"))
    })
}

/// Whether a loose list can be spelled at all. The blank line that makes it
/// loose needs somewhere to go: between two items, or between two blocks of one
/// item. A single item holding a single block offers neither, leaving the tight
/// list — which means something else — as the only Markdown there is.
fn loose_list_is_writable(list: &Rc<Node>) -> bool {
    let mut items = items(list);
    let Some(first) = items.next() else {
        return false;
    };
    items.next().is_some() || block_child_count(&first) > 1
}

/// The `<li>` children of a list, ignoring the text nodes of whitespace between
/// them.
fn items(list: &Rc<Node>) -> impl Iterator<Item = Rc<Node>> {
    let children = list.children.borrow().clone();
    children
        .into_iter()
        .filter(|child| get_node_tag_name(child) == Some("li"))
}

/// How many blocks this list item holds, a blank line's worth of room lying
/// between each pair of them.
fn block_child_count(item: &Rc<Node>) -> usize {
    item.children
        .borrow()
        .iter()
        .filter(|child| get_node_tag_name(child).is_some_and(is_block_element))
        .count()
}

/// The `<ul>` or `<ol>` this item belongs to, if it is in one at all.
fn enclosing_list(item: &Rc<Node>) -> Option<Rc<Node>> {
    get_parent_node(item).filter(|list| matches!(get_node_tag_name(list), Some("ul" | "ol")))
}

struct ListChildContent {
    text: String,
    is_li: bool,
}

fn get_ol_content(handlers: &dyn Handlers, element: &Element) -> (String, bool) {
    let mut buffer: Vec<ListChildContent> = Vec::new();
    let mut li_count = 0;
    let mut all_translated = true;

    let start_idx = element
        .attrs
        .iter()
        .find(|attr| &attr.name.local == "start")
        .map(|attr| attr.value.to_string().parse::<i32>().unwrap_or(1).max(1) as usize)
        .unwrap_or(1);

    for child in element.node.children.borrow().iter() {
        let Some(res) = handlers.handle(child) else {
            continue;
        };
        if !res.markdown_translated {
            all_translated = false;
        }

        if let NodeData::Element { ref name, .. } = child.data
            && &name.local == "li"
        {
            buffer.push(ListChildContent {
                text: res.content,
                is_li: true,
            });
            li_count += 1;
        } else {
            buffer.push(ListChildContent {
                text: res.content,
                is_li: false,
            });
        }
    }

    // `start_idx` is one-based, not zero-based
    let highest_index = start_idx + li_count - 1;

    let mut curr_li_idx = start_idx - 1;

    // A blank line between two items is what makes a list loose, so a tight one
    // joins them on consecutive lines. The `<ul>` case is settled in the
    // document walker, which is what joins those items.
    let max_boundary_newlines = if is_tight_list(element.node) { 1 } else { 2 };

    let capacity = buffer.iter().map(|content| content.text.len()).sum();
    let mut contents = String::with_capacity(capacity);
    for content in buffer {
        let rendered = if content.is_li {
            curr_li_idx += 1;
            add_ol_li_marker(
                handlers.options(),
                &content.text,
                curr_li_idx,
                highest_index,
            )
        } else {
            content.text
        };
        append_block(&mut contents, &rendered, max_boundary_newlines);
    }

    (contents, all_translated)
}

fn digits(num: usize) -> usize {
    if num == 0 {
        1
    } else {
        num.ilog10() as usize + 1
    }
}

fn add_ol_li_marker(
    options: &Options,
    content: &str,
    index: usize,
    highest_index: usize,
) -> String {
    let index_str = index.to_string();
    let spacing =
        " ".repeat(options.ol_number_spacing as usize + digits(highest_index) - index_str.len());
    let content = content.trim_start_matches('\n');
    // As in the `<ul>` case, the trim this asks for spares a hard line break.
    let content = indent_text_except_first_line(content, index_str.len() + 1 + spacing.len(), true);
    concat_strings!("\n", index_str, ".", spacing, content)
}

#[cfg(test)]
mod tests {
    use crate::element_handler::list::digits;

    #[test]
    fn test_count_digits() {
        assert_eq!(1, digits(1));
        assert_eq!(1, digits(0));
        assert_eq!(2, digits(45));
        assert_eq!(3, digits(450));
    }
}
