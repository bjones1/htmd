//! Compares two HTML strings as the "Whitespace" section of
//! `testing_approach.md` defines: both are parsed with html5ever, whitespace
//! which a browser does not render is normalized away, and the two trees must
//! then be identical.
#![allow(dead_code)]

use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use html5ever::{ParseOpts, ns, parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

/// Panic with a diff of the normalized trees unless `actual` and `expected`
/// are equivalent.
pub fn assert_html_equivalent(actual: &str, expected: &str) {
    pretty_assertions::assert_eq!(
        normalize_html(actual),
        normalize_html(expected),
        "HTML differs after whitespace normalization\n  actual:   {actual:?}\n  expected: {expected:?}"
    );
}

/// Whether `a` and `b` are equivalent.
pub fn html_equivalent(a: &str, b: &str) -> bool {
    normalize_html(a) == normalize_html(b)
}

/// The normalized tree of `html`, one node per line, indented by depth. Two
/// strings are equivalent exactly when their normalized trees are equal.
pub fn normalize_html(html: &str) -> String {
    let dom = parse_document(RcDom::default(), ParseOpts::default()).one(html);
    let mut nodes = convert_children(&dom.document);
    remove_whitespace_between_blocks(&mut nodes);
    let mut line = LineState::new();
    collapse_inline_whitespace(&nodes, &mut line);
    line.end();
    let mut out = String::new();
    write_nodes(&nodes, 0, &mut out);
    out
}

/// An owned copy of the DOM. Text sits behind `Rc<RefCell>` so that the
/// whitespace pass can drop a trailing space from a text node it has already
/// left, once it reaches the end of the line.
enum Node {
    Element {
        name: String,
        attrs: BTreeMap<String, String>,
        children: Vec<Node>,
    },
    Text(Rc<RefCell<String>>),
    Comment(String),
    Doctype(String),
    ProcessingInstruction(String),
}

fn convert_children(handle: &Handle) -> Vec<Node> {
    handle.children.borrow().iter().map(convert).collect()
}

fn convert(handle: &Handle) -> Node {
    match &handle.data {
        NodeData::Document => unreachable!("the document is never a child"),
        NodeData::Doctype { name, .. } => Node::Doctype(name.to_string()),
        NodeData::Text { contents } => {
            Node::Text(Rc::new(RefCell::new(contents.borrow().to_string())))
        }
        NodeData::Comment { contents } => Node::Comment(contents.to_string()),
        NodeData::ProcessingInstruction { target, contents } => {
            Node::ProcessingInstruction(format!("{target} {contents}"))
        }
        NodeData::Element {
            name,
            attrs,
            template_contents,
            ..
        } => {
            let element_name = if name.ns == ns!(html) {
                name.local.to_string()
            } else {
                format!("{{{}}}{}", name.ns, name.local)
            };
            let attrs = attrs
                .borrow()
                .iter()
                .map(|attr| {
                    let key = if attr.name.ns.is_empty() {
                        attr.name.local.to_string()
                    } else {
                        format!("{{{}}}{}", attr.name.ns, attr.name.local)
                    };
                    (key, attr.value.to_string())
                })
                .collect();
            let children = match &*template_contents.borrow() {
                Some(contents) => convert_children(contents),
                None => convert_children(handle),
            };
            Node::Element {
                name: element_name,
                attrs,
                children,
            }
        }
    }
}

/// How an element takes part in whitespace processing under the default
/// style sheet ([HTML § 15](https://html.spec.whatwg.org/multipage/rendering.html)).
#[derive(PartialEq)]
enum Kind {
    /// `white-space: pre`, displayed as a block.
    Preformatted,
    /// `<textarea>`: `white-space: pre-wrap`, displayed inline as one box.
    Textarea,
    /// A box of its own in the inline content, like a single character.
    AtomicInline,
    LineBreak,
    /// Displayed as a block, a list item or a table part: it ends the line
    /// before it and starts a new one after it.
    Block,
    /// `display: none`; its content is compared exactly, except that
    /// whitespace-only text in `<head>` is removed.
    Hidden,
    Inline,
}

fn kind(name: &str) -> Kind {
    match name {
        "pre" | "listing" | "xmp" | "plaintext" => Kind::Preformatted,
        "textarea" => Kind::Textarea,
        "img" | "input" | "select" | "button" | "iframe" | "video" | "audio" | "canvas"
        | "embed" | "object" | "meter" | "progress" => Kind::AtomicInline,
        "br" => Kind::LineBreak,
        "head" | "script" | "style" | "template" | "title" | "meta" | "link" | "base" | "area"
        | "datalist" | "param" | "rp" => Kind::Hidden,
        _ if is_block(name) || is_table_part(name) => Kind::Block,
        _ => Kind::Inline,
    }
}

fn is_block(name: &str) -> bool {
    matches!(
        name,
        "html"
            | "body"
            | "address"
            | "article"
            | "aside"
            | "blockquote"
            | "center"
            | "dd"
            | "details"
            | "dialog"
            | "dir"
            | "div"
            | "dl"
            | "dt"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hgroup"
            | "hr"
            | "legend"
            | "li"
            | "listing"
            | "main"
            | "menu"
            | "nav"
            | "ol"
            | "optgroup"
            | "option"
            | "p"
            | "plaintext"
            | "pre"
            | "search"
            | "section"
            | "summary"
            | "ul"
            | "xmp"
    )
}

fn is_table_part(name: &str) -> bool {
    matches!(
        name,
        "table" | "caption" | "colgroup" | "col" | "thead" | "tbody" | "tfoot" | "tr" | "td" | "th"
    )
}

/// The characters which CSS collapses
/// ([CSS Text § 4.1](https://www.w3.org/TR/css-text-3/#white-space-rules)). A
/// form feed is not one of them.
fn is_collapsible(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r')
}

/// Rule 3: a text node of ASCII whitespace (form feed included) between two
/// blocks or table parts, or between one of them and the edge of its parent,
/// is removed. Comments are skipped when deciding what the neighbors are, so a
/// run of such text nodes and comments is judged by what surrounds the run.
fn remove_whitespace_between_blocks(nodes: &mut Vec<Node>) {
    #[derive(Clone, Copy, PartialEq)]
    enum Side {
        Edge,
        Block,
        Other,
    }
    let side = |node: Option<&Node>| match node {
        None => Side::Edge,
        Some(Node::Element { name, .. }) if is_block(name) || is_table_part(name) => Side::Block,
        Some(_) => Side::Other,
    };
    let is_whitespace_text = |node: &Node| matches!(node, Node::Text(text) if text.borrow().chars().all(|c| c.is_ascii_whitespace()));
    let mut remove = vec![false; nodes.len()];
    let mut start = 0;
    while start < nodes.len() {
        let transparent =
            |node: &Node| is_whitespace_text(node) || matches!(node, Node::Comment(_));
        if !transparent(&nodes[start]) {
            start += 1;
            continue;
        }
        let mut end = start;
        while end < nodes.len() && transparent(&nodes[end]) {
            end += 1;
        }
        let before = side(start.checked_sub(1).map(|i| &nodes[i]));
        let after = side(nodes.get(end));
        if before != Side::Other
            && after != Side::Other
            && (before, after) != (Side::Edge, Side::Edge)
        {
            for i in start..end {
                remove[i] = is_whitespace_text(&nodes[i]);
            }
        }
        start = end;
    }
    let mut index = 0;
    nodes.retain(|_| {
        index += 1;
        !remove[index - 1]
    });
    for node in nodes {
        let Node::Element { name, children, .. } = node else {
            continue;
        };
        if name == "head" {
            // Nothing in `<head>` is rendered, but the parser moves the line
            // feed after a leading `<style>` or `<script>` there.
            children.retain(|child| !is_whitespace_text(child));
        } else if matches!(kind(name), Kind::Inline | Kind::Block | Kind::AtomicInline) {
            remove_whitespace_between_blocks(children);
        }
    }
}

/// Where the inline whitespace pass stands within the current line.
struct LineState {
    /// The last character written on this line is a space, or the line has
    /// no characters yet; either way, further whitespace is dropped.
    after_space: bool,
    /// The text node whose last character is the space which ended the line
    /// so far, to be removed if the line ends here.
    trailing_space: Option<Rc<RefCell<String>>>,
}

impl LineState {
    fn new() -> Self {
        LineState {
            after_space: true,
            trailing_space: None,
        }
    }

    /// Rule 2: the line ends, so its trailing space is removed, and the next
    /// line starts with its leading whitespace dropped.
    fn end(&mut self) {
        if let Some(text) = self.trailing_space.take() {
            text.borrow_mut().pop();
        }
        self.after_space = true;
    }

    /// Something other than whitespace was rendered.
    fn content(&mut self) {
        self.after_space = false;
        self.trailing_space = None;
    }
}

/// Rules 1 and 2: each run of whitespace becomes one space, kept where the run
/// starts, and whitespace at the start or end of a line is removed.
fn collapse_inline_whitespace(nodes: &[Node], line: &mut LineState) {
    for node in nodes {
        match node {
            Node::Text(text) => {
                let mut collapsed = String::new();
                for c in text.borrow().chars() {
                    if is_collapsible(c) {
                        if !line.after_space {
                            collapsed.push(' ');
                            line.after_space = true;
                        }
                    } else {
                        collapsed.push(c);
                        line.after_space = false;
                    }
                }
                let ends_with_space = collapsed.ends_with(' ');
                let changed_line = !collapsed.is_empty();
                *text.borrow_mut() = collapsed;
                if ends_with_space {
                    line.trailing_space = Some(Rc::clone(text));
                } else if changed_line {
                    line.trailing_space = None;
                }
            }
            Node::Comment(_) | Node::Doctype(_) | Node::ProcessingInstruction(_) => {}
            Node::Element { name, children, .. } => match kind(name) {
                Kind::Hidden => {}
                Kind::LineBreak => line.end(),
                Kind::Preformatted => {
                    line.end();
                    drop_final_line_feed(children);
                    line.end();
                }
                Kind::Textarea => {
                    drop_final_line_feed(children);
                    line.content();
                }
                Kind::AtomicInline => {
                    let mut inner = LineState::new();
                    collapse_inline_whitespace(children, &mut inner);
                    inner.end();
                    line.content();
                }
                Kind::Block => {
                    line.end();
                    collapse_inline_whitespace(children, line);
                    line.end();
                }
                Kind::Inline => collapse_inline_whitespace(children, line),
            },
        }
    }
}

/// A single line feed which ends preformatted content, after a character
/// other than a line feed, is not rendered.
fn drop_final_line_feed(children: &[Node]) {
    let mut texts = Vec::new();
    collect_texts(children, &mut texts);
    let content: String = texts.iter().map(|text| text.borrow().clone()).collect();
    let mut from_end = content.chars().rev();
    if from_end.next() == Some('\n')
        && from_end.next().is_some_and(|c| c != '\n')
        && let Some(last) = texts.iter().rev().find(|text| !text.borrow().is_empty())
    {
        last.borrow_mut().pop();
    }
}

fn collect_texts(nodes: &[Node], texts: &mut Vec<Rc<RefCell<String>>>) {
    for node in nodes {
        match node {
            Node::Text(text) => texts.push(Rc::clone(text)),
            Node::Element { children, .. } => collect_texts(children, texts),
            _ => {}
        }
    }
}

/// Writes the tree with empty text nodes dropped and adjacent text nodes
/// merged, so that splitting text between nodes is not a difference.
fn write_nodes(nodes: &[Node], depth: usize, out: &mut String) {
    let indent = "  ".repeat(depth);
    let mut pending_text = String::new();
    let flush = |pending_text: &mut String, out: &mut String| {
        if !pending_text.is_empty() {
            out.push_str(&format!("{indent}{pending_text:?}\n"));
            pending_text.clear();
        }
    };
    for node in nodes {
        if let Node::Text(text) = node {
            pending_text.push_str(&text.borrow());
            continue;
        }
        flush(&mut pending_text, out);
        match node {
            Node::Element {
                name,
                attrs,
                children,
            } => {
                out.push_str(&format!("{indent}<{name}"));
                for (key, value) in attrs {
                    out.push_str(&format!(" {key}={value:?}"));
                }
                out.push_str(">\n");
                write_nodes(children, depth + 1, out);
            }
            Node::Comment(contents) => out.push_str(&format!("{indent}<!--{contents:?}-->\n")),
            Node::Doctype(name) => out.push_str(&format!("{indent}<!DOCTYPE {name}>\n")),
            Node::ProcessingInstruction(contents) => {
                out.push_str(&format!("{indent}<?{contents:?}>\n"))
            }
            Node::Text(_) => unreachable!(),
        }
    }
    flush(&mut pending_text, out);
}
