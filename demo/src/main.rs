use leptos::mount::mount_to_body;

mod app;
mod convert;
mod engine;
mod fidelity;
mod limits;
mod options_model;
mod panic_hook;
mod storage;

fn main() {
    // FR-7: the chained hook must be in place before mount.
    #[cfg(target_arch = "wasm32")]
    panic_hook::install();
    #[cfg(not(target_arch = "wasm32"))]
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
