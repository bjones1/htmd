//! FR-1 preview isolation: builds the complete `srcdoc` document for the
//! sandboxed preview `<iframe>` from generated body HTML. The document
//! carries FR-1's CSP `<meta>` and the demo's own
//! `<base target="_blank">`; everything the user's output contributed is
//! stripped of the elements and attributes FR-1:46 lists before it
//! enters the sandbox.

use html5ever::driver::ParseOpts;
use html5ever::parse_fragment;
use html5ever::serialize::{SerializeOpts, TraversalScope, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::{QualName, local_name, ns};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

/// FR-1:42, verbatim: the CSP delivered inside the srcdoc document
/// (`srcdoc` documents cannot receive response headers).
const CSP_META: &str = r#"<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; style-src 'unsafe-inline';">"#;

/// FR-1:47: the demo's own base, injected after the CSP `<meta>`.
const DEMO_BASE: &str = r#"<base target="_blank">"#;

/// Build the srcdoc document for the preview iframe (FR-1:45-47): parse
/// `body_html` as an HTML fragment (`<body>` context, `RcDom`), strip
/// every `<base>`, `<meta>`, `<link>`, `<noscript>`, and `<template>`
/// element whole and every `target` attribute with an iterative
/// (explicit-stack) walk, serialize what remains, and wrap it so the
/// document's first elements are the CSP `<meta>` then the demo's
/// `<base target="_blank">`.
pub fn build_srcdoc(body_html: &str) -> String {
    let dom = parse_fragment(
        RcDom::default(),
        ParseOpts::default(),
        QualName::new(None, ns!(html), local_name!("body")),
        vec![],
        true,
    )
    .one(body_html);

    let root = fragment_root(&dom);
    strip_for_preview(&root);
    let body = serialize_children(&root);
    format!("<!DOCTYPE html><html><head>{CSP_META}{DEMO_BASE}</head><body>{body}</body></html>")
}

/// The synthetic `<html>` element that fragment parsing appends to the
/// document; the fragment's top-level nodes are its children (FR-5:122's
/// fragment-parsing layout, which FR-1:45's parse shares).
fn fragment_root(dom: &RcDom) -> Handle {
    let children = dom.document.children.borrow();
    match children.first() {
        Some(handle)
            if matches!(
                &handle.data,
                NodeData::Element { name, .. }
                    if name.ns == ns!(html) && name.local == local_name!("html")
            ) =>
        {
            handle.clone()
        }
        // Unreachable with html5ever 0.39 (the root is created
        // unconditionally); keeps the walk sound if that ever changes.
        _ => dom.document.clone(),
    }
}

/// FR-1:46's removal list: whole-element removal of `<base>`, `<meta>`,
/// `<link>`, `<noscript>`, and `<template>` (with contents).
fn is_stripped_element(node: &Handle) -> bool {
    matches!(
        &node.data,
        NodeData::Element { name, .. }
            if name.ns == ns!(html)
                && matches!(
                    name.local,
                    local_name!("base")
                        | local_name!("meta")
                        | local_name!("link")
                        | local_name!("noscript")
                        | local_name!("template")
                )
    )
}

/// FR-1:45's iterative walk (explicit stack — a recursive walk would
/// overflow the wasm stack on 50,000 nested blockquotes): drop every
/// stripped element from its parent's child list — the subtree goes with
/// the `Rc` — and delete every `target` attribute.
fn strip_for_preview(root: &Handle) {
    let mut stack = vec![root.clone()];
    while let Some(node) = stack.pop() {
        if let NodeData::Element { attrs, .. } = &node.data {
            let mut list = std::mem::take(&mut *attrs.borrow_mut());
            list.retain(|attr| attr.name.local != local_name!("target"));
            *attrs.borrow_mut() = list;
        }
        let mut children = std::mem::take(&mut *node.children.borrow_mut());
        children.retain(|child| !is_stripped_element(child));
        stack.extend(children.iter().cloned());
        *node.children.borrow_mut() = children;
    }
}

/// Serialize each top-level child in document order per FR-5:130's
/// serialization rule (never the `RcDom` document node itself, which
/// panics). The `RcDom` serializer walks a work queue, not the call
/// stack, so 50,000-deep output serializes without overflow.
fn serialize_children(root: &Handle) -> String {
    let mut out = Vec::new();
    for child in root.children.borrow().iter() {
        serialize(
            &mut out,
            &SerializableHandle::from(child.clone()),
            SerializeOpts {
                traversal_scope: TraversalScope::IncludeNode,
                ..Default::default()
            },
        )
        .expect("serializing an RcDom subtree cannot fail");
    }
    String::from_utf8(out).expect("serializer output mirrors its UTF-8 input")
}

#[cfg(test)]
mod tests {
    use super::build_srcdoc;

    /// FR-1:42's CSP meta, verbatim from the spec line.
    const CSP_META: &str = r#"<meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; style-src 'unsafe-inline';">"#;
    /// FR-1:47's injected base.
    const DEMO_BASE: &str = r#"<base target="_blank">"#;

    fn body_of(srcdoc: &str) -> &str {
        let start =
            srcdoc.find("</head><body>").expect("srcdoc has a body") + "</head><body>".len();
        let end = srcdoc
            .rfind("</body></html>")
            .expect("srcdoc closes the document");
        &srcdoc[start..end]
    }

    #[test]
    fn head_is_csp_meta_then_demo_base_nothing_else() {
        // FR-1:42/47: the document's first elements are the CSP meta,
        // then the demo's own base; §8.6:292's "excluding the demo's own
        // injected base" makes these two the only permitted ones.
        let srcdoc = build_srcdoc("<p>hello</p>");
        assert_eq!(
            srcdoc,
            format!(
                "<!DOCTYPE html><html><head>{CSP_META}{DEMO_BASE}</head><body><p>hello</p></body></html>"
            )
        );
    }

    #[test]
    fn empty_input_yields_the_minimal_document() {
        // §8.6 brief: empty input → minimal document with CSP + base only.
        assert_eq!(
            build_srcdoc(""),
            format!("<!DOCTYPE html><html><head>{CSP_META}{DEMO_BASE}</head><body></body></html>")
        );
    }

    #[test]
    fn user_base_meta_and_link_elements_are_removed() {
        // FR-1:46: every <base>, <meta>, and <link> goes, whole —
        // including refresh redirection, x-dns-prefetch-control, and the
        // dns-prefetch/preconnect/prefetch/prerender link rels the
        // round-12-fixed wording covers by removing whole element types.
        let input = "<p>keep</p>\
                     <base href=\"https://attacker.example/\">\
                     <meta charset=\"utf-8\">\
                     <meta http-equiv=\"refresh\" content=\"0;url=https://x.example\">\
                     <meta http-equiv=\"x-dns-prefetch-control\" content=\"on\">\
                     <link rel=\"stylesheet\" href=\"evil.css\">\
                     <link rel=\"dns-prefetch\" href=\"//x.example\">\
                     <link rel=\"preconnect\" href=\"//y.example\">\
                     <link rel=\"prefetch\" href=\"//z.example\">\
                     <link rel=\"prerender\" href=\"https://x.example\">";
        let srcdoc = build_srcdoc(input);
        assert_eq!(body_of(&srcdoc), "<p>keep</p>");
        assert!(!srcdoc.contains("attacker"));
        assert!(!srcdoc.contains("x.example"));
        assert!(!srcdoc.contains("evil.css"));
        // Only the CSP meta and demo base exist in the whole document.
        assert_eq!(srcdoc.matches("<meta").count(), 1);
        assert_eq!(srcdoc.matches("<base").count(), 1);
        assert_eq!(srcdoc.matches("<link").count(), 0);
    }

    #[test]
    fn noscript_is_removed_whole_with_its_contents() {
        // §8.6:292's exact adversarial snippet: the nested refresh meta
        // and self-targeting anchor disappear with the <noscript> shell
        // (raw <noscript> text must not be re-parsed as live markup in
        // the scriptless sandbox, FR-1:46).
        let input = "<noscript><meta http-equiv=\"refresh\" content=\"0;url=https://x.example\"><a href=\"https://x.example\" target=\"_self\">x</a></noscript><p>after</p>";
        let srcdoc = build_srcdoc(input);
        assert_eq!(body_of(&srcdoc), "<p>after</p>");
        assert!(!srcdoc.contains("noscript"));
        assert!(!srcdoc.contains("x.example"));
    }

    #[test]
    fn template_is_removed_whole_with_its_contents() {
        // §8.6:292's declarative-shadow-DOM template.
        let srcdoc =
            build_srcdoc("<template shadowrootmode=\"open\"><span>x</span></template><p>after</p>");
        assert_eq!(body_of(&srcdoc), "<p>after</p>");
        assert!(!srcdoc.contains("template"));
        assert!(!srcdoc.contains("<span>x</span>"));
    }

    #[test]
    fn user_target_attributes_are_stripped_demo_base_kept() {
        // FR-1:46: every user target attribute disappears; FR-1:47: the
        // demo's own base keeps its target — the only one in the output.
        let srcdoc = build_srcdoc(
            "<p><a href=\"a\" target=\"_blank\">A</a><a href=\"b\" target=\"_self\">B</a><a href=\"c\">C</a></p>",
        );
        assert_eq!(
            body_of(&srcdoc),
            "<p><a href=\"a\">A</a><a href=\"b\">B</a><a href=\"c\">C</a></p>"
        );
        assert_eq!(srcdoc.matches("target=").count(), 1);
        assert!(srcdoc.contains(DEMO_BASE));
    }

    #[test]
    fn content_survives_the_strip_untouched() {
        // FR-1:46 removes exactly the five element types and target
        // attributes — everything else (text, attributes, data-URI
        // images, comments) renders as parsed, since the preview must
        // display the generated output, not a canonicalized rewrite.
        let srcdoc = build_srcdoc(
            "<p title=\"kept\">text &amp; &lt;markup&gt;</p>\
             <img src=\"data:image/gif;base64,R0lGOD\" alt=\"dot\">\
             <!-- kept: not on FR-1's removal list -->",
        );
        assert_eq!(
            body_of(&srcdoc),
            "<p title=\"kept\">text &amp; &lt;markup&gt;</p>\
             <img src=\"data:image/gif;base64,R0lGOD\" alt=\"dot\">\
             <!-- kept: not on FR-1's removal list -->"
        );
    }

    #[test]
    fn deep_nesting_strips_and_serializes_iteratively() {
        // FR-1:45: the walk must survive 50,000-deep input. 5,000 keeps
        // the fast suite fast (html5ever's fragment parse is quadratic in
        // nesting depth); the ignored test below proves the full depth.
        let depth = 5_000;
        let srcdoc = build_srcdoc(&"<blockquote>".repeat(depth));
        assert_eq!(srcdoc.matches("<blockquote>").count(), depth);
        assert_eq!(srcdoc.matches("</blockquote>").count(), depth);
        assert!(srcdoc.starts_with(
            "<!DOCTYPE html><html><head><meta http-equiv=\"Content-Security-Policy\""
        ));
        assert!(srcdoc.ends_with("</blockquote></body></html>"));
    }

    #[ignore = "slow (~1 min debug): proves the strip walk and serializer survive 50,000 levels without stack overflow; run with `cargo test -- --ignored`"]
    #[test]
    fn fifty_thousand_nested_blockquotes_build_without_stack_overflow() {
        // §8.6:292 / FR-1:45: 50,000 nested blockquotes (what 50,000 `>`
        // render to) must produce a preview document, not a stack
        // overflow. Timing note: html5ever's fragment parse is quadratic
        // in depth (~16 s at release, ~1 min at debug); serialization
        // itself is linear.
        let depth = 50_000;
        let srcdoc = build_srcdoc(&"<blockquote>".repeat(depth));
        assert!(!srcdoc.is_empty());
        assert_eq!(srcdoc.matches("<blockquote>").count(), depth);
        assert_eq!(srcdoc.matches("</blockquote>").count(), depth);
        assert!(srcdoc.ends_with("</blockquote></body></html>"));
    }
}
