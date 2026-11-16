//! The demo application (demo-spec.md §3–§5): Leptos CSR wiring of the
//! pure-logic modules — [`crate::engine`] decides *when* a conversion
//! runs, [`crate::convert`]/[`crate::fidelity`] produce its output,
//! [`crate::storage`] persists the workspace, [`crate::limits`] and
//! [`crate::samples`] own their small state machines, and this module
//! turns their inputs and actions into DOM. All user-derived text flows
//! through Leptos view macros (textContent); the only markup the app
//! ever assigns is the sandboxed iframe's `srcdoc` DOM property
//! (FR-1:40, FR-3:73).

use std::cell::RefCell;
use std::rc::Rc;

use htmd::options::TranslationMode;
use leptos::ev;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use wasm_bindgen::JsCast;

use crate::convert::{convert_html_to_md, convert_md_to_html};
use crate::engine::{Action, CommitOutcome, ConversionMode, Engine, EngineEvent};
use crate::fidelity::{DiffToken, WordDiff, canonicalize, word_diff};
use crate::limits::{BannerKind, Banners};
use crate::options_model::{CmarkFlags, HtmdOptions, demo_preset, parse_skip_tags};
use crate::panic_hook;
use crate::preview::build_srcdoc;
use crate::samples::{SAMPLES, SampleId, SampleState};
use crate::storage::{
    ActiveTab, BannerSignal, LoadOutcome, Mode as StoredMode, RecoveryChoice, StorageController,
    StorageEvent, Store,
};

/// FR-7's client-side processing ceiling, in UTF-8 bytes of the input.
const MAX_INPUT_BYTES: usize = 100_000;

/// §6:214 — Markdown → HTML output over this many UTF-8 bytes is cleared
/// instead of previewed, both tabs, banner shown.
const MAX_DISPLAY_OUTPUT_BYTES: usize = 2_000_000;

/// §6:214, verbatim: the output-cap banner.
const OUTPUT_CAP_TEXT: &str = "Output exceeds the 2 MB display limit.";

/// FR-8:178, verbatim: the quota-failure banner.
const SAVE_FAILED_TEXT: &str =
    "Workspace could not be saved; recent changes will be lost on reload.";

/// FR-8:179, verbatim: the multi-tab banner.
const EXTERNAL_WRITE_TEXT: &str =
    "This workspace was changed in another tab; saving here will overwrite it.";

/// FR-8:160's recovery banner copy (button labels are the spec's).
const RECOVERY_TEXT: &str = "The application closed unexpectedly during the previous session. The workspace was restored without converting anything.";

/// FR-5:116's explanatory note shown while `translation_mode` is Pure.
const FIDELITY_NOTE: &str = "The fidelity check runs in Faithful translation mode only: in Pure mode, intentional attribute loss would reduce the diff to noise.";

/// FR-2:62's undo toast copy.
const UNDO_TOAST_TEXT: &str = "The previous input was replaced.";

/// One mode's committed output: the Source-tab text, the Rendered-tab
/// srcdoc document, and FR-2's output-validity flag.
#[derive(Debug, Clone, PartialEq, Default)]
struct ModeOutput {
    source: String,
    preview: String,
    valid: bool,
}

/// The FR-5 word diff, as rendered by the fidelity panel.
#[derive(Debug, Clone, PartialEq, Default)]
enum FidelityDiff {
    #[default]
    Inclusive,
    TooLarge,
    Changes(Vec<DiffToken>),
}

/// The fidelity panel's committed content (FR-5). Hidden or replaced by
/// the Pure-mode note depending on mode and `translation_mode`.
#[derive(Debug, Clone, PartialEq, Default)]
struct FidelityPanel {
    original: String,
    roundtrip: String,
    diff: FidelityDiff,
}

/// What the fidelity section renders right now.
#[derive(Debug, Clone, PartialEq)]
enum FidelityView {
    /// Markdown → HTML mode: the panel does not apply (FR-5 is
    /// HTML-mode only).
    Hidden,
    /// HTML → Markdown with `translation_mode: Pure` (FR-5:116).
    Note,
    Panel(FidelityPanel),
}

/// Every reactive field of the UI. `Copy` so event-handler closures
/// capture it freely.
#[derive(Clone, Copy)]
struct Ui {
    mode: RwSignal<ConversionMode>,
    /// Drives the textarea's `value` property only; typing never writes
    /// it, so programmatic buffer swaps cannot reset the caret.
    editor_text: RwSignal<String>,
    buf_md: RwSignal<String>,
    buf_html: RwSignal<String>,
    scroll_md: RwSignal<f64>,
    scroll_html: RwSignal<f64>,
    sel_md: RwSignal<(u32, u32)>,
    sel_html: RwSignal<(u32, u32)>,
    cmark: RwSignal<CmarkFlags>,
    htmd: RwSignal<HtmdOptions>,
    /// Programmatic mirror of the `skip_tags` field (Reset/restore);
    /// the live text lives in [`Core::skip_tags_raw`].
    skip_tags: RwSignal<String>,
    active_tab: RwSignal<ActiveTab>,
    out_md: RwSignal<ModeOutput>,
    out_html: RwSignal<ModeOutput>,
    fidelity: RwSignal<FidelityPanel>,
    undo_visible: RwSignal<bool>,
    size_banner: RwSignal<bool>,
    nesting_banner: RwSignal<bool>,
    cap_banner: RwSignal<bool>,
    save_failed: RwSignal<bool>,
    external_write: RwSignal<bool>,
    recovery: RwSignal<bool>,
}

impl Ui {
    fn new() -> Self {
        let (cmark, htmd) = demo_preset();
        Ui {
            mode: RwSignal::new(ConversionMode::MarkdownToHtml),
            editor_text: RwSignal::new(String::new()),
            buf_md: RwSignal::new(String::new()),
            buf_html: RwSignal::new(String::new()),
            scroll_md: RwSignal::new(0.0),
            scroll_html: RwSignal::new(0.0),
            sel_md: RwSignal::new((0, 0)),
            sel_html: RwSignal::new((0, 0)),
            cmark: RwSignal::new(cmark),
            htmd: RwSignal::new(htmd),
            skip_tags: RwSignal::new(String::new()),
            active_tab: RwSignal::new(ActiveTab::Rendered),
            out_md: RwSignal::new(ModeOutput::default()),
            out_html: RwSignal::new(ModeOutput::default()),
            fidelity: RwSignal::new(FidelityPanel::default()),
            undo_visible: RwSignal::new(false),
            size_banner: RwSignal::new(false),
            nesting_banner: RwSignal::new(false),
            cap_banner: RwSignal::new(false),
            save_failed: RwSignal::new(false),
            external_write: RwSignal::new(false),
            recovery: RwSignal::new(false),
        }
    }

    fn buffer(self, mode: ConversionMode) -> RwSignal<String> {
        match mode {
            ConversionMode::MarkdownToHtml => self.buf_md,
            ConversionMode::HtmlToMarkdown => self.buf_html,
        }
    }

    fn scroll(self, mode: ConversionMode) -> RwSignal<f64> {
        match mode {
            ConversionMode::MarkdownToHtml => self.scroll_md,
            ConversionMode::HtmlToMarkdown => self.scroll_html,
        }
    }

    fn selection(self, mode: ConversionMode) -> RwSignal<(u32, u32)> {
        match mode {
            ConversionMode::MarkdownToHtml => self.sel_md,
            ConversionMode::HtmlToMarkdown => self.sel_html,
        }
    }

    fn output(self, mode: ConversionMode) -> RwSignal<ModeOutput> {
        match mode {
            ConversionMode::MarkdownToHtml => self.out_md,
            ConversionMode::HtmlToMarkdown => self.out_html,
        }
    }
}

/// The non-reactive application state: the pure-logic modules plus the
/// two pieces of UI bookkeeping that belong to no module (the
/// "Use Output as Input" undo slot and the raw `skip_tags` text).
struct Core {
    engine: Engine,
    banners: Banners,
    samples: SampleState,
    storage: StorageController<Box<dyn Store>>,
    /// FR-2:62 undo for "Use Output as Input" (sample-load undo lives in
    /// [`SampleState`]); the latest overwrite owns the single slot.
    use_output_undo: Option<(ConversionMode, String)>,
    /// FR-4:108 — the live `skip_tags` field text; not reactive so
    /// per-keystroke edits cannot rewrite the input's value mid-typing.
    skip_tags_raw: String,
    /// Set while the FR-8 recovery banner's "Convert restored input"
    /// run is in flight; its success removes the crash marker.
    recovery_converting: bool,
}

/// The shared core. `SendWrapper` because Leptos views (including their
/// event handlers) must be `Send` even in CSR; the wrapper is only ever
/// opened on the main thread.
type CoreRef = SendWrapper<Rc<RefCell<Core>>>;

thread_local! {
    /// Installed once by [`App`]. Event-handler closures re-fetch the
    /// core through [`core`] instead of capturing it, so no handler
    /// moves a non-`Copy` value and every closure stays `Fn` + `Send`.
    static CORE: RefCell<Option<CoreRef>> = const { RefCell::new(None) };
}

/// The shared application core.
///
/// # Panics
///
/// Only before [`App`] installs it — every caller runs inside the
/// mounted application.
fn core() -> CoreRef {
    CORE.with(|cell| cell.borrow().clone())
        .expect("Core installed by App")
}

/// The engine owns its [`Clock`](crate::engine::Clock), whose callbacks
/// re-enter [`Core`] — so the clocks are built against this cell, which
/// is filled right after `Core` exists and never emptied for the rest
/// of the page's life.
#[derive(Default)]
struct LateCore(RefCell<Option<CoreRef>>);

impl LateCore {
    fn core(&self) -> Option<CoreRef> {
        self.0.borrow().clone()
    }
}

fn other_mode(mode: ConversionMode) -> ConversionMode {
    match mode {
        ConversionMode::MarkdownToHtml => ConversionMode::HtmlToMarkdown,
        ConversionMode::HtmlToMarkdown => ConversionMode::MarkdownToHtml,
    }
}

fn storage_mode(mode: ConversionMode) -> StoredMode {
    match mode {
        ConversionMode::MarkdownToHtml => StoredMode::MdToHtml,
        ConversionMode::HtmlToMarkdown => StoredMode::HtmlToMd,
    }
}

fn engine_mode(mode: StoredMode) -> ConversionMode {
    match mode {
        StoredMode::MdToHtml => ConversionMode::MarkdownToHtml,
        StoredMode::HtmlToMd => ConversionMode::HtmlToMarkdown,
    }
}

fn other_tab(tab: ActiveTab) -> ActiveTab {
    match tab {
        ActiveTab::Rendered => ActiveTab::Source,
        ActiveTab::Source => ActiveTab::Rendered,
    }
}

// ---------- engine actions → conversion → commit ----------

/// Execute the actions an engine input returned. Dispatches convert
/// synchronously on the main thread (FR-2's execution model) and
/// commits in one step.
fn run_actions(core: &CoreRef, ui: &Ui, actions: Vec<Action>) {
    let mut queue = actions;
    while !queue.is_empty() {
        let action = queue.remove(0);
        let mut c = core.borrow_mut();
        match action {
            Action::DispatchConversion { seq } => {
                convert_and_commit(&mut c, ui, seq, &mut queue);
            }
            Action::ClearOutput => clear_output(&mut c, ui),
        }
    }
}

/// One dispatched run: convert the active buffer, then apply FR-2's
/// commit gate. A blocked HTML conversion enters its FR-7 banner and
/// clears the output instead of committing.
fn convert_and_commit(c: &mut Core, ui: &Ui, seq: u64, queue: &mut Vec<Action>) {
    let mode = c.engine.mode();
    let buffer = ui.buffer(mode).get_untracked();
    let cmark = ui.cmark.get_untracked();
    match mode {
        ConversionMode::MarkdownToHtml => {
            let html = convert_md_to_html(&buffer, &cmark);
            if html.len() > MAX_DISPLAY_OUTPUT_BYTES {
                // §6:214 — both tabs and the fidelity panel clear; the
                // empty-output rule disables "Use Output as Input".
                ui.cap_banner.set(true);
                queue.extend(c.engine.on_limit_entered());
                clear_output(c, ui);
                return;
            }
            match c.engine.on_conversion_ready(seq) {
                CommitOutcome::Commit => {
                    let preview = build_srcdoc(&html);
                    ui.output(mode).set(ModeOutput {
                        source: html,
                        preview,
                        valid: true,
                    });
                    conversion_succeeded(c, ui);
                }
                CommitOutcome::Stale => {}
            }
        }
        ConversionMode::HtmlToMarkdown => {
            let opts = ui.htmd.get_untracked();
            let skip = parse_skip_tags(&c.skip_tags_raw);
            match convert_html_to_md(&buffer, &opts, &skip) {
                Err(blocked) => {
                    enter_banner(c, ui, blocked.into());
                    queue.extend(c.engine.on_limit_entered());
                    clear_output(c, ui);
                }
                Ok(markdown) => match c.engine.on_conversion_ready(seq) {
                    CommitOutcome::Commit => {
                        // FR-3: the Rendered tab re-renders the Markdown
                        // with the sidebar's cmark flags.
                        let rerendered = convert_md_to_html(&markdown, &cmark);
                        let preview = build_srcdoc(&rerendered);
                        ui.output(mode).set(ModeOutput {
                            source: markdown.clone(),
                            preview,
                            valid: true,
                        });
                        if opts.translation_mode == TranslationMode::Faithful {
                            // FR-5 pipeline: canonicalize both sides,
                            // diff, render. The forward conversion above
                            // already used the sidebar options with
                            // Faithful forced (the panel only exists in
                            // Faithful mode).
                            let original = canonicalize(&buffer);
                            let roundtrip = canonicalize(&rerendered);
                            let diff = match word_diff(&original, &roundtrip) {
                                WordDiff::Inclusive => FidelityDiff::Inclusive,
                                WordDiff::TooLarge => FidelityDiff::TooLarge,
                                WordDiff::Changes(tokens) => FidelityDiff::Changes(tokens),
                            };
                            ui.fidelity.set(FidelityPanel {
                                original,
                                roundtrip,
                                diff,
                            });
                        } else {
                            ui.fidelity.set(FidelityPanel::default());
                        }
                        conversion_succeeded(c, ui);
                    }
                    CommitOutcome::Stale => {}
                },
            }
        }
    }
}

/// Shared tail of every successful commit: banners that the resumed
/// processing disproves clear (FR-7), and a pending recovery conversion
/// removes the crash marker (FR-8:160).
fn conversion_succeeded(c: &mut Core, ui: &Ui) {
    ui.cap_banner.set(false);
    exit_banner(c, ui, BannerKind::SizeLimit);
    exit_banner(c, ui, BannerKind::NestingLimit);
    if c.recovery_converting {
        c.recovery_converting = false;
        if !panic_hook::storage_halted() {
            c.storage.recovery_conversion_succeeded();
        }
    }
}

/// FR-2's `ClearOutput`: assign `""` to the preview srcdoc and the
/// Source text together, invalidate the output, and clear the fidelity
/// panel when it belongs to the active mode.
fn clear_output(c: &mut Core, ui: &Ui) {
    let mode = c.engine.mode();
    ui.output(mode).set(ModeOutput::default());
    if mode == ConversionMode::HtmlToMarkdown {
        ui.fidelity.set(FidelityPanel::default());
    }
}

fn enter_banner(c: &mut Core, ui: &Ui, kind: BannerKind) {
    c.banners.enter(kind);
    sync_banners(c, ui);
}

fn exit_banner(c: &mut Core, ui: &Ui, kind: BannerKind) {
    c.banners.exit(kind);
    sync_banners(c, ui);
}

/// Mirror the banner state machine into the region's signals; the
/// aria-live region announces each message when its element is added,
/// which is exactly the "once per transition" the drained announcement
/// bookkeeping tracks.
fn sync_banners(c: &mut Core, ui: &Ui) {
    ui.size_banner
        .set(c.banners.active().any(|k| k == BannerKind::SizeLimit));
    ui.nesting_banner
        .set(c.banners.active().any(|k| k == BannerKind::NestingLimit));
    let _ = c.banners.take_announced().count();
}

/// Mirror the storage controller's banner states after any interaction
/// with it; a successful write clears them (FR-8).
fn sync_storage_banners(c: &Core, ui: &Ui) {
    ui.save_failed
        .set(c.storage.banner_active(BannerSignal::SaveFailed));
    ui.external_write
        .set(c.storage.banner_active(BannerSignal::ExternalWrite));
}

fn dismiss_undo(c: &mut Core, ui: &Ui) {
    c.samples.dismiss_undo();
    c.use_output_undo = None;
    ui.undo_visible.set(false);
}

/// Write `mode`'s buffer and make `mode` active in the UI (the engine
/// write happens separately through its own input).
fn apply_buffer_write(ui: &Ui, mode: ConversionMode, text: &str) {
    ui.mode.set(mode);
    ui.buffer(mode).set(text.to_owned());
    ui.editor_text.set(text.to_owned());
}

// ---------- editor (textarea) ----------

fn editor_element() -> Option<web_sys::HtmlTextAreaElement> {
    let document = web_sys::window()?.document()?;
    document.get_element_by_id("editor")?.dyn_into().ok()
}

/// FR-2's `on_edit`: classify adaptively by byte length, keep the
/// storage debounce fed, and enter/exit the FR-7 size banner on
/// transitions of the active buffer's length.
fn on_editor_input(core: &CoreRef, ui: &Ui, ev: web_sys::Event) {
    let textarea = event_target::<web_sys::HtmlTextAreaElement>(&ev);
    let value = textarea.value();
    let mut c = core.borrow_mut();
    let mode = c.engine.mode();
    if let (Ok(Some(start)), Ok(Some(end))) = (textarea.selection_start(), textarea.selection_end())
    {
        ui.selection(mode).set((start, end));
    }
    ui.buffer(mode).set(value.clone());
    dismiss_undo(&mut c, ui);
    if !panic_hook::storage_halted() {
        c.storage.on_text_edit(storage_mode(mode), &value);
    }
    let oversized = value.len() > MAX_INPUT_BYTES;
    let actions = match (oversized, ui.size_banner.get_untracked()) {
        (true, false) => {
            enter_banner(&mut c, ui, BannerKind::SizeLimit);
            c.engine.on_limit_entered()
        }
        // Still over the ceiling: the engine refuses to arm a batch, but
        // the buffer mirror must follow the edit.
        (true, true) => c.engine.on_edit(&value),
        (false, _) => {
            exit_banner(&mut c, ui, BannerKind::SizeLimit);
            c.engine.on_edit(&value)
        }
    };
    drop(c);
    run_actions(core, ui, actions);
}

/// FR-8: scroll offsets ride the same 500 ms save debounce as text.
fn on_editor_scroll(core: &CoreRef, ui: &Ui, ev: web_sys::Event) {
    let mode = ui.mode.get_untracked();
    let textarea = event_target::<web_sys::HtmlTextAreaElement>(&ev);
    let top = f64::from(textarea.scroll_top());
    ui.scroll(mode).set(top);
    let mut c = core.borrow_mut();
    if !panic_hook::storage_halted() {
        c.storage.on_scroll(storage_mode(mode), top);
    }
}

/// Capture the editor's scroll offset and selection into the mode's
/// signals before the buffer swap hides them.
fn snapshot_editor(ui: &Ui) {
    let mode = ui.mode.get_untracked();
    let Some(textarea) = editor_element() else {
        return;
    };
    ui.scroll(mode).set(f64::from(textarea.scroll_top()));
    let selection = (
        textarea.selection_start().ok().flatten().unwrap_or(0),
        textarea.selection_end().ok().flatten().unwrap_or(0),
    );
    ui.selection(mode).set(selection);
}

/// Restore the active mode's scroll offset and selection after the
/// buffer swap; a frame later the value's layout has settled.
fn restore_editor_later(ui: &Ui) {
    let mode = ui.mode.get_untracked();
    let scroll = ui.scroll(mode).get_untracked();
    let (start, end) = ui.selection(mode).get_untracked();
    request_animation_frame(move || {
        let Some(textarea) = editor_element() else {
            return;
        };
        textarea.set_scroll_top(scroll as i32);
        let _ = textarea.set_selection_range(start, end);
    });
}

// ---------- mode switching, options, tabs ----------

/// FR-2 mode switch: dismiss the undo toast (its dismissal rules),
/// flush storage before the write, cancel the pending batch inside the
/// engine, and restore the target mode's editor view.
fn switch_mode(core: &CoreRef, ui: &Ui, target: ConversionMode) {
    snapshot_editor(ui);
    let mut c = core.borrow_mut();
    if c.engine.mode() == target {
        return;
    }
    dismiss_undo(&mut c, ui);
    if !panic_hook::storage_halted() {
        let _ = c.storage.set_mode(storage_mode(target));
        sync_storage_banners(&c, ui);
    }
    let actions = c.engine.on_mode_switch();
    ui.mode.set(target);
    ui.editor_text.set(ui.buffer(target).get_untracked());
    drop(c);
    run_actions(core, ui, actions);
    restore_editor_later(ui);
}

/// FR-2 "every option change" plus FR-8's flush-before-write.
fn on_options_changed(core: &CoreRef, ui: &Ui) {
    let mut c = core.borrow_mut();
    let skip_tags = c.skip_tags_raw.clone();
    if !panic_hook::storage_halted() {
        let _ = c.storage.set_options(
            ui.cmark.get_untracked(),
            ui.htmd.get_untracked(),
            &skip_tags,
        );
        sync_storage_banners(&c, ui);
    }
    let actions = c.engine.on_options_change();
    drop(c);
    run_actions(core, ui, actions);
}

/// FR-4:108 — `skip_tags` edits ride the 500 ms save timer and the
/// FR-2 adaptive conversion schedule (the engine's option-change input).
fn on_skip_tags_input(core: &CoreRef, ui: &Ui, ev: web_sys::Event) {
    let value = event_target::<web_sys::HtmlInputElement>(&ev).value();
    let mut c = core.borrow_mut();
    c.skip_tags_raw = value.clone();
    if !panic_hook::storage_halted() {
        c.storage.on_skip_tags_edit(&value);
    }
    drop(c);
    on_options_changed(core, ui);
}

/// FR-4 Reset: restore exactly the preset, then reconvert immediately
/// (the engine's reset input bypasses the debounce).
fn on_reset(core: &CoreRef, ui: &Ui) {
    let (cmark, htmd) = demo_preset();
    let mut c = core.borrow_mut();
    ui.cmark.set(cmark);
    ui.htmd.set(htmd);
    c.skip_tags_raw.clear();
    ui.skip_tags.set(String::new());
    if !panic_hook::storage_halted() {
        let _ = c.storage.reset_options();
        sync_storage_banners(&c, ui);
    }
    let actions = c.engine.on_reset();
    drop(c);
    run_actions(core, ui, actions);
}

fn select_tab(core: &CoreRef, ui: &Ui, tab: ActiveTab) {
    ui.active_tab.set(tab);
    let mut c = core.borrow_mut();
    if !panic_hook::storage_halted() {
        let _ = c.storage.set_active_tab(tab);
        sync_storage_banners(&c, ui);
    }
}

fn focus_tab(tab: ActiveTab) {
    let id = match tab {
        ActiveTab::Rendered => "tab-rendered",
        ActiveTab::Source => "tab-source",
    };
    let Some(el) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
    else {
        return;
    };
    if let Some(html) = el.dyn_ref::<web_sys::HtmlElement>() {
        let _ = html.focus();
    }
}

/// ARIA tabs keyboard support: arrows, Home, End (§6).
fn on_tablist_keydown(core: &CoreRef, ui: &Ui, ev: web_sys::KeyboardEvent) {
    let current = ui.active_tab.get_untracked();
    let next = match ev.key().as_str() {
        "ArrowRight" | "ArrowDown" => Some(other_tab(current)),
        "ArrowLeft" | "ArrowUp" => Some(other_tab(current)),
        "Home" => Some(ActiveTab::Rendered),
        "End" => Some(ActiveTab::Source),
        _ => None,
    };
    if let Some(next) = next {
        ev.prevent_default();
        select_tab(core, ui, next);
        focus_tab(next);
    }
}

// ---------- samples, Use Output as Input, undo ----------

/// FR-6 sample click: the pure module decides the target mode, the
/// text, and whether undo is offered; the click's selection becomes the
/// target mode's persisted sample.
fn on_sample_click(core: &CoreRef, ui: &Ui, id: SampleId) {
    let mut c = core.borrow_mut();
    let current = c.engine.mode();
    let target_buffer = ui.buffer(current).get_untracked();
    let action = c.samples.load(current, id, &target_buffer);
    c.use_output_undo = None;
    let undo = action.undo_offered;
    if !panic_hook::storage_halted() {
        if action.switches_mode {
            let _ = c.storage.set_mode(storage_mode(action.target));
        }
        c.storage
            .on_text_edit(storage_mode(action.target), action.text);
        match action.target {
            ConversionMode::MarkdownToHtml => {
                let selection = c.samples.md_selection();
                let _ = c.storage.select_md_sample(selection);
            }
            ConversionMode::HtmlToMarkdown => {
                let selection = c.samples.html_selection();
                let _ = c.storage.select_html_sample(selection);
            }
        }
        sync_storage_banners(&c, ui);
    }
    apply_buffer_write(ui, action.target, action.text);
    let text = action.text.to_owned();
    let actions = c.engine.on_buffer_overwrite(action.target, &text);
    drop(c);
    ui.undo_visible.set(undo);
    run_actions(core, ui, actions);
}

/// FR-6 Reload Sample: never changes mode, tabs, options, or selection;
/// a non-text interaction, so storage flushes before writing.
fn on_reload_sample(core: &CoreRef, ui: &Ui) {
    let mut c = core.borrow_mut();
    let mode = c.engine.mode();
    let buffer = ui.buffer(mode).get_untracked();
    let action = c.samples.reload_sample(mode, &buffer);
    c.use_output_undo = None;
    let undo = action.undo_offered;
    if !panic_hook::storage_halted() {
        c.storage.on_text_edit(storage_mode(mode), action.text);
        let _ = c.storage.flush();
        sync_storage_banners(&c, ui);
    }
    apply_buffer_write(ui, mode, action.text);
    let text = action.text.to_owned();
    let actions = c.engine.on_buffer_overwrite(mode, &text);
    drop(c);
    ui.undo_visible.set(undo);
    run_actions(core, ui, actions);
}

/// FR-2 Explicit Action: copy the active mode's Source output into the
/// opposite buffer, switch, convert; undo restores the displaced buffer.
fn on_use_output(core: &CoreRef, ui: &Ui) {
    let mut c = core.borrow_mut();
    let mode = c.engine.mode();
    let output = ui.output(mode).get_untracked();
    if !output.valid || output.source.is_empty() {
        return;
    }
    let target = other_mode(mode);
    let target_buffer = ui.buffer(target).get_untracked();
    c.samples.dismiss_undo();
    c.use_output_undo = (!target_buffer.is_empty()).then_some((target, target_buffer));
    let undo = c.use_output_undo.is_some();
    if !panic_hook::storage_halted() {
        let _ = c.storage.set_mode(storage_mode(target));
        c.storage.on_text_edit(storage_mode(target), &output.source);
        sync_storage_banners(&c, ui);
    }
    apply_buffer_write(ui, target, &output.source);
    let text = output.source.clone();
    let actions = c.engine.on_buffer_overwrite(target, &text);
    drop(c);
    ui.undo_visible.set(undo);
    run_actions(core, ui, actions);
}

/// FR-2:62 / FR-6:141 Undo: restore the displaced buffer of the mode
/// the overwrite targeted (always the active mode — a mode switch
/// dismisses the toast).
fn on_undo(core: &CoreRef, ui: &Ui) {
    let mut c = core.borrow_mut();
    let restore = c
        .samples
        .undo()
        .map(|action| (action.mode, action.text))
        .or_else(|| c.use_output_undo.take());
    let Some((mode, text)) = restore else {
        ui.undo_visible.set(false);
        return;
    };
    ui.undo_visible.set(false);
    if !panic_hook::storage_halted() {
        c.storage.on_text_edit(storage_mode(mode), &text);
    }
    apply_buffer_write(ui, mode, &text);
    let actions = c.engine.on_buffer_overwrite(mode, &text);
    drop(c);
    run_actions(core, ui, actions);
}

// ---------- crash recovery (FR-8:160) ----------

/// "Convert restored input": run the active buffer now; the marker is
/// removed by [`conversion_succeeded`] once the run commits.
fn on_recovery_convert(core: &CoreRef, ui: &Ui) {
    let mut c = core.borrow_mut();
    ui.recovery.set(false);
    c.recovery_converting = true;
    if !panic_hook::storage_halted() {
        // Marker removal defers to `recovery_conversion_succeeded`; the
        // call still reports the choice to the controller.
        let _ = c
            .storage
            .on_recovery_choice(RecoveryChoice::ConvertRestoredInput);
    }
    let mode = c.engine.mode();
    let text = ui.buffer(mode).get_untracked();
    let actions = c.engine.on_buffer_overwrite(mode, &text);
    drop(c);
    run_actions(core, ui, actions);
}

/// "Clear buffers": the controller empties both buffers, removes the
/// marker, and saves; the engine mirrors are cleared for both modes
/// (inactive first, so the active mode ends up active again).
fn on_recovery_clear(core: &CoreRef, ui: &Ui) {
    let mut c = core.borrow_mut();
    ui.recovery.set(false);
    if !panic_hook::storage_halted() {
        let _ = c.storage.on_recovery_choice(RecoveryChoice::ClearBuffers);
        sync_storage_banners(&c, ui);
    }
    let active = c.engine.mode();
    apply_buffer_write(ui, other_mode(active), "");
    let mut actions = c.engine.on_buffer_overwrite(other_mode(active), "");
    apply_buffer_write(ui, active, "");
    let more = c.engine.on_buffer_overwrite(active, "");
    actions.extend(more);
    drop(c);
    run_actions(core, ui, actions);
}

// ---------- startup ----------

/// Seed every signal from the restored workspace (the controller is the
/// owner and already holds this state).
fn seed_from_workspace(c: &mut Core, ui: &Ui) {
    let mode = engine_mode(c.storage.workspace().mode);
    ui.mode.set(mode);
    ui.buf_md
        .set(c.storage.workspace().input_md_to_html.clone());
    ui.buf_html
        .set(c.storage.workspace().input_html_to_md.clone());
    ui.editor_text.set(ui.buffer(mode).get_untracked());
    ui.cmark.set(c.storage.workspace().cmark);
    ui.htmd.set(c.storage.workspace().htmd);
    c.skip_tags_raw = c.storage.workspace().skip_tags.clone();
    ui.skip_tags.set(c.storage.workspace().skip_tags.clone());
    ui.active_tab.set(c.storage.workspace().active_tab);
    ui.scroll(ConversionMode::MarkdownToHtml)
        .set(c.storage.workspace().scroll_md_to_html);
    ui.scroll(ConversionMode::HtmlToMarkdown)
        .set(c.storage.workspace().scroll_html_to_md);
    c.samples.restore(
        c.storage.workspace().sample_md_to_html,
        c.storage.workspace().sample_html_to_md,
    );
}

/// Root component: builds the clocks and [`Core`], replays storage
/// (FR-8 safe reads plus the crash-recovery flow), and renders the §5
/// layout.
#[component]
pub fn App() -> impl IntoView {
    let ui = Ui::new();
    let late = Rc::new(LateCore::default());

    // The clock callbacks re-enter Core through `late`, which is filled
    // below — after Core exists but before any engine input can arm a
    // timer.
    let engine_clock: Rc<crate::clock::WebClock<EngineEvent>> =
        Rc::new(crate::clock::WebClock::new({
            let late = late.clone();
            move |event| {
                if let Some(core) = late.core() {
                    let actions = core.borrow_mut().engine.on_event(event);
                    run_actions(&core, &ui, actions);
                }
            }
        }));
    let storage_clock: Rc<crate::clock::WebClock<StorageEvent>> =
        Rc::new(crate::clock::WebClock::new({
            let late = late.clone();
            move |event| {
                // FR-7: a panic halts all further workspace writes,
                // including a save debounce that was already pending.
                if panic_hook::storage_halted() {
                    return;
                }
                if let Some(core) = late.core() {
                    let mut c = core.borrow_mut();
                    let _ = c.storage.on_event(event);
                    sync_storage_banners(&c, &ui);
                }
            }
        }));

    #[cfg(target_arch = "wasm32")]
    let store: Box<dyn Store> = crate::storage::web::store_or_fallback();
    #[cfg(not(target_arch = "wasm32"))]
    let store: Box<dyn Store> = Box::new(crate::storage::MemoryStore::new());

    let mut bootstrap = Core {
        engine: Engine::new(engine_clock),
        banners: Banners::new(),
        samples: SampleState::new(),
        storage: StorageController::new(store, storage_clock),
        use_output_undo: None,
        skip_tags_raw: String::new(),
        recovery_converting: false,
    };
    let outcome = bootstrap.storage.load();
    let core_ref: CoreRef = SendWrapper::new(Rc::new(RefCell::new(bootstrap)));
    CORE.with(|cell| *cell.borrow_mut() = Some(core_ref.clone()));
    *late.0.borrow_mut() = Some(core_ref.clone());

    // FR-8 startup: a restored workspace converts the active mode's
    // buffer once; with a crash marker it restores without converting
    // and shows the recovery banner instead.
    {
        let mut c = core_ref.borrow_mut();
        match outcome {
            LoadOutcome::Fresh => {}
            LoadOutcome::Restored | LoadOutcome::CrashRecovery => {
                seed_from_workspace(&mut c, &ui);
                let mode = engine_mode(c.storage.workspace().mode);
                let (md, html) = (ui.buf_md.get_untracked(), ui.buf_html.get_untracked());
                let convert = outcome == LoadOutcome::Restored;
                if convert && ui.buffer(mode).get_untracked().len() > MAX_INPUT_BYTES {
                    enter_banner(&mut c, &ui, BannerKind::SizeLimit);
                }
                if outcome == LoadOutcome::CrashRecovery {
                    ui.recovery.set(true);
                }
                let actions = c.engine.on_restore(mode, &md, &html, convert);
                drop(c);
                run_actions(&core_ref, &ui, actions);
            }
        }
    }

    // FR-8 window listeners: multi-tab storage events and the guaranteed
    // pagehide/visibilitychange flush. Page-lifetime listeners; the
    // handles' drop does not remove them.
    {
        let core = core_ref.clone();
        let _ = window_event_listener(ev::storage, move |event: web_sys::StorageEvent| {
            let Some(key) = event.key() else {
                return;
            };
            let mut c = core.borrow_mut();
            let _ = c.storage.on_storage_event(&key);
            sync_storage_banners(&c, &ui);
        });
    }
    for name in ["pagehide", "visibilitychange"] {
        let core = core_ref.clone();
        let _ = window_event_listener_untyped(name, move |_event| {
            if panic_hook::storage_halted() {
                return;
            }
            let mut c = core.borrow_mut();
            let _ = c.storage.flush();
            sync_storage_banners(&c, &ui);
        });
    }
    // Restore the startup mode's scroll offset once the editor exists.
    request_animation_frame(move || restore_editor_later(&ui));

    // Derived state for the output pane.
    let active_output = Memo::new(move |_| match ui.mode.get() {
        ConversionMode::MarkdownToHtml => ui.out_md.get(),
        ConversionMode::HtmlToMarkdown => ui.out_html.get(),
    });
    let use_output_disabled = move || {
        let output = match ui.mode.get() {
            ConversionMode::MarkdownToHtml => ui.out_md.get(),
            ConversionMode::HtmlToMarkdown => ui.out_html.get(),
        };
        !output.valid || output.source.is_empty()
    };
    let fidelity_state = Memo::new(move |_| match ui.mode.get() {
        ConversionMode::MarkdownToHtml => FidelityView::Hidden,
        ConversionMode::HtmlToMarkdown => match ui.htmd.get().translation_mode {
            TranslationMode::Pure => FidelityView::Note,
            TranslationMode::Faithful => FidelityView::Panel(ui.fidelity.get()),
        },
    });

    #[cfg(feature = "demo-panic-test")]
    let panic_button = view! {
        <button id="panic-test-button" on:click=|_| panic!("demo-panic-test")>
            "Deliberate panic (demo-panic-test)"
        </button>
    };
    #[cfg(not(feature = "demo-panic-test"))]
    let panic_button = ().into_any();

    let banner_items = move || {
        let mut items: Vec<(&'static str, &'static str)> = Vec::new();
        if ui.size_banner.get() {
            items.push(("banner-size", BannerKind::SizeLimit.text()));
        }
        if ui.nesting_banner.get() {
            items.push(("banner-nesting", BannerKind::NestingLimit.text()));
        }
        if ui.cap_banner.get() {
            items.push(("banner-cap", OUTPUT_CAP_TEXT));
        }
        if ui.save_failed.get() {
            items.push(("banner-save-failed", SAVE_FAILED_TEXT));
        }
        if ui.external_write.get() {
            items.push(("banner-external-write", EXTERNAL_WRITE_TEXT));
        }
        items
    };

    view! {
        <header class="app-header">
            <h1>"htmd ↔ pulldown-cmark demo"</h1>
            <fieldset class="mode-group">
                <legend>"Conversion mode"</legend>
                <input
                    id="mode-md"
                    type="radio"
                    name="conversion-mode"
                    value="md-to-html"
                    prop:checked=move || ui.mode.get() == ConversionMode::MarkdownToHtml
                    on:change=move |_| switch_mode(&core(), &ui, ConversionMode::MarkdownToHtml)
                />
                <label for="mode-md">"Markdown → HTML"</label>
                <input
                    id="mode-html"
                    type="radio"
                    name="conversion-mode"
                    value="html-to-md"
                    prop:checked=move || ui.mode.get() == ConversionMode::HtmlToMarkdown
                    on:change=move |_| switch_mode(&core(), &ui, ConversionMode::HtmlToMarkdown)
                />
                <label for="mode-html">"HTML → Markdown"</label>
            </fieldset>
            {panic_button}
        </header>

        <main class="panes">
            <aside class="sidebar" aria-label="Conversion options">
                <button
                    id="reset-options"
                    on:click=move |_| on_reset(&core(), &ui)
                >
                    "Reset Options to Demo Preset"
                </button>
                <fieldset>
                    <legend>"pulldown-cmark options"</legend>
                    {cmark_checkboxes(ui)}
                </fieldset>
                <fieldset>
                    <legend>"htmd options"</legend>
                    {htmd_controls(ui)}
                    <div class="option-row">
                        <label for="skip-tags">"Skipped tags (comma-separated)"</label>
                    </div>
                    <input
                        id="skip-tags"
                        type="text"
                        maxlength="200"
                        prop:value=move || ui.skip_tags.get()
                        on:input=move |ev: web_sys::Event| on_skip_tags_input(&core(), &ui, ev)
                    />
                    <p class="hint">
                        "Split on commas, trimmed and lowercased; invalid names are ignored."
                    </p>
                </fieldset>
            </aside>

            <section class="pane" aria-label="Input">
                <h2>"Input"</h2>
                <textarea
                    id="editor"
                    rows="20"
                    spellcheck="false"
                    aria-label="Input editor"
                    prop:value=move || ui.editor_text.get()
                    on:input=move |ev: web_sys::Event| on_editor_input(&core(), &ui, ev)
                    on:scroll=move |ev: web_sys::Event| on_editor_scroll(&core(), &ui, ev)
                ></textarea>
                <div class="samples-row">
                    {SAMPLES.iter().map(|sample| {
                        let id = sample.id;
                        view! {
                            <button
                                id=format!("sample-{}", sample_id_slug(id))
                                on:click=move |_| on_sample_click(&core(), &ui, id)
                            >
                                {sample.label}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                    <button id="reload-sample" on:click=move |_| on_reload_sample(&core(), &ui)>
                        "Reload Sample"
                    </button>
                </div>
                <button
                    id="use-output"
                    disabled=use_output_disabled
                    on:click=move |_| on_use_output(&core(), &ui)
                >
                    "Use Output as Input"
                </button>
            </section>

            <section class="pane" aria-label="Output">
                <h2>"Output"</h2>
                <div role="tablist" aria-label="Output views" on:keydown=move |ev: web_sys::KeyboardEvent| on_tablist_keydown(&core(), &ui, ev)>
                    <button
                        role="tab"
                        id="tab-rendered"
                        aria-selected=move || if ui.active_tab.get() == ActiveTab::Rendered { "true" } else { "false" }
                        aria-controls="panel-rendered"
                        tabindex=move || if ui.active_tab.get() == ActiveTab::Rendered { "0" } else { "-1" }
                        on:click=move |_| select_tab(&core(), &ui, ActiveTab::Rendered)
                    >
                        "Rendered"
                    </button>
                    <button
                        role="tab"
                        id="tab-source"
                        aria-selected=move || if ui.active_tab.get() == ActiveTab::Source { "true" } else { "false" }
                        aria-controls="panel-source"
                        tabindex=move || if ui.active_tab.get() == ActiveTab::Source { "0" } else { "-1" }
                        on:click=move |_| select_tab(&core(), &ui, ActiveTab::Source)
                    >
                        "Source"
                    </button>
                </div>
                <div
                    role="tabpanel"
                    id="panel-rendered"
                    aria-labelledby="tab-rendered"
                    tabindex="0"
                    hidden=move || ui.active_tab.get() != ActiveTab::Rendered
                >
                    // FR-1: srcdoc is a DOM property (prop:), the frame
                    // is fully sandboxed, and it stays in the DOM while
                    // its panel is hidden (FR-2:55).
                    <iframe
                        id="preview-frame"
                        sandbox=""
                        title="Rendered preview"
                        referrerpolicy="no-referrer"
                        prop:srcdoc=move || active_output.get().preview
                        on:load=move |_| {
                            let core = core();
                            let mut c = core.borrow_mut();
                            let seq = c.engine.seq();
                            let actions = c.engine.on_iframe_load(seq);
                            drop(c);
                            run_actions(&core, &ui, actions);
                        }
                    ></iframe>
                </div>
                <div
                    role="tabpanel"
                    id="panel-source"
                    aria-labelledby="tab-source"
                    tabindex="0"
                    hidden=move || ui.active_tab.get() != ActiveTab::Source
                >
                    <pre class="source-view" id="source-view">
                        {move || active_output.get().source}
                    </pre>
                </div>
            </section>
        </main>

        {move || match fidelity_state.get() {
            FidelityView::Hidden => ().into_any(),
            FidelityView::Note => view! {
                <section class="fidelity-section" aria-label="Pipeline fidelity check">
                    <h2>"Pipeline Fidelity Check"</h2>
                    <p class="fidelity-note">{FIDELITY_NOTE}</p>
                </section>
            }.into_any(),
            FidelityView::Panel(panel) => view! {
                <section class="fidelity-section" aria-label="Pipeline fidelity check">
                    <h2>"Pipeline Fidelity Check"</h2>
                    <div class="fidelity-panel">
                        <div class="fidelity-col">
                            <h3>"Original (canonical)"</h3>
                            <pre class="canon">{panel.original.clone()}</pre>
                        </div>
                        <div class="fidelity-col">
                            <h3>"Round-tripped (canonical)"</h3>
                            <pre class="canon">{panel.roundtrip.clone()}</pre>
                        </div>
                        <div class="fidelity-col">
                            <h3>"Word diff"</h3>
                            <div class="tok-diff">
                                {move || match ui.fidelity.get().diff {
                                    FidelityDiff::Inclusive => view! {
                                        <p class="diff-ok">"No differences."</p>
                                    }.into_any(),
                                    FidelityDiff::TooLarge => view! {
                                        <p class="diff-cap">"Diff too large to display."</p>
                                    }.into_any(),
                                    FidelityDiff::Changes(tokens) => tokens
                                        .iter()
                                        .map(|token| match token {
                                            DiffToken::Equal(text) => view! {
                                                <span class="tok-eq">{text.clone()}</span>
                                            }.into_any(),
                                            DiffToken::Removed(text) => view! {
                                                <del class="tok-rem">{text.clone()}</del>
                                            }.into_any(),
                                            DiffToken::Inserted(text) => view! {
                                                <ins class="tok-ins">{text.clone()}</ins>
                                            }.into_any(),
                                        })
                                        .collect::<Vec<_>>()
                                        .into_any(),
                                }}
                            </div>
                        </div>
                    </div>
                </section>
            }.into_any(),
        }}

        // §5: the banner region is always in the accessibility tree and
        // only visually collapses when it has no children.
        <div id="banner-region" aria-live="assertive">
            {move || banner_items()
                .into_iter()
                .map(|(id, text)| view! { <p class="banner" id=id>{text}</p> }.into_any())
                .collect::<Vec<_>>()
                .into_any()}
            <Show when=move || ui.recovery.get() fallback=|| ()>
                {recovery_banner(ui)}
            </Show>
        </div>

        <div id="toast-region" aria-live="polite">
            <Show when=move || ui.undo_visible.get() fallback=|| ()>
                {undo_toast(ui)}
            </Show>
        </div>
    }
}

/// FR-8:160's recovery banner. Built as a function (not inline in the
/// `Show` children) so each reconstruction clones its own `core`
/// handle — `Show` re-runs its children, which would otherwise move the
/// captured value out of an `Fn` closure.
fn recovery_banner(ui: Ui) -> impl IntoView {
    view! {
        <p class="banner recovery" id="banner-recovery">
            {RECOVERY_TEXT}
            <button
                id="recovery-convert"
                on:click=move |_| on_recovery_convert(&core(), &ui)
            >
                "Convert restored input"
            </button>
            <button id="recovery-clear" on:click=move |_| on_recovery_clear(&core(), &ui)>
                "Clear buffers"
            </button>
        </p>
    }
}

/// FR-2:62's undo toast, with the same re-run-safe construction as
/// [`recovery_banner`].
fn undo_toast(ui: Ui) -> impl IntoView {
    view! {
        <p class="toast" id="undo-toast">
            {UNDO_TOAST_TEXT}
            <button id="undo-button" on:click=move |_| on_undo(&core(), &ui)>"Undo"</button>
            <button
                id="undo-dismiss"
                on:click=move |_| {
                    let core = core();
                    let mut c = core.borrow_mut();
                    dismiss_undo(&mut c, &ui);
                }
            >
                "Dismiss"
            </button>
        </p>
    }
}

fn sample_id_slug(id: SampleId) -> &'static str {
    match id {
        SampleId::Basics => "basics",
        SampleId::InlineHtml => "inline-html",
        SampleId::Math => "math",
    }
}

// ---------- sidebar controls (FR-4) ----------

/// One FR-4 pulldown-cmark flag: its sidebar label, DOM-id slug, and
/// field accessors.
type FlagGet = fn(&CmarkFlags) -> bool;
type FlagSet = fn(&mut CmarkFlags, bool);

const CMARK_FIELDS: [(&str, &str, FlagGet, FlagSet); 15] = [
    ("Tables", "tables", |f| f.tables, |f, v| f.tables = v),
    (
        "Footnotes",
        "footnotes",
        |f| f.footnotes,
        |f, v| f.footnotes = v,
    ),
    (
        "Strikethrough",
        "strikethrough",
        |f| f.strikethrough,
        |f, v| f.strikethrough = v,
    ),
    (
        "Task lists",
        "task-lists",
        |f| f.task_lists,
        |f, v| f.task_lists = v,
    ),
    (
        "Smart punctuation",
        "smart-punctuation",
        |f| f.smart_punctuation,
        |f, v| f.smart_punctuation = v,
    ),
    (
        "Heading attributes",
        "heading-attributes",
        |f| f.heading_attributes,
        |f, v| f.heading_attributes = v,
    ),
    ("Math", "math", |f| f.math, |f, v| f.math = v),
    (
        "YAML-style metadata blocks",
        "yaml-style-metadata-blocks",
        |f| f.yaml_style_metadata_blocks,
        |f, v| f.yaml_style_metadata_blocks = v,
    ),
    (
        "Pluses-delimited metadata blocks",
        "pluses-delimited-metadata-blocks",
        |f| f.pluses_delimited_metadata_blocks,
        |f, v| f.pluses_delimited_metadata_blocks = v,
    ),
    (
        "Old footnotes",
        "old-footnotes",
        |f| f.old_footnotes,
        |f, v| f.old_footnotes = v,
    ),
    (
        "GitHub Flavored Markdown (GFM)",
        "gfm",
        |f| f.gfm,
        |f, v| f.gfm = v,
    ),
    (
        "Definition lists",
        "definition-list",
        |f| f.definition_list,
        |f, v| f.definition_list = v,
    ),
    (
        "Superscript",
        "superscript",
        |f| f.superscript,
        |f, v| f.superscript = v,
    ),
    (
        "Subscript",
        "subscript",
        |f| f.subscript,
        |f, v| f.subscript = v,
    ),
    (
        "Wikilinks",
        "wikilinks",
        |f| f.wikilinks,
        |f, v| f.wikilinks = v,
    ),
];

/// The 15 FR-4 checkboxes.
fn cmark_checkboxes(ui: Ui) -> impl IntoView {
    CMARK_FIELDS
        .iter()
        .map(|&(label, slug, get, set)| {
            let id = format!("cmark-{slug}");
            view! {
                <div class="option-row">
                    <input
                        id=id.clone()
                        type="checkbox"
                        prop:checked=move || get(&ui.cmark.get())
                        on:change=move |ev: web_sys::Event| {
                            let checked = event_target_checked(&ev);
                            ui.cmark.update(|flags| set(flags, checked));
                            on_options_changed(&core(), &ui);
                        }
                    />
                    <label for=id.clone()>{label}</label>
                </div>
            }
        })
        .collect::<Vec<_>>()
}

/// Which htmd option a sidebar control edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HtmdField {
    HeadingStyle,
    HrStyle,
    BrStyle,
    LinkStyle,
    LinkReferenceStyle,
    CodeBlockStyle,
    CodeBlockFence,
    BulletListMarker,
    TranslationMode,
    UlBulletSpacing,
    OlNumberSpacing,
    PreformattedCode,
}

impl HtmdField {
    fn id_slug(self) -> &'static str {
        match self {
            HtmdField::HeadingStyle => "heading-style",
            HtmdField::HrStyle => "hr-style",
            HtmdField::BrStyle => "br-style",
            HtmdField::LinkStyle => "link-style",
            HtmdField::LinkReferenceStyle => "link-reference-style",
            HtmdField::CodeBlockStyle => "code-block-style",
            HtmdField::CodeBlockFence => "code-block-fence",
            HtmdField::BulletListMarker => "bullet-list-marker",
            HtmdField::TranslationMode => "translation-mode",
            HtmdField::UlBulletSpacing => "ul-bullet-spacing",
            HtmdField::OlNumberSpacing => "ol-number-spacing",
            HtmdField::PreformattedCode => "preformatted-code",
        }
    }

    fn label(self) -> &'static str {
        match self {
            HtmdField::HeadingStyle => "Heading style",
            HtmdField::HrStyle => "Horizontal rule style",
            HtmdField::BrStyle => "Line break style",
            HtmdField::LinkStyle => "Link style",
            HtmdField::LinkReferenceStyle => "Link reference style",
            HtmdField::CodeBlockStyle => "Code block style",
            HtmdField::CodeBlockFence => "Code block fence",
            HtmdField::BulletListMarker => "Bullet list marker",
            HtmdField::TranslationMode => "Translation mode",
            HtmdField::UlBulletSpacing => "Unordered list bullet spacing",
            HtmdField::OlNumberSpacing => "Ordered list number spacing",
            HtmdField::PreformattedCode => "Preformatted code",
        }
    }

    /// The select's option labels, in FR-4 order; enum variants'
    /// `Debug` names, which are also their persisted spellings.
    fn options(self) -> &'static [&'static str] {
        match self {
            HtmdField::HeadingStyle => &["Atx", "Setex"],
            HtmdField::HrStyle => &["Asterisks", "Dashes", "Underscores"],
            HtmdField::BrStyle => &["TwoSpaces", "Backslash"],
            HtmdField::LinkStyle => &["Inlined", "InlinedPreferAutolinks", "Referenced"],
            HtmdField::LinkReferenceStyle => &["Full", "Collapsed", "Shortcut"],
            HtmdField::CodeBlockStyle => &["Fenced", "Indented"],
            HtmdField::CodeBlockFence => &["Backticks", "Tildes"],
            HtmdField::BulletListMarker => &["Asterisk", "Dash"],
            HtmdField::TranslationMode => &["Pure", "Faithful"],
            HtmdField::UlBulletSpacing | HtmdField::OlNumberSpacing => &["1", "2", "3", "4"],
            HtmdField::PreformattedCode => &["false", "true"],
        }
    }
}

/// The field's current value as its control shows it — the enum
/// variants' names, which are also their persisted spellings.
fn htmd_field_value(opts: &HtmdOptions, field: HtmdField) -> String {
    match field {
        HtmdField::HeadingStyle => format!("{:?}", opts.heading_style),
        HtmdField::HrStyle => format!("{:?}", opts.hr_style),
        HtmdField::BrStyle => format!("{:?}", opts.br_style),
        HtmdField::LinkStyle => format!("{:?}", opts.link_style),
        HtmdField::LinkReferenceStyle => format!("{:?}", opts.link_reference_style),
        HtmdField::CodeBlockStyle => format!("{:?}", opts.code_block_style),
        HtmdField::CodeBlockFence => format!("{:?}", opts.code_block_fence),
        HtmdField::BulletListMarker => format!("{:?}", opts.bullet_list_marker),
        HtmdField::TranslationMode => format!("{:?}", opts.translation_mode),
        HtmdField::UlBulletSpacing => opts.ul_bullet_spacing.to_string(),
        HtmdField::OlNumberSpacing => opts.ol_number_spacing.to_string(),
        HtmdField::PreformattedCode => opts.preformatted_code.to_string(),
    }
}

/// Apply a control's value; invalid input (only possible from the free
/// number fields) is ignored, keeping the last valid option.
fn apply_htmd_field(opts: &mut HtmdOptions, field: HtmdField, value: &str) {
    use htmd::options as o;
    match field {
        HtmdField::HeadingStyle => {
            opts.heading_style = if value == "Setex" {
                o::HeadingStyle::Setex
            } else {
                o::HeadingStyle::Atx
            };
        }
        HtmdField::HrStyle => {
            opts.hr_style = match value {
                "Dashes" => o::HrStyle::Dashes,
                "Underscores" => o::HrStyle::Underscores,
                _ => o::HrStyle::Asterisks,
            };
        }
        HtmdField::BrStyle => {
            opts.br_style = if value == "Backslash" {
                o::BrStyle::Backslash
            } else {
                o::BrStyle::TwoSpaces
            };
        }
        HtmdField::LinkStyle => {
            opts.link_style = match value {
                "InlinedPreferAutolinks" => o::LinkStyle::InlinedPreferAutolinks,
                "Referenced" => o::LinkStyle::Referenced,
                _ => o::LinkStyle::Inlined,
            };
        }
        HtmdField::LinkReferenceStyle => {
            opts.link_reference_style = match value {
                "Collapsed" => o::LinkReferenceStyle::Collapsed,
                "Shortcut" => o::LinkReferenceStyle::Shortcut,
                _ => o::LinkReferenceStyle::Full,
            };
        }
        HtmdField::CodeBlockStyle => {
            opts.code_block_style = if value == "Indented" {
                o::CodeBlockStyle::Indented
            } else {
                o::CodeBlockStyle::Fenced
            };
        }
        HtmdField::CodeBlockFence => {
            opts.code_block_fence = if value == "Tildes" {
                o::CodeBlockFence::Tildes
            } else {
                o::CodeBlockFence::Backticks
            };
        }
        HtmdField::BulletListMarker => {
            opts.bullet_list_marker = if value == "Dash" {
                o::BulletListMarker::Dash
            } else {
                o::BulletListMarker::Asterisk
            };
        }
        HtmdField::TranslationMode => {
            opts.translation_mode = if value == "Faithful" {
                o::TranslationMode::Faithful
            } else {
                o::TranslationMode::Pure
            };
        }
        HtmdField::UlBulletSpacing => {
            if let Ok(n) = value.parse::<u8>()
                && (1..=4).contains(&n)
            {
                opts.ul_bullet_spacing = n;
            }
        }
        HtmdField::OlNumberSpacing => {
            if let Ok(n) = value.parse::<u8>()
                && (1..=4).contains(&n)
            {
                opts.ol_number_spacing = n;
            }
        }
        HtmdField::PreformattedCode => {
            opts.preformatted_code = value == "true";
        }
    }
}

/// The 12 FR-4 htmd controls in schema order: 9 selects, 2 spacing
/// numbers, 1 checkbox. `br_style` is disabled while Faithful is
/// active (FR-4:107 — the crate ignores it there).
fn htmd_controls(ui: Ui) -> impl IntoView {
    let enum_fields = [
        HtmdField::HeadingStyle,
        HtmdField::HrStyle,
        HtmdField::BrStyle,
        HtmdField::LinkStyle,
        HtmdField::LinkReferenceStyle,
        HtmdField::CodeBlockStyle,
        HtmdField::CodeBlockFence,
        HtmdField::BulletListMarker,
        HtmdField::TranslationMode,
    ];
    let mut views: Vec<AnyView> = enum_fields
        .iter()
        .map(|&field| htmd_select(ui, field).into_any())
        .collect();
    views.push(htmd_select(ui, HtmdField::UlBulletSpacing).into_any());
    views.push(htmd_select(ui, HtmdField::OlNumberSpacing).into_any());
    views.push(htmd_select(ui, HtmdField::PreformattedCode).into_any());
    views
}

/// One htmd option control. Spacings render as number inputs; enums and
/// the boolean render as selects (the boolean's two options read better
/// than an English toggle next to eleven selects).
fn htmd_select(ui: Ui, field: HtmdField) -> impl IntoView {
    let id = format!("htmd-{}", field.id_slug());
    let is_number = matches!(
        field,
        HtmdField::UlBulletSpacing | HtmdField::OlNumberSpacing
    );
    let disabled = move || {
        field == HtmdField::BrStyle && ui.htmd.get().translation_mode == TranslationMode::Faithful
    };
    let input = if is_number {
        view! {
            <input
                id=id.clone()
                type="number"
                min="1"
                max="4"
                step="1"
                prop:value=move || htmd_field_value(&ui.htmd.get(), field)
                on:change=move |ev: web_sys::Event| {
                    let value = event_target::<web_sys::HtmlInputElement>(&ev).value();
                    let mut opts = ui.htmd.get_untracked();
                    apply_htmd_field(&mut opts, field, &value);
                    // Set unconditionally so an ignored input resyncs
                    // the field to the still-current option.
                    ui.htmd.set(opts);
                    on_options_changed(&core(), &ui);
                }
            />
        }
        .into_any()
    } else {
        view! {
            <select
                id=id.clone()
                prop:value=move || htmd_field_value(&ui.htmd.get(), field)
                disabled=disabled
                on:change=move |ev: web_sys::Event| {
                    let value = event_target::<web_sys::HtmlSelectElement>(&ev).value();
                    let mut opts = ui.htmd.get_untracked();
                    apply_htmd_field(&mut opts, field, &value);
                    ui.htmd.set(opts);
                    on_options_changed(&core(), &ui);
                }
            >
                {field
                    .options()
                    .iter()
                    .map(|option| view! { <option value=*option>{*option}</option> })
                    .collect::<Vec<_>>()}
            </select>
        }
        .into_any()
    };
    view! {
        <div class="option-row">
            <label for=id.clone()>{field.label()}</label>
            {input}
        </div>
    }
}
