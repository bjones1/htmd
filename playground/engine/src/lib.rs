//! The playground's converters. The page loads this crate as a wasm instance
//! of its own, separate from the UI's, and calls it through the global
//! `htmdPlaygroundEngine` (see `static/main.js`). A panic or a stack overflow
//! in a converter leaves its wasm instance unusable; keeping the converters
//! apart lets the UI survive that and show the crash (spec, Limits and
//! failures).

use htmd::HtmlToMarkdown;
use htmd::options as h;
use htmd_playground_core::settings::*;
use pulldown_cmark::{Options as CmarkOptions, Parser, html};

/// Convert HTML to Markdown with htmd.
pub fn html_to_markdown(input: &str, settings: &Settings) -> std::io::Result<String> {
    let options = &settings.htmd;
    let mut builder = HtmlToMarkdown::builder()
        .options(htmd_options(options))
        .scripting_enabled(options.noscript_as_raw_text);
    let skip_tags = parse_skip_tags(&options.skip_tags);
    // htmd asserts that the list it is given is not empty.
    if !skip_tags.names.is_empty() {
        builder = builder.skip_tags(skip_tags.names.iter().map(String::as_str).collect());
    }
    builder.build().convert(input)
}

/// Convert Markdown to HTML with pulldown-cmark.
pub fn markdown_to_html(input: &str, extensions: Extensions) -> String {
    let mut output = String::with_capacity(input.len() * 3 / 2);
    html::push_html(
        &mut output,
        Parser::new_ext(input, cmark_options(extensions)),
    );
    output
}

fn htmd_options(options: &HtmdOptions) -> h::Options {
    h::Options {
        heading_style: match options.heading_style {
            HeadingStyle::Atx => h::HeadingStyle::Atx,
            HeadingStyle::Setext => h::HeadingStyle::Setex,
        },
        hr_style: match options.hr_style {
            HrStyle::Asterisks => h::HrStyle::Asterisks,
            HrStyle::Dashes => h::HrStyle::Dashes,
            HrStyle::Underscores => h::HrStyle::Underscores,
        },
        br_style: match options.br_style {
            BrStyle::TwoSpaces => h::BrStyle::TwoSpaces,
            BrStyle::Backslash => h::BrStyle::Backslash,
        },
        link_style: match options.link_style {
            LinkStyle::Inline => h::LinkStyle::Inlined,
            LinkStyle::InlineAutolinks => h::LinkStyle::InlinedPreferAutolinks,
            LinkStyle::Referenced => h::LinkStyle::Referenced,
        },
        link_reference_style: match options.reference_style {
            ReferenceStyle::Full => h::LinkReferenceStyle::Full,
            ReferenceStyle::Collapsed => h::LinkReferenceStyle::Collapsed,
            ReferenceStyle::Shortcut => h::LinkReferenceStyle::Shortcut,
        },
        code_block_style: match options.code_block_style {
            CodeBlockStyle::Fenced => h::CodeBlockStyle::Fenced,
            CodeBlockStyle::Indented => h::CodeBlockStyle::Indented,
        },
        code_block_fence: match options.code_fence {
            CodeFence::Backticks => h::CodeBlockFence::Backticks,
            CodeFence::Tildes => h::CodeBlockFence::Tildes,
        },
        bullet_list_marker: match options.bullet_marker {
            BulletMarker::Asterisk => h::BulletListMarker::Asterisk,
            BulletMarker::Dash => h::BulletListMarker::Dash,
        },
        ul_bullet_spacing: options.bullet_spacing,
        ol_number_spacing: options.number_spacing,
        preformatted_code: options.preserve_code_whitespace,
        translation_mode: match options.translation_mode {
            TranslationMode::Pure => h::TranslationMode::Pure,
            TranslationMode::Faithful => h::TranslationMode::Faithful,
        },
    }
}

fn cmark_options(extensions: Extensions) -> CmarkOptions {
    extensions
        .iter()
        .map(|ext| match ext {
            Extension::Tables => CmarkOptions::ENABLE_TABLES,
            Extension::Math => CmarkOptions::ENABLE_MATH,
            Extension::Footnotes => CmarkOptions::ENABLE_FOOTNOTES,
            Extension::OldFootnotes => CmarkOptions::ENABLE_OLD_FOOTNOTES,
            Extension::Strikethrough => CmarkOptions::ENABLE_STRIKETHROUGH,
            Extension::TaskLists => CmarkOptions::ENABLE_TASKLISTS,
            Extension::SmartPunctuation => CmarkOptions::ENABLE_SMART_PUNCTUATION,
            Extension::HeadingAttributes => CmarkOptions::ENABLE_HEADING_ATTRIBUTES,
            Extension::YamlMetadata => CmarkOptions::ENABLE_YAML_STYLE_METADATA_BLOCKS,
            Extension::PlusesMetadata => CmarkOptions::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS,
            Extension::Gfm => CmarkOptions::ENABLE_GFM,
            Extension::DefinitionLists => CmarkOptions::ENABLE_DEFINITION_LIST,
            Extension::Superscript => CmarkOptions::ENABLE_SUPERSCRIPT,
            Extension::Subscript => CmarkOptions::ENABLE_SUBSCRIPT,
            Extension::Wikilinks => CmarkOptions::ENABLE_WIKILINKS,
        })
        .fold(CmarkOptions::empty(), |all, option| all | option)
}

#[cfg(target_arch = "wasm32")]
mod js {
    //! The functions `static/main.js` exposes to the UI. Settings arrive in
    //! the permalink format of `htmd_playground_core::codec`.

    use htmd_playground_core::codec;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    fn start() {
        // Throwing turns a panic into a JS exception carrying its message,
        // which the UI catches like any other crash. This instance is then
        // left in an undefined state, and the UI never calls it again.
        std::panic::set_hook(Box::new(|info| wasm_bindgen::throw_str(&info.to_string())));
    }

    fn settings(text: &str) -> Result<htmd_playground_core::settings::Settings, JsError> {
        codec::decode_settings(text).ok_or_else(|| JsError::new("unreadable settings"))
    }

    #[wasm_bindgen(js_name = htmlToMarkdown)]
    pub fn html_to_markdown(input: &str, settings_text: &str) -> Result<String, JsError> {
        Ok(super::html_to_markdown(input, &settings(settings_text)?)?)
    }

    #[wasm_bindgen(js_name = markdownToHtml)]
    pub fn markdown_to_html(input: &str, settings_text: &str) -> Result<String, JsError> {
        Ok(super::markdown_to_html(
            input,
            settings(settings_text)?.extensions,
        ))
    }
}
