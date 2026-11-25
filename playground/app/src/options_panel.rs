//! The options panel (spec, Options).

use htmd_playground_core::settings::*;
use leptos::prelude::*;
use web_sys::HtmlInputElement;

use crate::state::Playground;

/// The panel. It starts expanded only on wide screens.
#[component]
pub fn OptionsPanel(playground: Playground, initially_open: bool) -> impl IntoView {
    let is_html_to_markdown = move || playground.direction.get() == Direction::HtmlToMarkdown;
    view! {
        <details class="options" open=initially_open>
            <summary>"Options"</summary>
            <div class="options-body">
                <Show when=is_html_to_markdown>
                    <HtmdOptionsGroup playground />
                </Show>
                <fieldset class="option-group">
                    <legend>"pulldown-cmark extensions"</legend>
                    <p class="group-note">
                        "Used to render Markdown in both directions."
                    </p>
                    <div class="checkbox-grid">
                        {Extension::ALL
                            .into_iter()
                            .map(|ext| extension_checkbox(playground, ext))
                            .collect_view()}
                    </div>
                </fieldset>
                <button
                    type="button"
                    class="button"
                    on:click=move |_| playground.update_settings(|settings| *settings = Settings::default())
                >
                    "Reset to defaults"
                </button>
            </div>
        </details>
    }
}

#[component]
fn HtmdOptionsGroup(playground: Playground) -> impl IntoView {
    let htmd = move |read: fn(&HtmdOptions) -> bool| {
        move || playground.settings.with(|settings| read(&settings.htmd))
    };
    let faithful = htmd(|o| o.translation_mode == TranslationMode::Faithful);
    let fenced = htmd(|o| o.code_block_style == CodeBlockStyle::Fenced);
    let referenced = htmd(|o| o.link_style == LinkStyle::Referenced);
    let never = || false;

    view! {
        <fieldset class="option-group">
            <legend>"htmd"</legend>
            <div class="option-grid">
                {choice_select(
                    playground,
                    "translation-mode",
                    "Translation mode",
                    "Pure writes plain Markdown, dropping what it cannot express, such as attributes; Faithful keeps such HTML as raw HTML.",
                    |o| o.translation_mode,
                    |o, v| o.translation_mode = v,
                    never,
                )}
                {choice_select(
                    playground,
                    "heading-style",
                    "Heading style",
                    "Setext underlines <h1> and <h2>; other headings use ATX.",
                    |o| o.heading_style,
                    |o, v| o.heading_style = v,
                    never,
                )}
                {choice_select(
                    playground,
                    "hr-style",
                    "Horizontal rule",
                    "How <hr> is written.",
                    |o| o.hr_style,
                    |o, v| o.hr_style = v,
                    never,
                )}
                {choice_select(
                    playground,
                    "br-style",
                    "Line break",
                    "How <br> is written. Faithful mode always writes <br>.",
                    |o| o.br_style,
                    |o, v| o.br_style = v,
                    faithful,
                )}
                {choice_select(
                    playground,
                    "bullet-marker",
                    "Bullet marker",
                    "The character that starts each bullet list item.",
                    |o| o.bullet_marker,
                    |o, v| o.bullet_marker = v,
                    never,
                )}
                {spacing_input(
                    playground,
                    "bullet-spacing",
                    "Spaces after bullet",
                    "Spaces between the bullet and the item's content, 1–4.",
                    |o| o.bullet_spacing,
                    |o, v| o.bullet_spacing = v,
                )}
                {spacing_input(
                    playground,
                    "number-spacing",
                    "Spaces after list number",
                    "Spaces after the period of the widest-numbered item, 1–4.",
                    |o| o.number_spacing,
                    |o, v| o.number_spacing = v,
                )}
                {choice_select(
                    playground,
                    "code-block-style",
                    "Code block style",
                    "Fenced blocks keep the language name; indented blocks are indented four spaces.",
                    |o| o.code_block_style,
                    |o, v| o.code_block_style = v,
                    never,
                )}
                <Show when=fenced>
                    {choice_select(
                        playground,
                        "code-fence",
                        "Code fence",
                        "The fence around fenced code blocks.",
                        |o| o.code_fence,
                        |o, v| o.code_fence = v,
                        never,
                    )}
                </Show>
                {choice_select(
                    playground,
                    "link-style",
                    "Link style",
                    "Inline links; autolinks (<url>) where a link's text is its URL; or references listed at the end.",
                    |o| o.link_style,
                    |o, v| o.link_style = v,
                    never,
                )}
                <Show when=referenced>
                    {choice_select(
                        playground,
                        "reference-style",
                        "Reference style",
                        "How a referenced link names its definition.",
                        |o| o.reference_style,
                        |o, v| o.reference_style = v,
                        never,
                    )}
                </Show>
                {checkbox(
                    playground,
                    "preserve-code-whitespace",
                    "Preserve code whitespace",
                    "Keep the whitespace inside inline <code> instead of trimming it.",
                    |o| o.preserve_code_whitespace,
                    |o, v| o.preserve_code_whitespace = v,
                )}
                {checkbox(
                    playground,
                    "noscript-raw",
                    "Treat <noscript> content as raw text",
                    "Parse as a browser with scripting on does, so <noscript> holds text, not elements.",
                    |o| o.noscript_as_raw_text,
                    |o, v| o.noscript_as_raw_text = v,
                )}
                <SkipTagsField playground />
            </div>
        </fieldset>
    }
}

fn help_id(id: &str) -> String {
    format!("{id}-help")
}

fn choice_select<T: Choice + Send + Sync>(
    playground: Playground,
    id: &'static str,
    label: &'static str,
    help: &'static str,
    get: fn(&HtmdOptions) -> T,
    set: fn(&mut HtmdOptions, T),
    disabled: impl Fn() -> bool + Send + Sync + 'static,
) -> impl IntoView {
    let current = move || playground.settings.with(|settings| get(&settings.htmd));
    view! {
        <div class="option">
            <label for=id>{label}</label>
            <select
                id=id
                aria-describedby=help_id(id)
                disabled=disabled
                on:change=move |event| {
                    if let Some(value) = T::from_code(&event_target_value(&event)) {
                        playground.update_settings(|settings| set(&mut settings.htmd, value));
                    }
                }
            >
                {T::ALL
                    .iter()
                    .map(|choice| {
                        let choice = *choice;
                        view! {
                            <option value=choice.code() prop:selected=move || current() == choice>
                                {choice.label()}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>
            <small class="help" id=help_id(id)>{help}</small>
        </div>
    }
}

fn spacing_input(
    playground: Playground,
    id: &'static str,
    label: &'static str,
    help: &'static str,
    get: fn(&HtmdOptions) -> u8,
    set: fn(&mut HtmdOptions, u8),
) -> impl IntoView {
    let current = move || playground.settings.with(|settings| get(&settings.htmd));
    let current_untracked = move || {
        playground
            .settings
            .with_untracked(|settings| get(&settings.htmd))
    };
    view! {
        <div class="option">
            <label for=id>{label}</label>
            <input
                type="number"
                id=id
                min="1"
                max="4"
                step="1"
                inputmode="numeric"
                aria-describedby=help_id(id)
                prop:value=move || current().to_string()
                on:input=move |event| {
                    let field: HtmlInputElement = event_target(&event);
                    // A number field reports "" while it holds no valid
                    // number; the last value stays in effect until blur.
                    let Ok(number) = field.value().trim().parse::<f64>() else {
                        return;
                    };
                    let value = clamp_list_spacing(number.round() as i64);
                    if field.value() != value.to_string() {
                        field.set_value(&value.to_string());
                    }
                    if value != current_untracked() {
                        playground.update_settings(|settings| set(&mut settings.htmd, value));
                    }
                }
                on:blur=move |event| {
                    let field: HtmlInputElement = event_target(&event);
                    field.set_value(&current_untracked().to_string());
                }
            />
            <small class="help" id=help_id(id)>{help}</small>
        </div>
    }
}

fn checkbox(
    playground: Playground,
    id: &'static str,
    label: &'static str,
    help: &'static str,
    get: fn(&HtmdOptions) -> bool,
    set: fn(&mut HtmdOptions, bool),
) -> impl IntoView {
    view! {
        <div class="option option-checkbox">
            <input
                type="checkbox"
                id=id
                aria-describedby=help_id(id)
                prop:checked=move || playground.settings.with(|settings| get(&settings.htmd))
                on:change=move |event| {
                    let checked = event_target_checked(&event);
                    playground.update_settings(|settings| set(&mut settings.htmd, checked));
                }
            />
            <label for=id>{label}</label>
            <small class="help" id=help_id(id)>{help}</small>
        </div>
    }
}

#[component]
fn SkipTagsField(playground: Playground) -> impl IntoView {
    let text = move || {
        playground
            .settings
            .with(|settings| settings.htmd.skip_tags.clone())
    };
    let ignored = move || {
        playground
            .settings
            .with(|settings| parse_skip_tags(&settings.htmd.skip_tags).ignored)
    };
    view! {
        <div class="option option-wide">
            <label for="skip-tags">"Skip tags"</label>
            <input
                type="text"
                id="skip-tags"
                spellcheck="false"
                autocomplete="off"
                autocapitalize="off"
                placeholder="e.g. script, style"
                aria-describedby="skip-tags-help skip-tags-ignored"
                prop:value=text
                on:input=move |event| {
                    let value = event_target_value(&event);
                    playground.update_settings(|settings| settings.htmd.skip_tags = value);
                }
            />
            <small class="help" id="skip-tags-help">
                "Elements to remove together with their content, separated by commas or spaces. Case does not matter."
            </small>
            <small class="help ignored" id="skip-tags-ignored">
                {move || {
                    let ignored = ignored();
                    (!ignored.is_empty())
                        .then(|| format!("Ignored, not valid tag names: {}", ignored.join(", ")))
                }}
            </small>
        </div>
    }
}

fn extension_checkbox(playground: Playground, ext: Extension) -> impl IntoView {
    let id = format!("ext-{}", ext.code());
    let help = help_id(&id);
    view! {
        <div class="option option-checkbox">
            <input
                type="checkbox"
                id=id.clone()
                aria-describedby=help.clone()
                prop:checked=move || playground.settings.with(|settings| settings.extensions.contains(ext))
                on:change=move |event| {
                    let checked = event_target_checked(&event);
                    playground.update_settings(|settings| settings.extensions.set(ext, checked));
                }
            />
            <label for=id>{ext.label()}</label>
            <small class="help" id=help>{ext.help()}</small>
        </div>
    }
}
