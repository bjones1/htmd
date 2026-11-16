use htmd::HtmlToMarkdown;
use markup5ever_rcdom::{Handle, NodeData};

/// Parse with htmd's document options and extract HTML body children (§4).
pub fn parse_body_children(html: &str) -> std::io::Result<Vec<Handle>> {
    let document = HtmlToMarkdown::builder().build().html_to_tree(html)?;
    let html_element = document
        .children
        .borrow()
        .iter()
        .find(|node| {
            matches!(&node.data, NodeData::Element { name, .. }
            if name.ns.as_ref() == "http://www.w3.org/1999/xhtml" && name.local.as_ref() == "html")
        })
        .cloned()
        .expect("document parser must insert an HTML root");
    let body = html_element
        .children
        .borrow()
        .iter()
        .find(|node| {
            matches!(&node.data, NodeData::Element { name, .. }
            if name.ns.as_ref() == "http://www.w3.org/1999/xhtml" && name.local.as_ref() == "body")
        })
        .cloned()
        .expect("body-content fixture must produce an HTML body");
    // Detach before dropping the document: RcDom nodes drain descendants on drop.
    Ok(std::mem::take(&mut *body.children.borrow_mut()))
}

/// Versioned copy of htmd's private block set for the oracle (§6).
pub const BLOCK_ELEMENTS: &[&str] = &[
    "address",
    "article",
    "aside",
    "base",
    "basefont",
    "blockquote",
    "body",
    "caption",
    "center",
    "col",
    "colgroup",
    "dd",
    "details",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hr",
    "html",
    "iframe",
    "legend",
    "li",
    "link",
    "main",
    "menu",
    "menuitem",
    "nav",
    "noframes",
    "ol",
    "optgroup",
    "option",
    "p",
    "param",
    "pre",
    "script",
    "search",
    "section",
    "style",
    "summary",
    "table",
    "tbody",
    "td",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "track",
    "ul",
];
