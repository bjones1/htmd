//! Bidirectional conversion: pulldown-cmark rendering and the gated
//! HTML → Markdown pipeline (spec FR-1, FR-7).

use html5ever::driver::{ParseOpts, Parser};
use html5ever::parse_document;
use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::tree_builder::TreeBuilderOpts;
use htmd::{HtmlToMarkdown, Node};
use markup5ever_rcdom::{NodeData, RcDom};
use pulldown_cmark::{Event, Tag, TagEnd};
use std::rc::Rc;

use crate::options_model::{CmarkFlags, HtmdOptions};

/// How much input the depth-limited parse feeds between depth checks. At
/// ≥ 4 bytes per level, one chunk can deepen the tree by at most ~1,000
/// levels past the limit before the parse stops.
const PARSE_CHUNK_BYTES: usize = 4_096;

/// The FR-7 client-side processing ceiling, in UTF-8 bytes. The single
/// shared definition: the size gate here, the engine's halt path, and
/// the UI's banner transitions all cite it.
pub const MAX_INPUT_BYTES: usize = 100_000;

/// The FR-7 maximum element nesting depth.
const MAX_ELEMENT_DEPTH: usize = 512;

/// §6 adversarial bounds: any generated output (Markdown or HTML) over this
/// many UTF-8 bytes is cleared instead of displayed.
pub const MAX_DISPLAY_OUTPUT_BYTES: usize = 2_000_000;

/// Render Markdown to HTML with `pulldown-cmark` under the sidebar's flags.
pub fn convert_md_to_html(md: &str, cmark: &CmarkFlags) -> String {
    let parser = pulldown_cmark::Parser::new_ext(md, cmark.to_pulldown());
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    html
}

/// Markdown → HTML mode's gated render (FR-7, §6). Nesting is counted on
/// the event stream as it renders, and the render stops at the first
/// event past 512 levels, so 50,000 `>` never become 50,000 nested
/// blockquotes. Only tags that render as an element count, so this early
/// check never exceeds the rendered tree's depth. The exact rule is the
/// preview's depth-limited parse, which also sees raw HTML.
pub fn convert_md_to_html_gated(md: &str, cmark: &CmarkFlags) -> Result<String, Blocked> {
    if md.len() > MAX_INPUT_BYTES {
        return Err(Blocked::TooLarge);
    }
    let mut depth = 0usize;
    let mut too_deep = false;
    let events =
        pulldown_cmark::Parser::new_ext(md, cmark.to_pulldown()).take_while(|event| {
            match event {
                Event::Start(Tag::HtmlBlock | Tag::MetadataBlock(_))
                | Event::End(TagEnd::HtmlBlock | TagEnd::MetadataBlock(_)) => {}
                Event::Start(_) => depth += 1,
                Event::End(_) => depth = depth.saturating_sub(1),
                _ => {}
            }
            too_deep = depth > MAX_ELEMENT_DEPTH;
            !too_deep
        });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, events);
    if too_deep {
        return Err(Blocked::TooDeep);
    }
    within_display_cap(html)
}

/// Pass `output` through unless it exceeds the §6 display cap.
pub fn within_display_cap(output: String) -> Result<String, Blocked> {
    if output.len() > MAX_DISPLAY_OUTPUT_BYTES {
        Err(Blocked::OutputTooLarge)
    } else {
        Ok(output)
    }
}

/// Why a gated conversion halted (FR-7, §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blocked {
    /// The input exceeds the 100,000-byte ceiling and was never parsed.
    TooLarge,
    /// The input nests elements deeper than 512 levels.
    TooDeep,
    /// The output exceeds, or a table's padded size is estimated to exceed,
    /// the 2,000,000-byte display cap.
    OutputTooLarge,
    /// FR-7:150's future-proof `Err` path for a fallible future `htmd`.
    /// Unreachable today: the demo's own parse cannot fail, and
    /// `tree_to_markdown` returns a plain `String`.
    #[allow(dead_code)]
    ParseFailed,
}

/// Which `html5ever` entry point produced a tree, which fixes where FR-7's
/// depth 1 sits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedAs {
    /// `parse_document`: children of `<head>`/`<body>` are depth 1.
    Document,
    /// `parse_fragment` (`<body>` context): there is no `<body>` element,
    /// so the top-level elements are depth 1.
    Fragment,
}

/// Feed `html` to `parser` in [`PARSE_CHUNK_BYTES`] chunks and stop as soon
/// as the partial tree nests deeper than 512 levels (FR-7). `html5ever`'s
/// tree builder scans the stack of open elements on every block start tag,
/// so a whole parse is quadratic in nesting depth (~16 s at native release
/// for 50,000 levels). Checking between chunks bounds that cost by the
/// depth a single chunk can add.
pub fn parse_within_depth_limit(
    mut parser: Parser<RcDom>,
    html: &str,
    parsed_as: ParsedAs,
) -> Result<RcDom, Blocked> {
    let exceeds_limit = |root: &Rc<Node>| {
        let top_level_offset = match parsed_as {
            ParsedAs::Document => 0,
            ParsedAs::Fragment => 1,
        };
        max_element_depth(root) + top_level_offset > MAX_ELEMENT_DEPTH
    };
    let mut start = 0;
    while start < html.len() {
        let mut end = (start + PARSE_CHUNK_BYTES).min(html.len());
        while !html.is_char_boundary(end) {
            end += 1;
        }
        parser.process(StrTendril::from_slice(&html[start..end]));
        if exceeds_limit(&parser.tokenizer.sink.sink.document) {
            return Err(Blocked::TooDeep);
        }
        start = end;
    }
    let dom = parser.finish();
    if exceeds_limit(&dom.document) {
        return Err(Blocked::TooDeep);
    }
    Ok(dom)
}

/// Convert HTML to Markdown, enforcing the FR-7 limits and the §6 display
/// cap. The size gate runs first so an oversized input is never parsed.
/// The parse stops early past 512 levels, and the finished tree's padded
/// table size is estimated before that same tree is converted.
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
    // The same parse as `HtmlToMarkdown::html_to_tree` (htmd 0.5.5, whose
    // `scripting_enabled` defaults to true), fed in chunks so FR-7's depth
    // check can stop it early; `html_to_tree` only parses in one shot.
    let parser = parse_document(
        RcDom::default(),
        ParseOpts {
            tree_builder: TreeBuilderOpts {
                scripting_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
    );
    let tree = parse_within_depth_limit(parser, html, ParsedAs::Document)?.document;
    if estimated_table_output_bytes(&tree) > MAX_DISPLAY_OUTPUT_BYTES {
        return Err(Blocked::OutputTooLarge);
    }
    within_display_cap(converter.tree_to_markdown(&tree))
}

/// §6's pre-conversion estimate of the Markdown that htmd's pipe tables can
/// emit. htmd pads every cell to its column's widest cell and every row to
/// the widest row (htmd's `src/element_handler/table.rs`). A table's output therefore
/// grows with rows × the sum of column widths, not with its input size: a
/// 100 kB table can produce over 100 MB, and the post-conversion cap
/// would only see that string after it had been built. Each cell's width
/// is approximated by the serialized size of its contents.
fn estimated_table_output_bytes(root: &Rc<Node>) -> usize {
    let mut total = 0usize;
    let mut stack = vec![root.clone()];
    while let Some(node) = stack.pop() {
        if is_element(&node, "table") {
            total = total.saturating_add(padded_table_bytes(&node));
        }
        stack.extend(node.children.borrow().iter().cloned());
    }
    total
}

/// One table's padded size: `(rows + 2) × Σ (width + 3)`. htmd frames each
/// cell as ` cell |`, gives every column at least one `-`, and may add a
/// header row and always adds a delimiter row.
fn padded_table_bytes(table: &Rc<Node>) -> usize {
    let mut column_widths: Vec<usize> = Vec::new();
    let mut rows = 0usize;
    for row in table_rows(table) {
        rows += 1;
        let cells = row.children.borrow();
        let cells = cells
            .iter()
            .filter(|cell| is_element(cell, "td") || is_element(cell, "th"));
        for (index, cell) in cells.enumerate() {
            let width = serialized_content_size(cell).max(1);
            match column_widths.get_mut(index) {
                Some(widest) => *widest = (*widest).max(width),
                None => column_widths.push(width),
            }
        }
    }
    let row_bytes = column_widths
        .iter()
        .fold(2usize, |sum, width| sum.saturating_add(width.saturating_add(3)));
    row_bytes.saturating_mul(rows.saturating_add(2))
}

/// The `<tr>` children of `table` and of its own `<thead>`/`<tbody>`/`<tfoot>`
/// sections; rows of nested tables belong to those tables.
fn table_rows(table: &Rc<Node>) -> Vec<Rc<Node>> {
    let mut rows = Vec::new();
    for child in table.children.borrow().iter() {
        if is_element(child, "tr") {
            rows.push(child.clone());
        } else if ["thead", "tbody", "tfoot"]
            .iter()
            .any(|section| is_element(child, section))
        {
            rows.extend(
                child
                    .children
                    .borrow()
                    .iter()
                    .filter(|row| is_element(row, "tr"))
                    .cloned(),
            );
        }
    }
    rows
}

/// Approximate serialized byte size of `node`'s descendants: text bytes,
/// plus each element's start and end tags with their attributes.
fn serialized_content_size(node: &Rc<Node>) -> usize {
    let mut size = 0usize;
    let mut stack: Vec<Rc<Node>> = node.children.borrow().iter().cloned().collect();
    while let Some(descendant) = stack.pop() {
        let own = match &descendant.data {
            NodeData::Text { contents } => contents.borrow().len(),
            NodeData::Element { name, attrs, .. } => attrs
                .borrow()
                .iter()
                .fold(2 * name.local.len() + 5, |sum, attr| {
                    sum.saturating_add(attr.name.local.len() + attr.value.len() + 4)
                }),
            _ => 0,
        };
        size = size.saturating_add(own);
        stack.extend(descendant.children.borrow().iter().cloned());
    }
    size
}

fn is_element(node: &Node, tag: &str) -> bool {
    matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == tag)
}

/// Maximum element depth per FR-7: children of `<head>`/`<body>` sit at
/// depth 1; the document node and the `<html>` scaffold do not deepen.
/// Iterative (explicit stack) so 50,000-deep trees cannot overflow the
/// wasm call stack.
///
/// A `<template>`'s contents count at the depth of its children. `RcDom`
/// keeps them in `template_contents`, outside `children`, but the tree
/// builder still holds them on its stack of open elements, so they cost
/// the same quadratic parse as any other nesting.
fn max_element_depth(root: &Rc<Node>) -> usize {
    let mut max_depth = 0;
    let mut stack: Vec<(Rc<Node>, usize)> = vec![(root.clone(), 0)];
    while let Some((node, depth)) = stack.pop() {
        let child_depth = match &node.data {
            NodeData::Element {
                name,
                template_contents,
                ..
            } => {
                let child_depth = if name.local.as_ref() == "html" {
                    depth
                } else {
                    max_depth = max_depth.max(depth);
                    depth + 1
                };
                if let Some(contents) = template_contents.borrow().as_ref() {
                    stack.push((contents.clone(), child_depth));
                }
                child_depth
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

    /// §8.6:275's flat shape: 600 unclosed `<p>` tags. html5ever
    /// auto-closes a `<p>` before the next, so these are siblings,
    /// not depth.
    #[test]
    fn six_hundred_unclosed_paragraphs_convert() {
        let input = "<p>".repeat(600);
        assert!(input.len() < MAX_INPUT_BYTES);
        assert!(convert_html_to_md(&input, &defaults(), &[]).is_ok());
    }

    /// §8.6:275's flat shape: 600 `<br>` siblings.
    #[test]
    fn six_hundred_br_siblings_convert() {
        let input = "<br>".repeat(600);
        assert!(input.len() < MAX_INPUT_BYTES);
        assert!(convert_html_to_md(&input, &defaults(), &[]).is_ok());
    }

    /// §8.6:275's 512-level stack of nesting-valid shapes, converted
    /// in both translation modes: 73 blockquote/ul/li/table/tr/td
    /// cycles (7 parsed levels each, the implied `<tbody>` counted)
    /// reach 511; a trailing `<blockquote>` tops the stack at exactly
    /// 512, the deepest level that must still convert.
    #[test]
    fn five_hundred_twelve_deep_stack_converts_in_both_modes() {
        let mut input = "<blockquote><ul><li><table><tr><td>".repeat(73);
        input.push_str("<blockquote>");
        assert!(input.len() < MAX_INPUT_BYTES);
        let tree = HtmlToMarkdown::builder()
            .build()
            .html_to_tree(&input)
            .expect("infallible parse");
        assert_eq!(max_element_depth(&tree), 512);
        assert!(convert_html_to_md(&input, &defaults(), &[]).is_ok());
        let mut faithful = defaults();
        faithful.translation_mode = TranslationMode::Faithful;
        assert!(convert_html_to_md(&input, &faithful, &[]).is_ok());
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

    /// A `<table>` whose header row holds `columns` cells of `width`
    /// characters, followed by `rows` copies of `row`.
    fn padded_table(columns: usize, width: usize, rows: usize, row: &str) -> String {
        let mut html = String::from("<table><tr>");
        for _ in 0..columns {
            html.push_str(&format!("<th>{}</th>", "w".repeat(width)));
        }
        html.push_str("</tr>");
        html.push_str(&row.repeat(rows));
        html.push_str("</table>");
        html
    }

    fn faithful() -> HtmdOptions {
        HtmdOptions {
            translation_mode: TranslationMode::Faithful,
            ..defaults()
        }
    }

    /// §6 — 50 header cells of 1,000 characters pad every empty body cell
    /// to 1,000 characters (~110× amplification), so a sub-100 kB table is
    /// rejected before htmd builds its multi-megabyte Markdown.
    #[test]
    fn padded_table_amplification_is_rejected_in_both_modes() {
        let row = format!("<tr>{}</tr>", "<td></td>".repeat(50));
        let input = padded_table(50, 1_000, 100, &row);
        assert!(input.len() <= MAX_INPUT_BYTES);
        for opts in [defaults(), faithful()] {
            assert_eq!(
                convert_html_to_md(&input, &opts, &[]),
                Err(Blocked::OutputTooLarge)
            );
        }
    }

    /// §6 — Pure mode also pads short rows out to the widest row, so
    /// one-cell rows under a wide 50-column header are rejected too.
    #[test]
    fn padded_short_rows_are_rejected() {
        let input = padded_table(50, 1_000, 2_500, "<tr><td></td></tr>");
        assert!(input.len() <= MAX_INPUT_BYTES);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::OutputTooLarge)
        );
    }

    /// The estimate never undercounts htmd's padding on plain-text cells,
    /// so the pre-conversion check cannot be the gate that lets a padded
    /// table through.
    #[test]
    fn table_estimate_covers_actual_padded_output() {
        let row = format!("<tr>{}</tr>", "<td>x</td>".repeat(20));
        let input = padded_table(20, 100, 200, &row);
        for opts in [defaults(), faithful()] {
            let markdown = convert_html_to_md(&input, &opts, &[]).unwrap();
            let tree = HtmlToMarkdown::builder()
                .build()
                .html_to_tree(&input)
                .expect("infallible parse");
            assert!(estimated_table_output_bytes(&tree) >= markdown.len());
        }
    }

    /// An ordinary 100 kB-scale table stays far under the estimate's cap.
    #[test]
    fn ordinary_table_converts() {
        let row = format!("<tr>{}</tr>", "<td>cell</td>".repeat(8));
        let input = padded_table(8, 6, 700, &row);
        assert!(input.len() <= MAX_INPUT_BYTES);
        assert!(convert_html_to_md(&input, &defaults(), &[]).is_ok());
    }

    /// FR-7 — Markdown nesting counts each element-producing start event:
    /// 511 `>` plus a paragraph is depth 512 and renders; one more `>`
    /// is rejected.
    #[test]
    fn markdown_nesting_limit_boundary() {
        let flags = CmarkFlags::default();
        let at_limit = format!("{} a", ">".repeat(511));
        assert!(convert_md_to_html_gated(&at_limit, &flags).is_ok());
        let over_limit = format!("{} a", ">".repeat(512));
        assert_eq!(
            convert_md_to_html_gated(&over_limit, &flags),
            Err(Blocked::TooDeep)
        );
    }

    /// FR-1/FR-7 — 50,000 `>` stops rendering at depth 513 instead of
    /// producing 50,000 nested blockquotes for the preview to re-parse.
    #[test]
    fn fifty_thousand_blockquotes_are_rejected() {
        assert_eq!(
            convert_md_to_html_gated(&">".repeat(50_000), &CmarkFlags::default()),
            Err(Blocked::TooDeep)
        );
    }

    /// FR-7 — 25,000 nested `<ul>` (exactly 100 kB) stop parsing within one
    /// chunk of passing 512 levels instead of paying the full quadratic
    /// parse.
    #[test]
    fn deep_html_input_is_rejected_during_the_parse() {
        let input = "<ul>".repeat(25_000);
        assert_eq!(input.len(), MAX_INPUT_BYTES);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::TooDeep)
        );
    }

    /// FR-7 — nesting inside a `<template>` counts. Its contents live
    /// outside `children` in `RcDom`, but they still cost the quadratic
    /// parse, so 100 kB of nested `<ul>` behind a `<template>` is rejected
    /// during the parse like the bare case. The template (a `<head>` child)
    /// is depth 1, so 511 levels inside it reach exactly 512.
    #[test]
    fn nesting_inside_a_template_counts() {
        let input = format!("<template>{}", "<ul>".repeat(24_997));
        assert!(input.len() <= MAX_INPUT_BYTES);
        assert_eq!(
            convert_html_to_md(&input, &defaults(), &[]),
            Err(Blocked::TooDeep)
        );
        let at_limit = format!("<template>{}", "<div>".repeat(511));
        assert!(convert_html_to_md(&at_limit, &defaults(), &[]).is_ok());
        let over_limit = format!("<template>{}", "<div>".repeat(512));
        assert_eq!(
            convert_html_to_md(&over_limit, &defaults(), &[]),
            Err(Blocked::TooDeep)
        );
    }

    /// The chunked parse builds the same tree as htmd's one-shot
    /// `html_to_tree` for input spanning several chunks, with CRLF line
    /// endings and multibyte text.
    #[test]
    fn chunked_parse_matches_htmd_one_shot_parse() {
        let input = "<p>é <b>x</b>\r\n<a href=\"y\">z</a></p>".repeat(400);
        assert!(input.len() > 3 * PARSE_CHUNK_BYTES);
        for opts in [defaults(), faithful()] {
            let expected = HtmlToMarkdown::builder()
                .options(opts.to_htmd_options())
                .build()
                .convert(&input)
                .expect("infallible parse");
            assert_eq!(convert_html_to_md(&input, &opts, &[]), Ok(expected));
        }
    }

    /// §6 — the gated Markdown render applies the 2 MB display cap.
    #[test]
    fn gated_markdown_render_applies_display_cap() {
        let mut md = "|".repeat(51);
        md.push('\n');
        md.push_str(&"|:-".repeat(50));
        md.push_str("|\n");
        md.push_str(&format!("{}\n", "|".repeat(51)).repeat(1_200));
        assert!(md.len() <= MAX_INPUT_BYTES);
        let flags = CmarkFlags {
            tables: true,
            ..Default::default()
        };
        assert_eq!(
            convert_md_to_html_gated(&md, &flags),
            Err(Blocked::OutputTooLarge)
        );
    }
}
