Testing approach
================

Contributions
-------------

* Miras Shaltayev: wrote the initial draft of this document.
* Armanzhan Onggarbay: 
* Nurlybek Orynbassar:
* Aidyn Tazhibay:

Every test of a translation checks two things: the Markdown which htmd writes, and that this Markdown renders as the HTML it came from. The second check is a round trip, and `round_trip` in `tests/common/mod.rs` performs it. Today most tests make only the first check, so a test can pass while the Markdown means something else to pulldown-cmark.

### Properties

For an HTML input `h` and a translation mode:

1. **Expected Markdown**: `convert(h)` equals the Markdown in the test.
2. **Round trip**: in `TranslationMode::Faithful`, `render_markdown(convert(h), options)` is the HTML which `h` is expected to produce. Where the two strings differ but a browser parses them to the same DOM (for example `<p>a</p>` comes back as `<p>a</p>\n`, since pulldown-cmark ends a block with a line ending), the test states the expected string, not a normalized one.
3. **Idempotency**: let `m = convert(h)` and `h2 = render_markdown(m)`. Then `convert(h2)` equals `m`.

This approach covers `TranslationMode::Faithful` only. `TranslationMode::Pure` is out of scope.

### Standard test helper

One helper in `tests/common/mod.rs` checks all three properties. A test calls it instead of repeating `assert_eq!` pairs. It checks the round trip first, so an escape which stops working reports the harm it does, such as a paragraph dissolved into a heading, rather than a mismatched Markdown string.

`assert_faithful(html: &str, expected_markdown: &str, expected_round_trip: &str)` converts in Faithful mode with the default htmd options, and reads the Markdown back with `CommonMarkOptions::empty()`, as `round_trip` does. It panics with a diff on a mismatch, as `assert_eq!` does.

Tests also need other options, of two kinds. `assert_faithful_options(html, htmd_options, commonmark_options, expected_markdown, expected_round_trip)` takes both: the htmd options (for example `HeadingStyle::Setex`, which `translation_approach_tests.rs` uses) and the pulldown-cmark options (for example `ENABLE_MATH | ENABLE_TABLES`). `assert_faithful` calls it with the defaults.

The expected round-trip HTML is always given, never omitted: pulldown-cmark wraps inline content in `<p>` and ends each block with a line ending, so the output seldom equals the input, and an explicit string shows what the trip produces.

### Known failures

A case whose trip is lossy by design (property 2) still uses `assert_faithful`, with the lossy string as the expected round trip. The test adds a one-line reason and names the section of `translation_approach.md` which explains the loss. A case which fails idempotency (property 3) is a bug to fix, or has its own test which asserts the unstable second Markdown, with the same reason. A test never hides a failure by skipping a property without that reason.

### Migration order

1. Add the helpers and convert `basic_tests.rs` and `link_tests.rs`, which already use `round_trip` in part. The existing helpers in `basic_tests.rs` become thin callers of the new ones, or are deleted where `assert_faithful` replaces them: `assert_round_trips`, `assert_math_round_trips` (needs `assert_faithful_options`) and `assert_paragraph_survives`. `round_trip_pure_table` tests Pure mode, which is out of scope, so it stays as it is.
2. Convert `table_tests.rs`, `list_tests.rs`, `code_tests.rs`, `context_tests.rs` and `event_tests.rs`, which never call it.
3. Convert `translation_approach_tests.rs`: each table row gets the three properties.
4. Record each failure found as a known failure, or as a bug to fix.
