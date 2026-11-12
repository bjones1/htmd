use pretty_assertions::assert_eq;
use pulldown_cmark::Options as ParserOptions;

mod common;
use common::{convert_faithful, convert_pure, render_markdown};

#[test]
fn faithful_specification_examples() {
    let examples = [
        (r#"<del>old text</del>"#, r#"~~old text~~"#),
        (r#"a<del>b</del>c"#, r#"a~~b~~c"#),
        (r#"a<del>!</del>b"#, r#"a<del>!</del>b"#),
        (r#"<del><em>old</em></del>"#, r#"<del>*old*</del>"#),
        (r#"<del><a href="u">old</a></del>"#, r#"<del>[old](u)</del>"#),
        (r#"<del>a<br>b</del>"#, r#"~~a<br>b~~"#),
        (r#"<del><br></del>"#, r#"<del><br></del>"#),
        (r#"a<del></del>b"#, r#"a<del></del>b"#),
        (r#"a<del> </del>b"#, r#"a<del> </del>b"#),
        (r#"a<del> b </del>c"#, r#"a<del> b </del>c"#),
        (r#"<del>a</del><del>b</del>"#, r#"<del>a</del><del>b</del>"#),
        (r#"<del><del>a</del></del>"#, r#"<del>~~a~~</del>"#),
        (r#"<del cite="u">a</del>"#, r#"<del cite="u">a</del>"#),
        (r#"<s>a</s>"#, r#"<s>a</s>"#),
        (r#"<del><em><del>a</del></em></del>"#, r#"<del><em>~~a~~</em></del>"#),
        (r#"<del><p>a</p><p>b</p></del>"#, r#"<del><p>a</p><p>b</p></del>"#),
        (r#"<del>strike <em>bang</em> through</del>"#, r#"~~strike *bang* through~~"#),
        (r#"<em>bang <del>strike</del> boom</em>"#, r#"*bang ~~strike~~ boom*"#),
        (r#"<del><span><p>a</p></span></del>"#, r#"<del><span><p>a</p></span></del>"#),
        (r#"<em><del>a</del></em>"#, r#"<em>~~a~~</em>"#),
        (r#"<strong><del>a</del></strong>"#, r#"<strong>~~a~~</strong>"#),
        (r#"<del>a~b</del>"#, r#"~~a\~b~~"#),
    ];
    for (html, expected) in examples {
        assert_eq!(expected, convert_faithful(html).unwrap(), "{html}");
    }
}

#[test]
fn pure_mode_handles_lossy_cases() {
    let examples = [
        ("<del>old</del>", "~~old~~"),
        ("<s>old</s>", "~~old~~"),
        ("<strike>old</strike>", "~~old~~"),
        (r#"<del cite="u">old</del>"#, "~~old~~"),
        ("a<del></del>b", "ab"),
        ("a<del> </del>b", "a b"),
        ("a<del> b </del>c", "a ~~b~~ c"),
        ("a<del>!</del>b", "a!b"),
        ("a<del> ! </del>b", "a ! b"),
        ("<del><del>a</del></del>", "~~a~~"),
        ("<del>a</del><del>b</del>", "~~ab~~"),
        ("<del>a</del><s>b</s>", "~~ab~~"),
        ("<s>a</s><strike>b</strike>", "~~ab~~"),
        ("<del><em>a</em></del><del>b</del>", "*a*~~b~~"),
        (r#"<del cite="u">a</del><del>b</del>"#, "~~ab~~"),
        ("<del>a</del><!--x--><del>b</del>", "~~ab~~"),
        ("<del>a</del><span><del>b</del></span>", "~~ab~~"),
        ("<span><del>a</del></span><strike>b</strike>", "~~ab~~"),
        ("<del>a</del><span></span><strike>b</strike>", "~~ab~~"),
        ("<del>a</del><s>b</s><strike>c</strike>", "~~abc~~"),
        ("<del>a</del> <s>b</s>", "~~a~~ ~~b~~"),
        ("<del><p>a</p><p>b</p></del>", "a\n\nb"),
        ("<del><span><p>a</p></span></del>", "a"),
        (r#"<del>a!</del><a href="u">x</a>"#, r"a\![x](u)"),
    ];
    for (html, expected) in examples {
        assert_eq!(expected, convert_pure(html).unwrap(), "{html}");
    }
}

#[test]
fn literal_tildes_are_escaped_without_changing_literal_contexts() {
    let examples = [
        ("a~~~b", r"a\~\~\~b"),
        ("~~~", r"\~\~\~"),
        ("<del>~a</del>", r"~~\~a~~"),
        ("<del>a~</del>", r"<del>a\~</del>"),
        ("<code>~a~</code>", "`~a~`"),
        (r#"<span class="math math-inline">~a~</span>"#, "$~a~$"),
        (
            r#"<a href="https://x/~u">https://x/~u</a>"#,
            r"[https://x/\~u](https://x/~u)",
        ),
        (r#"<del cite="~u">a</del>"#, r#"<del cite="~u">a</del>"#),
        (r#"<img src="u" alt="~a~">"#, r"![\~a\~](u)"),
    ];
    for (html, expected) in examples {
        assert_eq!(expected, convert_faithful(html).unwrap(), "{html}");
    }
}

#[test]
fn autolink_destinations_keep_literal_tildes() {
    let options = htmd::options::Options {
        link_style: htmd::options::LinkStyle::InlinedPreferAutolinks,
        ..Default::default()
    };
    let markdown = common::convert_faithful_options(
        r#"<a href="https://x/~u">https://x/~u</a>"#,
        options,
    )
    .unwrap();
    assert_eq!("<https://x/~u>", markdown);
    assert_eq!(
        "<p><a href=\"https://x/~u\">https://x/~u</a></p>\n",
        render_markdown(&markdown, ParserOptions::ENABLE_STRIKETHROUGH)
    );
}

#[test]
fn faithful_preserves_unicode_edges_and_complex_adjacency() {
    for html in [
        "a<del>\u{a0}b</del>c",
        "a<del>b\u{a0}</del>c",
        "a<del>b—</del>c",
        "a<del>b★</del>c",
        "<del>a<s>b</s></del>",
        "<del>x<iframe></iframe>y</del>",
        "<del><em>a</em></del><del>b</del>",
        "<del>a</del><del><em>b</em></del>",
    ] {
        let expected = html.replace("<em>a</em>", "*a*").replace("<em>b</em>", "*b*");
        assert_eq!(expected, convert_faithful(html).unwrap(), "{html}");
    }
    assert_eq!("~~a~~ ~~b~~", convert_faithful("<del>a</del> <del>b</del>").unwrap());
}

#[test]
fn supported_markdown_round_trips_with_strikethrough_enabled() {
    let options = ParserOptions::ENABLE_STRIKETHROUGH | ParserOptions::ENABLE_TABLES;
    let examples = [
        ("<p>a<del>b</del>c</p>", "<p>a<del>b</del>c</p>\n"),
        ("<p>a<del>!b</del></p>", "<p>a<del>!b</del></p>\n"),
        ("<p><del>strike <em>bang</em> through</del></p>",
         "<p><del>strike <em>bang</em> through</del></p>\n"),
        ("<h2><del>old</del></h2>", "<h2><del>old</del></h2>\n"),
        ("<p><del>a<br>b</del></p>", "<p><del>a<br>b</del></p>\n"),
        ("<p><del>a~b</del></p>", "<p><del>a~b</del></p>\n"),
    ];
    for (html, expected) in examples {
        let markdown = convert_faithful(html).unwrap();
        assert_eq!(expected, render_markdown(&markdown, options), "{markdown}");
    }
    let markdown = convert_faithful("<table><tr><th><del>old</del></th></tr><tr><td>new</td></tr></table>").unwrap();
    assert!(markdown.contains("~~old~~"));
    assert!(render_markdown(&markdown, options).contains("<th><del>old</del></th>"));
}

#[test]
fn pure_adjacent_strikethrough_has_no_visible_delimiters() {
    for html in [
        "<del>a</del><s>b</s>",
        r#"<del cite="u">a</del><del>b</del>"#,
        "<del>a</del><!--x--><del>b</del>",
        "<del>a</del><span><del>b</del></span>",
        "<span><del>a</del></span><strike>b</strike>",
    ] {
        let markdown = convert_pure(html).unwrap();
        assert_eq!(
            "<p><del>ab</del></p>\n",
            render_markdown(&markdown, ParserOptions::ENABLE_STRIKETHROUGH),
            "{html}: {markdown}"
        );
    }
    let markdown = convert_pure("<del><em>a</em></del><del>b</del>").unwrap();
    assert_eq!(
        "<p><em>a</em><del>b</del></p>\n",
        render_markdown(&markdown, ParserOptions::ENABLE_STRIKETHROUGH)
    );
    assert_eq!("x\n\ny", convert_pure("<del>x<iframe></iframe>y</del>").unwrap());
}

#[test]
fn parser_punctuation_regression() {
    for markdown in ["a~~!~~b", "~~*old*~~x", "a~~<br>~~b"] {
        let html = render_markdown(markdown, ParserOptions::ENABLE_STRIKETHROUGH);
        assert!(!html.contains("<del>"), "{markdown}: {html}");
    }
    assert_eq!("<p>a<del>b</del>c</p>\n", render_markdown("a~~b~~c", ParserOptions::ENABLE_STRIKETHROUGH));
}
