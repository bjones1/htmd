use std::rc::Rc;

use markup5ever_rcdom::{Node, NodeData};

use crate::{
    Element,
    dom_walker::is_block_element,
    element_handler::{HandlerResult, Handlers, element_util::serialize_element},
    node_util::{get_node_tag_name, get_parent_node},
    options::TranslationMode,
    serialize_if_faithful,
    text_util::concat_strings,
};

pub(super) fn emphasis_handler(
    handlers: &dyn Handlers,
    element: Element,
    marker: &str,
) -> Option<HandlerResult> {
    serialize_if_faithful!(handlers, element, 0);
    let content = handlers.walk_children(element.node).content;
    if content.is_empty() {
        return None;
    }
    let is_pure = handlers.options().translation_mode == TranslationMode::Pure;

    // Note: this is whitespace, NOT document whitespace, per the
    // [Commonmark spec](https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis).
    let leading_len = leading_hoist(&content, is_pure);
    let (leading, rest) = content.split_at(leading_len);
    let trailing_len = trailing_hoist(rest, is_pure);
    let (rest, trailing) = rest.split_at(rest.len() - trailing_len);

    if rest.is_empty() {
        // Nothing is left for the markers to wrap, so none are written. A hard
        // break that moved out still has to be: it was the element's whole
        // content, and dropping it would lose the `<br>` it came from. Plain
        // whitespace goes, as it always has.
        return if leading.contains('\n') || trailing.contains('\n') {
            Some(concat_strings!(leading, trailing).into())
        } else {
            None
        };
    }

    // Markers that do not flank are literal text rather than emphasis, so the
    // element survives only as HTML. Pure mode has no such fallback; the hoist
    // above is what keeps a marker off a break there.
    if !is_pure && !markers_flank(element.node, rest, leading, trailing) {
        return Some(HandlerResult {
            content: serialize_element(handlers, &element),
            markdown_translated: false,
        });
    }

    // Both markers also have to land in the same paragraph, and a paragraph is
    // the only block that can carry them at all. Neither is something the
    // hoists can arrange: a blank line with content on both sides goes nowhere,
    // and every other block spells its meaning with the characters at a line's
    // edge, which is exactly where a marker would have to go. Pure mode writes
    // such an element block by block instead — the emphasis the source asked
    // for, spelled the only way Markdown has of spelling it.
    //
    // Only an element that really holds a block goes that way. Content that
    // merely reads like one is text, and the markers are what keep it text; see
    // [`holds_block_element`].
    if is_pure
        && holds_block_element(element.node)
        && (rest.lines().any(is_blank_line) || !is_paragraph(rest))
    {
        return Some(concat_strings!(leading, emphasize_paragraphs(rest, marker), trailing).into());
    }

    Some(concat_strings!(leading, marker, rest, marker, trailing).into())
}

/// How much of `content`'s start has to be written outside the opening marker.
///
/// Leading whitespace always moves out, since a marker against whitespace does
/// not open emphasis. A leading hard break moves too, but only in pure mode: the
/// break renders the same on either side of the markers, while in faithful mode
/// moving it would move the `<br>` it came from out of the element — there the
/// break instead makes the markers unwritable and the element is serialized.
///
/// A blank line is block separation rather than a break, so it moves out in both
/// modes.
///
/// A newline here means `\n` and nothing else: html5ever normalizes the source's
/// CR and CRLF away, so a `\r` reaching this is a literal character carried
/// through a `<pre>` verbatim, not a line ending, and treating it as one would
/// cut the hoist short.
fn leading_hoist(content: &str, is_pure: bool) -> usize {
    let whitespace_len = content.len() - content.trim_start().len();
    if !is_pure {
        let whitespace = &content[..whitespace_len];
        return match whitespace.find('\n') {
            Some(_) if whitespace.contains("\n\n") => whitespace_len,
            Some(break_start) => break_start,
            None => whitespace_len,
        };
    }
    // Take any run of hard breaks along with the whitespace. Only the backslash
    // spelling needs looking for: the two-space one is whitespace already.
    let mut len = whitespace_len;
    while content[len..].starts_with('\\') && content[len + 1..].starts_with('\n') {
        let after = &content[len + 1..];
        len += 1 + (after.len() - after.trim_start().len());
    }
    len
}

/// How much of `content`'s end has to be written outside the closing marker,
/// the mirror of [`leading_hoist`].
fn trailing_hoist(content: &str, is_pure: bool) -> usize {
    let trimmed_len = content.trim_end().len();
    if !is_pure {
        let whitespace = &content[trimmed_len..];
        return match whitespace.rfind('\n') {
            Some(_) if whitespace.contains("\n\n") => whitespace.len(),
            // Only what follows the break's newline may move out, which for a
            // break the content ends on is nothing at all.
            Some(break_end) => whitespace.len() - break_end - 1,
            None => whitespace.len(),
        };
    }
    let mut end = trimmed_len;
    while content[end..].starts_with('\n') && ends_in_break_marker(&content[..end]) {
        end = content[..end - 1].trim_end().len();
    }
    content.len() - end
}

/// Whether `node` holds a block element anywhere beneath it, which is what says
/// the content came back as blocks htmd wrote rather than as text that only
/// reads like one.
///
/// The two are indistinguishable by the time the content is a string, and they
/// want opposite treatment. htmd does not escape every character that can open a
/// block — a bare `#` or a `---` with nothing after it reaches the output as
/// itself — so `<em>#</em>` writes a `#` that is text, and writing markers
/// around it is the whole of what keeps it text. `<em><h1>a</h1></em>` writes a
/// `# a` that is a heading, and writing markers around *that* destroys it.
///
/// Only the source says which is which, so the source is what gets asked. (The
/// escaper leaving those characters alone is a hole of its own; until it closes,
/// the markers are the shield, and this is what keeps them on.)
fn holds_block_element(node: &Rc<Node>) -> bool {
    node.children.borrow().iter().any(|child| {
        get_node_tag_name(child).is_some_and(is_block_element) || holds_block_element(child)
    })
}

/// Writes a pair of `marker`s around each paragraph of `content` rather than one
/// pair around the whole of it.
///
/// A blank line ends a paragraph, and a delimiter run is read within one, so a
/// pair of markers straddling a blank line is not emphasis at all: both markers
/// are literal text, one stranded at either end of the content. An inline
/// element holding a block is what puts a blank line there — the block writes
/// one — and while the hoists can move a blank line off either *edge* of the
/// content, an interior one has content on both sides and can go nowhere.
///
/// Emphasizing each paragraph in turn says what the element said, in the only
/// way Markdown can say it. It is also what the same document already produces
/// when the HTML parser splits the element for us: a `<div>` inside a `<p>`
/// closes the paragraph, so html5ever reconstructs the `<em>` around each piece
/// and every piece is emphasized on its own.
///
/// Only a paragraph can carry the markers, which is the other reason to come
/// here — content that is a single block has no blank line to split on and
/// still cannot be wrapped. Everything htmd writes that is not a paragraph
/// spells its own meaning with the characters at a line's edge: a fence, an
/// indent, a bullet, the delimiter row under a table's header. A marker written
/// against those lands inside the block and changes what it says rather than
/// emphasizing it, so those blocks go out unmarked. That loses the emphasis on
/// them alone, and keeps the block.
fn emphasize_paragraphs(content: &str, marker: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut paragraph_start = 0;
    let mut offset = 0;
    // The delimiter of the code fence currently open, if any. A blank line
    // inside a fence is part of the code and ends no paragraph.
    let mut fence: Option<&str> = None;
    for line in content.split_inclusive('\n') {
        let line_start = offset;
        offset += line.len();
        let text = line_text(line);
        match fence {
            Some(open) => {
                if closes_fence(text, open) {
                    fence = None;
                }
            }
            None if is_blank_line(text) => {
                push_paragraph(&mut result, &content[paragraph_start..line_start], marker);
                result.push_str(line);
                paragraph_start = offset;
            }
            None => fence = fence_delimiter(text),
        }
    }
    push_paragraph(&mut result, &content[paragraph_start..], marker);
    result
}

/// Writes `paragraph` into `result`, with `marker` around it if it can carry a
/// pair.
///
/// The hoists run again here for the same reason they run over the element's own
/// content: these edges are a paragraph's edges too, and a marker against
/// whitespace — or against the backslash of a hard break, which is what the
/// blank line ahead of a paragraph may leave behind — does not open or close
/// emphasis. See [`leading_hoist`] and [`trailing_hoist`].
fn push_paragraph(result: &mut String, paragraph: &str, marker: &str) {
    if !is_paragraph(paragraph) {
        result.push_str(paragraph);
        return;
    }
    let (leading, rest) = paragraph.split_at(leading_hoist(paragraph, true));
    let trailing_len = trailing_hoist(rest, true);
    let (rest, trailing) = rest.split_at(rest.len() - trailing_len);
    if rest.is_empty() || fuses_with_marker(rest, marker) {
        result.push_str(paragraph);
        return;
    }
    result.push_str(leading);
    result.push_str(marker);
    result.push_str(rest);
    result.push_str(marker);
    result.push_str(trailing);
}

/// Whether a marker written against `rest` would join a delimiter run already
/// there rather than open one of its own.
///
/// What a marker means is the length of the run it belongs to, and CommonMark
/// reads the whole run before deciding: one `*` is emphasis, two are strong. An
/// inline child sitting at the paragraph's edge has already written markers of
/// its own there, so a marker put beside them lands in the same run and both
/// change meaning — a `*` against the `*b*` of a nested `<em>` reads as the `**`
/// of strong, and where the run lengths no longer pair the surplus is left as
/// literal asterisks in the text.
///
/// Nothing can be hoisted out of the way, since a delimiter run is content
/// rather than whitespace. So the paragraph goes out unmarked: it loses the
/// emphasis this element would have added, which is the lesser harm next to
/// breaking the emphasis already written inside it.
fn fuses_with_marker(rest: &str, marker: &str) -> bool {
    let symbol = marker.chars().next().expect("marker is not empty");
    // An escaped marker is literal text and no run at all. Only the trailing
    // edge can hide one: a leading `\*` opens with the backslash, not the `*`.
    let ends_in_run = rest.ends_with(symbol) && !ends_in_break_marker(&rest[..rest.len() - 1]);
    rest.starts_with(symbol) || ends_in_run
}

/// Whether every line of `paragraph` is ordinary text, so that the whole of it
/// is the one kind of block a pair of emphasis markers can wrap.
///
/// A GFM table is the one block here that no single line of it gives away: its
/// rows are ordinary text but for the pipes, and only the delimiter row under
/// the header says that the pipes are a table's rather than a paragraph's. So
/// that row is what the whole chunk is tested for — every line but the first,
/// since a delimiter row is only one when a header row precedes it, and a `|--|`
/// with nothing above it is a paragraph like any other.
fn is_paragraph(paragraph: &str) -> bool {
    paragraph
        .lines()
        .enumerate()
        .all(|(index, line)| is_paragraph_line(line, index == 0))
        && !paragraph.lines().skip(1).any(is_table_delimiter_row)
}

/// Whether `line` is the [delimiter row](https://github.github.com/gfm/#tables-extension-)
/// that turns the rows around it into a table: pipes, dashes, and the colons
/// that set a column's alignment, with at least one dash among them.
fn is_table_delimiter_row(line: &str) -> bool {
    let text = line.trim_matches([' ', '\t']);
    text.contains('-')
        && text.starts_with('|')
        && text
            .chars()
            .all(|ch| matches!(ch, '|' | '-' | ':' | ' ' | '\t'))
}

/// Whether `line` is ordinary text rather than the opening of some other block
/// or the underline of a Setext heading.
///
/// `line` is Markdown htmd has already written, so the question is really which
/// spellings htmd *emits* for a block. Those are exactly the ones its escaper
/// protects — [`is_markdown_atx_heading`](crate::text_util::is_markdown_atx_heading)
/// wants a space after the `#` run, and the ordered-list escape wants a `.` —
/// which is what makes the two sides line up: where htmd would escape the text,
/// an unescaped marker here really is the block.
///
/// Everything the escaper leaves alone it leaves alone because the text was
/// never a block: a bare `#`, a `1)`, a lone `-`, a pipe with no delimiter row
/// under it. Those must read as text, and not merely to save the emphasis on
/// them — dropping the markers around text that CommonMark *would* read as a
/// block turns it into that block and loses the text itself. `is_first` is what
/// tells a Setext underline from a line of dashes that has nothing above it.
fn is_paragraph_line(line: &str, is_first: bool) -> bool {
    let text = line.trim_start_matches(' ');
    // Four columns of indent open an indented code block, and a tab is four.
    if line.len() - text.len() >= 4 || text.starts_with('\t') {
        return false;
    }
    let Some(first) = text.chars().next() else {
        return true;
    };
    // A block quote.
    if first == '>' {
        return false;
    }
    // An ATX heading is a run of one to six `#` and then a space — the escaper's
    // own rule, so a run without one is text it judged safe to leave alone.
    if first == '#' {
        let run = text.len() - text.trim_start_matches('#').len();
        if run <= 6 && text[run..].starts_with(' ') {
            return false;
        }
    }
    if first == '<' && opens_html_block(text) {
        return false;
    }
    // A run of one symbol and nothing else is a thematic break, or — for `-` and
    // `=` — a Setext underline, which a closing marker after it would undo. htmd
    // escapes a `*`, `_`, or `=` run it means as text and writes `* * *` for a
    // thematic break, so an unescaped run of those really is the block. A `-` run
    // it neither escapes nor writes, which leaves it a block only where an
    // underline can go: beneath a line of the same paragraph.
    if matches!(first, '*' | '-' | '_' | '=')
        && text
            .chars()
            .all(|ch| ch == first || ch == ' ' || ch == '\t')
        && !(is_first && matches!(first, '-' | '='))
    {
        return false;
    }
    // A bullet list marker is the bullet and the whitespace after it; `*text`
    // is emphasis, and only the space tells the two apart.
    if matches!(first, '*' | '-' | '+') && text[1..].starts_with([' ', '\t']) {
        return false;
    }
    fence_delimiter(text).is_none() && !opens_ordered_list(text)
}

/// Whether the `<` that `text` opens with begins an HTML block rather than
/// something a paragraph can hold.
///
/// A tag names its element, so htmd's own notion of which elements are blocks
/// answers this directly. An inline tag is not a block, and neither is an
/// autolink — `prefer_autolinks` writes one as `<https://…>`, whose `https` is
/// no element at all — so both stay in the paragraph they belong to.
fn opens_html_block(text: &str) -> bool {
    let after = &text[1..];
    // A comment, a CDATA section, a declaration, or a processing instruction.
    if after.starts_with(['!', '?']) {
        return true;
    }
    let name = after.strip_prefix('/').unwrap_or(after);
    let end = name
        .find(|ch: char| !ch.is_ascii_alphanumeric())
        .unwrap_or(name.len());
    is_block_element(&name[..end].to_ascii_lowercase())
}

/// Whether `text` opens an ordered list: [no more than nine](https://spec.commonmark.org/0.31.2/#ordered-list-marker)
/// digits, then a `.`, then the whitespace that separates the marker from the
/// item. A longer run of digits is ordinary text.
///
/// CommonMark accepts a `)` here too, but htmd writes `.` and its escaper guards
/// only `.`, so an unescaped `1) x` is text that has to stay text.
fn opens_ordered_list(text: &str) -> bool {
    let digits = text.len()
        - text
            .trim_start_matches(|ch: char| ch.is_ascii_digit())
            .len();
    (1..=9).contains(&digits)
        && text[digits..].starts_with('.')
        && text[digits + 1..].starts_with([' ', '\t'])
}

/// The delimiter run of the code fence `line` opens, if it opens one.
///
/// A backtick fence's [info string may hold no backtick](https://spec.commonmark.org/0.31.2/#info-string)
/// of its own, and that rule is what keeps a code span from being read as a
/// fence. htmd writes a span whose content touches a backtick with a longer run
/// and padding spaces — ``​`` ``x `` ``​`` for `` ``x `` — which opens a line with
/// three backticks and carries the rest of the span behind them. Reading that
/// as a fence opens a block that either never closes, swallowing everything
/// after it, or closes on the next real fence and inverts the parity, which
/// writes emphasis markers into somebody's code.
fn fence_delimiter(line: &str) -> Option<&str> {
    let text = line.trim_start_matches(' ');
    if line.len() - text.len() >= 4 {
        return None;
    }
    let symbol = text.chars().next().filter(|ch| matches!(ch, '`' | '~'))?;
    let run = text.len() - text.trim_start_matches(symbol).len();
    if run < 3 || (symbol == '`' && text[run..].contains('`')) {
        return None;
    }
    Some(&text[..run])
}

/// Whether `line` is the closing fence of the block `open` opened: the same
/// symbol, run at least as long, and nothing after it but whitespace.
fn closes_fence(line: &str, open: &str) -> bool {
    fence_delimiter(line).is_some_and(|delimiter| {
        delimiter.starts_with(&open[..1])
            && delimiter.len() >= open.len()
            && line
                .trim_start_matches(' ')
                .trim_start_matches(&delimiter[..1])
                .trim_matches([' ', '\t'])
                .is_empty()
    })
}

/// What `line` — one piece of a [`str::split_inclusive`] on `\n` — holds without
/// the line ending that terminates it.
fn line_text(line: &str) -> &str {
    let text = line.strip_suffix('\n').unwrap_or(line);
    text.strip_suffix('\r').unwrap_or(text)
}

/// Whether `line` is [blank](https://spec.commonmark.org/0.31.2/#blank-lines):
/// empty, or holding nothing but spaces and tabs. A blank line ends the block
/// above it, which is what makes it a paragraph boundary here.
fn is_blank_line(line: &str) -> bool {
    line.trim_matches([' ', '\t']).is_empty()
}

/// Whether the backslash `text` ends on is a hard break's own marker rather than
/// half of an escaped literal backslash.
///
/// Both look alike from the newline that follows, so only the run's length tells
/// them apart: escaping doubles every literal, and a break's own marker is the
/// one extra that leaves the run odd. Getting this wrong is not cosmetic —
/// hoisting one backslash out of an escaped pair leaves the other escaping the
/// closing marker, losing the emphasis entirely.
pub(super) fn ends_in_break_marker(text: &str) -> bool {
    // Backslash is ASCII, so counting bytes cannot split a character.
    text.bytes().rev().take_while(|&byte| byte == b'\\').count() % 2 == 1
}

/// Whether the markers this element would be written with open and close
/// emphasis where they land.
///
/// A delimiter run opens emphasis only if it is
/// [left-flanking](https://spec.commonmark.org/0.31.2/#left-flanking-delimiter-run):
/// not followed by whitespace, and either not followed by punctuation or else
/// itself preceded by whitespace or punctuation — mirror-image for closing.
/// `rest` is what the markers would wrap, so its edges are what they sit against
/// on the inside; a non-empty `leading` or `trailing` is whitespace that moved
/// out, so the markers sit against whitespace on the outside.
///
/// Every spelling of a `<br>` puts punctuation at the content's edge — `<` and
/// `>` for the raw tag, `\` for the backslash break, whitespace for the
/// two-space one — which is why a break is what usually brings this test down.
fn markers_flank(node: &Rc<Node>, rest: &str, leading: &str, trailing: &str) -> bool {
    let first = rest.chars().next().expect("rest is not empty");
    let last = rest.chars().last().expect("rest is not empty");
    let opens = !first.is_whitespace()
        && (first.is_alphanumeric()
            || !leading.is_empty()
            || flanking_allows(preceding_char(node)));
    let closes = !last.is_whitespace()
        && (last.is_alphanumeric()
            || !trailing.is_empty()
            || flanking_allows(following_char(node)));
    opens && closes
}

/// Whether a marker against punctuation may still flank, given the character on
/// its other side. Anything but an alphanumeric will do — whitespace and
/// punctuation both allow it, and so does the line or block edge a `None`
/// stands for.
fn flanking_allows(adjacent: Option<char>) -> bool {
    !adjacent.is_some_and(char::is_alphanumeric)
}

/// What a walk out from an element toward one end of its line ran into.
enum Adjacent {
    /// The character the output holds there.
    Char(char),
    /// A line or block edge, which flanking counts as whitespace.
    Edge,
    /// Nothing that reaches the output; keep looking further out.
    Nothing,
}

/// The character the output holds ahead of this element, or `None` where the
/// element opens a line or a block.
fn preceding_char(node: &Rc<Node>) -> Option<char> {
    adjacent_char(node, Direction::Back)
}

/// The character the output holds after this element, or `None` where the
/// element ends a line or a block.
fn following_char(node: &Rc<Node>) -> Option<char> {
    adjacent_char(node, Direction::Forward)
}

/// Which way out of an element to look for the character next to it.
#[derive(Clone, Copy)]
enum Direction {
    Back,
    Forward,
}

/// Walks out from `node` through the inline elements around it, scanning each
/// element's siblings on the given side, until one of them reaches the output or
/// a block boundary ends the search.
fn adjacent_char(node: &Rc<Node>, direction: Direction) -> Option<char> {
    let mut current = node.clone();
    while let Some(parent) = get_parent_node(&current) {
        let found = {
            let children = parent.children.borrow();
            let index = children.iter().position(|c| Rc::ptr_eq(c, &current))?;
            match direction {
                Direction::Back => scan_back(&children[..index]),
                Direction::Forward => scan_forward(&children[index + 1..]),
            }
        };
        match found {
            Adjacent::Char(ch) => return Some(ch),
            Adjacent::Edge => return None,
            Adjacent::Nothing => {}
        }
        // Nothing lies that way within this element. The line reaches past it
        // only while it is inline: a block element starts a line of its own.
        if get_node_tag_name(&parent).is_none_or(is_block_element) {
            return None;
        }
        current = parent;
    }
    None
}

/// Scans `nodes` — the siblings ahead of an element, in document order — back to
/// front, stopping at the first one that reaches the output.
fn scan_back(nodes: &[Rc<Node>]) -> Adjacent {
    for node in nodes.iter().rev() {
        match &node.data {
            NodeData::Text { contents } => {
                let last = contents.borrow().chars().last();
                if let Some(ch) = last {
                    return Adjacent::Char(ch);
                }
            }
            NodeData::Element { name, .. } => {
                let tag = &*name.local;
                if tag == "br" || is_block_element(tag) {
                    // Both end the line they sit on.
                    return Adjacent::Edge;
                }
                if tag == "img" {
                    // Childless, but it still writes a link, which ends in `)`.
                    return Adjacent::Char(')');
                }
                // An element writing markers of its own puts punctuation here
                // rather than the text this finds, which only makes the answer
                // stricter: the element is serialized where it need not have
                // been, never written where it cannot be.
                match scan_back(&node.children.borrow()) {
                    Adjacent::Nothing => {}
                    found => return found,
                }
            }
            _ => {}
        }
    }
    Adjacent::Nothing
}

/// Scans `nodes` — the siblings after an element, in document order — front to
/// back, the mirror of [`scan_back`].
fn scan_forward(nodes: &[Rc<Node>]) -> Adjacent {
    for node in nodes {
        match &node.data {
            NodeData::Text { contents } => {
                let first = contents.borrow().chars().next();
                if let Some(ch) = first {
                    return Adjacent::Char(ch);
                }
            }
            NodeData::Element { name, .. } => {
                let tag = &*name.local;
                if tag == "br" || is_block_element(tag) {
                    return Adjacent::Edge;
                }
                if tag == "img" {
                    // The link it writes starts with `!`.
                    return Adjacent::Char('!');
                }
                match scan_forward(&node.children.borrow()) {
                    Adjacent::Nothing => {}
                    found => return found,
                }
            }
            _ => {}
        }
    }
    Adjacent::Nothing
}

#[cfg(test)]
mod tests {
    use super::{emphasize_paragraphs, leading_hoist, trailing_hoist};

    /// Each paragraph carries its own pair of markers, and what separates them
    /// comes back untouched — a blank line of spaces as much as an empty one,
    /// and the backslash of a hard break, which can no more sit against a
    /// marker at a paragraph's edge than at the element's own.
    #[test]
    fn every_paragraph_is_emphasized_on_its_own() {
        assert_eq!("*a*\n\n*b*", emphasize_paragraphs("a\n\nb", "*"));
        assert_eq!("**a**\n\n**b**", emphasize_paragraphs("a\n\nb", "**"));
        assert_eq!("*a*\n \n*b*", emphasize_paragraphs("a\n \nb", "*"));
        assert_eq!("*a*\n\n\\\n*b*", emphasize_paragraphs("a\n\n\\\nb", "*"));
    }

    /// Emphasis is a paragraph's to carry. Every other block spells itself with
    /// the characters at a line's edge, which is exactly where a marker would
    /// have to go — so it goes out as it came in, emphasis and all.
    ///
    /// `emphasis_around_a_block_keeps_its_markers` in `basic_tests.rs` carries
    /// the paragraph case end to end; these blocks reach an `<em>` too rarely to
    /// pin each of them that way.
    #[test]
    fn a_block_that_is_not_a_paragraph_goes_out_unmarked() {
        for block in [
            "```\nx\n```",  // A fenced code block,
            "~~~\nx\n~~~",  // however it is fenced,
            "    x",        // or indented instead.
            "*   x\n*   y", // A bullet list,
            "1.  x",        // or a numbered one.
            "> x",          // A quote,
            "# x",          // an ATX heading,
            "x\n===",       // a Setext one,
            // A table, which only the delimiter row gives away as one.
            "| h |\n| - |\n| x |",
            "* * *",        // a thematic break,
            "<div>x</div>", // and a block of raw HTML.
        ] {
            assert_eq!(
                format!("*a*\n\n{block}\n\n*b*"),
                emphasize_paragraphs(&format!("a\n\n{block}\n\nb"), "*"),
                "{block:?}"
            );
        }
    }

    /// A blank line inside a fence is part of the code rather than a paragraph
    /// boundary, so the block it sits in comes back whole. Only a run at least
    /// as long as the one that opened the fence closes it.
    #[test]
    fn a_fence_holds_its_own_blank_lines() {
        assert_eq!(
            "*a*\n\n```\nx\n\ny\n```\n\n*b*",
            emphasize_paragraphs("a\n\n```\nx\n\ny\n```\n\nb", "*")
        );
        assert_eq!(
            "````\nx\n\n```\n\ny\n````",
            emphasize_paragraphs("````\nx\n\n```\n\ny\n````", "*")
        );
    }

    /// A code span is not a fence, however many backticks it opens with — the
    /// backtick in what would be its info string is what says so.
    ///
    /// htmd writes a span whose content touches a backtick this way, so reading
    /// one as a fence is reachable from ordinary input. The block it wrongly
    /// opens either swallows the rest of the content or, worse, closes on the
    /// next real fence and hands that block's lines to `push_paragraph`, which
    /// writes markers into what is somebody's code.
    #[test]
    fn a_code_span_does_not_open_a_fence() {
        assert_eq!(None, super::fence_delimiter("``` ``x ```"));
        assert_eq!(Some("```"), super::fence_delimiter("```"));
        assert_eq!(Some("```"), super::fence_delimiter("```rust"));
        // Only a backtick fence restricts its info string; a tilde one does not.
        assert_eq!(Some("~~~"), super::fence_delimiter("~~~ `x`"));

        // The span is a paragraph's to hold, so it takes the markers like any
        // other text — a code span binds tighter than emphasis, so this reads
        // back as an emphasized span rather than as a fence.
        assert_eq!(
            "*``` ``x ```*\n\n*d*",
            emphasize_paragraphs("``` ``x ```\n\nd", "*")
        );
        // The real fence that follows still opens and closes its own block, so
        // nothing is written inside it.
        assert_eq!(
            "*``` ``x ```*\n\n````\na```b\n\nc\n````\n\n*d*",
            emphasize_paragraphs("``` ``x ```\n\n````\na```b\n\nc\n````\n\nd", "*")
        );
    }

    /// A marker is only as long as the run it lands in, so one written against
    /// markers an inline child already wrote joins them instead of opening a
    /// pair of its own. The paragraph goes out unmarked rather than let that
    /// happen: `*` against `*b*` would read as strong, and `*b*z` would come
    /// back as a literal `**` with the emphasis moved onto the wrong word.
    #[test]
    fn a_marker_never_joins_a_delimiter_run_already_there() {
        assert_eq!("*a*\n\n*b*", emphasize_paragraphs("a\n\n*b*", "*"));
        assert_eq!("*a*\n\n*b*z", emphasize_paragraphs("a\n\n*b*z", "*"));
        assert_eq!("*a*\n\nz*b*", emphasize_paragraphs("a\n\nz*b*", "*"));
        assert_eq!("**a**\n\n**b**", emphasize_paragraphs("a\n\n**b**", "**"));
        // The run has to be the marker's own character to fuse with it.
        assert_eq!("*a*\n\n*_b_*", emphasize_paragraphs("a\n\n_b_", "*"));

        // An escaped asterisk is text rather than a run, wherever it sits. The
        // trailing edge is the one that can hide one — a leading `\*` opens with
        // the backslash — so it is the one worth pinning.
        assert_eq!(r"*a\*b*", emphasize_paragraphs(r"a\*b", "*"));
        assert_eq!(
            concat!("*a*\n\n", r"*2 \* 3 = 6 \**"),
            emphasize_paragraphs(concat!("a\n\n", r"2 \* 3 = 6 \*"), "*")
        );
    }

    /// Text that merely opens with a block's character is still text, and it
    /// keeps its emphasis. htmd leaves these unescaped precisely because they
    /// were never blocks, so reading them as blocks costs an ordinary paragraph
    /// the markers it could have carried.
    #[test]
    fn text_that_only_looks_like_a_block_is_still_a_paragraph() {
        for text in [
            "#hashtag",        // No space, so no ATX heading,
            "####### x",       // and seven `#` is one too many.
            "| x |",           // A pipe with no delimiter row under it,
            "|--|",            // and a delimiter row with no header row above
            "| :-: |",         // it is no table either, however it is spelled.
            "<https://e.com>", // An autolink,
            "<span>x</span>",  // and an inline tag.
            "1234567890. x",   // Ten digits is past an ordered list's nine.
            "-dash",           // A bullet needs the space after it.
        ] {
            assert_eq!(
                format!("*a*\n\n*{text}*"),
                emphasize_paragraphs(&format!("a\n\n{text}"), "*"),
                "{text:?}"
            );
        }
    }

    /// A `\r` is whitespace, so it moves out with the rest of the run, but it is
    /// not a *line ending*, so it never marks the break position the hoists stop
    /// at. The end-to-end behavior is identical either way, so only the hoists'
    /// own contract can pin the distinction.
    #[test]
    fn carriage_return_is_not_a_line_ending() {
        // Faithful mode: the whole whitespace run moves out, as it would for a
        // run holding no line ending at all.
        assert_eq!(1, leading_hoist("\ra", false));
        assert_eq!(1, trailing_hoist("a\r", false));

        // A real `\n` in the run still stops it, `\r` or no `\r`.
        assert_eq!(1, leading_hoist("\r\na", false));
        assert_eq!(0, trailing_hoist("a\r\n", false));

        // Pure mode: a `\` before a `\r` is a literal backslash next to a
        // carriage return, not a backslash hard break, so nothing is taken.
        assert_eq!(0, leading_hoist("\\\ra", true));
        assert_eq!(1, trailing_hoist("a\\\r", true));

        // The same `\` before a real `\n` is a break, and is taken.
        assert_eq!(2, leading_hoist("\\\na", true));
        assert_eq!(2, trailing_hoist("a\\\n", true));
    }

    /// `trailing_backslash_is_not_a_hard_break` in `basic_tests.rs` pins the
    /// consequence end to end; only these pin the odd/even boundary itself.
    #[test]
    fn an_escaped_backslash_is_not_a_break_marker() {
        // Even runs: the newline alone moves out, leaving every backslash of
        // the pair inside the markers.
        assert_eq!(1, trailing_hoist(concat!(r"a\\", "\n"), true));
        assert_eq!(1, trailing_hoist(concat!(r"a\\\\", "\n"), true));

        // Odd runs: the unpaired backslash is the break's marker and moves out
        // with the newline — just the one, however long the run.
        assert_eq!(2, trailing_hoist(concat!(r"a\", "\n"), true));
        assert_eq!(2, trailing_hoist(concat!(r"a\\\", "\n"), true));
    }
}
