use std::rc::Rc;

use markup5ever_rcdom::{Node, NodeData};

use crate::{
    Context, ContextKind, Element,
    element_handler::{HandlerResult, Handlers, element_util::serialize_element},
    options::TranslationMode,
    util::{
        node::{get_node_tag_name, get_parent_node, is_block_element},
        text::{StripWhitespace, concat_strings},
        unicode::is_unicode_punctuation,
    },
};

/// Translates `<sub>` to the single-tilde subscript of pulldown-cmark's
/// `Options::ENABLE_SUBSCRIPT`. See the "Subscript" section of
/// `translation_approach.md`.
pub(super) fn sub_handler(handlers: &dyn Handlers, element: &Element) -> Option<HandlerResult> {
    let context = Context {
        kind: ContextKind::Inline,
        ..element.context
    };
    if handlers.options().translation_mode == TranslationMode::Faithful {
        return Some(faithful_sub(handlers, element, context));
    }
    // Wrapping both levels of a nested subscript would write `~~a~~`, which is
    // no subscript at all; pure mode keeps only the outermost level.
    if has_sub_ancestor(element.node) {
        return Some(handlers.walk_children_content(element.node, context).into());
    }
    // Delimiters of adjacent subscripts would join into a `~~` run, so the
    // first subscript of a run writes the content of all of them and the rest
    // write nothing. `dom_walker::can_combine` leaves `<sub>` to this handler,
    // since it combines only elements holding a single text node.
    let Some(run) = adjacent_sub_run(element.node) else {
        return Some("".into());
    };
    let content: String = run
        .iter()
        .map(|node| handlers.walk_children_content(node, context))
        .collect();
    let (content, leading_whitespace) = content.strip_leading_whitespace();
    let (content, trailing_whitespace) = content.strip_trailing_whitespace();
    // `~~` is a strikethrough delimiter, not an empty subscript, so pure mode
    // drops the element and keeps only the whitespace it held.
    if content.is_empty() {
        return Some(leading_whitespace.unwrap_or("").into());
    }
    Some(surround_with_tildes(
        leading_whitespace,
        content,
        trailing_whitespace,
    ))
}

/// Faithful mode's Approach steps 1-5: `~content~` where both delimiters are
/// sure to flank, the element as HTML otherwise.
fn faithful_sub(handlers: &dyn Handlers, element: &Element, context: Context) -> HandlerResult {
    let html = || HandlerResult::html(serialize_element(handlers, element));
    if !element.attrs.is_empty()
        || has_comment_child(element.node)
        || has_sub_ancestor(element.node)
    {
        return html();
    }
    let content = handlers.walk_children_content(element.node, context);
    let (content, leading_whitespace) = content.strip_leading_whitespace();
    let (content, trailing_whitespace) = content.strip_trailing_whitespace();
    if content.is_empty()
        || content.starts_with(is_unicode_punctuation)
        || content.ends_with(is_unicode_punctuation)
    {
        return html();
    }
    let opens = leading_whitespace.is_some() || side_allows_delimiter(element.node, Side::Before);
    let closes = trailing_whitespace.is_some() || side_allows_delimiter(element.node, Side::After);
    if !(opens && closes) {
        return html();
    }
    surround_with_tildes(leading_whitespace, content, trailing_whitespace)
}

fn surround_with_tildes(
    leading_whitespace: Option<&str>,
    content: &str,
    trailing_whitespace: Option<&str>,
) -> HandlerResult {
    concat_strings!(
        leading_whitespace.unwrap_or(""),
        "~",
        content,
        "~",
        trailing_whitespace.unwrap_or("")
    )
    .into()
}

#[derive(Clone, Copy)]
enum Side {
    Before,
    After,
}

/// Approach step 4: whether the delimiter on `side` of `node` is sure to flank,
/// judged from the DOM since a handler cannot see the emitted neighbors.
/// Escaping writes whitespace and punctuation as whitespace or punctuation, but
/// `\~` would put a `~` next to the delimiter, which pulldown-cmark treats
/// specially.
fn side_allows_delimiter(node: &Rc<Node>, side: Side) -> bool {
    let Some(parent) = get_parent_node(node) else {
        return false;
    };
    let siblings = parent.children.borrow();
    let Some(index) = siblings
        .iter()
        .position(|sibling| Rc::ptr_eq(sibling, node))
    else {
        return false;
    };
    let sibling = match side {
        Side::Before => index.checked_sub(1).map(|before| &siblings[before]),
        Side::After => siblings.get(index + 1),
    };
    let Some(sibling) = sibling else {
        // The delimiter starts or ends the inline content of a block.
        return get_node_tag_name(&parent).is_some_and(is_block_element);
    };
    let NodeData::Text { contents } = &sibling.data else {
        return false;
    };
    let contents = contents.borrow();
    let nearest = match side {
        Side::Before => contents.chars().next_back(),
        Side::After => contents.chars().next(),
    };
    nearest.is_some_and(|ch| ch != '~' && (ch.is_whitespace() || is_unicode_punctuation(ch)))
}

fn has_comment_child(node: &Rc<Node>) -> bool {
    node.children
        .borrow()
        .iter()
        .any(|child| matches!(child.data, NodeData::Comment { .. }))
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

/// The run of adjacent `<sub>` siblings which `node` begins, or `None` when an
/// earlier `<sub>` sibling begins it. Pure mode drops comments, so a comment
/// between two subscripts does not separate them.
fn adjacent_sub_run(node: &Rc<Node>) -> Option<Vec<Rc<Node>>> {
    let Some(parent) = get_parent_node(node) else {
        return Some(vec![node.clone()]);
    };
    let siblings = parent.children.borrow();
    let Some(index) = siblings
        .iter()
        .position(|sibling| Rc::ptr_eq(sibling, node))
    else {
        return Some(vec![node.clone()]);
    };
    let is_sub = |sibling: &&Rc<Node>| get_node_tag_name(sibling) == Some("sub");
    let is_comment = |sibling: &&Rc<Node>| matches!(sibling.data, NodeData::Comment { .. });
    if siblings[..index]
        .iter()
        .rev()
        .find(|sibling| !is_comment(sibling))
        .is_some_and(|sibling| is_sub(&sibling))
    {
        return None;
    }
    Some(
        siblings[index..]
            .iter()
            .filter(|sibling| !is_comment(sibling))
            .take_while(is_sub)
            .cloned()
            .collect(),
    )
}
