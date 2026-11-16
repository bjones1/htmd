//! The htmd playground's user interface, a Leptos client-side app. The page
//! loads it after the converter (see `static/main.js`). The behavior is
//! specified in `pulldown-cmark-htmd-demo-2/htmddemo2-spec.md`.

mod address;
mod app;
mod clipboard;
mod engine;
mod frame;
mod options_panel;
mod state;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
