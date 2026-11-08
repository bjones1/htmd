use std::{cell::RefCell, collections::HashMap, rc::Rc};

use html5ever::Attribute;
use markup5ever_rcdom::{Node, NodeData};

use crate::{
    Context, Element,
    element_handler::{
        HandlerResult, Handlers,
        element_util::{serialize_element, serialize_inline_subtree},
    },
    options::{CodeBlockStyle, Options, TranslationMode},
    util::{
        node::{get_node_tag_name, get_parent_node},
        text::{frame_as_block, indent_text_except_first_line},
    },
};

#[derive(Clone)]
enum Role {
    Reference(String),
    Definition {
        label: String,
        label_node: Option<usize>,
    },
}

/// Faithful footnote elements to translate during one document walk.
#[derive(Default)]
pub(crate) struct FootnotePlan {
    roles: HashMap<usize, Role>,
}

struct ReferenceCandidate {
    node_key: usize,
    number: Option<String>,
}

#[derive(Default)]
struct DefinitionInfo {
    label_node: Option<usize>,
    number: Option<String>,
}

thread_local! {
    static PLANS: RefCell<Vec<FootnotePlan>> = const { RefCell::new(Vec::new()) };
}

fn node_key(node: &Rc<Node>) -> usize {
    Rc::as_ptr(node) as usize
}

fn attributes(node: &Node) -> Option<std::cell::Ref<'_, Vec<Attribute>>> {
    match &node.data {
        NodeData::Element { attrs, .. } => Some(attrs.borrow()),
        _ => None,
    }
}

fn attribute<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|attr| attr.name.local.as_ref() == name)
        .map(|attr| attr.value.as_ref())
}

fn class_token(attrs: &[Attribute], token: &str) -> bool {
    attribute(attrs, "class")
        .is_some_and(|class| class.split_ascii_whitespace().any(|part| part == token))
}

fn exact_attrs(attrs: &[Attribute], expected: &[&str]) -> bool {
    attrs.len() == expected.len()
        && expected
            .iter()
            .all(|name| attrs.iter().any(|attr| attr.name.local.as_ref() == *name))
}

fn exact_class(attrs: &[Attribute], class: &str, expected: &[&str]) -> bool {
    exact_attrs(attrs, expected) && attribute(attrs, "class") == Some(class)
}

fn valid_id(id: &str) -> bool {
    (1..=999).contains(&id.len())
        && !id
            .chars()
            .any(|ch| ch.is_whitespace() || ch.is_control() || matches!(ch, '[' | ']' | '\\' | '|'))
}

fn text(node: &Node) -> Option<String> {
    let children = node.children.borrow();
    if children.len() != 1 {
        return None;
    }
    match &children[0].data {
        NodeData::Text { contents } => Some(contents.borrow().to_string()),
        _ => None,
    }
}

fn decimal(node: &Node) -> Option<String> {
    text(node).filter(|value| {
        !value.is_empty()
            && value.bytes().all(|byte| byte.is_ascii_digit())
            && value.bytes().any(|byte| byte != b'0')
    })
}

fn descendants(node: &Rc<Node>, nodes: &mut Vec<Rc<Node>>) {
    nodes.push(node.clone());
    for child in node.children.borrow().iter() {
        descendants(child, nodes);
    }
}

fn only_element_child(parent: &Rc<Node>, allow_blank_text: bool) -> Option<Rc<Node>> {
    let mut element = None;
    for child in parent.children.borrow().iter() {
        match &child.data {
            NodeData::Element { .. } if element.is_none() => element = Some(child.clone()),
            NodeData::Text { contents }
                if allow_blank_text && contents.borrow().trim().is_empty() => {}
            _ => return None,
        }
    }
    element
}

fn reference_anchor(sup: &Rc<Node>, pure: bool) -> Option<Rc<Node>> {
    let mut current = sup.clone();
    loop {
        let child = only_element_child(&current, pure)?;
        if get_node_tag_name(&child) == Some("a") {
            return Some(child);
        }
        if !pure {
            return None;
        }
        current = child;
    }
}

// Include malformed references so Faithful mode can reject their note.
fn reference_ids(sup: &Rc<Node>) -> Vec<String> {
    let mut nodes = Vec::new();
    descendants(sup, &mut nodes);
    nodes
        .into_iter()
        .filter(|node| get_node_tag_name(node) == Some("a"))
        .filter_map(|node| {
            let attrs = attributes(&node)?;
            let id = attribute(&attrs, "href")?.strip_prefix('#')?;
            valid_id(id).then(|| id.to_owned())
        })
        .collect()
}

fn reference_data(sup: &Rc<Node>, pure: bool) -> Option<(String, String)> {
    let sup_attrs = attributes(sup)?;
    if !pure && !exact_class(&sup_attrs, "footnote-reference", &["class"]) {
        return None;
    }
    let anchor = reference_anchor(sup, pure)?;
    let anchor_attrs = attributes(&anchor)?;
    if !pure && !exact_attrs(&anchor_attrs, &["href"]) {
        return None;
    }
    let id = attribute(&anchor_attrs, "href")?.strip_prefix('#')?;
    let number = decimal(&anchor)?;
    valid_id(id).then(|| (id.to_owned(), number))
}

fn definition_info(definition: &Rc<Node>, pure: bool) -> Option<DefinitionInfo> {
    let attrs = attributes(definition)?;
    if !pure && !exact_class(&attrs, "footnote-definition", &["class", "id"]) {
        return None;
    }
    let first = definition
        .children
        .borrow()
        .iter()
        .find(|child| match &child.data {
            NodeData::Text { contents } => !contents.borrow().trim().is_empty(),
            _ => true,
        })
        .cloned();
    let Some(mut current) = first else {
        return pure.then_some(DefinitionInfo::default());
    };
    let label_node = node_key(&current);
    loop {
        if get_node_tag_name(&current) == Some("sup")
            && let Some(label_attrs) = attributes(&current)
            && class_token(&label_attrs, "footnote-definition-label")
        {
            if !pure && !exact_class(&label_attrs, "footnote-definition-label", &["class"]) {
                return None;
            }
            return Some(DefinitionInfo {
                label_node: Some(label_node),
                number: Some(decimal(&current)?),
            });
        }
        if !pure || get_node_tag_name(&current).is_none() {
            break;
        }
        let Some(child) = only_element_child(&current, true) else {
            break;
        };
        current = child;
    }
    pure.then_some(DefinitionInfo::default())
}

fn pure_role(element: &Element, options: &Options) -> Option<Role> {
    match element.tag {
        "sup" if class_token(element.attrs, "footnote-reference") => {
            reference_data(element.node, true).map(|(id, _)| Role::Reference(id))
        }
        "div"
            if element.context.is_block() && class_token(element.attrs, "footnote-definition") =>
        {
            let id = attribute(element.attrs, "id")?;
            if !valid_id(id) || following_indented_code(element.node, options) {
                return None;
            }
            let info = definition_info(element.node, true)?;
            Some(Role::Definition {
                label: id.to_owned(),
                label_node: info.label_node,
            })
        }
        _ => None,
    }
}

fn has_nested_definition(definition: &Rc<Node>) -> bool {
    definition.children.borrow().iter().any(|child| {
        (get_node_tag_name(child) == Some("div")
            && attributes(child).is_some_and(|attrs| class_token(&attrs, "footnote-definition")))
            || has_nested_definition(child)
    })
}

fn has_empty_translated_block(container: &Rc<Node>) -> bool {
    container.children.borrow().iter().any(|child| {
        let tag = get_node_tag_name(child);
        if matches!(tag, Some("p" | "blockquote"))
            && child.children.borrow().iter().all(|content| {
                matches!(&content.data, NodeData::Text { contents } if contents.borrow().trim().is_empty())
            })
        {
            return true;
        }
        matches!(tag, Some("blockquote" | "ul" | "ol" | "li"))
            && has_empty_translated_block(child)
    })
}

// A following indented code block parses as footnote content.
fn following_indented_code(definition: &Rc<Node>, options: &Options) -> bool {
    if options.code_block_style != CodeBlockStyle::Indented {
        return false;
    }
    let Some(parent) = get_parent_node(definition) else {
        return false;
    };
    let siblings = parent.children.borrow();
    siblings
        .iter()
        .skip_while(|sibling| !Rc::ptr_eq(sibling, definition))
        .skip(1)
        .find(|sibling| match &sibling.data {
            NodeData::Text { contents } => !contents.borrow().trim().is_empty(),
            _ => true,
        })
        .is_some_and(|sibling| get_node_tag_name(sibling) == Some("pre"))
}

impl FootnotePlan {
    /// Identify footnotes by their literal ID before walking the document.
    pub(crate) fn build(tree: &Rc<Node>, options: &Options) -> Self {
        let mut nodes = Vec::new();
        descendants(tree, &mut nodes);
        let mut definitions = HashMap::<String, Vec<Rc<Node>>>::new();
        let mut references = HashMap::<String, Vec<ReferenceCandidate>>::new();

        for node in &nodes {
            let Some(attrs) = attributes(node) else {
                continue;
            };
            match get_node_tag_name(node) {
                Some("div") if class_token(&attrs, "footnote-definition") => {
                    if let Some(id) = attribute(&attrs, "id").filter(|id| valid_id(id)) {
                        definitions
                            .entry(id.to_owned())
                            .or_default()
                            .push(node.clone());
                    }
                }
                Some("sup") if class_token(&attrs, "footnote-reference") => {
                    let parsed = reference_data(node, false);
                    for id in reference_ids(node) {
                        let number = parsed.as_ref().and_then(|(matched_id, number)| {
                            (matched_id == &id).then(|| number.clone())
                        });
                        references.entry(id).or_default().push(ReferenceCandidate {
                            node_key: node_key(node),
                            number,
                        });
                    }
                }
                _ => {}
            }
        }

        let mut plan = Self::default();
        for (id, definitions) in definitions {
            let Some(references) = references.get(&id).filter(|refs| !refs.is_empty()) else {
                continue;
            };
            plan.add_if_valid(id, &definitions, references, options);
        }
        plan
    }

    fn add_if_valid(
        &mut self,
        id: String,
        definitions: &[Rc<Node>],
        references: &[ReferenceCandidate],
        options: &Options,
    ) {
        let [definition] = definitions else {
            return;
        };
        if has_nested_definition(definition)
            || has_empty_translated_block(definition)
            || following_indented_code(definition, options)
        {
            return;
        }
        let Some(info) = definition_info(definition, false) else {
            return;
        };
        let Some(number) = info.number else {
            return;
        };
        if references
            .iter()
            .any(|reference| reference.number.as_deref() != Some(number.as_str()))
        {
            return;
        }
        self.roles.insert(
            node_key(definition),
            Role::Definition {
                label: id.clone(),
                label_node: info.label_node,
            },
        );
        for reference in references {
            self.roles
                .insert(reference.node_key, Role::Reference(id.clone()));
        }
    }
}

/// Make a Faithful conversion plan available during one document walk.
/// The stack restores an enclosing conversion's plan after a nested conversion.
pub(crate) fn with_plan<R>(plan: FootnotePlan, convert: impl FnOnce() -> R) -> R {
    struct PlanGuard;
    impl Drop for PlanGuard {
        fn drop(&mut self) {
            PLANS.with(|plans| {
                plans.borrow_mut().pop();
            });
        }
    }
    PLANS.with(|plans| plans.borrow_mut().push(plan));
    let _guard = PlanGuard;
    convert()
}

/// Escape a colon that would turn an emitted footnote reference into a definition.
pub(crate) fn escape_colon_after_reference(output: &mut String, following: &str) {
    if !following.starts_with(':') || !output.ends_with(']') {
        return;
    }
    let closing_bracket = output.len() - 1;
    let suffix = &output.as_bytes()[closing_bracket.saturating_sub(1001)..closing_bracket];
    let Some(opening_bracket) = suffix.windows(2).rposition(|pair| pair == b"[^") else {
        return;
    };
    let Some(label) = std::str::from_utf8(&suffix[opening_bracket + 2..]).ok() else {
        return;
    };
    if valid_id(label) {
        output.push('\\');
    }
}

fn render_reference(label: &str) -> HandlerResult {
    format!("[^{label}]").into()
}

struct BorrowedChildrenGuard(Rc<Node>);

impl Drop for BorrowedChildrenGuard {
    fn drop(&mut self) {
        // rcdom's Node::drop drains descendants even when another tree owns them.
        self.0.children.borrow_mut().clear();
    }
}

fn render_definition(
    handlers: &dyn Handlers,
    node: &Rc<Node>,
    label: &str,
    label_node: Option<usize>,
) -> HandlerResult {
    let body = Node::new(NodeData::Document);
    let _guard = BorrowedChildrenGuard(body.clone());
    body.children.borrow_mut().extend(
        node.children
            .borrow()
            .iter()
            .filter(|child| label_node != Some(node_key(child)))
            .cloned(),
    );
    let content = handlers.walk_children_content(&body, Context::BLOCK);
    let content = content.trim_matches('\n').trim_end();
    let first_is_paragraph = body
        .children
        .borrow()
        .iter()
        .find(|child| get_node_tag_name(child).is_some())
        .is_some_and(|child| get_node_tag_name(child) == Some("p"));
    let mut definition = format!("[^{label}]:");
    if !content.is_empty() {
        let indented = indent_text_except_first_line(content, 4, false);
        if first_is_paragraph {
            definition.push(' ');
        } else {
            definition.push_str("\n    ");
        }
        definition.push_str(&indented);
    }
    frame_as_block(&definition).into()
}

pub(super) fn footnote_handler(
    handlers: &dyn Handlers,
    element: &Element,
) -> Option<HandlerResult> {
    let role = if handlers.options().translation_mode == TranslationMode::Pure {
        pure_role(element, handlers.options())
    } else {
        PLANS.with(|plans| {
            plans
                .borrow()
                .last()
                .and_then(|plan| plan.roles.get(&node_key(element.node)).cloned())
        })
    };
    match role {
        Some(Role::Reference(label)) => Some(render_reference(&label)),
        Some(Role::Definition { label, label_node }) if element.context.is_block() => Some(
            render_definition(handlers, element.node, &label, label_node),
        ),
        _ if element.tag == "sup"
            && handlers.options().translation_mode == TranslationMode::Faithful
            && class_token(element.attrs, "footnote-reference") =>
        {
            Some(HandlerResult::html(
                serialize_inline_subtree(handlers, element)
                    .unwrap_or_else(|_| serialize_element(handlers, element)),
            ))
        }
        _ => handlers.fallback(element),
    }
}
