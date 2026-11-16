// Tests of the test helpers in `tests/common/mod.rs`, as "Tests of the
// helpers" in the "Test helpers" section of `testing_approach.md` lists them:
// for each helper, one case which passes and one failing case per check, each
// with only the expected value of that check wrong.
mod common;

use common::{
    CMARK_OPTIONS, assert_faithful, assert_faithful_options, assert_faithful_unstable,
    assert_faithful_unstable_options,
};
use htmd::options::Options;

const STABLE_HTML: &str = "<p>a</p>";
const STABLE_MARKDOWN: &str = "a";

// `<code>` keeps its line feed only as HTML; the second conversion sees a
// space, which a code span can spell.
const UNSTABLE_HTML: &str = "<p><code>a\nb</code></p>";
const UNSTABLE_MARKDOWN: &str = "<code>a b</code>";
const UNSTABLE_SECOND_MARKDOWN: &str = "`a b`";

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
#[should_panic(expected = "property 3:")]
fn faithful_check_4_fails() {
    assert_faithful(UNSTABLE_HTML, UNSTABLE_MARKDOWN, UNSTABLE_HTML);
}

// `assert_faithful_options`

#[test]
fn faithful_options_passes() {
    assert_faithful_options(
        STABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        STABLE_MARKDOWN,
        STABLE_HTML,
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
#[should_panic(expected = "property 3:")]
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
#[should_panic(expected = "property 3:")]
fn faithful_unstable_check_4_fails() {
    assert_faithful_unstable(UNSTABLE_HTML, UNSTABLE_MARKDOWN, UNSTABLE_HTML, "`a`");
}

// `assert_faithful_unstable_options`

#[test]
fn faithful_unstable_options_passes() {
    assert_faithful_unstable_options(
        UNSTABLE_HTML,
        &Options::default(),
        CMARK_OPTIONS,
        UNSTABLE_MARKDOWN,
        UNSTABLE_HTML,
        UNSTABLE_SECOND_MARKDOWN,
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
#[should_panic(expected = "property 3:")]
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
