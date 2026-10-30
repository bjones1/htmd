use htmd::{
    Element, HtmlToMarkdown,
    element_handler::{ElementHandler, EventSubscription, EventTypes, HandlerResult, Handlers},
    options::{LinkStyle, Options, TranslationMode},
};
use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Node, NodeData, RcDom};
use pulldown_cmark::Options as ParserOptions;
use std::sync::{Arc, Mutex};

mod common;
use common::{convert_faithful, convert_faithful_options, convert_pure, render_markdown};

fn render(markdown: &str) -> String {
    render_markdown(markdown, ParserOptions::ENABLE_DEFINITION_LIST)
}

fn structure(html: &str) -> Vec<String> {
    fn visit(node: &Node, output: &mut Vec<String>) {
        let children = node.children.borrow();
        let structural = match &node.data {
            NodeData::Document => true,
            NodeData::Element { name, .. } => {
                matches!(name.local.as_ref(), "html" | "head" | "body" | "dl")
                    || (name.local.as_ref() == "dd" && children.iter().any(|child|
                        matches!(&child.data, NodeData::Element { name, .. } if name.local.as_ref() == "p")))
            }
            _ => false,
        };
        match &node.data {
            NodeData::Element { name, attrs, .. } => {
                output.push(format!("<{name:?} {:?}>", attrs.borrow()))
            }
            NodeData::Text { contents } => {
                output.push(format!("text:{:?}", contents.borrow().as_ref()))
            }
            NodeData::Comment { contents } => output.push(format!("comment:{contents}")),
            _ => {}
        }
        for child in children.iter() {
            if structural
                && matches!(&child.data, NodeData::Text { contents }
                if contents.borrow().chars().all(|c| matches!(c, ' ' | '\n' | '\t' | '\r' | '\u{c}')))
            {
                continue;
            }
            visit(child, output);
        }
        if let NodeData::Element { name, .. } = &node.data {
            output.push(format!("</{name:?}>"));
        }
    }
    let dom = parse_document(RcDom::default(), Default::default()).one(html);
    let mut output = Vec::new();
    visit(&dom.document, &mut output);
    output
}

#[test]
fn parser_generated_lists_round_trip() {
    for markdown in [
        "T\n: D",
        "T  X\n: D  E",
        "A\n\\:B\n: D",
        "T\n\n: first\n  second\n\n  third",
        "T\n: D1\n: D2",
        "A\n: D1\n\nB\n: D2",
        "A\nB\n: D",
        "*A\nB*\n: D",
        "*T*\n: **D** and [link](https://example.com) and `code`",
        "T\n: first\n  second",
        "T\n: first\n\n  second",
        "T\n\n: D1\n\n: D2",
        "T\n:",
        "T\n:\n: D",
        "A\n:\nB\n: D",
        "A\n\n: P\n\nB\n: Q",
        "T\n: D\n:",
        "T\n\n:\n: D",
        "T\n: a:b and :literal",
        "T\n: \\:literal",
        "before\n\nT\n: D\n\nafter",
    ] {
        let html = render(markdown);
        for mode in [TranslationMode::Faithful, TranslationMode::Pure] {
            let converted = HtmlToMarkdown::builder()
                .options(Options {
                    translation_mode: mode,
                    ..Options::default()
                })
                .build()
                .convert(&html)
                .unwrap();
            assert!(
                !converted.contains("<dl>"),
                "unexpected fallback: {markdown:?}: {converted:?}"
            );
            assert_eq!(
                structure(&html),
                structure(&render(&converted)),
                "{mode:?}: {markdown:?} -> {converted:?}"
            );
        }
    }
}

#[test]
fn literal_colons_only_escape_at_line_starts() {
    for (html, expected) in [
        ("<p>A</p><p>:literal</p>", "A\n\n\\:literal"),
        ("<p>a:b</p>", "a:b"),
        ("<p><em>a</em>:b</p>", "*a*:b"),
        ("<p><code>:literal</code></p>", "`:literal`"),
        (
            "<p><a href=\"https://example.com\">a:b</a></p>",
            "[a:b](https://example.com)",
        ),
    ] {
        let markdown = convert_faithful(html).unwrap();
        assert_eq!(expected, markdown);
        assert_eq!(structure(html), structure(&render(&markdown)), "{html}");
    }
    assert_eq!("\\:literal", convert_faithful("<p>  :literal</p>").unwrap());
    assert_eq!("a  \n \\:b", convert_pure("<p>a<br> :b</p>").unwrap());
}

#[test]
fn unsupported_structures_fall_back_without_losing_dom() {
    for html in [
        "<dl></dl>",
        "<dl><dt></dt><dd>D</dd></dl>",
        "<dl><dt>A\n\nB</dt><dd>D</dd></dl>",
        "<dl><dt>T</dt><dd>D\n\nE</dd></dl>",
        "<dl><dt>T</dt><dd> </dd></dl>",
        "<dl><dt>T</dt><dd><p> </p></dd></dl>",
        "<dl><dt>T</dt></dl>",
        "<dl><dd>D</dd></dl>",
        "<dl><dt>A</dt><dt>B</dt><dd>D</dd></dl>",
        "<dl id=\"x\"><dt>T</dt><dd>D</dd></dl>",
        "<dl><dt class=\"x\">T</dt><dd>D</dd></dl>",
        "<dl><dt>T</dt><dd><p></p></dd></dl>",
        "<dl><dt>T</dt><dd>D</dd><dd><p>P</p></dd></dl>",
        "<dl><dt>T</dt><!--note--><dd>D</dd></dl>",
        "<dl><dt>T</dt><dd><ul><li>D</li></ul></dd></dl>",
        "<dl><dt>A</dt><dd>D</dd></dl><dl><dt>B</dt><dd>E</dd></dl>",
        "<dl><dt>A</dt><dd>D</dd></dl><p>after</p>",
    ] {
        let markdown = convert_faithful(html).unwrap();
        assert_eq!(
            structure(html),
            structure(&render(&markdown)),
            "{html} -> {markdown}"
        );
    }
    assert_eq!(
        "A\n\nB\n\nD",
        convert_pure("<dl><dt>A</dt><dt>B</dt><dd>D</dd></dl>").unwrap()
    );
}

#[derive(Clone, Default)]
struct Listener(Arc<Mutex<Vec<String>>>);
impl ElementHandler for Listener {
    fn handle(&self, _: &dyn Handlers, _: &Element) -> Option<HandlerResult> {
        None
    }
    fn event_subscription(&self, _: &Options) -> EventSubscription {
        EventSubscription {
            events: EventTypes::ELEMENT_EVENTS,
            tags: &["dl", "dt", "dd", "em"],
        }
    }
    fn on_element_enter(&self, element: &Element) {
        self.0.lock().unwrap().push(format!("+{}", element.tag));
    }
    fn on_element_leave(&self, element: &Element, _: Option<&HandlerResult>) {
        self.0.lock().unwrap().push(format!("-{}", element.tag));
    }
}

#[test]
fn custom_handlers_and_events_use_normal_dispatch_once() {
    let listener = Listener::default();
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ..Options::default()
        })
        .add_handler(vec!["watcher"], listener.clone())
        .add_handler(vec!["dl"], |handlers: &dyn Handlers, element: &Element| {
            handlers.fallback(element)
        })
        .add_handler(vec!["dt"], |handlers: &dyn Handlers, element: &Element| {
            handlers.fallback(element)
        })
        .add_handler(vec!["em"], |_: &dyn Handlers, _: &Element| {
            Some("**custom**".into())
        })
        .build();
    assert_eq!(
        "T\n: **custom**",
        converter
            .convert("<dl><dt>T</dt><dd><em>D</em></dd></dl>")
            .unwrap()
    );
    assert_eq!(
        *listener.0.lock().unwrap(),
        ["+dl", "+dt", "-dt", "+dd", "+em", "-em", "-dd", "-dl"]
    );
}

#[test]
fn reference_links_survive_success_and_roll_back_on_fallback() {
    let options = Options {
        link_style: LinkStyle::Referenced,
        ..Options::default()
    };
    let html = "<dl><dt>T</dt><dd><a href=\"/url\">link</a></dd></dl>";
    let markdown = convert_faithful_options(html, options.clone()).unwrap();
    assert_eq!(structure(html), structure(&render(&markdown)));
    let html = "<dl><dt>T</dt><dd><a href=\"/url\">link</a><span class=\"x\">text</span></dd></dl>";
    let markdown = convert_faithful_options(html, options).unwrap();
    assert!(!markdown.contains("[1]:"), "{markdown}");
    assert_eq!(structure(html), structure(&render(&markdown)));
}

#[test]
fn overrides_skips_and_orphan_items_keep_their_meaning() {
    let html = "<dl><dt>T</dt><dd>D</dd><dd>E</dd></dl>";
    let converter = HtmlToMarkdown::builder()
        .add_handler(vec!["dl"], |_: &dyn Handlers, _: &Element| {
            Some("replacement".into())
        })
        .build();
    assert_eq!("replacement", converter.convert(html).unwrap());
    let converter = HtmlToMarkdown::builder()
        .add_handler(vec!["dt"], |_: &dyn Handlers, _: &Element| {
            Some("term".into())
        })
        .add_handler(vec!["dd"], |_: &dyn Handlers, _: &Element| {
            Some("definition".into())
        })
        .build();
    assert_eq!(
        "term\n: definition\n: definition",
        converter.convert(html).unwrap()
    );
    let converter = HtmlToMarkdown::builder().skip_tags(vec!["dd"]).build();
    assert_eq!("T", converter.convert(html).unwrap());
    let orphan = "<dt>T</dt><dd>D</dd>";
    assert_eq!(
        structure(orphan),
        structure(&render(&convert_faithful(orphan).unwrap()))
    );
    assert_eq!(
        "T\n: D",
        convert_pure("<dl id=\"x\"><dt class=\"x\">T</dt><dd lang=\"en\">D</dd></dl>").unwrap()
    );
}

#[test]
fn fallback_does_not_replay_descendant_events() {
    let listener = Listener::default();
    let converter = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ..Options::default()
        })
        .add_handler(vec!["watcher"], listener.clone())
        .build();
    let html = "<dl><dt>T</dt><dd><em class=\"x\">D</em></dd></dl>";
    assert_eq!(
        structure(html),
        structure(&render(&converter.convert(html).unwrap()))
    );
    assert_eq!(
        *listener.0.lock().unwrap(),
        ["+dl", "+dt", "-dt", "+dd", "+em", "-em", "-dd", "-dl"]
    );
}

#[test]
fn regression_source_whitespace_does_not_create_hard_breaks() {
    for (html, normalized) in [
        (
            "<dl><dt>A  \nB</dt><dd>D</dd></dl>",
            "<dl><dt>A\nB</dt><dd>D</dd></dl>",
        ),
        (
            "<dl><dt>T</dt><dd>A  \nB</dd></dl>",
            "<dl><dt>T</dt><dd>A\nB</dd></dl>",
        ),
        (
            "<dl><dt>T</dt><dd><p>A  \nB</p></dd></dl>",
            "<dl><dt>T</dt><dd><p>A\nB</p></dd></dl>",
        ),
        (
            "<dl><dt>A\t \nB</dt><dd>D</dd></dl>",
            "<dl><dt>A\nB</dt><dd>D</dd></dl>",
        ),
    ] {
        let faithful = render(&convert_faithful(html).unwrap());
        assert!(!faithful.contains("<br"), "{faithful}");
        assert_eq!(structure(html), structure(&faithful));
        let pure = render(&convert_pure(html).unwrap());
        assert!(!pure.contains("<br"), "{pure}");
        assert_eq!(structure(normalized), structure(&pure));
    }
}

#[test]
fn regression_unsupported_descendants_keep_normal_whitespace() {
    for (html, expected) in [
        (
            "<dl><dt>T</dt><dd><h1>A\nB</h1></dd></dl>",
            "<p>T</p><h1>A B</h1>",
        ),
        (
            "<dl><dt>T</dt><dd><p>A\nB</p><h1>C\nD</h1></dd></dl>",
            "<p>T</p><p>A B</p><h1>C D</h1>",
        ),
    ] {
        assert_eq!(
            structure(expected),
            structure(&render(&convert_pure(html).unwrap()))
        );
        assert_eq!(
            structure(html),
            structure(&render(&convert_faithful(html).unwrap()))
        );
    }
    assert_eq!(
        structure("<h1>A B</h1>"),
        structure(&render(&convert_pure("<h1>A\nB</h1>").unwrap()))
    );
}

#[test]
fn regression_skipped_separator_keeps_faithful_lists_separate() {
    let first = "<dl><dt>A</dt><dd>D</dd></dl>";
    let second = "<dl><dt>B</dt><dd>E</dd></dl>";
    let html = format!("{first}<hr>{second}");
    for mode in [TranslationMode::Faithful, TranslationMode::Pure] {
        let converter = HtmlToMarkdown::builder()
            .options(Options {
                translation_mode: mode,
                ..Options::default()
            })
            .skip_tags(vec!["hr"])
            .build();
        let actual = render(&converter.convert(&html).unwrap());
        assert!(!actual.contains("<hr"), "{actual}");
        let expected = if mode == TranslationMode::Faithful {
            format!("{first}{second}")
        } else {
            // Pure mode retains the contents but does not promise separate containers.
            "<dl><dt>A</dt><dd>D</dd><dt>B</dt><dd>E</dd></dl>".to_string()
        };
        assert_eq!(structure(&expected), structure(&actual), "{mode:?}");
    }
}

fn assert_carriage_returns_do_not_create_hard_breaks(ending: &str) {
    for template in [
        "<dl><dt>A  ENDINGB</dt><dd>D</dd></dl>",
        "<dl><dt>T</dt><dd>A  ENDINGB</dd></dl>",
        "<dl><dt>T</dt><dd><p>A  ENDINGB</p></dd></dl>",
    ] {
        let html = template.replace("ENDING", ending);
        let faithful_markdown = convert_faithful(&html).unwrap();
        assert!(faithful_markdown.contains("<dl>"), "{faithful_markdown}");
        let faithful = render(&faithful_markdown);
        assert!(!faithful.contains("<br"), "{faithful}");
        // Raw HTML line endings normalize to LF when the fallback is reparsed.
        assert_eq!(
            structure(&template.replace("ENDING", "\n")),
            structure(&faithful)
        );
        let pure = render(&convert_pure(&html).unwrap());
        assert!(!pure.contains("<br"), "{pure}");
        assert_eq!(
            structure(&template.replace("  ENDING", "\n")),
            structure(&pure)
        );
    }
}

#[test]
fn regression_cr_does_not_create_hard_breaks() {
    assert_carriage_returns_do_not_create_hard_breaks("&#13;");
    assert_carriage_returns_do_not_create_hard_breaks("\r");
}

#[test]
fn regression_crlf_does_not_create_hard_breaks() {
    assert_carriage_returns_do_not_create_hard_breaks("&#13;&#10;");
    assert_carriage_returns_do_not_create_hard_breaks("\r\n");
}

#[test]
fn regression_source_newline_after_br_keeps_one_hard_break() {
    for template in [
        "<dl><dt>A<br>ENDINGB</dt><dd>D</dd></dl>",
        "<dl><dt>T</dt><dd>A<br>ENDINGB</dd></dl>",
        "<dl><dt>T</dt><dd><p>A<br>ENDINGB</p></dd></dl>",
    ] {
        for ending in ["\n", "\r", "\r\n"] {
            let html = template.replace("ENDING", ending);
            for mode in [TranslationMode::Faithful, TranslationMode::Pure] {
                for br_style in [
                    htmd::options::BrStyle::TwoSpaces,
                    htmd::options::BrStyle::Backslash,
                ] {
                    let converter = HtmlToMarkdown::builder()
                        .options(Options {
                            translation_mode: mode,
                            br_style,
                            ..Options::default()
                        })
                        .build();
                    let markdown = converter.convert(&html).unwrap();
                    assert_eq!(
                        structure(&html),
                        structure(&render(&markdown)),
                        "{mode:?}: {markdown:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn regression_suppressed_terms_leave_content_without_markers() {
    let html = "<dl><dt>T</dt><dd>D</dd><dd>E</dd></dl>";
    for mode in [TranslationMode::Faithful, TranslationMode::Pure] {
        let options = Options {
            translation_mode: mode,
            ..Options::default()
        };
        let skipped = HtmlToMarkdown::builder()
            .options(options.clone())
            .skip_tags(vec!["dt"])
            .build();
        let custom = HtmlToMarkdown::builder()
            .options(options.clone())
            .add_handler(vec!["dt"], |_: &dyn Handlers, _: &Element| Some("".into()))
            .build();
        for converter in [skipped, custom] {
            let markdown = converter.convert(html).unwrap();
            assert_eq!("D\n\nE", markdown);
            assert_eq!(structure("<p>D</p><p>E</p>"), structure(&render(&markdown)));
        }
        let selective = HtmlToMarkdown::builder()
            .options(options)
            .add_handler(vec!["dt"], |handlers: &dyn Handlers, element: &Element| {
                let content = handlers.walk_children_content(element.node, element.context);
                if content == "skip" {
                    None
                } else {
                    Some(content.into())
                }
            })
            .build();
        let markdown = selective
            .convert("<dl><dt>keep</dt><dd>D</dd><dt>skip</dt><dd>E</dd></dl>")
            .unwrap();
        assert_eq!(
            structure("<dl><dt>keep</dt><dd>D</dd></dl><p>E</p>"),
            structure(&render(&markdown))
        );
    }
}

fn assert_inline_content_after_br(leading: &str) {
    for template in [
        "<dl><dt>A<br>INLINE</dt><dd>D</dd></dl>",
        "<dl><dt>T</dt><dd>A<br>INLINE</dd></dl>",
        "<dl><dt>T</dt><dd><p>A<br>INLINE</p></dd></dl>",
    ] {
        for tag in ["em", "strong"] {
            let html = template.replace("INLINE", &format!("<{tag}>{leading}B</{tag}>"));
            for br_style in [
                htmd::options::BrStyle::TwoSpaces,
                htmd::options::BrStyle::Backslash,
            ] {
                let converter = HtmlToMarkdown::builder()
                    .options(Options {
                        translation_mode: TranslationMode::Pure,
                        br_style,
                        ..Options::default()
                    })
                    .build();
                let markdown = converter.convert(&html).unwrap();
                // Inline delimiters move leading whitespace outside the element.
                let expected = template.replace("INLINE", &format!("\n<{tag}>B</{tag}>"));
                assert_eq!(
                    structure(&expected),
                    structure(&render(&markdown)),
                    "{br_style:?}: {html:?} -> {markdown:?}"
                );
            }
        }
    }
}

#[test]
fn regression_inline_newline_after_br_keeps_one_hard_break() {
    assert_inline_content_after_br("\n");
    assert_inline_content_after_br("\r\n");
}

#[test]
fn regression_inline_without_newline_after_br_is_unchanged() {
    assert_inline_content_after_br("");
}
