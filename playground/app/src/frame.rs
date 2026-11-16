//! Documents for the Rendered and Original tabs (spec, Constraints: Safety).
//!
//! The tabs show untrusted HTML in an `<iframe sandbox="">` through
//! `srcdoc`. The empty sandbox blocks scripts, forms, popups, and navigating
//! the playground, and gives the frame an opaque origin. The frame's styles
//! cannot reach the playground. What the sandbox does not stop is handled
//! here, before the HTML reaches the frame:
//!
//! * Network requests. A Content Security Policy, placed first in the
//!   document, blocks every fetch except `data:` URLs; an `<img>` with an
//!   `https:` source shows as broken without a request. Elements a policy
//!   does not reliably cover are removed: `<meta http-equiv>` (a refresh
//!   navigates the frame), `<link>` (DNS prefetch and preconnect hints),
//!   `<base>`, nested frames, and plugins.
//! * Navigating the frame itself, which the sandbox allows. Every link gets
//!   `target="_blank"`; the sandbox blocks popups, so clicking does nothing.
//!   Declarative shadow roots are inside `<template>`, so templates are
//!   processed too.

use wasm_bindgen::JsCast;
use web_sys::{DomParser, Element, HtmlTemplateElement, NodeList, SupportedType};

/// Inserted at the start of `<head>`. `srcdoc` documents also inherit the
/// playground's own policy (`static/index.html`), which permits these inline
/// styles.
const PREAMBLE: &str = concat!(
    r#"<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; media-src data:; font-src data:; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'">"#,
    r#"<meta name="color-scheme" content="light dark">"#,
    "<style>",
    ":root{color-scheme:light dark;font-family:system-ui,-apple-system,'Segoe UI',sans-serif;line-height:1.5}",
    "body{margin:1rem 1.25rem;overflow-wrap:break-word}",
    "img{max-width:100%}",
    "pre{overflow:auto;padding:.75rem;border-radius:6px;background:color-mix(in srgb,currentColor 8%,transparent)}",
    "code,pre,kbd,samp{font-family:ui-monospace,'SF Mono',Menlo,Consolas,monospace;font-size:.92em}",
    "table{border-collapse:collapse}",
    "th,td{border:1px solid color-mix(in srgb,currentColor 30%,transparent);padding:.25rem .6rem}",
    "blockquote{margin-inline:0;padding-left:1rem;border-left:4px solid color-mix(in srgb,currentColor 25%,transparent)}",
    "</style>",
);

const REMOVED: &str = "script, iframe, frame, frameset, object, embed, applet, portal, fencedframe, base, link, meta[http-equiv]";

/// Build the `srcdoc` for showing `html` as a page.
pub fn page(html: &str) -> String {
    let Ok(parser) = DomParser::new() else {
        return String::new();
    };
    // DOMParser documents run no scripts and load nothing.
    let Ok(parsed) = parser.parse_from_string(html, SupportedType::TextHtml) else {
        return String::new();
    };
    clean(&|selector| parsed.query_selector_all(selector).ok());
    if let Some(head) = parsed.head() {
        let _ = head.insert_adjacent_html("afterbegin", PREAMBLE);
    }
    match parsed.document_element() {
        Some(root) => format!("<!DOCTYPE html>{}", root.outer_html()),
        None => String::new(),
    }
}

fn elements(list: Option<NodeList>) -> Vec<Element> {
    let Some(list) = list else {
        return Vec::new();
    };
    (0..list.length())
        .filter_map(|index| list.item(index)?.dyn_into::<Element>().ok())
        .collect()
}

fn clean(select: &dyn Fn(&str) -> Option<NodeList>) {
    for element in elements(select(REMOVED)) {
        element.remove();
    }
    for link in elements(select("a, area")) {
        let _ = link.set_attribute("target", "_blank");
    }
    // An SVG <use> may reference an external document.
    for usage in elements(select("use")) {
        for attribute in ["href", "xlink:href"] {
            if usage
                .get_attribute(attribute)
                .is_some_and(|target| !target.starts_with('#'))
            {
                let _ = usage.remove_attribute(attribute);
            }
        }
    }
    for template in elements(select("template")) {
        if let Ok(template) = template.dyn_into::<HtmlTemplateElement>() {
            let content = template.content();
            clean(&|selector| content.query_selector_all(selector).ok());
        }
    }
}
