//! The `wasm-bindgen-test` suite (§7 Deliverable 3, §8.2), run by
//! `wasm-bindgen-test-runner` in headless Chromium; CI runs it in debug
//! and `--release`.
//!
//! The FR-7 stack-depth cases run only in release. They check `htmd`'s
//! recursive traversal against the real 1 MiB wasm call stack, and §8.6
//! sizes the 512-level ceiling for release frames: debug frames are far
//! larger (the native debug tests already need `RUST_MIN_STACK`).
//!
//! A real panic traps the test instance under wasm's abort-only panics,
//! so the crash cases drive [`panic_hook::enter_crashed_state`] directly.
//! `e2e/crash-recovery.spec.ts` covers the hook end to end.

use htmd::options::TranslationMode;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

use crate::convert::{Blocked, convert_html_to_md, convert_md_to_html};
use crate::options_model::{CmarkFlags, HtmdOptions};
use crate::panic_hook;
use crate::preview::build_srcdoc;
use crate::storage::CRASH_MARKER_KEY;

wasm_bindgen_test_configure!(run_in_browser);

const TRANSLATION_MODES: [TranslationMode; 2] = [TranslationMode::Pure, TranslationMode::Faithful];

/// Stack-heavy shapes from §8.6, each with the number of parsed levels
/// one repetition adds (html5ever implies `<tbody>` inside `<table>`).
const STACK_HEAVY_SHAPES: [(&str, &str, usize); 4] = [
    ("nested lists", "<ul><li>", 2),
    ("blockquotes", "<blockquote>", 1),
    ("tables", "<table><tr><td>", 4),
    ("mixed", "<blockquote><ul><li><table><tr><td>", 7),
];

fn options(translation_mode: TranslationMode) -> HtmdOptions {
    HtmdOptions {
        translation_mode,
        ..HtmdOptions::default()
    }
}

/// `shape` repeated and topped up with `<blockquote>`s to exactly `depth`
/// parsed levels, with text in the innermost element.
fn nested(shape: &str, levels_per_repeat: usize, depth: usize) -> String {
    let repeats = depth / levels_per_repeat;
    let top_up = depth - repeats * levels_per_repeat;
    format!("{}{}x", shape.repeat(repeats), "<blockquote>".repeat(top_up))
}

/// §8.6 *Nesting depth*: 512 levels convert without overflowing the
/// stack, and one level more is rejected. Together the two assertions
/// pin the parsed depth at exactly 512.
#[cfg_attr(debug_assertions, ignore = "release-only: sized for release stack frames")]
#[wasm_bindgen_test]
fn five_hundred_twelve_levels_convert_and_five_hundred_thirteen_are_rejected() {
    for (name, shape, levels_per_repeat) in STACK_HEAVY_SHAPES {
        let at_limit = nested(shape, levels_per_repeat, 512);
        let over_limit = nested(shape, levels_per_repeat, 513);
        for mode in TRANSLATION_MODES {
            let converted = convert_html_to_md(&at_limit, &options(mode), &[]);
            assert!(
                converted.as_deref().is_ok_and(|md| md.contains('x')),
                "{name}, {mode:?}: 512 levels should convert, got {converted:?}"
            );
            assert_eq!(
                convert_html_to_md(&over_limit, &options(mode), &[]),
                Err(Blocked::TooDeep),
                "{name}, {mode:?}: 513 levels should be rejected"
            );
        }
    }
}

/// §8.6 *Nesting depth*: html5ever closes each `<p>` before the next and
/// `<br>` is void, so both inputs are 600 siblings, not 600 levels.
#[cfg_attr(debug_assertions, ignore = "release-only: sized for release stack frames")]
#[wasm_bindgen_test]
fn six_hundred_flat_siblings_convert() {
    for input in ["<p>x".repeat(600), "x<br>".repeat(600)] {
        for mode in TRANSLATION_MODES {
            assert!(
                convert_html_to_md(&input, &options(mode), &[]).is_ok(),
                "{mode:?}: {} should convert",
                &input[..6]
            );
        }
    }
}

/// §8.6 *Preview stripping*: Markdown → HTML is exempt from the nesting
/// limit, so 50,000 `>` reach the preview's strip walk and serializer.
/// Slow (html5ever's fragment parse is quadratic in depth).
#[cfg_attr(debug_assertions, ignore = "release-only: sized for release stack frames")]
#[wasm_bindgen_test]
fn fifty_thousand_blockquote_markers_build_a_preview() {
    let html = convert_md_to_html(&">".repeat(50_000), &CmarkFlags::default());
    let depth = html.matches("<blockquote>").count();
    assert!(depth > 512, "expected deep nesting, got {depth} levels");
    let srcdoc = build_srcdoc(&html);
    assert_eq!(srcdoc.matches("<blockquote>").count(), depth);
    assert_eq!(srcdoc.matches("</blockquote>").count(), depth);
}

fn document() -> web_sys::Document {
    web_sys::window()
        .and_then(|window| window.document())
        .expect("browser document")
}

fn local_storage() -> web_sys::Storage {
    web_sys::window()
        .and_then(|window| window.local_storage().ok().flatten())
        .expect("localStorage")
}

/// Adds `index.html`'s crash markup to the test page: the app root (with
/// a focused control inside) and the hidden overlay outside it.
fn mount_crash_markup() {
    let body = document().body().expect("document body");
    body.insert_adjacent_html(
        "beforeend",
        r#"<div id="app-root"><button id="app-control">control</button></div>
           <div id="crash-overlay" role="alertdialog" hidden><p>overlay</p></div>"#,
    )
    .expect("insert crash markup");
    element("app-control")
        .dyn_into::<web_sys::HtmlElement>()
        .expect("button is an HtmlElement")
        .focus()
        .expect("focus control");
}

fn element(id: &str) -> web_sys::Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("#{id} is mounted"))
}

fn unmount_crash_markup() {
    for id in ["app-root", "crash-overlay"] {
        if let Some(element) = document().get_element_by_id(id) {
            element.remove();
        }
    }
    local_storage()
        .remove_item(CRASH_MARKER_KEY)
        .expect("clear crash marker");
}

/// FR-7 *Rust panics*: marker, storage halt, overlay, `inert` root, and
/// focus moved out of the root onto the overlay.
#[wasm_bindgen_test]
fn crash_state_marks_storage_shows_overlay_and_inerts_the_app() {
    unmount_crash_markup();
    mount_crash_markup();

    panic_hook::enter_crashed_state();

    assert_eq!(
        local_storage().get_item(CRASH_MARKER_KEY).ok().flatten(),
        Some("1".to_string())
    );
    assert!(panic_hook::storage_halted());
    assert!(!element("crash-overlay").has_attribute("hidden"));
    assert!(element("app-root").has_attribute("inert"));
    let focused = document().active_element().map(|element| element.id());
    assert_eq!(focused.as_deref(), Some("crash-overlay"));

    unmount_crash_markup();
}

/// FR-7: the hook is null-safe. Without the overlay or app root it still
/// writes the marker and returns instead of panicking inside the hook.
#[wasm_bindgen_test]
fn crash_state_without_markup_still_writes_the_marker() {
    unmount_crash_markup();

    panic_hook::enter_crashed_state();

    assert_eq!(
        local_storage().get_item(CRASH_MARKER_KEY).ok().flatten(),
        Some("1".to_string())
    );
    unmount_crash_markup();
}
