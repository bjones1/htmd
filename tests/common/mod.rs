// Shared code for all integration tests.
//
// Each test binary compiles this module separately, so a helper only some of
// them use is dead code in the rest.
#![allow(dead_code)]

use htmd::{
    HtmlToMarkdown,
    options::{HeadingStyle, Options, TranslationMode},
};

/// The faithful translation mode, which `unsupported_html.md` is written for.
pub fn faithful_options() -> Options {
    Options {
        translation_mode: TranslationMode::Faithful,
        ..Default::default()
    }
}

pub fn convert_with(options: Options, html: &str) -> std::io::Result<String> {
    HtmlToMarkdown::builder()
        .options(options)
        .build()
        .convert(html)
}

// By default, use the faithful translation mode, which is more stringent.
pub fn convert_faithful(html: &str) -> std::io::Result<String> {
    convert_with(faithful_options(), html)
}
