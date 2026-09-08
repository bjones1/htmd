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

/// [`convert_faithful`] writing an `<h1>` or an `<h2>` as a setext heading.
pub fn convert_faithful_setext(html: &str) -> String {
    convert_with(
        Options {
            heading_style: HeadingStyle::Setex,
            ..faithful_options()
        },
        html,
    )
    .unwrap()
}

/// The one-column, one-row table the table cells section of
/// `unsupported_html.md` writes its rows against, with `cell` as the body cell.
pub fn one_cell_table(cell: &str) -> String {
    format!(
        "<table><thead><tr><th>h</th></tr></thead>\
         <tbody><tr><td>{cell}</td></tr></tbody></table>"
    )
}

/// [`one_cell_table`] with `cell` as the *heading* cell instead.
pub fn one_cell_table_heading(cell: &str) -> String {
    format!(
        "<table><thead><tr><th>{cell}</th></tr></thead>\
         <tbody><tr><td>c</td></tr></tbody></table>"
    )
}
