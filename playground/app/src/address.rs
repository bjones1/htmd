//! Reading and writing the page address (spec, Permalinks). The state lives
//! in the fragment, in the format of `htmd_playground_core::codec`.

use htmd_playground_core::MAX_ADDRESS_LENGTH;
use htmd_playground_core::codec;
use htmd_playground_core::settings::State;
use leptos::prelude::window;
use wasm_bindgen::JsValue;

/// The address's fragment, including its `#`.
pub fn fragment() -> String {
    window().location().hash().unwrap_or_default()
}

/// The address without its fragment: the page with no saved state.
pub fn without_fragment() -> String {
    let href = window().location().href().unwrap_or_default();
    match href.split_once('#') {
        Some((base, _)) => base.to_string(),
        None => href,
    }
}

/// The full address.
pub fn href() -> String {
    window().location().href().unwrap_or_default()
}

/// Record `state` in the address and report whether the input fit. Replacing
/// the current history entry, rather than pushing one, keeps the playground
/// from adding history entries, and it fires no `hashchange`.
pub fn write(state: &State) -> bool {
    let base_length = without_fragment().len() + 1;
    let mut fragment = codec::encode(state, true);
    let fits = base_length + fragment.len() <= MAX_ADDRESS_LENGTH;
    if !fits {
        fragment = codec::encode(state, false);
    }
    if let Ok(history) = window().history() {
        let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&format!("#{fragment}")));
    }
    fits
}
