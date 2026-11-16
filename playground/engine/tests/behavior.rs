//! Spec behavior beyond the Expected results, checked through the
//! playground's own API: presets, Skip tags end to end, limits, and
//! permalinks restoring what the converter shows
//! (`pulldown-cmark-htmd-demo-2/htmddemo2-spec.md`).

use htmd_playground_core::codec::{Decoded, decode, encode};
use htmd_playground_core::presets::{
    HTML_PRESETS, MARKDOWN_PRESETS, default_example, replacing_needs_confirmation,
};
use htmd_playground_core::settings::*;
use htmd_playground_core::{LARGE_INPUT_BYTES, MAX_ADDRESS_LENGTH};
use htmd_playground_engine::{html_to_markdown, markdown_to_html};

fn convert(state: &State) -> String {
    match state.direction {
        Direction::HtmlToMarkdown => html_to_markdown(&state.input, &state.settings).unwrap(),
        Direction::MarkdownToHtml => markdown_to_html(&state.input, state.settings.extensions),
    }
}

/// Examples: the HTML presets are the spec's table, in order, and the HTML
/// Basic preset is the default example.
#[test]
fn html_presets_follow_the_spec_table() {
    let names: Vec<_> = HTML_PRESETS.iter().map(|preset| preset.name).collect();
    assert_eq!(
        names,
        [
            "Basic",
            "Headings",
            "Lists",
            "Links and images",
            "Code",
            "Tables",
            "Blockquotes",
            "Math",
            "Article",
            "Edge cases",
        ]
    );
    assert_eq!(default_example(), HTML_PRESETS[0].text);
    assert!(!replacing_needs_confirmation(default_example()));
}

/// The page opens with the default example converted, and every preset
/// converts without error.
#[test]
fn every_preset_converts_with_default_settings() {
    for preset in HTML_PRESETS {
        let markdown = html_to_markdown(preset.text, &Settings::default())
            .unwrap_or_else(|e| panic!("{}: {e}", preset.name));
        assert!(!markdown.trim().is_empty(), "{}", preset.name);
    }
    for preset in MARKDOWN_PRESETS {
        let html = markdown_to_html(preset.text, Extensions::default());
        assert!(!html.trim().is_empty(), "{}", preset.name);
    }
}

/// Skip tags: names are case-insensitive, separated by commas or spaces, and
/// invalid entries are ignored without stopping the valid ones.
#[test]
fn skip_tags_apply_end_to_end() {
    let html = "<p>a</p><script>s()</script><style>p{}</style><p>b</p>";
    let mut settings = Settings::default();
    settings.htmd.skip_tags = "SCRIPT, <bad> Style 1x".into();
    assert_eq!(
        html_to_markdown(html, &settings).unwrap().trim_end(),
        "a\n\nb"
    );
    assert_eq!(
        parse_skip_tags(&settings.htmd.skip_tags).ignored,
        ["<bad>", "1x"]
    );

    settings.htmd.skip_tags = "1x <bad>".into();
    assert_eq!(
        html_to_markdown(html, &settings).unwrap().trim_end(),
        "a\n\ns()\n\np{}\n\nb"
    );
}

/// Line break is disabled in Faithful mode, which always writes `<br>`.
#[test]
fn faithful_mode_ignores_the_line_break_option() {
    let mut settings = Settings::default();
    settings.htmd.translation_mode = TranslationMode::Faithful;
    settings.htmd.br_style = BrStyle::Backslash;
    assert_eq!(
        html_to_markdown("<p>a<br>b</p>", &settings)
            .unwrap()
            .trim_end(),
        "a<br>b"
    );
}

/// Limits and failures, Permalinks: the two size limits are the spec's.
#[test]
fn limits_match_the_spec() {
    assert_eq!(LARGE_INPUT_BYTES, 1_048_576);
    assert_eq!(MAX_ADDRESS_LENGTH, 32_768);
}

/// Permalinks: each option that differs from its default adds exactly one
/// entry to the address, and options at their default add none.
#[test]
fn each_changed_option_is_recorded_once() {
    let base = encode(&State::default(), false);
    type Change = fn(&mut HtmdOptions);
    let changes: [Change; 14] = [
        |o| o.translation_mode = TranslationMode::Faithful,
        |o| o.heading_style = HeadingStyle::Setext,
        |o| o.hr_style = HrStyle::Dashes,
        |o| o.br_style = BrStyle::Backslash,
        |o| o.bullet_marker = BulletMarker::Dash,
        |o| o.bullet_spacing = 1,
        |o| o.number_spacing = 4,
        |o| o.code_block_style = CodeBlockStyle::Indented,
        |o| o.code_fence = CodeFence::Tildes,
        |o| o.link_style = LinkStyle::Referenced,
        |o| o.reference_style = ReferenceStyle::Shortcut,
        |o| o.preserve_code_whitespace = true,
        |o| o.noscript_as_raw_text = false,
        |o| o.skip_tags = "script".into(),
    ];
    for (i, change) in changes.iter().enumerate() {
        let mut state = State::default();
        change(&mut state.settings.htmd);
        let fragment = encode(&state, false);
        assert_eq!(
            fragment.matches('&').count(),
            base.matches('&').count() + 1,
            "option {i}: {fragment}"
        );
        assert_eq!(decode(&fragment), Decoded::State(state), "option {i}");
    }
    for ext in Extension::ALL {
        let mut state = State::default();
        let on = !Extensions::default().contains(ext);
        state.settings.extensions.set(ext, on);
        let fragment = encode(&state, false);
        assert_ne!(fragment, base, "{}", ext.label());
        assert_eq!(decode(&fragment), Decoded::State(state), "{}", ext.label());
    }
}

/// Permalinks: opening an address restores the direction, input and options
/// exactly, so the converter shows the same output.
#[test]
fn opening_an_address_reproduces_the_output() {
    for (direction, presets) in [
        (Direction::HtmlToMarkdown, HTML_PRESETS),
        (Direction::MarkdownToHtml, MARKDOWN_PRESETS),
    ] {
        for preset in presets {
            let mut state = State {
                direction,
                input: preset.text.to_string(),
                settings: Settings::default(),
            };
            state.settings.htmd.heading_style = HeadingStyle::Setext;
            if let Some(ext) = preset.extension {
                state.settings.extensions.set(ext, true);
            }
            let Decoded::State(opened) = decode(&format!("#{}", encode(&state, true))) else {
                panic!("{}: invalid address", preset.name);
            };
            assert_eq!(opened, state, "{}", preset.name);
            assert_eq!(convert(&opened), convert(&state), "{}", preset.name);
        }
    }
}
