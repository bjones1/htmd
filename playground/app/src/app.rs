//! The page (spec, Page layout).

use htmd_playground_core::presets::{self, Preset};
use htmd_playground_core::settings::{Choice, Direction};
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};

use crate::address;
use crate::clipboard::{self, MANUAL_COPY};
use crate::frame;
use crate::options_panel::OptionsPanel;
use crate::state::{AddressNotice, Phase, Playground, Tab};

const REPOSITORY: &str = "https://github.com/dkaimekinmasters/paradigms-htmd";

#[component]
pub fn App() -> impl IntoView {
    let playground = Playground::new();
    // The view is mounted synchronously, so the input exists by the time
    // this runs.
    queue_microtask(move || playground.load_from_address());
    let listener = window_event_listener(ev::hashchange, move |_| playground.load_from_address());
    on_cleanup(move || listener.remove());
    let wide = window()
        .match_media("(min-width: 800px)")
        .ok()
        .flatten()
        .is_some_and(|query| query.matches());

    view! {
        <header class="page-header">
            <h1>"htmd playground"</h1>
            <p class="versions">
                {format!(
                    "htmd {} · pulldown-cmark {}",
                    env!("HTMD_VERSION"),
                    env!("PULLDOWN_CMARK_VERSION"),
                )}
            </p>
            <nav aria-label="Links">
                <a href=REPOSITORY>"Source"</a>
                <a href="https://docs.rs/htmd">"docs.rs"</a>
                <a href="https://crates.io/crates/htmd">"crates.io"</a>
            </nav>
        </header>
        <main>
            <Toolbar playground />
            <OptionsPanel playground initially_open=wide />
            <div class="panes">
                <InputPane playground />
                <div class="pane output-pane">
                    <OutputPane playground />
                    <StatusLine playground />
                </div>
            </div>
        </main>
    }
}

#[component]
fn Toolbar(playground: Playground) -> impl IntoView {
    let share_field = NodeRef::<html::Input>::new();
    let share = move |_| {
        playground.flush();
        if playground.address_notice.get_untracked() == Some(AddressNotice::TooLargeToShare) {
            // The notice already says the input is too large to share.
            return;
        }
        let href = address::href();
        clipboard::copy(&href.clone(), move |copied| {
            if copied {
                playground.message.set(Some("Link copied.".into()));
            } else {
                playground.share_fallback.set(Some(href));
                playground.message.set(Some(MANUAL_COPY.into()));
                request_animation_frame(move || {
                    if let Some(field) = share_field.get_untracked() {
                        let _ = field.focus();
                        field.select();
                    }
                });
            }
        });
    };

    view! {
        <div class="toolbar">
            <fieldset class="direction" disabled=move || playground.is_stale()>
                <legend class="visually-hidden">"Direction"</legend>
                {Direction::ALL
                    .iter()
                    .map(|direction| {
                        let direction = *direction;
                        let id = format!("direction-{}", direction.code());
                        view! {
                            <input
                                type="radio"
                                name="direction"
                                id=id.clone()
                                prop:checked=move || playground.direction.get() == direction
                                on:change=move |event| {
                                    if !playground.switch_direction() {
                                        // The output went stale; undo the click.
                                        let radio: HtmlInputElement = event_target(&event);
                                        radio.set_checked(false);
                                        let current = playground.direction.get_untracked();
                                        if let Some(other) = document()
                                            .get_element_by_id(&format!("direction-{}", current.code()))
                                            .and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
                                        {
                                            other.set_checked(true);
                                        }
                                    }
                                }
                            />
                            <label for=id>{direction.label()}</label>
                        }
                    })
                    .collect_view()}
            </fieldset>
            <button type="button" class="button" on:click=share>
                "Share"
            </button>
            <Show when=move || playground.share_fallback.with(Option::is_some)>
                <label class="share-fallback">
                    <span class="visually-hidden">"Link to this playground state"</span>
                    <input
                        type="text"
                        readonly
                        node_ref=share_field
                        prop:value=move || playground.share_fallback.get().unwrap_or_default()
                    />
                </label>
            </Show>
        </div>
    }
}

#[component]
fn InputPane(playground: Playground) -> impl IntoView {
    let language = move || playground.direction.get().input_language();
    let clear = move |_| {
        if confirm_replacing(&playground, "Clear your input?") {
            playground.replace_input("");
        }
    };
    view! {
        <div class="pane input-pane">
            <div class="pane-bar">
                <label for="input" class="pane-title">{language}</label>
                <div class="pane-actions">
                    <ExamplesMenu playground />
                    <button type="button" class="button" on:click=clear>"Clear"</button>
                </div>
            </div>
            <textarea
                id="input"
                class="editor"
                spellcheck="false"
                autocomplete="off"
                autocapitalize="off"
                node_ref=playground.input
                on:input=move |_| playground.input_changed()
            ></textarea>
        </div>
    }
}

/// Ask before discarding input the user wrote (spec, Input pane).
fn confirm_replacing(playground: &Playground, question: &str) -> bool {
    !presets::replacing_needs_confirmation(&playground.input_text())
        || window().confirm_with_message(question).unwrap_or(false)
}

/// A menu button listing the current direction's presets.
#[component]
fn ExamplesMenu(playground: Playground) -> impl IntoView {
    let open = RwSignal::new(false);
    let button = NodeRef::<html::Button>::new();
    let list = NodeRef::<html::Ul>::new();
    let close = move |refocus: bool| {
        open.set(false);
        if refocus && let Some(button) = button.get_untracked() {
            let _ = button.focus();
        }
    };
    let pick = move |preset: Preset| {
        close(true);
        if !confirm_replacing(&playground, "Replace your input with this example?") {
            return;
        }
        if let Some(ext) = preset.extension
            && !playground
                .settings
                .with_untracked(|s| s.extensions.contains(ext))
        {
            playground.settings.update(|s| s.extensions.set(ext, true));
        }
        playground.replace_input(preset.text);
    };
    let menu_items = move || -> Vec<HtmlElement> {
        list.get_untracked()
            .map(|list| {
                let items = list.query_selector_all("button").ok();
                items
                    .map(|items| {
                        (0..items.length())
                            .filter_map(|i| items.item(i)?.dyn_into::<HtmlElement>().ok())
                            .collect()
                    })
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    };
    let focus_item = move |offset: i32| {
        let items = menu_items();
        if items.is_empty() {
            return;
        }
        let active = document().active_element();
        let current = items
            .iter()
            .position(|item| active.as_ref() == Some(item.unchecked_ref()));
        let count = items.len() as i32;
        let next = match current {
            Some(index) => (index as i32 + offset).rem_euclid(count),
            None if offset < 0 => count - 1,
            None => 0,
        };
        let _ = items[next as usize].focus();
    };
    let toggle = move |_| {
        let now_open = !open.get_untracked();
        open.set(now_open);
        if now_open {
            request_animation_frame(move || focus_item(1));
        }
    };
    let on_keydown = move |event: KeyboardEvent| match event.key().as_str() {
        "Escape" => {
            event.prevent_default();
            close(true);
        }
        "ArrowDown" => {
            event.prevent_default();
            focus_item(1);
        }
        "ArrowUp" => {
            event.prevent_default();
            focus_item(-1);
        }
        "Home" => {
            event.prevent_default();
            focus_item(-1000 * menu_items().len() as i32);
        }
        "End" => {
            event.prevent_default();
            if let Some(last) = menu_items().last() {
                let _ = last.focus();
            }
        }
        _ => {}
    };
    // Close when focus leaves the menu, e.g. on Tab or a click elsewhere.
    let on_focusout = move |event: web_sys::FocusEvent| {
        let container: web_sys::Element = event_target::<web_sys::Element>(&event)
            .closest(".menu")
            .ok()
            .flatten()
            .expect("menu container");
        let inside = event
            .related_target()
            .and_then(|target| target.dyn_into::<web_sys::Node>().ok())
            .is_some_and(|target| container.contains(Some(&target)));
        if !inside {
            close(false);
        }
    };

    view! {
        <div class="menu" on:keydown=on_keydown on:focusout=on_focusout>
            <button
                type="button"
                class="button"
                aria-haspopup="true"
                aria-expanded=move || open.get().to_string()
                aria-controls="examples-list"
                node_ref=button
                on:click=toggle
            >
                "Examples"
                <span aria-hidden="true">" ▾"</span>
            </button>
            <Show when=move || open.get()>
                <ul class="menu-list" id="examples-list" node_ref=list aria-label="Examples">
                    {move || {
                        presets::presets(playground.direction.get())
                            .iter()
                            .map(|preset| {
                                let preset = *preset;
                                view! {
                                    <li>
                                        <button type="button" on:click=move |_| pick(preset)>
                                            {preset.name}
                                        </button>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}
                </ul>
            </Show>
        </div>
    }
}

#[component]
fn OutputPane(playground: Playground) -> impl IntoView {
    view! {
        <Show
            when=move || !matches!(playground.phase.get(), Phase::Crashed(_))
            fallback=move || view! { <CrashPanel playground /> }
        >
            <OutputTabs playground />
        </Show>
    }
}

fn tab_id(tab: Tab) -> String {
    format!("tab-{}", tab.label().to_lowercase())
}

fn panel_id(tab: Tab) -> String {
    format!("panel-{}", tab.label().to_lowercase())
}

#[component]
fn OutputTabs(playground: Playground) -> impl IntoView {
    let tabs = move || Tab::all(playground.direction.get());
    let select = move |tab: Tab| {
        playground.tab.set(tab);
        if let Some(element) = document()
            .get_element_by_id(&tab_id(tab))
            .and_then(|e| e.dyn_into::<HtmlElement>().ok())
        {
            let _ = element.focus();
        }
    };
    // Arrow keys move between tabs, per the WAI-ARIA tabs pattern.
    let on_keydown = move |event: KeyboardEvent| {
        let tabs = tabs();
        let Some(index) = tabs
            .iter()
            .position(|tab| *tab == playground.tab.get_untracked())
        else {
            return;
        };
        let next = match event.key().as_str() {
            "ArrowRight" => (index + 1) % tabs.len(),
            "ArrowLeft" => (index + tabs.len() - 1) % tabs.len(),
            "Home" => 0,
            "End" => tabs.len() - 1,
            _ => return,
        };
        event.prevent_default();
        select(tabs[next]);
    };
    let output_language = move || playground.direction.get().output_language();
    let copy = move |_| {
        playground.flush();
        if playground.is_stale() {
            return;
        }
        let text = playground
            .output
            .with_untracked(|output| output.source.clone());
        clipboard::copy(&text, move |copied| {
            if copied {
                playground.message.set(Some("Copied.".into()));
            } else {
                if let Some(source) = document()
                    .get_element_by_id("output-source")
                    .and_then(|e| e.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
                {
                    let _ = source.focus();
                    source.select();
                }
                playground.message.set(Some(MANUAL_COPY.into()));
            }
        });
    };

    view! {
        <div class="pane-bar">
            <div role="tablist" aria-label="Output" class="tabs" on:keydown=on_keydown>
                {move || {
                    tabs()
                        .iter()
                        .map(|tab| {
                            let tab = *tab;
                            let selected = move || playground.tab.get() == tab;
                            view! {
                                <button
                                    type="button"
                                    role="tab"
                                    id=tab_id(tab)
                                    aria-controls=panel_id(tab)
                                    aria-selected=move || selected().to_string()
                                    tabindex=move || if selected() { "0" } else { "-1" }
                                    on:click=move |_| playground.tab.set(tab)
                                >
                                    {tab.label()}
                                </button>
                            }
                        })
                        .collect_view()
                }}
            </div>
            <Show when=move || playground.tab.get() == Tab::Source>
                <button
                    type="button"
                    class="button"
                    disabled=move || playground.is_stale()
                    on:click=copy
                >
                    "Copy"
                </button>
            </Show>
        </div>
        <div
            class="tab-panel"
            role="tabpanel"
            id=move || panel_id(playground.tab.get())
            aria-labelledby=move || tab_id(playground.tab.get())
        >
            {move || match playground.tab.get() {
                Tab::Source => {
                    view! {
                        <textarea
                            id="output-source"
                            class="editor"
                            readonly
                            spellcheck="false"
                            aria-label=move || format!("{} output", output_language())
                            prop:value=move || playground.output.with(|output| output.source.clone())
                        ></textarea>
                    }
                        .into_any()
                }
                Tab::Rendered => {
                    let title = move || format!("Rendered {}", output_language_rendered(playground));
                    view! {
                        <iframe
                            class="page-frame"
                            sandbox=""
                            title=title
                            srcdoc=move || playground.output.with(|output| frame::page(&output.rendered_html))
                        ></iframe>
                    }
                        .into_any()
                }
                Tab::Original => {
                    view! {
                        <iframe
                            class="page-frame"
                            sandbox=""
                            title="Original HTML"
                            srcdoc=move || playground.output.with(|output| frame::page(&output.original_html))
                        ></iframe>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}

/// What the Rendered tab shows: the Markdown output converted back, or the
/// HTML output.
fn output_language_rendered(playground: Playground) -> &'static str {
    match playground.direction.get() {
        Direction::HtmlToMarkdown => "Markdown",
        Direction::MarkdownToHtml => "HTML",
    }
}

#[component]
fn CrashPanel(playground: Playground) -> impl IntoView {
    let message = move || match playground.phase.get() {
        Phase::Crashed(message) => message,
        _ => String::new(),
    };
    let copy_input = move |_| {
        clipboard::copy(&playground.input_text(), move |copied| {
            if copied {
                playground.message.set(Some("Input copied.".into()));
            } else {
                if let Some(input) = playground.input.get_untracked() {
                    let _ = input.focus();
                    input.select();
                }
                playground.message.set(Some(MANUAL_COPY.into()));
            }
        });
    };
    let reload = move |event: ev::MouseEvent| {
        event.prevent_default();
        let _ = window().location().reload();
    };
    let start_over_href = address::without_fragment();
    let start_over = {
        let href = start_over_href.clone();
        move |event: ev::MouseEvent| {
            event.prevent_default();
            // Replacing keeps the crashing address out of the history.
            let _ = window().location().replace(&href);
        }
    };
    view! {
        <div class="crash" role="alert">
            <h2>"The converter crashed"</h2>
            <p class="crash-message">{message}</p>
            <p>
                "Nothing will convert until the page is reloaded. You can still edit the input. "
                "If the saved input itself causes the crash, start over without it."
            </p>
            <div class="crash-actions">
                <button type="button" class="button" on:click=copy_input>
                    "Copy input"
                </button>
                <a href=address::href() on:click=reload>"Reload"</a>
                <a href=start_over_href on:click=start_over>"Start over"</a>
            </div>
        </div>
    }
}

#[component]
fn StatusLine(playground: Playground) -> impl IntoView {
    let metrics = move || {
        if matches!(playground.phase.get(), Phase::Crashed(_)) {
            return None;
        }
        playground.output.with(|output| {
            output.millis.map(|millis| {
                let unit = if output.chars == 1 {
                    "character"
                } else {
                    "characters"
                };
                format!("{millis:.1} ms · {} {unit}", output.chars)
            })
        })
    };
    let notices = move || {
        let mut notices = Vec::new();
        if playground.phase.get() == Phase::Paused {
            notices.push(
                "The input is over 1 MiB, so conversion is paused; the output is out of date."
                    .to_string(),
            );
        }
        match playground.address_notice.get() {
            Some(AddressNotice::InvalidLink) => notices
                .push("The link was invalid, so the default example is shown.".to_string()),
            Some(AddressNotice::TooLargeToShare) => notices.push(
                "The input is too large to share: the address keeps only the direction and options."
                    .to_string(),
            ),
            None => {}
        }
        if let Some(message) = playground.message.get() {
            notices.push(message);
        }
        notices.join(" ")
    };
    view! {
        <div class="status">
            <span class="metrics">{metrics}</span>
            <span class="notices" role="status">{notices}</span>
            <Show when=move || playground.phase.get() == Phase::Paused>
                <button type="button" class="button" on:click=move |_| playground.convert_anyway()>
                    "Convert anyway"
                </button>
            </Show>
        </div>
    }
}
