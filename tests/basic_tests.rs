use std::{sync::Arc, thread::JoinHandle};

use indoc::indoc;
use pretty_assertions::assert_eq;

use htmd::{
    Element, HtmlToMarkdown,
    element_handler::Handlers,
    options::{BrStyle, HeadingStyle, LinkStyle, Options, TranslationMode},
};
mod common;
use common::{convert_faithful, render};

#[test]
fn links_with_spaces() {
    let html = r#"
        <a href="https://example.com/Some Page.html">Example</a>
        "#;
    assert_eq!(
        "[Example](<https://example.com/Some Page.html>)",
        convert_faithful(html).unwrap(),
    )
}

#[test]
fn referenced_links_with_title() {
    let html = r#"
        <a href="https://example.com" title="Some title">Example</a>
        "#;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(
        "[Example][1]\n\n[1]: https://example.com \"Some title\"",
        &md
    )
}

#[test]
fn consecutive_referenced_links_with_title() {
    let html = r#"
        <a href="https://example.com" title="Some title">Example</a><a href="https://example.com" title="Some title">Another example</a>
        "#;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(
        indoc!(
            r#"
        [Example][1][Another example][2]

        [1]: https://example.com "Some title"
        [2]: https://example.com "Some title""#
        ),
        &md
    )
}

#[test]
fn images() {
    let html = r#"
        <img src="https://example.com" />
        <img src="https://example.com" alt="Image 1" />
        <img src="https://example.com" alt="Image 2" title="Hello" />
        "#;
    assert_eq!(
        "![](https://example.com) ![Image 1](https://example.com) \
            ![Image 2](https://example.com \"Hello\")",
        convert_faithful(html).unwrap(),
    )
}

#[test]
fn images_with_spaces_in_url() {
    let html = r#"
        <img src="https://example.com/Some Image.jpg" />
        "#;
    assert_eq!(
        "![](<https://example.com/Some Image.jpg>)",
        convert_faithful(html).unwrap(),
    )
}

#[test]
fn image_title_stays_outside_an_angle_bracket_destination() {
    let markdown = htmd::convert(
        r#"<img src="https://example.com/image name.png" alt="diagram" title="A title">"#,
    )
    .unwrap();

    assert_eq!(
        r#"![diagram](<https://example.com/image name.png> "A title")"#,
        markdown
    );
}

#[test]
fn headings() {
    let html = r#"
        <h1>Heading 1</h1>
        <h2>Heading 2</h2>
        <h3>Heading 3</h3>
        <h4>Heading 4</h4>
        <h5>Heading 5</h5>
        <h6>Heading 6</h6>
        "#;
    assert_eq!(
        "# Heading 1\n\n## Heading 2\n\n### Heading 3\n\n\
             #### Heading 4\n\n##### Heading 5\n\n###### Heading 6",
        convert_faithful(html).unwrap(),
    )
}

#[test]
fn paragraphs() {
    let html = r#"
        <p>The first.</p>
        <p>The <span>second.</span></p>
        "#;
    assert_eq!(
        "The first.\n\nThe <span>second.</span>",
        convert_faithful(html).unwrap()
    );
}

#[test]
fn quotes() {
    let html = r#"
        <blockquote>Once upon a time</blockquote>
        "#;
    assert_eq!("> Once upon a time", convert_faithful(html).unwrap());
}

#[test]
fn br() {
    let html = r#"
        Hi<br>there<br><br>!"#;
    // The second `<br>` of the pair opens an empty line, where two spaces would
    // be invisible and leave a blank line that ends the paragraph, so it falls
    // back to a backslash break.
    assert_eq!("Hi  \nthere  \n\\\n!", convert_faithful(html).unwrap());

    let md = HtmlToMarkdown::builder()
        .options(Options {
            br_style: BrStyle::Backslash,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("Hi\\\nthere\\\n\\\n!", &md);
}

#[test]
fn hr() {
    let html = r#"Hi <hr/> there"#;
    assert_eq!("Hi\n\n* * *\n\nthere", convert_faithful(html).unwrap());
}

#[test]
fn strong_italic() {
    let html = r#"<i>Italic</i><em>Also italic</em><strong>Strong</strong><b>Stronger</b>"#;
    assert_eq!(
        "*ItalicAlso italic***StrongStronger**",
        convert_faithful(html).unwrap()
    );
}

#[test]
fn italic_inside_word() {
    let html = r#"It<i>al</i>ic St<b>ro</b>ng"#;
    assert_eq!("It*al*ic St**ro**ng", convert_faithful(html).unwrap());
}

/// A literal backslash ending an emphasis element is written `\\`, whose second
/// backslash sits against the following newline — exactly where a backslash hard
/// break's marker sits. Pure mode, which moves such a break outside the closing
/// marker, must not mistake this for one: hoisting one backslash out of the pair
/// leaves the other escaping the marker and loses the emphasis.
#[test]
fn trailing_backslash_is_not_a_hard_break() {
    for html in [
        // A block child is what puts a newline after the backslash pair.
        r"<em>path C:\<div></div></em>",
        r"<strong>path C:\<div></div></strong>",
        // Here a real break follows the literal, so the run is odd and the break
        // does move out — but the pair it sits behind must stay whole.
        r"<em>path C:\<br>x</em>",
    ] {
        for mode in [TranslationMode::Pure, TranslationMode::Faithful] {
            let md = HtmlToMarkdown::builder()
                .options(Options {
                    translation_mode: mode,
                    ..Default::default()
                })
                .build()
                .convert(html)
                .unwrap();
            assert!(
                md.contains(r"C:\\"),
                "{html:?} ({mode:?}) became {md:?}, which split the escaped backslash pair"
            );
        }
    }
}

fn convert_in(html: &str, translation_mode: TranslationMode) -> String {
    HtmlToMarkdown::builder()
        .options(Options {
            translation_mode,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap()
}

// ---------------------------------------------------------------------------
// Emphasis wrapping a *block*.
//
// `emphasis_handler` checks only that its markers flank. CommonMark asks
// something else as well: both markers must land in the same paragraph, since a
// blank line ends one — and a block child writes a blank line. The handler's
// hoists move a blank line at either *edge* outside the markers, so only an
// interior block leaves one with content on both sides, where nothing can hoist
// it away.
//
// Pure mode answers that by writing the content block by block: a pair of
// markers around each paragraph the blank lines leave, and every other kind of
// block left exactly as it stands, since a paragraph is the only one that can
// carry a pair. Faithful mode has no such answer yet, and the defect that
// leaves is pinned below.
//
// Whether the emphasis reaches the output therefore turns on the mode and on
// what encloses the element. Both tests run the same `<em>a<div>b</div>c</em>`
// and differ only in what they expect of it: the first holds every context
// under pure mode, along with the faithful outputs that keep their markers, and
// the second holds the faithful outputs that do not.
// ---------------------------------------------------------------------------

/// The emphasis survives its block child here.
///
/// Pure mode manages it wherever the blocks are paragraphs, by emphasizing each
/// on its own — the same thing the markers asked for, said the only way Markdown
/// can say it. A block that is *not* a paragraph goes out unmarked instead and
/// loses the emphasis on itself alone, which
/// `emphasis_loses_only_what_cannot_carry_markers` holds. Faithful mode manages
/// it only where something else has already taken the blank line out of the
/// markers' way:
///
/// * In a `<p>`, the HTML parser gets there first. A `<div>` closes an open
///   paragraph, so html5ever reconstructs the `<em>` around each piece and htmd
///   is handed three well-formed elements instead of one.
/// * In a `<div>` under faithful mode, the `<div>` is serialized whole, so the
///   `<em>` inside it never has to write markers at all.
/// * In a table cell, the cell flattens its content onto one line, which puts
///   both markers back in the same paragraph after the fact.
#[test]
fn emphasis_around_a_block_keeps_its_markers() {
    let cell = "<table><thead><tr><th>h</th></tr></thead><tbody><tr>\
                <td>x<em>a<div>b</div>c</em></td></tr></tbody></table>";
    // A `None` is a faithful output asserted in the test below rather than one
    // that goes unchecked; the pure output is always asserted, so no entry here
    // can pass without pinning something.
    for (html, faithful, pure) in [
        (
            "<p><em>a<div>b</div>c</em></p>",
            Some("*a*\n\n<div><em>b</em></div>\n\n*c*"),
            "*a*\n\n*b*\n\n*c*",
        ),
        (
            "<div><em>a<div>b</div>c</em></div>",
            Some("<div><em>a<div>b</div>c</em></div>"),
            "*a*\n\n*b*\n\n*c*",
        ),
        (
            cell,
            Some("| h                     |\n| --------------------- |\n| x*a  <div>b</div>  c* |"),
            // The cell flattens the paragraphs, but only after the emphasis has
            // been written onto each, so the source's one `<em>` comes back as
            // three and the spaces that separated them fall outside all of
            // them. Faithful mode keeps the single run here. That is the price
            // of deciding the emphasis before the cell has said what it will do
            // with the content — the handler cannot know a cell is above it.
            "| h              |\n| -------------- |\n| x*a*  *b*  *c* |",
        ),
        // The contexts that neither split the element nor flatten it. Faithful
        // mode loses the emphasis in each; that half is asserted below.
        (
            "<ul><li><em>a<div>b</div>c</em></li></ul>",
            None,
            "*   *a*\n\n    *b*\n\n    *c*",
        ),
        (
            "<blockquote><em>a<div>b</div>c</em></blockquote>",
            None,
            "> *a*\n> \n> *b*\n> \n> *c*",
        ),
        ("<em>a<div>b</div>c</em>", None, "*a*\n\n*b*\n\n*c*"),
    ] {
        let md = convert_in(html, TranslationMode::Pure);
        assert_eq!(pure, md, "{html:?}");
        if let Some(faithful) = faithful {
            assert_eq!(
                faithful,
                convert_in(html, TranslationMode::Faithful),
                "{html:?}"
            );
        }
        // The markers really do pair up: each of these reads back as emphasis.
        for md in [Some(md.as_str()), faithful].into_iter().flatten() {
            assert!(
                render(md).contains("<em>"),
                "{html:?} became {md:?}, which reads back with no emphasis"
            );
        }
    }
}

/// The same emphasis under faithful mode, in the contexts that neither split it
/// nor flatten it. The markers straddle a blank line, so CommonMark reads them
/// as literal asterisks and the `<em>` is lost — `*a\n\nb\n\nc*` reads back as
/// three paragraphs, the first opening with a stray `*` and the last closing
/// with one.
///
/// **This pins a known defect, not intended behavior.** The flanking check
/// should reject these and let faithful mode serialize the element, the way it
/// already does for the `<br>` shapes in `br_tests`. Fixing it will fail these
/// assertions — that is what they are for. See `emphasis_handler`.
#[test]
fn emphasis_around_a_block_loses_its_markers() {
    for (html, faithful) in [
        (
            "<ul><li><em>a<div>b</div>c</em></li></ul>",
            "*   *a\n\n    <div>b</div>\n\n    c*",
        ),
        (
            "<blockquote><em>a<div>b</div>c</em></blockquote>",
            "> *a\n> \n> <div>b</div>\n> \n> c*",
        ),
        ("<em>a<div>b</div>c</em>", "*a\n\n<div>b</div>\n\nc*"),
    ] {
        assert_eq!(
            faithful,
            convert_in(html, TranslationMode::Faithful),
            "{html:?}"
        );

        // The defect itself: no emphasis survives the round trip.
        assert!(
            !render(faithful).contains("<em>"),
            "{html:?} became {faithful:?}, which now reads back as emphasis — if this was \
             fixed on purpose, move the shape into the test above"
        );
    }
}

/// A block that cannot carry a pair of markers comes back as the block it is,
/// losing the emphasis rather than the block.
///
/// The block is the whole content in each of these, so there is no interior
/// blank line and the markers would have been written. What they would have
/// spelled is not emphasis, though: set against a `#`, a bullet, or a fence they
/// land *inside* the block and change what it says — `<em><h1>a</h1></em>` wrote
/// `*# a*`, which is no longer a heading at all.
#[test]
fn emphasis_loses_only_what_cannot_carry_markers() {
    for (html, pure, rendered) in [
        ("<em><h1>a</h1></em>", "# a", "<h1>a</h1>"),
        ("<em><ul><li>a</li></ul></em>", "*   a", "<li>a</li>"),
        ("<em><ol><li>a</li></ol></em>", "1.  a", "<li>a</li>"),
        ("<em><hr></em>", "* * *", "<hr />"),
        (
            "<em><pre><code>c1\nc2</code></pre></em>",
            "```\nc1\nc2\n```",
            "<code>c1\nc2\n</code>",
        ),
        (
            "<em><blockquote><p>q1</p><p>q2</p></blockquote></em>",
            "> q1\n> \n> q2",
            "<blockquote>",
        ),
    ] {
        let md = convert_in(html, TranslationMode::Pure);
        assert_eq!(pure, md, "{html:?}");
        // The block itself survives, which is what the markers would have cost.
        assert!(
            render(&md).contains(rendered),
            "{html:?} became {md:?}, which no longer holds its block"
        );
    }
}

/// A code span htmd wrote does not open a fence, so no marker is written into
/// the code block that follows it.
///
/// `code_handler` spells a span whose content touches a backtick with a longer
/// run and padding spaces, which puts three backticks at the start of a line.
/// Read as a fence, that span opens a block which then closes on the next *real*
/// fence — and the real block's own lines land where markers get written, which
/// puts them inside somebody's code.
#[test]
fn emphasis_does_not_write_markers_into_a_code_block() {
    let md = convert_in(
        "<em><code>``x</code><pre><code>a```b\n\nc\n\nd</code></pre></em>",
        TranslationMode::Pure,
    );
    assert_eq!("*``` ``x ```*\n\n````\na```b\n\nc\n\nd\n````", md);
    // The code comes back exactly as it went in, and the span reads as an
    // emphasized span rather than as a fence.
    let html = render(&md);
    assert!(html.contains("a```b\n\nc\n\nd"), "{md:?} lost its code");
    assert!(html.contains("<em><code>``x</code></em>"), "{md:?}");

    // With no second fence to close on, the span used to swallow everything
    // after it instead.
    assert_eq!(
        "*``` ``x ```*\n\n*d1*\n\n*d2*",
        convert_in(
            "<em><code>``x</code><div>d1</div><div>d2</div></em>",
            TranslationMode::Pure
        )
    );
}

/// A marker set against markers an inline child already wrote would join their
/// delimiter run rather than open one of its own, so the paragraph goes out
/// unmarked and keeps the emphasis it already has.
///
/// Getting this wrong costs more than the outer emphasis: a `*` against the
/// `*b*` of a nested `<em>` reads as the `**` of strong, and against `*b*z` it
/// strands a literal `**` in the text and moves the emphasis onto the `z`.
#[test]
fn emphasis_leaves_a_delimiter_run_it_would_join() {
    for (html, pure, rendered) in [
        (
            "<em>a<div><em>b</em></div>c</em>",
            "*a*\n\n*b*\n\n*c*",
            "<p><em>b</em></p>",
        ),
        (
            "<em>a<div><em>b</em>z</div>c</em>",
            "*a*\n\n*b*z\n\n*c*",
            "<p><em>b</em>z</p>",
        ),
        (
            "<em>a<div>z<em>b</em></div>c</em>",
            "*a*\n\nz*b*\n\n*c*",
            "<p>z<em>b</em></p>",
        ),
        (
            "<strong>a<div><strong>b</strong></div>c</strong>",
            "**a**\n\n**b**\n\n**c**",
            "<p><strong>b</strong></p>",
        ),
    ] {
        let md = convert_in(html, TranslationMode::Pure);
        assert_eq!(pure, md, "{html:?}");
        let html_out = render(&md);
        // The child's own emphasis is intact,
        assert!(
            html_out.contains(rendered),
            "{html:?} became {md:?}, whose inner emphasis moved"
        );
        // and no marker was left behind as literal text.
        assert!(
            !html_out.contains('*'),
            "{html:?} became {md:?}, which leaves a literal marker"
        );
    }
}

/// Text that merely opens with a block's character is still text, and it keeps
/// its emphasis.
///
/// htmd leaves a `#` against a word and a lone pipe unescaped precisely because
/// neither is a block — `#x` is no heading without the space, and pipes are no
/// table without a delimiter row. Reading them as blocks would cost an ordinary
/// paragraph the markers it can carry perfectly well.
#[test]
fn emphasis_survives_text_that_only_looks_like_a_block() {
    for (html, pure) in [
        ("<em>a<div>#x</div>c</em>", "*a*\n\n*#x*\n\n*c*"),
        ("<em>a<div>|x|</div>c</em>", "*a*\n\n*|x|*\n\n*c*"),
    ] {
        let md = convert_in(html, TranslationMode::Pure);
        assert_eq!(pure, md, "{html:?}");
        assert_eq!(
            3,
            render(&md).matches("<em>").count(),
            "{html:?} became {md:?}, which lost an emphasis"
        );
    }
}

/// Text that htmd left unescaped keeps the markers that are the only thing
/// still making it text.
///
/// The escaper writes `\#` for the `# ` that would open a heading, but leaves a
/// bare `#`, a `---`, a lone `-`, and a `1) x` alone — none of which it needs to
/// escape while something else is keeping them off the line's edge. By the time
/// the content is a string it is indistinguishable from a block htmd wrote:
/// `<em>#</em>` and `<em><h1></h1></em>` both hold exactly `#`. So the element
/// is what gets asked, not the string.
///
/// Dropping the markers here would not merely lose the emphasis, the way it does
/// for a block that really is one. It would turn the text into the block it was
/// imitating and lose the text itself.
#[test]
fn emphasis_shields_text_that_htmd_left_unescaped() {
    for (html, pure, block_tag) in [
        ("<em>#</em>", "*#*", "<h1>"),
        ("<em>######</em>", "*######*", "<h6>"),
        ("<em>---</em>", "*---*", "<hr"),
        ("<em>-</em>", "*-*", "<ul>"),
        ("<em>1) x</em>", "*1) x*", "<ol>"),
        ("<em>|--|</em>", "*|--|*", "<table>"),
        ("<em>a<br>---</em>", "*a  \n---*", "<h2>"),
        // The same text beside a block child, which is the harder half: the
        // element really does hold a block, so the content is written block by
        // block, and each piece still has to be judged for what it is.
        ("<em>a<div>b</div>#</em>", "*a*\n\n*b*\n\n*#*", "<h1>"),
        ("<em>#<div>b</div>c</em>", "*#*\n\n*b*\n\n*c*", "<h1>"),
        ("<em>a<div>---</div>c</em>", "*a*\n\n*---*\n\n*c*", "<hr"),
        ("<em>a<div>1) x</div>c</em>", "*a*\n\n*1) x*\n\n*c*", "<ol>"),
        ("<em>a<div>-</div>c</em>", "*a*\n\n*-*\n\n*c*", "<ul>"),
    ] {
        let md = convert_in(html, TranslationMode::Pure);
        assert_eq!(pure, md, "{html:?}");
        let rendered = render(&md);
        // The text is still text, and still emphasized.
        assert!(
            rendered.contains("<em>"),
            "{html:?} became {md:?}, which reads back with no emphasis"
        );
        assert!(
            !rendered.contains(block_tag),
            "{html:?} became {md:?}, whose text turned into a {block_tag} block"
        );
    }
}

/// A tab after a `#` run costs the text the run was written with.
///
/// The escaper guards a `#` run closed by a *space* — `is_markdown_atx_heading`
/// in `text_util` — so `#\tx` goes out unescaped. Every tab has become a space
/// by the time `emphasis_handler` sees the content, which leaves it holding
/// `# x`: a string nothing can tell from the heading that `<h1>x</h1>` would
/// have written. `is_paragraph_line` reads it as the heading it now looks like,
/// and the markers that were the text's only shield come off.
///
/// **This pins a known defect, not intended behavior.** The fix belongs to the
/// escaper, which should guard a `#` run closed by any whitespace rather than by
/// a space alone; nothing the emphasis handler can see distinguishes these two
/// cases. `emphasis_shields_text_that_htmd_left_unescaped` holds the shapes the
/// escaper's rule already covers. Fixing it will fail these assertions — that is
/// what they are for.
///
/// Faithful mode keeps the text here, since it never takes the block-by-block
/// path; what it does to these shapes instead is
/// `emphasis_around_a_block_loses_its_markers`.
#[test]
fn a_tab_after_a_hash_turns_the_text_into_a_heading() {
    for (html, pure) in [
        ("<em>#\tx<div>b</div>c</em>", "# x\n\n*b*\n\n*c*"),
        ("<em>a<div>b</div>#\tx</em>", "*a*\n\n*b*\n\n# x"),
        (
            "<strong>#\tx<div>b</div>c</strong>",
            "# x\n\n**b**\n\n**c**",
        ),
    ] {
        let md = convert_in(html, TranslationMode::Pure);
        assert_eq!(pure, md, "{html:?}");

        // The defect itself: the `#` the author wrote is gone from the document,
        // and what was text is a heading.
        assert!(
            render(&md).contains("<h1>x</h1>"),
            "{html:?} became {md:?}, which no longer reads back as a heading — if this was \
             fixed on purpose, move the shape into \
             emphasis_shields_text_that_htmd_left_unescaped"
        );
    }
}

/// Emphasis nested directly inside emphasis fuses the two pairs of markers.
///
/// The inner element writes `*b*` and the outer writes a `*` straight against
/// it, which CommonMark reads as one run of two: `**b*z*` opens strong where the
/// source meant to close the inner emphasis, and the marker left over becomes
/// literal text. Both modes do it — the flanking check looks at whether a marker
/// *can* flank, never at what is already sitting at the content's edge.
///
/// **This pins a known defect, not intended behavior.** `push_paragraph` refuses
/// exactly this through `fuses_with_marker`, but it runs only where a block child
/// split the content into paragraphs; with nothing to split, `emphasis_handler`
/// writes the pair unconditionally. Extending the guard to that path is what
/// would fix these, and doing so will fail these assertions — that is what they
/// are for. `emphasis_leaves_a_delimiter_run_it_would_join` holds the shapes
/// already covered.
#[test]
fn emphasis_directly_inside_emphasis_fuses_its_markers() {
    for (html, md, rendered) in [
        // A stranded `**`, and the emphasis moved onto the wrong word.
        ("<em><em>b</em>z</em>", "**b*z*", "<p>**b<em>z</em></p>\n"),
        ("<em>z<em>b</em></em>", "*z*b**", "<p><em>z</em>b**</p>\n"),
        (
            "<strong><strong>b</strong>z</strong>",
            "****b**z**",
            "<p>****b<strong>z</strong></p>\n",
        ),
        // Nothing is stranded here, but one `<em>` too few came back and the
        // other changed strength.
        (
            "<em><em>b</em></em>",
            "**b**",
            "<p><strong>b</strong></p>\n",
        ),
    ] {
        for mode in [TranslationMode::Pure, TranslationMode::Faithful] {
            assert_eq!(md, convert_in(html, mode), "{html:?} in {mode:?}");
        }

        // The defect itself: what reads back is not the emphasis that went in.
        assert_eq!(
            rendered,
            render(md),
            "{html:?} no longer round-trips wrongly — if this was fixed on purpose, move \
             the shape into emphasis_leaves_a_delimiter_run_it_would_join"
        );
    }
}

/// A carriage return is a character of the document, not a line ending.
///
/// html5ever normalizes the source's own CR and CRLF to `\n`, and ordinary text
/// is whitespace-collapsed on top of that, so `&#13;` disappears from a
/// paragraph entirely. A `<pre>` is the exception, carrying one through verbatim
/// — the one way a `\r` reaches the emphasis handler's hoists, which look for
/// `\n` alone.
///
/// These pin the output only: a carriage return rides through and changes
/// nothing. Adding `\r` back to the hoists' searches leaves every one of them
/// passing, so `emphasis::tests::carriage_return_is_not_a_line_ending` pins that
/// decision against the hoists directly.
#[test]
fn carriage_return_is_text_not_a_line_ending() {
    // Collapsed out of ordinary text before any of this can matter.
    assert_eq!(
        "x *a*y",
        convert_in("<p>x<em>&#13;a</em>y</p>", TranslationMode::Pure)
    );

    for (html, faithful, pure) in [
        // A `<pre>` keeps the CR, and the emphasis inside it still resolves.
        (
            "<pre><em>&#13;a</em>b</pre>",
            "<pre><em>\ra</em>b</pre>",
            "\r*a*b",
        ),
        (
            "<pre>x<em>a&#13;</em>y</pre>",
            "<pre>x<em>a\r</em>y</pre>",
            "x*a*\ry",
        ),
        // A CR the hoist has to step over on its way out of the element.
        (
            "<em><pre>&#13;</pre>x</em>",
            "*<pre>\r</pre>\n\nx*",
            "\r\n\n*x*",
        ),
        // Two CRLFs — the shape a blank-line test spelled `\n\n` would miss if
        // `\r` counted as a line ending here.
        (
            "<em><pre>&#13;&#10;&#13;&#10;</pre>x</em>",
            "*<pre>\r\n&#13;&#10;</pre>\n\nx*",
            "\r\n\r\n\n*x*",
        ),
    ] {
        assert_eq!(
            faithful,
            convert_in(html, TranslationMode::Faithful),
            "{html:?}"
        );
        assert_eq!(pure, convert_in(html, TranslationMode::Pure), "{html:?}");
    }
}

/// A Setext underline attaches to the whole paragraph above it, so multi-line
/// heading content is fine; only a blank line, which ends that paragraph, forces
/// ATX. ATX is not a safe default to reach for — being single-line, it loses
/// everything past the first line to a paragraph of its own.
#[test]
fn setext_falls_back_to_atx_only_for_a_blank_line() {
    fn setext(html: &str) -> String {
        HtmlToMarkdown::builder()
            .options(Options {
                translation_mode: TranslationMode::Faithful,
                heading_style: HeadingStyle::Setex,
                ..Default::default()
            })
            .build()
            .convert(html)
            .unwrap()
    }

    fn setext_pure(html: &str) -> String {
        HtmlToMarkdown::builder()
            .options(Options {
                translation_mode: TranslationMode::Pure,
                heading_style: HeadingStyle::Setex,
                ..Default::default()
            })
            .build()
            .convert(html)
            .unwrap()
    }

    // A block child writes a blank line, so the underline could not reach the
    // heading's own text. ATX at least keeps that text in a heading.
    assert_eq!(
        "# a\n\n<div>x</div>\n\nb",
        setext("<h1>a<div>x</div>b</h1>")
    );
    // Non-ASCII whitespace is not document whitespace, so it survives the
    // heading's trim and opens the content — which changes nothing here.
    assert_eq!(
        "# \u{a0}\n\n<div>x</div>",
        setext("<h1>&nbsp;<div>x</div></h1>")
    );
    // A line of nothing but spaces is blank too, and a `<pre>` is where one
    // reaches a heading: its whitespace is kept rather than compressed. Setext
    // here would underline `y` alone and leave `x` outside the heading.
    assert_eq!("# x\n \ny", setext_pure("<h1><pre>x\n \ny</pre></h1>"));

    // No blank line: Setext holds, however little the first line carries. Each
    // of these reads back as a single heading.
    for (html, expected) in [
        ("<h1>a<br>b</h1>", "a  \nb\n====="),
        ("<h1>&nbsp;a</h1>", "\u{a0}a\n=="),
        // A raw `<br>` that *opens* the line is the one break shape that must
        // use ATX: it starts an HTML block which eats the underline.
        ("<h1><br>a</h1>", "# <br>a"),
        // ...but one with anything ahead of it is an inline tag, so Setext is
        // still fine.
        ("<h1>a<br></h1>", "a<br>\n====="),
    ] {
        assert_eq!(expected, setext(html), "{html:?}");
    }
    for html in ["<h1>a<br>b</h1>", "<h1>&nbsp;a</h1>", "<h1>a<br></h1>"] {
        let rendered = render(&setext(html));
        assert!(
            rendered.starts_with("<h1>") && rendered.matches("<h1>").count() == 1,
            "{html:?} became {:?}, which reads back as {rendered:?}",
            setext(html)
        );
    }
}

/// `br_handler` decides how to write a `<br>` from the heading's level and style
/// alone, since the content `can_use_setext` judges does not exist until the
/// walk it is part of returns. So it can write a hard break into a heading that
/// then falls back to ATX, where the break ends the heading; `fold_hard_breaks`
/// rewrites those.
///
/// The invariant: asking for Setext is never worse than not asking, so where
/// Setext cannot be used the output is the one ATX gives for the same input.
#[test]
fn setext_falling_back_to_atx_rewrites_hard_breaks() {
    fn convert_with(html: &str, heading_style: HeadingStyle, mode: TranslationMode) -> String {
        HtmlToMarkdown::builder()
            .options(Options {
                translation_mode: mode,
                heading_style,
                br_style: BrStyle::Backslash,
                ..Default::default()
            })
            .build()
            .convert(html)
            .unwrap()
    }

    // A block child puts a blank line in the content, which rules Setext out in
    // either mode — after the `<br>` ahead of it has already been written.
    for html in [
        "<h1>a<br>b<div>c</div></h1>",
        "<h2>a<br>b<div>c</div></h2>",
        "<h1>a<br>b<hr></h1>",
        // A break *after* the blank line, which a repair that stopped at the
        // first one would leave behind.
        "<h1>a<div>c</div>d<br>e</h1>",
    ] {
        for mode in [TranslationMode::Pure, TranslationMode::Faithful] {
            assert_eq!(
                convert_with(html, HeadingStyle::Atx, mode),
                convert_with(html, HeadingStyle::Setex, mode),
                "{html:?} ({mode:?}) came out worse under Setex than under Atx"
            );
        }
    }

    // A raw `<br>` opening the first line rules Setext out as well, but only
    // faithful mode writes one: pure mode drops it, leaving the content to open
    // with the `a` after it and Setext usable.
    for html in ["<h1><br>a<br>b</h1>", "<h1><a><br></a>a<br>b</h1>"] {
        assert_eq!(
            convert_with(html, HeadingStyle::Atx, TranslationMode::Faithful),
            convert_with(html, HeadingStyle::Setex, TranslationMode::Faithful),
            "{html:?} came out worse under Setex than under Atx"
        );
        // Pure mode keeps the break as a break, which is the better answer of
        // the two and must not be folded away with it.
        assert_eq!(
            "a\\\nb\n====",
            convert_with(html, HeadingStyle::Setex, TranslationMode::Pure),
            "{html:?}"
        );
    }

    // The one shape ATX can hold outright also has to read back as the heading
    // it came from, which is what the stray `\` used to cost.
    for mode in [TranslationMode::Pure, TranslationMode::Faithful] {
        let md = convert_with("<h1><br>a<br>b</h1>", HeadingStyle::Setex, mode);
        let rendered = render(&md);
        assert!(
            rendered.starts_with("<h1>") && rendered.matches("<h1>").count() == 1,
            "({mode:?}) became {md:?}, which reads back as {rendered:?}"
        );
    }

    // An escaped literal backslash is not a break marker, so the newline after
    // it is the block's own and the pair must survive whole.
    let md = convert_with(
        r"<h1>path C:\<div>c</div></h1>",
        HeadingStyle::Setex,
        TranslationMode::Faithful,
    );
    assert!(
        md.contains(r"C:\\"),
        "{md:?} split the escaped backslash pair"
    );
}

#[test]
fn inline_raw_html_escaping() {
    let html = r#"Test &lt;code&gt;tags&lt;/code&gt;, &lt;!-- comments --&gt;, &lt;?processing instructions?&gt;, &lt;!A declaration&gt;, and &lt;![CDATA[character data]]&gt;."#;
    assert_eq!(
        r#"Test \<code>tags\</code>, \<!-- comments -->, \<?processing instructions?>, \<!A declaration>, and <!\[CDATA\[character data\]\]>."#,
        convert_faithful(html).unwrap()
    );
}

#[test]
fn multiline_raw_html_escaping() {
    let html = indoc!(
        r#"
    Test &lt;code&gt;multi-line
    tags&lt;/code&gt;, &lt;!-- multi-line
    comments --&gt;, &lt;?multi-line
    processing instructions?&gt;, &lt;!A multi-line
    declaration&gt;, and &lt;![CDATA[multi-line
    character data]]&gt;.
    "#
    );
    assert_eq!(
        indoc!(
            r#"Test \<code>multi-line tags\</code>, \<!-- multi-line comments -->, \<?multi-line processing instructions?>, \<!A multi-line declaration>, and <!\[CDATA\[multi-line character data\]\]>."#
        ),
        convert_faithful(html).unwrap()
    );
}

#[test]
fn html_escaping() {
    let html = indoc!(
        r#"
        <p>&lt;pre</p>
        <p>&lt;script</p>
        <p>&lt;style</p>
        <p>&lt;textarea</p>
        <p>&lt;address</p>
        <p>&lt;ul</p>
        "#
    );
    assert_eq!(
        indoc!(
            r#"\<pre

            \<script

            \<style

            \<textarea

            \<address

            \<ul"#
        ),
        convert_faithful(html).unwrap()
    );
}

#[test]
fn faithful_mode_inline() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<p>
                <img src="one.png" alt="yyy" title="zzz" scale="50%">
                <em bar>Testing</em>
                <strong foo>Testing</strong>
                <a href="http://foo.com" bar>link</a>
                <code class="not-a-language">code</code>
                <br foo>
            </p>"#
        ))
        .unwrap(),
        indoc!(
            r#"<img src="one.png" alt="yyy" title="zzz" scale="50%"> <em bar="">Testing</em> <strong foo="">Testing</strong> <a href="http://foo.com" bar="">link</a> <code class="not-a-language">code</code> <br foo="">"#
        )
    );
}

#[test]
fn faithful_mode_hr() {
    assert_eq!(
        convert_faithful(indoc!(r#"<hr bar>"#)).unwrap(),
        indoc!(r#"<hr bar="">"#)
    );
}

#[test]
fn faithful_mode_blockquote() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<blockquote style="foo">
            <em>Testing</em>

            <blockquote>Nested</blockquote>
        </blockquote>"#
        ))
        .unwrap(),
        indoc!(
            r#"<blockquote style="foo">
                <em>Testing</em>
            &#10;    <blockquote>Nested</blockquote>
            </blockquote>"#
        )
    );
}

#[test]
fn faithful_mode_h1() {
    assert_eq!(
        convert_faithful(indoc!(r#"<h1 class="foo">Heading</h1>"#)).unwrap(),
        indoc!(r#"<h1 class="foo">Heading</h1>"#)
    );
}

#[test]
fn faithful_mode_p() {
    assert_eq!(
        convert_faithful(indoc!(r#"<p dir="ltr">Test 1</p>"#)).unwrap(),
        indoc!(r#"<p dir="ltr">Test 1</p>"#)
    );
}

#[test]
fn faithful_mode_ol1() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<ol>
            <li>Test 1</li>
            <li foo>Test 2</li>
            <li>Test 3</li>
        </ol>"#
        ))
        .unwrap(),
        indoc!(
            r#"<ol>
                <li>Test 1</li>
                <li foo="">Test 2</li>
                <li>Test 3</li>
            </ol>"#
        )
    );
}

#[test]
fn faithful_mode_ol2() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<ol foo>
            <li>Test</li>
        </ol>"#
        ))
        .unwrap(),
        indoc!(
            r#"<ol foo="">
                <li>Test</li>
            </ol>"#
        )
    );
}

#[test]
fn faithful_mode_comment() {
    assert_eq!(
        convert_faithful(indoc!(r#"<!-- Test -->"#)).unwrap(),
        indoc!(r#"<!-- Test -->"#)
    );
}

#[test]
fn faithful_mode_html() {
    let html = indoc!(
        r#"<details>
            <summary>Test

                1</summary>
            Test 2
        </details>"#
    );
    let md = convert_faithful(html).unwrap();
    assert_eq!(
        indoc!(
            r#"<details>
                <summary>Test
            &#10;        1</summary>
                Test 2
            </details>"#
        ),
        md
    );
}

#[test]
fn faithful_mode_table() {
    assert_eq!(
        convert_faithful(indoc!(
            r#"<table>
            <tr>
                <th>Header 1</th>
                <th>Header 2</th>
            </tr>
            <tr>
                <td foo>Cell 1</td>
                <td>Cell 2</td>
            </tr>
            <tr>
                <td>Cell 3</td>
                <td>Cell 4</td>
            </tr>
        </table>
"#
        ))
        .unwrap(),
        indoc!(
            r#"<table>
            <tbody><tr>
                <th>Header 1</th>
                <th>Header 2</th>
            </tr>
            <tr>
                <td foo="">Cell 1</td>
                <td>Cell 2</td>
            </tr>
            <tr>
                <td>Cell 3</td>
                <td>Cell 4</td>
            </tr>
        </tbody></table>"#
        )
    );
}

#[test]
fn faithful_mode_serializes_a_table_when_its_caption_requires_html() {
    let html = concat!(
        r#"<table><caption><span class="label">Caption</span></caption>"#,
        "<tr><th>Header</th></tr><tr><td>Cell</td></tr></table>"
    );
    let expected = concat!(
        r#"<table><caption><span class="label">Caption</span></caption>"#,
        "<tbody><tr><th>Header</th></tr><tr><td>Cell</td></tr></tbody></table>"
    );

    assert_eq!(expected, convert_faithful(html).unwrap());
}

#[test]
fn faithful_mode_nested_inline_html() {
    assert_eq!(
        convert_faithful("<p>Nested <foo><bar><em>content</em></bar></foo></p>").unwrap(),
        "Nested <foo><bar>*content*</bar></foo>"
    );
}

#[test]
fn spaces_check() {
    let html = r#"<i>Italic</i> <em>Also italic</em>  <strong>Strong</strong> <b>Stronger </b>"#;
    assert_eq!(
        "*Italic* *Also italic* **Strong** **Stronger**",
        convert_faithful(html).unwrap()
    );
}

#[test]
fn consecutive_blocks() {
    let html = r#"<p>One</p><p>Two</p>"#;
    assert_eq!(
        indoc!(
            "
        One

        Two"
        ),
        convert_faithful(html).unwrap()
    );
}

#[test]
fn raw_text() {
    let html = r#"Hello world!"#;
    assert_eq!("Hello world!", convert_faithful(html).unwrap());
}

#[test]
fn nested_divs() {
    let html = r#"
    <div>
        <div>
            <div>Hi</div>
        </div>
        <div></div>
        <div>there</div>
    </div>
    "#;
    assert_eq!("Hi\n\nthere", htmd::convert(html).unwrap());
}

#[test]
fn with_head() {
    let html = r#"
    <html>
        <head>
            <title>Demo</title>
            <script>console.log('Hello');</script>
            <style>body {}</style>
        </head>
        <body>
            Content
        </body>
    </html>
    "#;
    assert_eq!(
        "Demo\n\nconsole.log('Hello');\n\nbody {}\n\nContent",
        htmd::convert(html).unwrap()
    );
}

#[test]
fn with_custom_rules() {
    // Remove element
    let html = r#"<img src="https://example.com"/>"#;
    let md = HtmlToMarkdown::builder()
        .add_handler(vec!["img"], |_: &dyn Handlers, _element: Element| None)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("", &md);
}

#[test]
fn with_custom_rules_and_fallback() {
    let html = r#"<img src="https://example.com"/>"#;
    let converter = HtmlToMarkdown::builder()
        .add_handler(vec!["img"], |handlers: &dyn Handlers, element: Element| {
            if element
                .attrs
                .iter()
                .any(|attr| &attr.name.local == "id" && attr.value.as_ref() == "do_not_skip_me")
            {
                handlers.fallback(element)
            } else {
                None
            }
        })
        .options(Options {
            ..Default::default()
        })
        .build();
    assert_eq!("", &converter.convert(html).unwrap());

    let html = r#"<img src="https://example.com" id="do_not_skip_me"/>"#;
    assert_eq!(
        "![](https://example.com)",
        &converter.convert(html).unwrap()
    );
}

#[test]
fn upper_case_tags() {
    let html = r#"<H1>Hello</H1> <P>World</P>"#;
    assert_eq!("# Hello\n\nWorld", convert_faithful(html).unwrap());
}

#[test]
fn html_entities() {
    let html = r#"<p><a href="/my%20&amp;uri" title="my%20&amp;title">my%20&amp;link</a></p>"#;
    assert_eq!(
        r#"[my%20&link](/my%20&uri "my%20&title")"#,
        convert_faithful(html).unwrap()
    );

    let html_plain = r#"<p>This &amp; that, then &lt; &gt; now.</p>"#;
    assert_eq!(
        r#"This & that, then < > now."#,
        convert_faithful(html_plain).unwrap()
    );
}

#[test]
fn scripting_option() {
    let html = r#"<noscript><p>Hello</p></noscript>"#;
    let md = HtmlToMarkdown::builder()
        .scripting_enabled(true)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(r#"\<p>Hello\</p>"#, md);

    let md = HtmlToMarkdown::builder()
        .scripting_enabled(false)
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("Hello", md);
}

#[test]
fn multithreading() {
    let html = r#"<a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    <a href="https://example.com">Example</a>
    "#;
    let expected = "[Example][1] [Example][2] [Example][3] [Example][4] [Example][5]\n\n\
    [1]: https://example.com\n[2]: https://example.com\n[3]: https://example.com\n\
    [4]: https://example.com\n[5]: https://example.com";
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            // We use a global vec to store all referenced links of the doc in
            // the anchor element handler, this is unsafe for multithreading
            // usage if we do nothing
            link_style: LinkStyle::Referenced,
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        })
        .build();
    let converter = Arc::new(converter);
    let mut handlers: Vec<JoinHandle<()>> = vec![];
    for _ in 0..20 {
        let converter_clone = converter.clone();
        let handle = std::thread::spawn(move || {
            let md = converter_clone.convert(html).unwrap();
            assert_eq!(expected, md);
        });
        handlers.push(handle);
    }
    for handle in handlers {
        handle.join().unwrap();
    }
}

#[test]
fn unterminated_html() {
    // The `<i>` tag isn't terminated. Make sure the conversion still works.
    assert_eq!("# *A*", convert_faithful("<h1><i>A</h1>").unwrap());
}

#[test]
fn misnested_formatting_does_not_duplicate_or_lose_text() {
    let markdown = htmd::convert("<p><b>one<i>two</b>three</i>four").unwrap();

    assert_eq!("**one*two****three*four", markdown);
}

#[test]
fn math() {
    assert_eq!(
        "$x^2$",
        convert_faithful(r#"<p><span class="math math-inline">x^2</span></p>"#).unwrap()
    );

    assert_eq!(
        "$$x^2$$",
        convert_faithful(r#"<p><span class="math math-display">x^2</span></p>"#).unwrap()
    );

    // Test escaping -- values inside math should not be escaped.
    assert_eq!(
        "$${a}_1, b_{2}, a*1, b*2, [a](b), 3 <a> b, a \\; b$$",
        convert_faithful(r#"<p><span class="math math-display">{a}_1, b_{2}, a*1, b*2, [a](b), 3 &lt;a&gt; b, a \; b</span></p>"#).unwrap()
    );
}

// Document white space characters don't include non-breaking spaces; these should be preserved.
#[test]
fn document_whitespace() {
    assert_eq!(
        "bar\u{a0}\n\n*   foo\u{a0}",
        convert_faithful(indoc!(
            "
            <p>bar&nbsp;</p>
            <ul>
              <li>foo&nbsp;</li>
            </ul>
            "
        ))
        .unwrap()
    );
}

// Multi-byte UTF-8 characters before a markdown ordered list dot must not
// cause a panic due to byte/char index confusion in escape_text.
#[test]
fn multibyte_ordered_list_escape_half() {
    // U+00BD (½) is 2 bytes in UTF-8
    let md = convert_faithful("<p>2½. Long shot</p>").unwrap();
    assert_eq!(r"2½\. Long shot", md);
}

#[test]
fn multibyte_ordered_list_escape_accented() {
    // e-acute before dot -- not numeric, so the dot is not an ordered list marker
    let md = convert_faithful("<p>1é. text</p>").unwrap();
    assert_eq!(r"1é. text", md);
}

#[test]
fn multibyte_ordered_list_escape_trademark() {
    // trademark symbol is not numeric
    let md = convert_faithful("<p>3™. text</p>").unwrap();
    assert_eq!(r"3™. text", md);
}

#[test]
fn ascii_ordered_list_escape() {
    let md = convert_faithful("<p>10. normal</p>").unwrap();
    assert_eq!(r"10\. normal", md);
}

#[test]
fn multibyte_no_dot() {
    // No dot, should not be affected
    let md = convert_faithful("<p>2½</p>").unwrap();
    assert_eq!("2½", md);
}

#[test]
fn cjk_before_ordered_list() {
    // CJK chars are not numeric in Rust's is_numeric(), so this is not an ordered list pattern
    let md = convert_faithful("<p>日本語1. test</p>").unwrap();
    assert_eq!(r"日本語1. test", md);
}

#[test]
fn multibyte_atx_heading_escape() {
    let md = convert_faithful("<p># héading</p>").unwrap();
    assert_eq!(r"\# héading", md);
}

#[test]
fn multibyte_atx_heading_escape_umlaut() {
    let md = convert_faithful("<p>## über</p>").unwrap();
    assert_eq!(r"\## über", md);
}
