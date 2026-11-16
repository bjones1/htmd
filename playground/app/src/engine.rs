//! Calls into the converter, which runs in a wasm instance of its own
//! (`engine/src/lib.rs`, exposed as `htmdPlaygroundEngine` by
//! `static/main.js`).
//!
//! A panic or a stack overflow in the converter surfaces here as a JS
//! exception. The exception unwinds only the converter's instance, so this
//! instance stays intact and can show the crash; the converter's instance is
//! left in an undefined state and must not be called again.

use htmd_playground_core::codec;
use htmd_playground_core::settings::Settings;
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = htmdPlaygroundEngine, js_name = htmlToMarkdown)]
    fn engine_html_to_markdown(input: &str, settings: &str) -> Result<String, JsValue>;

    #[wasm_bindgen(catch, js_namespace = htmdPlaygroundEngine, js_name = markdownToHtml)]
    fn engine_markdown_to_html(input: &str, settings: &str) -> Result<String, JsValue>;
}

/// Convert HTML to Markdown. An `Err` describes a crash.
pub fn html_to_markdown(input: &str, settings: &Settings) -> Result<String, String> {
    engine_html_to_markdown(input, &codec::encode_settings(settings)).map_err(describe)
}

/// Convert Markdown to HTML. An `Err` describes a crash.
pub fn markdown_to_html(input: &str, settings: &Settings) -> Result<String, String> {
    engine_markdown_to_html(input, &codec::encode_settings(settings)).map_err(describe)
}

fn describe(error: JsValue) -> String {
    match error.dyn_ref::<js_sys::Error>() {
        Some(error) => format!("{}: {}", error.name(), error.message()),
        None => error.as_string().unwrap_or_else(|| format!("{error:?}")),
    }
}
