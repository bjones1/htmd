use htmd::{
    HtmlToMarkdown,
    options::{Options, TranslationMode},
};
mod common;
use common::convert_faithful;

fn convert_pure(html: &str) -> String {
    HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Pure,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap()
}

#[test]
fn unordered_lists() {
    let html = r#"
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ul>
        "#;
    assert_eq!(
        "*   Item 1\n*   Item 2\n*   Item 3",
        convert_faithful(html).unwrap()
    )
}

#[test]
fn unordered_lists_custom_bullet_spacing() {
    let html = r#"
        <ul>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ul>
        "#;
    let ul_bullet_spacing = 2;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ul_bullet_spacing,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("*  Item 1\n*  Item 2\n*  Item 3", md)
}

#[test]
fn ordered_lists() {
    let html = r#"
        <ol>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        "#;
    assert_eq!(
        "1.  Item 1\n2.  Item 2\n3.  Item 3",
        convert_faithful(html).unwrap()
    )
}

#[test]
fn ordered_lists_custom_bullet_spacing() {
    let html = r#"
        <ol>
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        "#;
    let ol_number_spacing = 1;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ol_number_spacing,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!("1. Item 1\n2. Item 2\n3. Item 3", md)
}

#[test]
fn ordered_lists_start_with_zero_or_negative() {
    let html = r#"
        <ol start="0">
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        <ol start="-100">
            <li>Item 1</li>
            <li>Item 2</li>
            <li>Item 3</li>
        </ol>
        "#;
    let ol_number_spacing = 1;
    let md = HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Faithful,
            ol_number_spacing,
            ..Default::default()
        })
        .build()
        .convert(html)
        .unwrap();
    assert_eq!(
        "1. Item 1\n2. Item 2\n3. Item 3\n\n1. Item 1\n2. Item 2\n3. Item 3",
        md
    )
}

// ---------------------------------------------------------------------------
// Tight and loose lists. A list is one or the other throughout, and the two
// render differently: a loose list wraps every item's text in a `<p>`, a tight
// one leaves it bare. What spells the difference is a blank line — between two
// items, or between two blocks of one item — so a tight list can hold none
// anywhere, however many blocks its items are built from.
// ---------------------------------------------------------------------------

/// An item with no `<p>` of its own is tight, and stays tight however much it
/// holds: the blocks after its text follow on the very next line.
#[test]
fn a_tight_item_writes_no_blank_line() {
    assert_eq!(
        "*   a\n    > q",
        convert_faithful("<ul><li>a<blockquote>q</blockquote></li></ul>").unwrap()
    );
    // Three blocks in one item: the blank line the second wrote after itself
    // goes as well, not just the one the third would have written ahead of it.
    assert_eq!(
        "*   a\n    > q\n    ```\n    c\n    ```",
        convert_faithful("<ul><li>a<blockquote>q</blockquote><pre><code>c</code></pre></li></ul>")
            .unwrap()
    );
    // And the same between two items, under either kind of list marker.
    assert_eq!(
        "*   a\n    > q\n*   b",
        convert_faithful("<ul><li>a<blockquote>q</blockquote></li><li>b</li></ul>").unwrap()
    );
    assert_eq!(
        "1.  a\n    > q\n2.  b",
        convert_faithful("<ol><li>a<blockquote>q</blockquote></li><li>b</li></ol>").unwrap()
    );
}

/// A `<p>` under any one item makes the whole list loose, blank-lining every
/// item from the next — the `<p>`s all of them read back with are the point.
#[test]
fn a_paragraph_in_an_item_makes_the_list_loose() {
    assert_eq!(
        "*   a\n\n*   b",
        convert_faithful("<ul><li><p>a</p></li><li><p>b</p></li></ul>").unwrap()
    );
    assert_eq!(
        "*   a\n\n    > q",
        convert_faithful("<ul><li><p>a</p><blockquote>q</blockquote></li></ul>").unwrap()
    );
    assert_eq!(
        "1.  a\n\n2.  b",
        convert_faithful("<ol><li><p>a</p></li><li><p>b</p></li></ol>").unwrap()
    );
}

/// A loose list CommonMark cannot spell: a single item holding a single block
/// leaves the blank line that would say so nowhere to go — there is no second
/// item to precede, and no second block inside the item. The tight list is all
/// the syntax has, and it means something else, so faithful mode writes the list
/// as HTML. Pure mode drops the looseness, as it drops everything else Markdown
/// has no room for.
#[test]
fn a_loose_list_with_nowhere_to_say_so_stays_html() {
    for html in ["<ul><li><p>a</p></li></ul>", "<ol><li><p>a</p></li></ol>"] {
        assert_eq!(html, convert_faithful(html).unwrap());
    }
    // Only the list that cannot be written goes out as HTML; the writable list
    // around it is still a list, and its item holds the HTML as its content.
    assert_eq!(
        "*   <ul><li><p>a</p></li></ul>",
        convert_faithful("<ul><li><ul><li><p>a</p></li></ul></li></ul>").unwrap()
    );
    assert_eq!("*   a", convert_pure("<ul><li><p>a</p></li></ul>"));
    assert_eq!("1.  a", convert_pure("<ol><li><p>a</p></li></ol>"));
}
