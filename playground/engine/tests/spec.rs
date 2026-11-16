//! The spec's Expected results and Acceptance criteria
//! (`pulldown-cmark-htmd-demo-2/htmddemo2-spec.md`).

use htmd_playground_core::presets::{HTML_PRESETS, MARKDOWN_PRESETS, default_example};
use htmd_playground_core::settings::*;
use htmd_playground_engine::{html_to_markdown, markdown_to_html};

/// The spec writes a newline as `⏎` and a space as `␣`.
fn spec_text(text: &str) -> String {
    text.replace('⏎', "\n").replace('␣', " ")
}

fn to_markdown(html: &str, settings: &Settings) -> String {
    html_to_markdown(html, settings).unwrap()
}

fn htmd(change: impl FnOnce(&mut HtmdOptions)) -> Settings {
    let mut settings = Settings::default();
    change(&mut settings.htmd);
    settings
}

fn extensions(change: impl FnOnce(&mut Extensions)) -> Settings {
    let mut settings = Settings::default();
    change(&mut settings.extensions);
    settings
}

#[test]
fn html_to_markdown_expected_results() {
    let link = r#"<p><a href="https://e.com">t</a></p>"#;
    let rust_block = r#"<pre><code class="language-rust">fn x() {}</code></pre>"#;
    let rows: Vec<(&str, Settings, &str)> = vec![
        ("<h1>Hi</h1>", Settings::default(), "#␣Hi"),
        (
            "<h1>Hi</h1>",
            htmd(|o| o.heading_style = HeadingStyle::Setext),
            "Hi⏎==",
        ),
        (
            "<p><strong>a</strong> and <em>b</em></p>",
            Settings::default(),
            "**a**␣and␣*b*",
        ),
        (
            "<ul><li>a</li><li>b</li></ul>",
            Settings::default(),
            "*␣␣␣a⏎*␣␣␣b",
        ),
        (
            "<ul><li>a</li><li>b</li></ul>",
            htmd(|o| {
                o.bullet_marker = BulletMarker::Dash;
                o.bullet_spacing = 1;
            }),
            "-␣a⏎-␣b",
        ),
        (
            r#"<ol start="3"><li>a</li></ol>"#,
            Settings::default(),
            "3.␣␣a",
        ),
        (
            r#"<ol start="3"><li>a</li></ol>"#,
            htmd(|o| o.number_spacing = 1),
            "3.␣a",
        ),
        ("<hr>", Settings::default(), "*␣*␣*"),
        ("<hr>", htmd(|o| o.hr_style = HrStyle::Dashes), "-␣-␣-"),
        ("<p>a<br>b</p>", Settings::default(), "a␣␣⏎b"),
        (
            "<p>a<br>b</p>",
            htmd(|o| o.br_style = BrStyle::Backslash),
            "a\\⏎b",
        ),
        (
            "<p>a<br>b</p>",
            htmd(|o| o.translation_mode = TranslationMode::Faithful),
            "a<br>b",
        ),
        (link, Settings::default(), "[t](https://e.com)"),
        (
            link,
            htmd(|o| o.link_style = LinkStyle::Referenced),
            "[t][1]⏎⏎[1]:␣https://e.com",
        ),
        (
            link,
            htmd(|o| {
                o.link_style = LinkStyle::Referenced;
                o.reference_style = ReferenceStyle::Collapsed;
            }),
            "[t][]⏎⏎[t]:␣https://e.com",
        ),
        (
            link,
            htmd(|o| {
                o.link_style = LinkStyle::Referenced;
                o.reference_style = ReferenceStyle::Shortcut;
            }),
            "[t]⏎⏎[t]:␣https://e.com",
        ),
        (
            r#"<p><a href="https://e.com">https://e.com</a></p>"#,
            htmd(|o| o.link_style = LinkStyle::InlineAutolinks),
            "<https://e.com>",
        ),
        (
            r#"<p><img src="i.png" alt="cat"></p>"#,
            Settings::default(),
            "![cat](i.png)",
        ),
        (rust_block, Settings::default(), "```rust⏎fn x() {}⏎```"),
        (
            rust_block,
            htmd(|o| o.code_fence = CodeFence::Tildes),
            "~~~rust⏎fn x() {}⏎~~~",
        ),
        (
            "<pre><code>fn x() {}</code></pre>",
            htmd(|o| o.code_block_style = CodeBlockStyle::Indented),
            "␣␣␣␣fn x() {}",
        ),
        ("<p><code> a </code></p>", Settings::default(), "`a`"),
        (
            "<p><code> a </code></p>",
            htmd(|o| o.preserve_code_whitespace = true),
            "`␣␣a␣␣`",
        ),
        (
            "<table><tr><th>a</th></tr><tr><td>b</td></tr></table>",
            Settings::default(),
            "|␣a␣|⏎|␣-␣|⏎|␣b␣|",
        ),
        (
            r#"<p><span class="math math-inline">x^2</span></p>"#,
            Settings::default(),
            "$x^2$",
        ),
        (
            r#"<p><span class="math math-display">x^2</span></p>"#,
            Settings::default(),
            "$$x^2$$",
        ),
        (r#"<p class="x">a</p>"#, Settings::default(), "a"),
        (
            r#"<p class="x">a</p>"#,
            htmd(|o| o.translation_mode = TranslationMode::Faithful),
            r#"<p class="x">a</p>"#,
        ),
        (
            "<p>a</p><script>alert(1)</script>",
            Settings::default(),
            "a⏎⏎alert(1)",
        ),
        (
            "<p>a</p><script>alert(1)</script>",
            htmd(|o| o.skip_tags = "SCRIPT".into()),
            "a",
        ),
        (
            "<p>a</p><noscript><b>x</b></noscript>",
            Settings::default(),
            "a⏎⏎\\<b>x\\</b>",
        ),
        (
            "<p>a</p><noscript><b>x</b></noscript>",
            htmd(|o| o.noscript_as_raw_text = false),
            "a⏎⏎**x**",
        ),
    ];
    for (input, settings, expected) in rows {
        let output = to_markdown(input, &settings);
        assert_eq!(
            output.strip_suffix('\n').unwrap_or(&output),
            spec_text(expected),
            "{input} with {settings:?}"
        );
    }
}

#[test]
fn markdown_to_html_expected_results() {
    let rows: Vec<(&str, Extensions, &str)> = vec![
        ("#␣Hi", Extensions::default(), "<h1>Hi</h1>"),
        (
            "$x^2$",
            Extensions::default(),
            r#"<p><span class="math math-inline">x^2</span></p>"#,
        ),
        (
            "$$x^2$$",
            Extensions::default(),
            r#"<p><span class="math math-display">x^2</span></p>"#,
        ),
        (
            "$x^2$",
            extensions(|e| e.set(Extension::Math, false)).extensions,
            "<p>$x^2$</p>",
        ),
        ("~~a~~", Extensions::default(), "<p>~~a~~</p>"),
        (
            "~~a~~",
            extensions(|e| e.set(Extension::Strikethrough, true)).extensions,
            "<p><del>a</del></p>",
        ),
        (
            "\"a\"␣--␣b",
            extensions(|e| e.set(Extension::SmartPunctuation, true)).extensions,
            "<p>“a” – b</p>",
        ),
    ];
    for (input, extensions, expected) in rows {
        let output = markdown_to_html(&spec_text(input), extensions);
        assert_eq!(output.trim_end_matches('\n'), expected, "{input}");
    }

    let table = markdown_to_html("| a |\n|---|\n| b |", Extensions::default());
    assert!(
        table.contains("<thead><tr><th>a</th></tr></thead>"),
        "{table}"
    );
    assert!(table.contains("<tbody>\n<tr><td>b</td></tr>"), "{table}");

    let mut tasks = Extensions::default();
    tasks.set(Extension::TaskLists, true);
    let task = markdown_to_html("- [x] a", tasks);
    assert!(
        task.contains(r#"<input disabled="" type="checkbox" checked=""/>"#),
        "{task}"
    );
}

#[test]
fn round_trip() {
    let html = markdown_to_html("# Hi\n\n$x^2$", Extensions::default());
    assert_eq!(to_markdown(&html, &Settings::default()), "# Hi\n\n$x^2$");
}

/// Acceptance: switching direction twice on the default example gives back
/// the original HTML, ignoring whitespace between tags and a trailing newline.
#[test]
fn double_switch_restores_the_default_example() {
    let markdown = to_markdown(default_example(), &Settings::default());
    let html = markdown_to_html(&markdown, Extensions::default());
    let normalize = |text: &str| text.replace(">\n<", "><").trim_end().to_string();
    assert_eq!(normalize(&html), normalize(default_example()));
}

/// Acceptance: every option visibly changes the result of at least one
/// preset, in Source or Rendered.
#[test]
fn every_option_changes_a_preset() {
    type Change = fn(&mut Settings);
    let htmd_changes: Vec<(&str, Change)> = vec![
        ("translation mode", |s| {
            s.htmd.translation_mode = TranslationMode::Faithful
        }),
        ("heading style", |s| {
            s.htmd.heading_style = HeadingStyle::Setext
        }),
        ("horizontal rule", |s| {
            s.htmd.hr_style = HrStyle::Underscores
        }),
        ("line break", |s| s.htmd.br_style = BrStyle::Backslash),
        ("bullet marker", |s| {
            s.htmd.bullet_marker = BulletMarker::Dash
        }),
        ("bullet spacing", |s| s.htmd.bullet_spacing = 1),
        ("number spacing", |s| s.htmd.number_spacing = 4),
        ("code block style", |s| {
            s.htmd.code_block_style = CodeBlockStyle::Indented
        }),
        ("code fence", |s| s.htmd.code_fence = CodeFence::Tildes),
        ("link style", |s| {
            s.htmd.link_style = LinkStyle::InlineAutolinks
        }),
        ("reference style", |s| {
            s.htmd.link_style = LinkStyle::Referenced;
            s.htmd.reference_style = ReferenceStyle::Shortcut;
        }),
        ("preserve code whitespace", |s| {
            s.htmd.preserve_code_whitespace = true
        }),
        ("noscript", |s| s.htmd.noscript_as_raw_text = false),
        ("skip tags", |s| s.htmd.skip_tags = "em".into()),
    ];
    for (name, change) in htmd_changes {
        let mut changed = Settings::default();
        change(&mut changed);
        // The reference style is compared against referenced links in the
        // default (full) style, since it has no effect otherwise.
        let mut baseline = Settings::default();
        if name == "reference style" {
            baseline.htmd.link_style = LinkStyle::Referenced;
        }
        assert!(
            HTML_PRESETS
                .iter()
                .any(|preset| to_markdown(preset.text, &baseline)
                    != to_markdown(preset.text, &changed)),
            "{name}"
        );
    }

    for ext in Extension::ALL {
        let changes_preset = |direction_presets: &[htmd_playground_core::presets::Preset]| {
            direction_presets.iter().any(|preset| {
                let mut on = Extensions::default();
                on.set(ext, true);
                let mut off = on;
                off.set(ext, false);
                if ext == Extension::OldFootnotes {
                    // Old-style footnotes only make a difference with
                    // footnotes on.
                    off.set(Extension::Footnotes, true);
                }
                markdown_to_html(preset.text, on) != markdown_to_html(preset.text, off)
            })
        };
        assert!(changes_preset(MARKDOWN_PRESETS), "{}", ext.label());
    }
}

/// The presets for extensions that are off by default only show their
/// feature once picking them turns it on.
#[test]
fn extension_presets_show_their_feature() {
    for preset in &MARKDOWN_PRESETS[1..] {
        let ext = preset.extension.unwrap();
        let mut on = Extensions::default();
        on.set(ext, true);
        let html = markdown_to_html(preset.text, on);
        let marker = match ext {
            Extension::Tables => "<table>",
            Extension::Math => "math-display",
            Extension::Footnotes | Extension::OldFootnotes => "footnote-definition",
            Extension::Strikethrough => "<del>",
            Extension::TaskLists => "checkbox",
            Extension::SmartPunctuation => "“",
            Extension::HeadingAttributes => "custom-id",
            Extension::YamlMetadata | Extension::PlusesMetadata => "<p>The block above",
            Extension::Gfm => "markdown-alert-note",
            Extension::DefinitionLists => "<dl>",
            Extension::Superscript => "<sup>",
            Extension::Subscript => "<sub>",
            Extension::Wikilinks => "Main%20Page",
        };
        assert!(html.contains(marker), "{}: {html}", preset.name);
        if matches!(ext, Extension::YamlMetadata | Extension::PlusesMetadata) {
            assert!(!html.contains("title"), "{}: {html}", preset.name);
        }
    }
}

/// Deeply nested input must not hang; on wasm it may overflow the stack,
/// which the UI reports as a crash. Natively, a modest depth converts.
#[test]
fn moderately_nested_input_converts() {
    let html = "<blockquote>".repeat(100) + "x";
    assert!(to_markdown(&html, &Settings::default()).ends_with('x'));
}
