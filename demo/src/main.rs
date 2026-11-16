use leptos::mount::mount_to_body;

mod app;
mod convert;
mod engine;
mod fidelity;
mod options_model;
mod storage;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
