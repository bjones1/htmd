// Tests of the test helpers in `tests/common/mod.rs`, as "Tests of the
// helpers" in the "Test helpers" section of `testing_approach.md` lists them:
// for each helper, one case which passes and one failing case per check, each
// with only the expected value of that check wrong.
mod common;

use common::{
    CMARK_OPTIONS, assert_faithful, assert_faithful_options, assert_faithful_unstable,
    assert_faithful_unstable_options,
};
use htmd::options::{HeadingStyle, Options};

const STABLE_HTML: &str = "<p>a</p>";
const STABLE_MARKDOWN: &str = "a";

// htmd writes this `<code>` as raw HTML; the second conversion, given the same
// text, writes a code span. This instability is an htmd bug: the line feed is
// already lost in the first Markdown, so the raw HTML keeps nothing a code span
// could not. A fix in htmd needs a new unstable input here.
const UNSTABLE_HTML: &str = "<p><code>a\nb</code></p>";
const UNSTABLE_MARKDOWN: &str = "<code>a b</code>";
const UNSTABLE_SECOND_MARKDOWN: &str = "`a b`";

// The passing `_options` cases use setext headings, whose Markdown differs
// from the default, so they fail unless the options reach the conversion.
fn setext_options() -> Options {
    Options {
        heading_style: HeadingStyle::Setex,
        ..Default::default()
    }
}

// `assert_faithful`

#[test]
fn faithful_passes() {
    assert_faithful(STABLE_HTML, STABLE_MARKDOWN, STABLE_HTML);
}

#[test]
#[should_panic(expected = "property 2:")]
fn faithful_check_2_fails() {
    assert_faithful(STABLE_HTML, STABLE_MARKDOWN, "<h1>a</h1>");
}

#[test]
#[should_panic(expected = "property 1:")]
fn faithful_check_3_fails() {
    assert_faithful(STABLE_HTML, "b", STABLE_HTML);
}

#[test]
#[should_panic(expected = "property 3: converting the round trip")]
fn faithful_check_4_fails() {
    assert_faithful(UNSTABLE_HTML, UNSTABLE_MARKDOWN, UNSTABLE_HTML);
}

// `assert_faithful_options`

#[test]
fn faithful_options_passes() {
    assert_faithful_options(
        "<h1>a</h1>",
        &setext_options(),
        CMARK_OPTIONS,
        "a\n=",
        "<h1>a</h1>",
    );
}

#[test]
#[should_panic(expected = "property 2:")]
fn faithful_options_check_2_fails() {
    assert_faithful_options(
        STABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        STABLE_MARKDOWN,
        "<h1>a</h1>",
    );
}

#[test]
#[should_panic(expected = "property 1:")]
fn faithful_options_check_3_fails() {
    assert_faithful_options(
        STABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        "b",
        STABLE_HTML,
    );
}

#[test]
#[should_panic(expected = "property 3: converting the round trip")]
fn faithful_options_check_4_fails() {
    assert_faithful_options(
        UNSTABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        UNSTABLE_MARKDOWN,
        UNSTABLE_HTML,
    );
}

// `assert_faithful_unstable`

#[test]
fn faithful_unstable_passes() {
    assert_faithful_unstable(
        UNSTABLE_HTML,
        UNSTABLE_MARKDOWN,
        UNSTABLE_HTML,
        UNSTABLE_SECOND_MARKDOWN,
    );
}

#[test]
#[should_panic(expected = "unstable helper misused:")]
fn faithful_unstable_check_0_fails() {
    assert_faithful_unstable(STABLE_HTML, STABLE_MARKDOWN, STABLE_HTML, STABLE_MARKDOWN);
}

#[test]
#[should_panic(expected = "property 2:")]
fn faithful_unstable_check_2_fails() {
    assert_faithful_unstable(
        UNSTABLE_HTML,
        UNSTABLE_MARKDOWN,
        "<p><code>a</code></p>",
        UNSTABLE_SECOND_MARKDOWN,
    );
}

#[test]
#[should_panic(expected = "property 1:")]
fn faithful_unstable_check_3_fails() {
    assert_faithful_unstable(
        UNSTABLE_HTML,
        "<code>a</code>",
        UNSTABLE_HTML,
        UNSTABLE_SECOND_MARKDOWN,
    );
}

#[test]
#[should_panic(expected = "property 3: converting the round trip")]
fn faithful_unstable_check_4_fails() {
    assert_faithful_unstable(UNSTABLE_HTML, UNSTABLE_MARKDOWN, UNSTABLE_HTML, "`a`");
}

// `assert_faithful_unstable_options`

#[test]
fn faithful_unstable_options_passes() {
    assert_faithful_unstable_options(
        "<h1><code>a\nb</code></h1>",
        &setext_options(),
        CMARK_OPTIONS,
        "<code>a b</code>\n================",
        "<h1><code>a\nb</code></h1>",
        "`a b`\n=====",
    );
}

#[test]
#[should_panic(expected = "unstable helper misused:")]
fn faithful_unstable_options_check_0_fails() {
    assert_faithful_unstable_options(
        STABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        STABLE_MARKDOWN,
        STABLE_HTML,
        STABLE_MARKDOWN,
    );
}

#[test]
#[should_panic(expected = "property 2:")]
fn faithful_unstable_options_check_2_fails() {
    assert_faithful_unstable_options(
        UNSTABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        UNSTABLE_MARKDOWN,
        "<p><code>a</code></p>",
        UNSTABLE_SECOND_MARKDOWN,
    );
}

#[test]
#[should_panic(expected = "property 1:")]
fn faithful_unstable_options_check_3_fails() {
    assert_faithful_unstable_options(
        UNSTABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        "<code>a</code>",
        UNSTABLE_HTML,
        UNSTABLE_SECOND_MARKDOWN,
    );
}

#[test]
#[should_panic(expected = "property 3: converting the round trip")]
fn faithful_unstable_options_check_4_fails() {
    assert_faithful_unstable_options(
        UNSTABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        UNSTABLE_MARKDOWN,
        UNSTABLE_HTML,
        "`a`",
    );
}
