//! Canonical HTML form for the FR-5 fidelity diff: parse an HTML fragment
//! with html5ever, normalize the DOM per FR-5's "DOM Normalization" and
//! "Nodes & Entities" rules, and serialize each top-level node in document
//! order. Structurally equivalent fragments (spec §8.7's safe variations)
//! canonicalize to byte-identical strings, which is what makes the
//! word-level diff free of false positives.

use std::collections::BTreeSet;

use html5ever::driver::ParseOpts;
use html5ever::parse_fragment;
use html5ever::serialize::{SerializeOpts, TraversalScope, serialize};
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::{Attribute, LocalName, QualName, local_name, ns};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

/// Canonicalize an HTML fragment per FR-5 so the structurally equivalent
/// variations listed in spec §8.7 yield byte-identical strings for the
/// word-level diff. Both fidelity-panel sides go through this function, so
/// every rule is symmetric by construction.
// The fidelity-panel task that calls this is the module's only consumer;
// until it exists, `-D warnings` sees it as dead code.
#[allow(dead_code)]
pub fn canonicalize(html: &str) -> String {
    let dom = parse_fragment(
        RcDom::default(),
        ParseOpts::default(),
        QualName::new(None, ns!(html), local_name!("body")),
        vec![],
        true,
    )
    .one(html);

    let root = fragment_root(&dom);
    normalize_fragment(&root);

    let mut out = Vec::new();
    for child in root.children.borrow().iter() {
        // Serialize each top-level child separately: serializing the RcDom
        // document node itself panics (FR-5's "Deterministic Serialization").
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

/// The synthetic `<html>` element that fragment parsing appends to the
/// document; the fragment's top-level nodes are its children (FR-5's
/// "Fragment Parsing").
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

/// Which node's children a normalization frame applies to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ParentKind {
    FragmentRoot,
    Block,
    Other,
}

enum Frame {
    /// Normalize `parent`'s children; `preserved` says whether the parent
    /// sits inside a preserved subtree (FR-5), gating the text rules.
    Children {
        parent: Handle,
        kind: ParentKind,
        preserved: bool,
    },
    /// Fires once a `<pre>` subtree is fully normalized.
    TrimPre(Handle),
}

/// Iterative (explicit stack) walk: every parent's child list is normalized
/// before its element children are pushed, and each `<pre>` schedules its
/// trailing-newline trim to run after its whole subtree.
fn normalize_fragment(root: &Handle) {
    let mut stack = vec![Frame::Children {
        parent: root.clone(),
        kind: ParentKind::FragmentRoot,
        preserved: false,
    }];
    while let Some(frame) = stack.pop() {
        let (parent, kind, preserved) = match frame {
            Frame::Children {
                parent,
                kind,
                preserved,
            } => (parent, kind, preserved),
            Frame::TrimPre(pre) => {
                trim_pre_trailing_newline(&pre);
                continue;
            }
        };

        if let NodeData::Element { .. } = parent.data {
            normalize_attributes(&parent);
        }
        let subtree_preserved = preserved || is_preserved_element(&parent);
        normalize_children(&parent, kind, subtree_preserved);

        if is_pre_element(&parent) {
            stack.push(Frame::TrimPre(parent.clone()));
        }
        let element_children: Vec<Handle> = parent
            .children
            .borrow()
            .iter()
            .filter(|child| matches!(child.data, NodeData::Element { .. }))
            .cloned()
            .collect();
        for child in element_children.into_iter().rev() {
            let kind = if is_block_element(&child) {
                ParentKind::Block
            } else {
                ParentKind::Other
            };
            stack.push(Frame::Children {
                parent: child,
                kind,
                preserved: subtree_preserved,
            });
        }
    }
}

/// FR-5:125/126 — lowercase HTML-namespace attribute names (the tokenizer
/// already does; foreign names like `viewBox` must keep their case, which
/// is why the rebuild is HTML-only), collapse and sort `class` tokens, and
/// sort attributes by (namespace URL, local name) in code-point order.
/// Element tag names are immutable in `RcDom`; the HTML tokenizer's own
/// case folding already satisfies FR-5's lowercase rule for them.
fn normalize_attributes(element: &Handle) {
    let NodeData::Element { name, attrs, .. } = &element.data else {
        return;
    };
    let html_element = name.ns == ns!(html);
    let mut list = std::mem::take(&mut *attrs.borrow_mut());
    for attr in list.iter_mut() {
        if html_element && attr.name.local.bytes().any(|b| b.is_ascii_uppercase()) {
            attr.name.local = LocalName::from(attr.name.local.to_ascii_lowercase());
        }
        if attr.name.local == local_name!("class") {
            let tokens: BTreeSet<&str> = attr.value.split_ascii_whitespace().collect();
            let collapsed = tokens.into_iter().collect::<Vec<_>>().join(" ");
            attr.value = StrTendril::from(collapsed);
        }
    }
    list.sort_by(|a, b| (&*a.name.ns, &*a.name.local).cmp(&(&*b.name.ns, &*b.name.local)));
    *attrs.borrow_mut() = list;
}

/// FR-5:128/129 applied to one parent's child list: drop comment, doctype,
/// and `<template>` nodes everywhere; outside preserved subtrees also
/// replace U+00A0, collapse whitespace runs, drop whitespace-only text at
/// the fragment root or beside block siblings, and trim the text edges of
/// the fragment root and of block elements.
fn normalize_children(parent: &Handle, kind: ParentKind, preserved: bool) {
    let mut children: Vec<Handle> = std::mem::take(&mut *parent.children.borrow_mut());
    children.retain(|child| !is_discarded_node(child));

    if !preserved {
        for child in &children {
            if let NodeData::Text { contents } = &child.data {
                let collapsed = collapse_whitespace_runs(&contents.borrow());
                *contents.borrow_mut() = StrTendril::from(collapsed);
            }
        }

        let drops: Vec<bool> = children
            .iter()
            .enumerate()
            .map(|(i, child)| {
                let NodeData::Text { contents } = &child.data else {
                    return false;
                };
                is_whitespace_only(&contents.borrow())
                    && (kind == ParentKind::FragmentRoot
                        || i.checked_sub(1)
                            .is_some_and(|prev| is_block_sibling(&children, prev))
                        || is_block_sibling(&children, i + 1))
            })
            .collect();
        children = children
            .into_iter()
            .zip(drops)
            .filter_map(|(child, drop)| (!drop).then_some(child))
            .collect();

        if matches!(kind, ParentKind::FragmentRoot | ParentKind::Block) {
            trim_text_edge(&children, Edge::Leading);
            trim_text_edge(&children, Edge::Trailing);
            children.retain(|child| {
                !matches!(&child.data, NodeData::Text { contents } if contents.borrow().is_empty())
            });
        }
    }

    *parent.children.borrow_mut() = children;
}

enum Edge {
    Leading,
    Trailing,
}

/// Trim outer whitespace from the first/last child of a block edge when it
/// is a text node (FR-5:128); `normalize_children` drops emptied nodes.
fn trim_text_edge(children: &[Handle], edge: Edge) {
    let index = match edge {
        Edge::Leading => children.first().map(|_| 0),
        Edge::Trailing => children.len().checked_sub(1),
    };
    let Some(index) = index else {
        return;
    };
    if let NodeData::Text { contents } = &children[index].data {
        let trimmed = match edge {
            Edge::Leading => contents
                .borrow()
                .trim_start_matches(is_html_whitespace)
                .to_string(),
            Edge::Trailing => contents
                .borrow()
                .trim_end_matches(is_html_whitespace)
                .to_string(),
        };
        *contents.borrow_mut() = StrTendril::from(trimmed);
    }
}

/// FR-5:127 — exactly one trailing U+000A leaves the last text node in
/// document order inside the `<pre>` subtree, so CommonMark's mandatory
/// final line ending in code blocks cannot produce a false diff.
fn trim_pre_trailing_newline(pre: &Handle) {
    let mut stack = vec![pre.clone()];
    let mut last_text: Option<Handle> = None;
    while let Some(node) = stack.pop() {
        if matches!(node.data, NodeData::Text { .. }) {
            last_text = Some(node.clone());
        }
        let children = node.children.borrow();
        for child in children.iter().rev() {
            stack.push(child.clone());
        }
    }
    let Some(node) = last_text else {
        return;
    };
    let NodeData::Text { contents } = &node.data else {
        return;
    };
    let stripped = contents.borrow().strip_suffix('\n').map(str::to_string);
    if let Some(stripped) = stripped {
        *contents.borrow_mut() = StrTendril::from(stripped);
    }
}

/// FR-5:129 — comment, doctype, and `<template>` nodes disappear.
fn is_discarded_node(node: &Handle) -> bool {
    match &node.data {
        NodeData::Comment { .. }
        | NodeData::Doctype { .. }
        | NodeData::ProcessingInstruction { .. } => true,
        NodeData::Element { name, .. } => {
            name.ns == ns!(html) && name.local == local_name!("template")
        }
        _ => false,
    }
}

/// FR-5:128's verbatim block-level element list.
fn is_block_element(node: &Handle) -> bool {
    let NodeData::Element { name, .. } = &node.data else {
        return false;
    };
    name.ns == ns!(html)
        && matches!(
            name.local,
            local_name!("address")
                | local_name!("article")
                | local_name!("aside")
                | local_name!("blockquote")
                | local_name!("caption")
                | local_name!("dd")
                | local_name!("details")
                | local_name!("dialog")
                | local_name!("div")
                | local_name!("dl")
                | local_name!("dt")
                | local_name!("fieldset")
                | local_name!("figcaption")
                | local_name!("figure")
                | local_name!("footer")
                | local_name!("form")
                | local_name!("h1")
                | local_name!("h2")
                | local_name!("h3")
                | local_name!("h4")
                | local_name!("h5")
                | local_name!("h6")
                | local_name!("header")
                | local_name!("hgroup")
                | local_name!("hr")
                | local_name!("li")
                | local_name!("main")
                | local_name!("nav")
                | local_name!("ol")
                | local_name!("p")
                | local_name!("pre")
                | local_name!("section")
                | local_name!("summary")
                | local_name!("table")
                | local_name!("tbody")
                | local_name!("td")
                | local_name!("tfoot")
                | local_name!("th")
                | local_name!("thead")
                | local_name!("tr")
                | local_name!("ul")
        )
}

fn is_block_sibling(children: &[Handle], index: usize) -> bool {
    children.get(index).is_some_and(is_block_element)
}

fn is_pre_element(node: &Handle) -> bool {
    matches!(
        &node.data,
        NodeData::Element { name, .. }
            if name.ns == ns!(html) && name.local == local_name!("pre")
    )
}

/// FR-5:127 — `<pre>`, `<code>`, `<textarea>`, `<script>`, and `<style>`
/// subtrees, plus any element whose first `style` declaration sets
/// `white-space` to something other than `normal`.
fn is_preserved_element(node: &Handle) -> bool {
    let NodeData::Element { name, attrs, .. } = &node.data else {
        return false;
    };
    let tagged = name.ns == ns!(html)
        && matches!(
            name.local,
            local_name!("pre")
                | local_name!("code")
                | local_name!("textarea")
                | local_name!("script")
                | local_name!("style")
        );
    tagged || declares_non_normal_white_space(&attrs.borrow())
}

/// Read only the first `style` declaration, case-insensitively; the demo
/// never parses CSS, so later declarations stay invisible to the check.
fn declares_non_normal_white_space(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let name: &str = &attr.name.local;
        name.eq_ignore_ascii_case("style")
            && attr.value.split(';').next().is_some_and(|declaration| {
                let Some((property, value)) = declaration.split_once(':') else {
                    return false;
                };
                property.trim().eq_ignore_ascii_case("white-space")
                    && !value.trim().eq_ignore_ascii_case("normal")
            })
    })
}

fn is_html_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '\u{c}')
}

fn is_whitespace_only(text: &str) -> bool {
    text.chars().all(is_html_whitespace)
}

/// Collapse each whitespace run to a single space; U+00A0 is replaced
/// before collapsing, so it joins the runs it touches (FR-5:129).
fn collapse_whitespace_runs(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_run = false;
    for c in text.chars() {
        if is_html_whitespace(c) || c == '\u{a0}' {
            if !in_run {
                out.push(' ');
                in_run = true;
            }
        } else {
            out.push(c);
            in_run = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::canonicalize;

    // Golden tests below are spec §8.7's cases verbatim (demo-spec.md lines
    // 296-307); each cites its line. Rule-level tests cite FR-5 (lines
    // 122-130).

    /// §8.7:298 — tag & attribute casing.
    #[test]
    fn tag_and_attribute_casing() {
        assert_eq!(
            canonicalize(r#"<DIV ID="x">a</DIV>"#),
            r#"<div id="x">a</div>"#
        );
        assert_eq!(
            canonicalize(r#"<div id="x">a</div>"#),
            r#"<div id="x">a</div>"#
        );
    }

    /// §8.7:299 — attribute order.
    #[test]
    fn attribute_order() {
        assert_eq!(
            canonicalize(r##"<a href="#" id="x">t</a>"##),
            r##"<a href="#" id="x">t</a>"##
        );
        assert_eq!(
            canonicalize(r##"<a id="x" href="#">t</a>"##),
            r##"<a href="#" id="x">t</a>"##
        );
    }

    /// §8.7:300 — attribute quoting: single, double, or omitted.
    #[test]
    fn attribute_quoting() {
        let expected = r#"<a href="x">t</a>"#;
        assert_eq!(canonicalize(r#"<a href='x'>t</a>"#), expected);
        assert_eq!(canonicalize(r#"<a href=x>t</a>"#), expected);
        assert_eq!(canonicalize(r#"<a href="x">t</a>"#), expected);
    }

    /// §8.7:301 — void element syntax: self-closing slash vs open.
    #[test]
    fn void_element_syntax() {
        assert_eq!(canonicalize("<br />"), "<br>");
        assert_eq!(canonicalize("<br>"), "<br>");
    }

    /// §8.7:302 — entity representations: named, decimal, hexadecimal.
    #[test]
    fn entity_forms_unify() {
        let expected = "<p>a&amp;b</p>";
        assert_eq!(canonicalize("<p>a&amp;b</p>"), expected);
        assert_eq!(canonicalize("<p>a&#38;b</p>"), expected);
        assert_eq!(canonicalize("<p>a&#x26;b</p>"), expected);
    }

    /// §8.7:303 — insignificant whitespace: spacing and line breaks between
    /// elements, leading/trailing whitespace of the fragment and of block
    /// elements.
    #[test]
    fn insignificant_whitespace() {
        let expected = "<p>a</p>";
        assert_eq!(canonicalize("<p>a</p>"), expected);
        assert_eq!(canonicalize("<p>a</p>\n"), expected);

        let expected = "<div>a</div><div>b</div>";
        assert_eq!(canonicalize("<div>a</div>\n<div>b</div>"), expected);
        assert_eq!(canonicalize("<div>a</div> <div>b</div>"), expected);
        assert_eq!(canonicalize("<div>a</div><div>b</div>"), expected);

        assert_eq!(canonicalize("<p>\n  a\n</p>"), "<p>a</p>");
        assert_eq!(canonicalize("  a  "), "a");
    }

    /// §8.7:303 — a single trailing newline at the end of a `<pre>` block.
    #[test]
    fn pre_trailing_newline() {
        let expected = "<pre><code>x</code></pre>";
        assert_eq!(canonicalize("<pre><code>x</code></pre>"), expected);
        assert_eq!(canonicalize("<pre><code>x\n</code></pre>"), expected);
    }

    /// §8.7:304 — CDATA in foreign content equals its text form.
    #[test]
    fn cdata_in_foreign_content() {
        let expected = "<svg><text>a&lt;b</text></svg>";
        assert_eq!(
            canonicalize("<svg><text><![CDATA[a<b]]></text></svg>"),
            expected
        );
        assert_eq!(canonicalize("<svg><text>a&lt;b</text></svg>"), expected);
    }

    /// §8.7:305 — an empty fragment canonicalizes to the empty string;
    /// FR-5:128 — so does an all-whitespace fragment.
    #[test]
    fn empty_fragment_canonicalizes_to_empty_string() {
        assert_eq!(canonicalize(""), "");
        assert_eq!(canonicalize(" \n\t"), "");
    }

    /// §8.7:305 — `<p>a</p>` canonicalizes to `<p>a</p>`, no `<html>`
    /// wrapper, no panic.
    #[test]
    fn p_a_canonicalizes_without_html_wrapper() {
        assert_eq!(canonicalize("<p>a</p>"), "<p>a</p>");
    }

    /// §8.7:306 — a CJK paragraph and a combining-mark (U+0301) word
    /// round-trip with zero diff. The trailing-newline forms are what
    /// `pulldown-cmark` emits when re-rendering these paragraphs (verified
    /// against htmd defaults + ENABLE_TABLES | ENABLE_MATH); no NFC
    /// composition (FR-5:129).
    #[test]
    fn cjk_and_combining_mark_round_trip() {
        assert_eq!(canonicalize("<p>こんにちは</p>"), "<p>こんにちは</p>");
        assert_eq!(canonicalize("<p>cafe\u{0301}</p>"), "<p>cafe\u{0301}</p>");
        assert_eq!(canonicalize("<p>こんにちは</p>\n"), "<p>こんにちは</p>");
        assert_eq!(canonicalize("<p>cafe\u{0301}</p>\n"), "<p>cafe\u{0301}</p>");
    }

    /// §8.7:306 — `<span style="white-space: pre">` keeps its inner spacing,
    /// which is exactly why the round-tripped "a b" yields a nonzero diff.
    #[test]
    fn white_space_style_attr_preserves_whitespace() {
        assert_eq!(
            canonicalize(r#"<p><span style="white-space: pre">a  b</span></p>"#),
            r#"<p><span style="white-space: pre">a  b</span></p>"#
        );
    }

    /// §8.7:307 — `<pre><code>x</code></pre>` is golden corpus (zero diff).
    /// The newline-suffixed form is the pulldown-cmark re-render of htmd's
    /// Markdown (verified with Faithful defaults + ENABLE_TABLES |
    /// ENABLE_MATH): both canonicalize identically.
    #[test]
    fn pre_code_golden_zero_diff() {
        let expected = "<pre><code>x</code></pre>";
        assert_eq!(canonicalize("<pre><code>x</code></pre>"), expected);
        assert_eq!(canonicalize("<pre><code>x\n</code></pre>\n"), expected);
    }

    /// §8.7:307 + FR-6:135 — the shipped CommonMark-basics sample is part
    /// of the golden corpus: converted HTML → Markdown under Faithful
    /// crate defaults (empty `skip_tags`) and re-rendered under the fixed
    /// FR-5 flags `ENABLE_TABLES | ENABLE_MATH`, it canonicalizes
    /// byte-identically to the sample itself (zero diff). Re-pinned to
    /// the real sample content by the samples task.
    #[test]
    fn commonmark_basics_golden_zero_diff() {
        use crate::convert::{convert_html_to_md, convert_md_to_html};
        use crate::options_model::{CmarkFlags, HtmdOptions};
        use crate::samples::SampleId;
        use htmd::options::TranslationMode;

        let opts = HtmdOptions {
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        };
        let flags = CmarkFlags {
            tables: true,
            math: true,
            ..Default::default()
        };
        let html = SampleId::Basics
            .sample()
            .variant(crate::engine::ConversionMode::HtmlToMarkdown)
            .expect("basics has an HTML variant");
        let md = convert_html_to_md(html, &opts, &[]).unwrap();
        let round_tripped = convert_md_to_html(&md, &flags);
        assert_eq!(canonicalize(html), canonicalize(&round_tripped));
    }

    /// FR-5:125 — foreign content keeps its native case (`viewBox`); SVG
    /// text is not a preserved subtree, so its spacing collapses.
    #[test]
    fn svg_camel_case_preserved() {
        assert_eq!(
            canonicalize(r#"<svg viewBox="0 0 1 1"><text>a  b</text></svg>"#),
            r#"<svg viewBox="0 0 1 1"><text>a b</text></svg>"#
        );
    }

    /// FR-5:126 — class tokens collapse and sort by code point.
    #[test]
    fn class_tokens_sorted_and_deduplicated() {
        assert_eq!(
            canonicalize(r#"<p class="z  m a m">x</p>"#),
            r#"<p class="a m z">x</p>"#
        );
    }

    /// FR-5:126 — attributes sort by namespace URL, then local name.
    #[test]
    fn attributes_sorted_by_namespace_then_local_name() {
        assert_eq!(
            canonicalize(r##"<use xlink:href="#x" id="u"></use>"##),
            r##"<use id="u" xlink:href="#x"></use>"##
        );
    }

    /// FR-5:129 — U+00A0 becomes U+0020 and joins collapsed runs outside
    /// preserved subtrees.
    #[test]
    fn nbsp_becomes_space_outside_preserved_subtrees() {
        assert_eq!(canonicalize("a&nbsp;&nbsp;b"), "a b");
    }

    /// FR-5:127/129 — U+00A0 survives inside a preserved subtree.
    #[test]
    fn nbsp_kept_inside_preserved_subtree() {
        assert_eq!(canonicalize("<pre>a&nbsp;b</pre>"), "<pre>a&nbsp;b</pre>");
    }

    /// FR-5:129 — comments, doctypes, and `<template>` subtrees are
    /// discarded.
    #[test]
    fn comments_doctypes_templates_discarded() {
        assert_eq!(
            canonicalize("<!--c--><!DOCTYPE html><template><p>t</p></template><p>x</p>"),
            "<p>x</p>"
        );
    }

    /// FR-5:129 — in HTML content a CDATA section parses as a bogus comment,
    /// which the comment rule discards.
    #[test]
    fn cdata_in_html_content_discarded() {
        assert_eq!(canonicalize("<p><![CDATA[a<b]]></p>"), "<p></p>");
    }

    /// FR-5:128 — text edges of block elements are trimmed, including
    /// nested blocks; whitespace between block siblings is dropped.
    #[test]
    fn block_element_edges_trimmed() {
        assert_eq!(canonicalize("<p> a </p>"), "<p>a</p>");
        assert_eq!(
            canonicalize("<ul>\n<li> a </li>\n</ul>"),
            "<ul><li>a</li></ul>"
        );
    }

    /// FR-5:128 — leading/trailing text of the fragment root is trimmed.
    #[test]
    fn fragment_root_text_edges_trimmed() {
        assert_eq!(canonicalize(" text at root "), "text at root");
    }

    /// FR-5:128 — a whitespace-only text child of the fragment root is
    /// dropped (verbatim rule, even between inline siblings).
    #[test]
    fn whitespace_only_text_dropped_at_fragment_root() {
        assert_eq!(
            canonicalize("<span>a</span> <span>b</span>"),
            "<span>a</span><span>b</span>"
        );
    }

    /// FR-5:128 — spacing around inline siblings inside a block survives as
    /// single spaces.
    #[test]
    fn inline_spacing_kept_inside_blocks() {
        assert_eq!(canonicalize("<p>a  <b>x</b>  b</p>"), "<p>a <b>x</b> b</p>");
    }

    /// FR-5:127 — exactly one trailing U+000A leaves the last text node of
    /// a `<pre>`, wherever in the subtree that node sits.
    #[test]
    fn pre_trailing_newline_is_single() {
        assert_eq!(canonicalize("<pre>x\n\n</pre>"), "<pre>x\n</pre>");
        assert_eq!(
            canonicalize("<pre><code>x</code>\n</pre>"),
            "<pre><code>x</code></pre>"
        );
    }

    /// FR-5:127 — the `style` declaration is parsed case-insensitively;
    /// everything after the first `;` is ignored; `white-space: normal` does
    /// not preserve.
    #[test]
    fn style_attr_detection_rules() {
        assert_eq!(
            canonicalize(r#"<p style="WHITE-SPACE:PRE">a  b</p>"#),
            r#"<p style="WHITE-SPACE:PRE">a  b</p>"#
        );
        assert_eq!(
            canonicalize(r#"<p style="color:red; white-space:pre">a  b</p>"#),
            r#"<p style="color:red; white-space:pre">a b</p>"#
        );
        assert_eq!(
            canonicalize(r#"<p style="white-space: normal">a  b</p>"#),
            r#"<p style="white-space: normal">a b</p>"#
        );
    }
}
