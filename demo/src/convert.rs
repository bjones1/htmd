//! Bidirectional conversion: pulldown-cmark rendering and the gated
//! HTML → Markdown pipeline (spec FR-1, FR-7).

use htmd::{HtmlToMarkdown, Node};
use markup5ever_rcdom::NodeData;
use std::rc::Rc;

use crate::options_model::{CmarkFlags, HtmdOptions};

/// The FR-7 client-side processing ceiling, in UTF-8 bytes. The single
/// shared definition: the size gate here, the engine's halt path, and
/// the UI's banner transitions all cite it.
pub const MAX_INPUT_BYTES: usize = 100_000;

/// The FR-7 maximum element nesting depth.
const MAX_ELEMENT_DEPTH: usize = 512;

/// Render Markdown to HTML with `pulldown-cmark` under the sidebar's flags.
pub fn convert_md_to_html(md: &str, cmark: &CmarkFlags) -> String {
    let parser = pulldown_cmark::Parser::new_ext(md, cmark.to_pulldown());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

/// Why a gated HTML → Markdown conversion halted (FR-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blocked {
    /// The input exceeds the 100,000-byte ceiling and was never parsed.
    TooLarge,
    /// The parsed tree nests elements deeper than 512 levels.
    TooDeep,
}

/// Convert HTML to Markdown, enforcing the FR-7 limits: the size gate runs
/// first so an oversized input is never parsed, and the parsed tree's
/// element nesting is measured before the same tree is converted.
pub fn convert_html_to_md(
    html: &str,
    opts: &HtmdOptions,
    skip: &[String],
) -> Result<String, Blocked> {
    if html.len() > MAX_INPUT_BYTES {
        return Err(Blocked::TooLarge);
    }
    let mut builder = HtmlToMarkdown::builder().options(opts.to_htmd_options());
    if !skip.is_empty() {
        let tags: Vec<&str> = skip.iter().map(String::as_str).collect();
        builder = builder.skip_tags(tags);
    }
    let converter = builder.build();
    // The io::Result is future-proofing on htmd's side: html5ever reports
    // parse errors without failing, so a &str input always yields a tree.
    let tree = converter.html_to_tree(html).expect("infallible parse");
    if max_element_depth(&tree) > MAX_ELEMENT_DEPTH {
        return Err(Blocked::TooDeep);
    }
    Ok(converter.tree_to_markdown(&tree))
}

/// Maximum element depth per FR-7: children of `<head>`/`<body>` sit at
/// depth 1; the document node and the `<html>` scaffold do not deepen.
/// Iterative (explicit stack) so 50,000-deep trees cannot overflow the
/// wasm call stack.
fn max_element_depth(root: &Rc<Node>) -> usize {
    let mut max_depth = 0;
    let mut stack: Vec<(Rc<Node>, usize)> = vec![(root.clone(), 0)];
    while let Some((node, depth)) = stack.pop() {
        let child_depth = match &node.data {
            NodeData::Element { name, .. } => {
                if name.local.as_ref() == "html" {
                    depth
                } else {
                    max_depth = max_depth.max(depth);
                    depth + 1
                }
            }
            _ => depth,
        };
        for child in node.children.borrow().iter() {
            stack.push((child.clone(), child_depth));
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use htmd::options::TranslationMode;

    fn defaults() -> HtmdOptions {
        HtmdOptions::default()
    }

    fn sized_paragraph(bytes: usize) -> String {
        assert!(bytes >= 7);
        format!("<p>{}</p>", "a".repeat(bytes - 7))
    }

    fn nested_divs(count: usize) -> String {
        "<div>".repeat(count)
    }

    #[test]
    fn hundred_thousand_byte_input_converts() {
        let input = sized_paragraph(100_000);
        assert_eq!(input.len(), 100_000);
        let output = convert_html_to_md(&input, &defaults(), &[]).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn hundred_thousand_and_one_bytes_halts_too_large() {
        let input = sized_paragraph(100_001);
        assert_eq!(input.len(), 100_001);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::TooLarge)
        );
    }

    #[test]
    fn oversized_deep_input_still_halts_too_large() {
        let mut input = nested_divs(600);
        input.push_str(&"a".repeat(100_002 - input.len()));
        assert_eq!(input.len(), 100_002);
        assert_eq!(input.matches("<div>").count(), 600);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::TooLarge)
        );
    }

    #[test]
    fn five_hundred_twelve_deep_input_converts() {
        let input = nested_divs(512);
        assert_eq!(input.matches("<div>").count(), 512);
        assert!(convert_html_to_md(&input, &defaults(), &[]).is_ok());
    }

    #[test]
    fn five_hundred_thirteen_deep_input_is_too_deep() {
        let input = nested_divs(513);
        assert_eq!(input.matches("<div>").count(), 513);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::TooDeep)
        );
    }

    /// §8.6:275's stack-heavy shapes mixed — blockquote, list, and
    /// table nesting in one tree, all elements that genuinely nest.
    /// Each `<blockquote><ul><li><table><tr><td>` cycle is 7 parsed
    /// levels (html5ever implies the `<tbody>`), so 86 cycles reach
    /// depth 602: measured exactly by the depth walk, rejected by the
    /// 512 gate.
    #[test]
    fn six_hundred_deep_mixed_shapes_measure_and_reject() {
        let input = "<blockquote><ul><li><table><tr><td>".repeat(86);
        assert!(input.len() < MAX_INPUT_BYTES);
        let tree = HtmlToMarkdown::builder()
            .build()
            .html_to_tree(&input)
            .expect("infallible parse");
        assert_eq!(max_element_depth(&tree), 602);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::TooDeep)
        );
    }

    #[test]
    fn malformed_html_converts_without_panic() {
        let output = convert_html_to_md("<div><p>unclosed <b>text", &defaults(), &[]);
        assert!(output.is_ok());
    }

    #[test]
    fn empty_input_converts_to_empty_string() {
        assert_eq!(convert_html_to_md("", &defaults(), &[]), Ok(String::new()));
    }

    #[test]
    fn skip_tags_are_forwarded() {
        let skipped = vec!["img".to_string()];
        assert_eq!(
            convert_html_to_md("<img src=x>", &defaults(), &skipped),
            Ok(String::new())
        );
        let unskipped = convert_html_to_md("<img src=x>", &defaults(), &[]).unwrap();
        assert!(!unskipped.is_empty());
    }

    #[test]
    fn faithful_mode_emits_literal_br() {
        let mut opts = defaults();
        opts.translation_mode = TranslationMode::Faithful;
        let faithful = convert_html_to_md("a<br>b", &opts, &[]).unwrap();
        assert!(faithful.contains("<br>"));
        let pure = convert_html_to_md("a<br>b", &defaults(), &[]).unwrap();
        assert!(!pure.contains("<br>"));
    }

    #[ignore = "slow (~1 min): proves the depth walk survives 50,000 levels; run with `cargo test -- --ignored`"]
    #[test]
    fn depth_walk_survives_fifty_thousand_levels() {
        // 150,000 bytes of input is unreachable through convert_html_to_md's
        // 100,000-byte gate; the walk is exercised directly. Unclosed divs
        // nest (a div may contain a div), so html5ever produces 50,000
        // levels, and the iterative walk must not overflow the stack.
        let html = "<div>".repeat(50_000);
        let tree = HtmlToMarkdown::builder()
            .build()
            .html_to_tree(&html)
            .expect("infallible parse");
        assert_eq!(max_element_depth(&tree), 50_000);
    }

    #[test]
    fn md_to_html_renders_headings() {
        assert_eq!(
            convert_md_to_html("# Hi", &CmarkFlags::default()),
            "<h1>Hi</h1>\n"
        );
    }

    #[test]
    fn md_to_html_forwards_cmark_flags() {
        let table = "| a | b |\n| - | - |\n| 1 | 2 |";
        let flags = CmarkFlags {
            tables: true,
            ..Default::default()
        };
        assert!(convert_md_to_html(table, &flags).contains("<table>"));
        assert!(!convert_md_to_html(table, &CmarkFlags::default()).contains("<table>"));
    }
}
