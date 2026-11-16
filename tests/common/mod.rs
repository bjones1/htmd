// Shared code for all integration tests.
pub mod html_compare;

use htmd::{
    HtmlToMarkdown,
    options::{HeadingStyle, Options, TranslationMode},
};
use pulldown_cmark::{Options as CommonMarkOptions, Parser};

// By default, use the faithful translation mode, which is more stringent.
pub fn convert_faithful(html: &str) -> std::io::Result<String> {
    convert_faithful_options(html, Options::default())
}

// `convert_faithful` with the given options. The translation mode is
// overridden, so callers vary only the options they care about.
pub fn convert_faithful_options(html: &str, options: Options) -> std::io::Result<String> {
    HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ..options
        })
        .build()
        .convert(html)
}

// `convert_faithful` with setext headings, the style whose h1 and h2 leave
// their content at the start of a line.
#[allow(dead_code)]
pub fn convert_faithful_setext(html: &str) -> std::io::Result<String> {
    convert_faithful_options(
        html,
        Options {
            heading_style: HeadingStyle::Setex,
            ..Default::default()
        },
    )
}

// The default translation mode, which drops what CommonMark cannot spell.
#[allow(dead_code)]
pub fn convert_pure(html: &str) -> std::io::Result<String> {
    HtmlToMarkdown::builder().build().convert(html)
}

// Takes `html` back to HTML the long way round: `convert_faithful` writes the
// Markdown, and pulldown-cmark reads that Markdown back.
//
// What comes back is HTML *source*, not a DOM, so a character reference in it
// is decoded only later, by whatever HTML parser reads the result. That is why
// several callers still assert a `&#10;`: it decodes to the line ending it
// replaced, so the trip is faithful even though the strings differ. Where a
// trip loses something instead, the assertion says what.
#[allow(dead_code)]
pub fn round_trip(html: &str) -> String {
    render_markdown(&convert_faithful(html).unwrap(), CommonMarkOptions::empty())
}

// The second half of a round trip: the CommonMark of the first half read back
// as HTML.
#[allow(dead_code)]
pub fn render_markdown(markdown: &str, options: CommonMarkOptions) -> String {
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, Parser::new_ext(markdown, options));
    html_output
}

/// The pulldown-cmark extensions which the helpers without `_options` render
/// with. See "Test helpers" in `testing_approach.md` for why these six, and
/// why they are spelled out rather than derived from `Options::all()`.
#[allow(dead_code)]
pub const CMARK_OPTIONS: CommonMarkOptions = CommonMarkOptions::ENABLE_TABLES
    .union(CommonMarkOptions::ENABLE_MATH)
    .union(CommonMarkOptions::ENABLE_FOOTNOTES)
    .union(CommonMarkOptions::ENABLE_STRIKETHROUGH)
    .union(CommonMarkOptions::ENABLE_TASKLISTS)
    .union(CommonMarkOptions::ENABLE_GFM);

/// Check the three properties of `testing_approach.md` for one stable case.
#[allow(dead_code)]
#[track_caller]
pub fn assert_faithful(html: &str, expected_markdown: &str, expected_round_trip: &str) {
    assert_faithful_options(
        html,
        &Options::default(),
        CMARK_OPTIONS,
        expected_markdown,
        expected_round_trip,
    );
}

/// [`assert_faithful`] with the given options. `htmd_options.translation_mode`
/// is ignored: the conversion is always Faithful.
#[allow(dead_code)]
#[track_caller]
pub fn assert_faithful_options(
    html: &str,
    htmd_options: &Options,
    cmark_options: CommonMarkOptions,
    expected_markdown: &str,
    expected_round_trip: &str,
) {
    check_faithful(
        html,
        htmd_options,
        cmark_options,
        expected_markdown,
        expected_round_trip,
        None,
    );
}

/// Check one case whose second conversion gives `expected_second_markdown`
/// instead of `expected_markdown`, and whose second Markdown is stable.
#[allow(dead_code)]
#[track_caller]
pub fn assert_faithful_unstable(
    html: &str,
    expected_markdown: &str,
    expected_round_trip: &str,
    expected_second_markdown: &str,
) {
    assert_faithful_unstable_options(
        html,
        &Options::default(),
        CMARK_OPTIONS,
        expected_markdown,
        expected_round_trip,
        expected_second_markdown,
    );
}

/// [`assert_faithful_unstable`] with the given options, as
/// [`assert_faithful_options`] takes them.
#[allow(dead_code)]
#[track_caller]
pub fn assert_faithful_unstable_options(
    html: &str,
    htmd_options: &Options,
    cmark_options: CommonMarkOptions,
    expected_markdown: &str,
    expected_round_trip: &str,
    expected_second_markdown: &str,
) {
    assert!(
        expected_second_markdown != expected_markdown,
        "unstable helper misused: the second Markdown equals the first, so this \
         case belongs in a stable helper\n  html:     {html:?}\n  markdown: {expected_markdown:?}"
    );
    check_faithful(
        html,
        htmd_options,
        cmark_options,
        expected_markdown,
        expected_round_trip,
        Some(expected_second_markdown),
    );
}

/// Checks 1 to 4 of "Order of checks" in `testing_approach.md`. A `None`
/// second Markdown means the case is stable.
#[track_caller]
fn check_faithful(
    html: &str,
    htmd_options: &Options,
    cmark_options: CommonMarkOptions,
    expected_markdown: &str,
    expected_round_trip: &str,
    expected_second_markdown: Option<&str>,
) {
    let markdown = convert_or_panic(html, htmd_options);
    let round_trip = render_markdown(&markdown, cmark_options);

    if !html_compare::html_equivalent(&round_trip, expected_round_trip) {
        panic!(
            "property 2: the round trip is not equivalent to the expected HTML\n  \
             html:     {html:?}\n  markdown: {markdown:?}\n  actual:   {round_trip:?}\n  \
             expected: {expected_round_trip:?}\nnormalized trees:\n{}",
            pretty_assertions::StrComparison::new(
                &html_compare::normalize_html(&round_trip),
                &html_compare::normalize_html(expected_round_trip),
            )
        );
    }

    assert!(
        markdown == expected_markdown,
        "property 1: the Markdown differs\n  html:     {html:?}\n  actual:   {markdown:?}\n  \
         expected: {expected_markdown:?}"
    );

    let second_markdown = convert_or_panic(&round_trip, htmd_options);
    let expected_second = expected_second_markdown.unwrap_or(expected_markdown);
    assert!(
        second_markdown == expected_second,
        "property 3: converting the round trip gives different Markdown\n  \
         html:       {html:?}\n  markdown:   {markdown:?}\n  converted:  {round_trip:?}\n  \
         actual:     {second_markdown:?}\n  expected:   {expected_second:?}"
    );

    if let Some(second) = expected_second_markdown {
        let second_round_trip = render_markdown(second, cmark_options);
        let third_markdown = convert_or_panic(&second_round_trip, htmd_options);
        assert!(
            third_markdown == second,
            "property 3: the second Markdown is not stable\n  html:       {html:?}\n  \
             markdown:   {markdown:?}\n  second:     {second:?}\n  \
             converted:  {second_round_trip:?}\n  actual:     {third_markdown:?}\n  \
             expected:   {second:?}"
        );
    }
}

/// Check 1's conversion. A fn rather than a closure, since `#[track_caller]`
/// does not reach through a closure to the test which failed.
#[track_caller]
fn convert_or_panic(html: &str, htmd_options: &Options) -> String {
    convert_faithful_options(html, htmd_options.clone())
        .expect("the Faithful conversion cannot fail")
}
