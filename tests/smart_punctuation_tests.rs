use htmd::{
    HtmlToMarkdown,
    options::{Options, TranslationMode},
};

fn convert(html: &str, mode: TranslationMode) -> String {
    HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: mode,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap()
}

#[test]
fn reverses_typographic_characters_in_both_modes() {
    for mode in [TranslationMode::Faithful, TranslationMode::Pure] {
        for (html, expected) in [
            ("<p>“Hello,” said Bob.</p>", "\"Hello,\" said Bob."),
            ("<p>It’s fine.</p>", "It's fine."),
            ("<p>Wait…</p>", "Wait..."),
            ("<p>2020–2025</p>", "2020--2025"),
            ("<p>Wait—stop!</p>", "Wait---stop!"),
            ("<p>&ldquo;Hello&rdquo;</p>", "\"Hello\""),
            ("<p>Wait&hellip;</p>", "Wait..."),
            ("<p>‘Hello’</p>", "'Hello'"),
            ("<p>“Wait…” — really?</p>", "\"Wait...\" --- really?"),
        ] {
            assert_eq!(convert(html, mode), expected, "input: {html}");
        }
    }
}

#[test]
fn keeps_literal_code_punctuation() {
    for mode in [TranslationMode::Faithful, TranslationMode::Pure] {
        assert_eq!(convert("<p><code>“Wait…”</code></p>", mode), "`“Wait…”`");
    }
}

#[test]
fn faithful_preserves_structural_and_quote_exceptions() {
    for html in [
        "<p>—</p>",
        "<ul><li>—</li></ul>",
        "<blockquote><p>—</p></blockquote>",
        "<p>‘Twas the night</p>",
    ] {
        assert_eq!(convert(html, TranslationMode::Faithful), html);
    }
}

#[test]
fn escapes_original_ascii_markers() {
    assert_eq!(convert("<p>---</p>", TranslationMode::Faithful), r"\---");
    assert_eq!(
        convert("<p># Hello</p>", TranslationMode::Faithful),
        r"\# Hello"
    );
}

#[test]
fn retained_block_html_preserves_punctuation_and_attributes() {
    let html = r#"<div data-label="“unchanged”">“Hello” <code>“code…”</code> Wait…</div>"#;
    assert_eq!(convert(html, TranslationMode::Faithful), html);
}

#[test]
fn retained_paragraph_preserves_typography_consistently() {
    let html = "<p>‘Tis a lovely day…</p>";
    assert_eq!(convert(html, TranslationMode::Faithful), html);
    assert_eq!(convert(html, TranslationMode::Pure), "'Tis a lovely day...");
}

#[test]
fn normal_paragraph_still_reverses_smart_punctuation() {
    assert_eq!(
        convert("<p>“Hello”…</p>", TranslationMode::Faithful),
        "\"Hello\"..."
    );
}

#[test]
fn isolated_em_dash_in_mixed_containers_keeps_html_structure() {
    for html in [
        "<ul><li>First</li><li>—</li></ul>",
        "<blockquote><p>First</p><p>—</p></blockquote>",
        "<ol><li>First</li><li>—</li></ol>",
    ] {
        let output = convert(html, TranslationMode::Faithful);
        assert!(output.contains("—"), "{html}: {output}");
        assert!(!output.contains("---"), "{html}: {output}");
    }
}

#[test]
fn unmatched_opening_quotes_fall_back_in_faithful_mode() {
    for html in ["<p>‘Tis the season.</p>", "<p>‘Twas the night.</p>"] {
        assert_eq!(convert(html, TranslationMode::Faithful), html);
    }
    assert_eq!(
        convert("<p>‘Hello’</p>", TranslationMode::Faithful),
        "'Hello'"
    );
}

#[test]
fn literal_code_em_dash_does_not_force_container_html_fallback() {
    for html in [
        "<ul><li><code>—</code></li><li>“Hello”</li></ul>",
        "<blockquote><p><code>—</code></p><p>“Hello”</p></blockquote>",
    ] {
        let output = convert(html, TranslationMode::Faithful);
        assert!(output.contains("`—`"), "input: {html}; output: {output}");
        assert!(
            output.contains("\"Hello\""),
            "ordinary text should be reversed: input: {html}; output: {output}"
        );
        assert!(
            !output.contains("<ul>") && !output.contains("<blockquote>"),
            "code punctuation must not force a raw container: input: {html}; output: {output}"
        );
    }
}

#[test]
fn unrelated_apostrophe_does_not_close_leading_open_quote() {
    let html = "<p>‘Tis what they call ’em.</p>";
    assert_eq!(convert(html, TranslationMode::Faithful), html);
    assert_eq!(
        convert(html, TranslationMode::Pure),
        "'Tis what they call 'em."
    );
}

#[test]
fn unmatched_opening_quote_in_heading_is_preserved_in_faithful_mode() {
    for html in [
        "<h1>‘Tis the season.</h1>",
        "<h2>‘Twas the night.</h2>",
        "<h3>‘Tis what they call ’em.</h3>",
    ] {
        assert_eq!(convert(html, TranslationMode::Faithful), html);
    }
    // Ordinary balanced quotations still use Markdown heading syntax.
    assert_eq!(
        convert("<h1>‘Hello’</h1>", TranslationMode::Faithful),
        "# 'Hello'"
    );
    assert_eq!(
        convert("<h1>‘Tis the season.</h1>", TranslationMode::Pure),
        "# 'Tis the season."
    );
}
