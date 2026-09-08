//! Whether a run of translated CommonMark, written at the start of a line,
//! would open an [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks).
//!
//! Content which meets a start condition dissolves the paragraph holding it, so
//! that `<p>` must be written as HTML instead; a setext heading is exposed the
//! same way and falls back to ATX. See the "Special case for paragraphs"
//! section of `unsupported_html.md`.

use crate::dom_walker::{is_block_element, is_type_1_element};

/// Whether `content`, placed at the start of a line, meets one of CommonMark's
/// seven [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks) start
/// conditions.
///
/// Only the first line is examined: `unsupported_html.md` requires every
/// handler to encode, collapse or drop a line ending it writes in an inline
/// context, so there is no later line for a block to open on.
pub(crate) fn opens_html_block(content: &str) -> bool {
    let Some(first) = content.lines().next() else {
        return false;
    };
    line_opens_html_block(first)
}

/// Whether the one `line` meets a start condition; see [`opens_html_block`].
fn line_opens_html_block(line: &str) -> bool {
    // A fourth space of indentation makes the line indented code instead.
    let indented = line.trim_start_matches(' ');
    if line.len() - indented.len() > 3 {
        return false;
    }
    let Some(rest) = indented.strip_prefix('<') else {
        return false;
    };

    // Conditions 2-5: a comment, a processing instruction, a declaration and a
    // CDATA section.
    if rest.starts_with("!--")
        || rest.starts_with('?')
        || rest.starts_with("![CDATA[")
        || rest
            .strip_prefix('!')
            .is_some_and(|after| after.starts_with(|ch: char| ch.is_ascii_alphabetic()))
    {
        return true;
    }

    let (is_closing, after_mark) = match rest.strip_prefix('/') {
        Some(after) => (true, after),
        None => (false, rest),
    };
    let Some(name_len) = tag_name_len(after_mark) else {
        return false;
    };
    let (name, after_name) = after_mark.split_at(name_len);
    // `dom_walker::BLOCK_ELEMENTS` is the type 1 list plus the type 6 one.
    let is_raw_text = matches_ignore_ascii_case(name, is_type_1_element);
    let is_type_6 = !is_raw_text && matches_ignore_ascii_case(name, is_block_element);

    // Condition 1: an opening raw text tag only.
    if !is_closing && is_raw_text && ends_tag_name(after_name, false) {
        return true;
    }

    // Condition 6: a block-level tag, opening or closing.
    if is_type_6 && ends_tag_name(after_name, true) {
        return true;
    }

    // Condition 7: a complete tag of any other name with nothing but whitespace
    // after it. The spec excludes the raw text tags from the *open* tag only,
    // so a lone `</pre>` opens a block here even though `<pre/>` does not.
    if is_raw_text && !is_closing {
        return false;
    }
    let Some(after_tag) = complete_tag_end(after_name, is_closing) else {
        return false;
    };
    after_tag.trim_matches([' ', '\t']).is_empty()
}

/// Whether the [tag name](https://spec.commonmark.org/0.31.2/#tag-name) ends
/// where `after_name` begins. A condition 6 tag may also be self-closing, which
/// `allow_self_closing` adds; a condition 1 tag may not.
fn ends_tag_name(after_name: &str, allow_self_closing: bool) -> bool {
    // CommonMark [whitespace](https://spec.commonmark.org/0.31.2/#whitespace-character)
    // is more than a space and a tab.
    after_name.is_empty()
        || after_name.starts_with([' ', '\t', '\u{0B}', '\u{0C}', '\r', '>'])
        || (allow_self_closing && after_name.starts_with("/>"))
}

/// Trims the spaces and tabs at the start of `text`, the only whitespace a tag
/// may hold between its name, its attributes and its `>`.
fn trim_tab_space(text: &str) -> &str {
    text.trim_start_matches([' ', '\t'])
}

/// Whether `predicate` holds for `name` read case-insensitively, which is how
/// CommonMark reads the tag name of every start condition.
fn matches_ignore_ascii_case(name: &str, predicate: fn(&str) -> bool) -> bool {
    if name.bytes().any(|byte| byte.is_ascii_uppercase()) {
        predicate(&name.to_ascii_lowercase())
    } else {
        predicate(name)
    }
}

/// The length of the [tag name](https://spec.commonmark.org/0.31.2/#tag-name)
/// at the start of `text`, or `None` when there is not one there.
fn tag_name_len(text: &str) -> Option<usize> {
    if !text.as_bytes().first()?.is_ascii_alphabetic() {
        return None;
    }
    Some(
        text.bytes()
            .take_while(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
            .count(),
    )
}

/// What follows the complete
/// [open](https://spec.commonmark.org/0.31.2/#open-tag) or
/// [closing tag](https://spec.commonmark.org/0.31.2/#closing-tag) whose name
/// ends at the start of `after_name`, or `None` when the tag is not complete.
fn complete_tag_end(after_name: &str, is_closing: bool) -> Option<&str> {
    let mut rest = after_name;
    if !is_closing {
        while let Some(after_attribute) = attribute_end(rest) {
            rest = after_attribute;
        }
    }
    let rest = trim_tab_space(rest);
    let rest = if is_closing {
        rest
    } else {
        rest.strip_prefix('/').unwrap_or(rest)
    };
    rest.strip_prefix('>')
}

/// What follows the one
/// [attribute](https://spec.commonmark.org/0.31.2/#attribute) at the start of
/// `text`, or `None` when there is not one there.
fn attribute_end(text: &str) -> Option<&str> {
    let rest = trim_tab_space(text.strip_prefix([' ', '\t'])?);
    let rest = &rest[attribute_name_len(rest)?..];
    let Some(after_equals) = trim_tab_space(rest).strip_prefix('=') else {
        return Some(rest);
    };
    attribute_value_end(trim_tab_space(after_equals))
}

/// The length of the
/// [attribute name](https://spec.commonmark.org/0.31.2/#attribute-name) at the
/// start of `text`, or `None` when there is not one there.
fn attribute_name_len(text: &str) -> Option<usize> {
    let first = *text.as_bytes().first()?;
    if !first.is_ascii_alphabetic() && first != b'_' && first != b':' {
        return None;
    }
    Some(
        text.bytes()
            .take_while(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b':' | b'-')
            })
            .count(),
    )
}

/// What follows the
/// [attribute value](https://spec.commonmark.org/0.31.2/#attribute-value) at
/// the start of `text`, or `None` when there is not one there.
fn attribute_value_end(text: &str) -> Option<&str> {
    if let Some(after_quote) = text.strip_prefix('\'') {
        return after_quote.split_once('\'').map(|(_, rest)| rest);
    }
    if let Some(after_quote) = text.strip_prefix('"') {
        return after_quote.split_once('"').map(|(_, rest)| rest);
    }
    let unquoted_len = text
        .bytes()
        .take_while(|byte| {
            !matches!(
                byte,
                b' ' | b'\t' | b'"' | b'\'' | b'=' | b'<' | b'>' | b'`'
            )
        })
        .count();
    (unquoted_len > 0).then(|| &text[unquoted_len..])
}

#[cfg(test)]
mod tests {
    use super::opens_html_block;

    #[test]
    fn matches_the_raw_text_and_block_level_conditions() {
        // Condition 1.
        assert!(opens_html_block("<pre>a"));
        assert!(opens_html_block(r#"<script src="s">a"#));
        assert!(opens_html_block("<textarea"));
        assert!(!opens_html_block("<pre/>"));
        assert!(!opens_html_block("<pre-x>a"));
        // Condition 6.
        assert!(opens_html_block("<div>a"));
        assert!(opens_html_block("</div>a"));
        assert!(opens_html_block("<hr/>"));
        assert!(opens_html_block("<P>a"));
        assert!(!opens_html_block("<divx>a"));
    }

    #[test]
    fn matches_the_comment_declaration_and_cdata_conditions() {
        assert!(opens_html_block("<!--a-->b"));
        assert!(opens_html_block("<?php ?>a"));
        assert!(opens_html_block("<!DOCTYPE html>"));
        assert!(opens_html_block("<![CDATA[a]]>b"));
        assert!(!opens_html_block("<!1>"));
    }

    /// Condition 7 needs a complete tag with nothing but whitespace after it,
    /// which separates a lone `<br>` from a run of them.
    #[test]
    fn matches_a_lone_complete_tag_only() {
        assert!(opens_html_block("<br>"));
        assert!(opens_html_block("<br>  "));
        assert!(opens_html_block("<br />"));
        assert!(opens_html_block("</em>"));
        assert!(opens_html_block(r#"<em foo="1" bar='2' baz=3>"#));
        // The raw text exclusion is on the open tag alone.
        assert!(opens_html_block("</pre>"));
        assert!(opens_html_block("</script>  "));
        assert!(!opens_html_block("</pre>a"));
        assert!(!opens_html_block("<br><br>"));
        assert!(!opens_html_block("<br>a"));
        assert!(!opens_html_block("<em foo=>"));
        assert!(!opens_html_block("<em"));
    }

    #[test]
    fn ignores_content_which_opens_no_block() {
        assert!(!opens_html_block(""));
        assert!(!opens_html_block("a<br>"));
        assert!(!opens_html_block("*a*"));
        assert!(opens_html_block("   <br>"));
        assert!(!opens_html_block("    <br>"));
    }

    /// See [`opens_html_block`].
    #[test]
    fn ignores_every_line_after_the_first() {
        assert!(!opens_html_block("a\n<div>b"));
        assert!(!opens_html_block("a\n<!--b-->c"));
        assert!(!opens_html_block("a\n<pre>b"));
        assert!(!opens_html_block("a\n<br>"));
        assert!(opens_html_block("<div>a\nb"));
    }
}
