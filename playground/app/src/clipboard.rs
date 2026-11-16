//! Copying text, with a manual-copy fallback (spec, Output pane and
//! Permalinks).

use leptos::prelude::window;
use leptos::task::spawn_local;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

/// Copy `text`, then call `done` with whether it worked. Must run inside a
/// click handler: Safari only allows `writeText` while handling the click,
/// so it is called before the first `await`.
pub fn copy(text: &str, done: impl FnOnce(bool) + 'static) {
    let navigator = window().navigator();
    // `navigator.clipboard` is missing on pages not served over HTTPS.
    let has_clipboard = js_sys::Reflect::get(&navigator, &JsValue::from_str("clipboard"))
        .is_ok_and(|clipboard| !clipboard.is_undefined());
    if !has_clipboard {
        done(false);
        return;
    }
    let promise = navigator.clipboard().write_text(text);
    spawn_local(async move { done(JsFuture::from(promise).await.is_ok()) });
}

/// The message shown when copying failed and the text was selected instead.
pub const MANUAL_COPY: &str = "Your browser did not allow copying. The text is selected: press Ctrl+C (⌘C on a Mac) to copy it.";
