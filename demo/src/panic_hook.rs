//! FR-7 *Rust panics*: the single custom panic hook. It delegates to
//! `console_error_panic_hook` first (only one global hook exists, so it
//! must chain), writes the crash marker, halts all further workspace
//! storage writes, reveals the static fallback overlay, sets `inert` on
//! the application root, and moves focus to the overlay. Raw `web-sys`
//! only -- Leptos signals may be borrowed at the panic site, and
//! re-entering them would panic again before the overlay shows.
//!
//! The browser suite (`browser_tests.rs`) drives [`enter_crashed_state`]
//! directly; `e2e/crash-recovery.spec.ts` verifies the whole hook with a
//! real panic in `demo-panic-test` builds. The hook's behavior does not
//! depend on that feature.

use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(target_arch = "wasm32")]
use crate::storage::CRASH_MARKER_KEY;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Task 0's static markup contract (`index.html`): the `alertdialog`
/// revealed by removing its `hidden` attribute.
#[cfg(target_arch = "wasm32")]
const OVERLAY_ID: &str = "crash-overlay";
/// Task 0's static markup contract: the element that receives `inert`.
#[cfg(target_arch = "wasm32")]
const APP_ROOT_ID: &str = "app-root";

/// Set once [`install`] has registered the hook; later calls no-op.
#[cfg(target_arch = "wasm32")]
static INSTALLED: AtomicBool = AtomicBool::new(false);

/// FR-7: set once the hook body has run its storage steps; from then
/// on all further workspace storage writes are halted (pending save
/// timers cancelled, `pagehide`/`visibilitychange` flushes ignored).
static STORAGE_HALTED: AtomicBool = AtomicBool::new(false);

/// Whether the panic hook halted workspace storage writes. Task 10's
/// save wiring consults this before every write and flush.
pub fn storage_halted() -> bool {
    STORAGE_HALTED.load(Ordering::SeqCst)
}

/// Claim `flag` for its owner: `true` on the first call, `false` on
/// every later one. The idempotence behind both [`install`] and the
/// storage halt; native-testable because it takes the flag as input.
// Natively `install` is compiled out, so this is reachable only from
// the unit tests there; wasm reaches it through `install`.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub(crate) fn mark_installed(flag: &AtomicBool) -> bool {
    !flag.swap(true, Ordering::SeqCst)
}

/// Install the FR-7 panic hook, chaining `console_error_panic_hook`
/// (wasm only; the hook body runs before `panic = "abort"` traps).
#[cfg(target_arch = "wasm32")]
pub fn install() {
    console_error_panic_hook::set_once();
    if mark_installed(&INSTALLED) {
        std::panic::set_hook(Box::new(|info| {
            console_error_panic_hook::hook(info);
            handle_panic(info);
        }));
    }
}

/// The hook body: FR-7's crash steps, then the log.
#[cfg(target_arch = "wasm32")]
fn handle_panic(info: &std::panic::PanicHookInfo<'_>) {
    enter_crashed_state();
    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(&info.to_string()));
}

/// FR-7's crash steps, in order: crash marker (the hook's only storage
/// write), storage halt, overlay reveal, `inert` root, focus. Every DOM
/// lookup is null-guarded -- an element missing means that step is
/// skipped, never a panic inside the panic hook. Separate from
/// [`handle_panic`] so the browser suite can run it without panicking,
/// which traps the test under wasm's abort-only panics.
#[cfg(target_arch = "wasm32")]
pub(crate) fn enter_crashed_state() {
    let window = web_sys::window();
    if let Some(window) = window.as_ref() {
        write_crash_marker(window);
    }
    STORAGE_HALTED.store(true, Ordering::SeqCst);
    let document = window.as_ref().and_then(web_sys::Window::document);
    let overlay = document
        .as_ref()
        .and_then(|doc| doc.get_element_by_id(OVERLAY_ID));
    if let Some(overlay) = overlay.as_ref() {
        let _ = overlay.remove_attribute("hidden");
    }
    if let Some(root) = document
        .as_ref()
        .and_then(|doc| doc.get_element_by_id(APP_ROOT_ID))
    {
        let _ = root.set_attribute("inert", "");
    }
    if let Some(overlay) = overlay.as_ref() {
        let _ = overlay.set_attribute("tabindex", "-1");
        if let Some(el) = overlay.dyn_ref::<web_sys::HtmlElement>() {
            let _ = el.focus();
        }
    }
}

/// FR-7 crash marker (`htmd_cmark_live_crashed`): a direct
/// `localStorage.setItem` guarded by the `Result` (the spec's
/// `try/catch`). The storage controller is bypassed entirely -- its
/// state may be borrowed at the panic site -- and the marker is the
/// hook's last storage write before the halt.
#[cfg(target_arch = "wasm32")]
fn write_crash_marker(window: &web_sys::Window) {
    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item(CRASH_MARKER_KEY, "1");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_installed_claims_exactly_once() {
        let flag = AtomicBool::new(false);
        assert!(mark_installed(&flag));
        assert!(!mark_installed(&flag));
        assert!(!mark_installed(&flag));
        assert!(flag.load(Ordering::SeqCst));
    }

    #[test]
    fn storage_halted_starts_clear() {
        assert!(!storage_halted());
    }
}
