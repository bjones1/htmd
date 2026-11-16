//! FR-8 workspace persistence: the stored-state schema, safe-read
//! validation (every §8.6 wipe case), the [`Store`] abstraction over
//! `localStorage`, and the [`StorageController`] implementing the 500 ms
//! debounced text saves, flush-before-write on non-text UI interactions,
//! quota and multi-tab banner transitions, and crash-marker recovery.
//! Pure logic throughout: the `web_sys` localStorage adapter is
//! `wasm32`-only (see [`web`]); everything here runs natively under test.

// The UI task that wires the controller into the app is this module's
// only consumer; until then, `-D warnings` sees the items as dead code.
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::engine::{Clock, TimerToken};
use crate::options_model::{CmarkFlags, HtmdOptions, demo_preset};

/// FR-8: the whole workspace JSON lives under this single localStorage
/// key (unified writes: one atomic object).
pub const WORKSPACE_KEY: &str = "htmd_cmark_live_workspace";
/// FR-8/FR-7: the crash-recovery marker; its presence at startup
/// triggers the recovery flow instead of the initial conversion.
pub const CRASH_MARKER_KEY: &str = "htmd_cmark_live_crashed";
/// FR-8: the debounce window for text-entry saves.
const SAVE_DEBOUNCE_MS: u32 = 500;
/// FR-8: `skip_tags` persists at most this many characters; a longer
/// stored value is invalid (§8.6) and wipes the workspace.
const SKIP_TAGS_MAX_CHARS: usize = 200;
/// FR-8: the only schema version [`validate`] accepts.
const SCHEMA_VERSION: u32 = 1;

/// Why a [`Store::set`] failed. Reads never fail: [`Store::get`] maps
/// errors to `None` (FR-8 safe reads).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    /// The storage quota was exceeded (browser `QuotaExceededError`).
    QuotaExceeded,
    /// Storage is blocked or threw a security exception (Incognito).
    Blocked,
}

/// The persistence backend: `localStorage` on the web ([`web::WebStore`]),
/// a [`MemoryStore`] when storage is blocked, and fakes under test.
pub trait Store {
    /// The value under `key`, or `None` when absent or unreadable.
    fn get(&self, key: &str) -> Option<String>;
    /// Store `value` under `key`; errors never panic (FR-8 quota guard).
    fn set(&self, key: &str, value: &str) -> Result<(), StoreError>;
    /// Delete `key`; errors are ignored.
    fn remove(&self, key: &str);
}

/// In-memory [`Store`]: FR-8's documented fallback when `localStorage`
/// is blocked (the app keeps running and converting; saves just do not
/// survive a reload), and the store behind native tests.
#[derive(Debug, Default)]
pub struct MemoryStore {
    map: RefCell<HashMap<String, String>>,
}

impl MemoryStore {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// A store pre-seeded with one key (fixture setup).
    pub fn with(key: &str, value: &str) -> Self {
        let store = Self::new();
        store
            .map
            .borrow_mut()
            .insert(key.to_owned(), value.to_owned());
        store
    }
}

impl Store for MemoryStore {
    fn get(&self, key: &str) -> Option<String> {
        self.map.borrow().get(key).cloned()
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StoreError> {
        self.map
            .borrow_mut()
            .insert(key.to_owned(), value.to_owned());
        Ok(())
    }

    fn remove(&self, key: &str) {
        self.map.borrow_mut().remove(key);
    }
}

/// FR-8 `mode`: `"md-to-html"` or `"html-to-md"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    MdToHtml,
    HtmlToMd,
}

/// FR-8 `active_tab`: `"rendered"` or `"source"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActiveTab {
    Rendered,
    Source,
}

/// FR-8 `sample_md_to_html`: `"basics"` or `"math"`, or `null`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MdSample {
    Basics,
    Math,
}

/// FR-8 `sample_html_to_md`: `"basics"`, `"inline_html"`, or `"math"`,
/// or `null`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HtmlSample {
    Basics,
    InlineHtml,
    Math,
}

/// FR-8's persisted-state schema, exactly: one JSON blob under
/// [`WORKSPACE_KEY`]. Every listed field is required on read (missing or
/// mistyped fields invalidate the whole workspace per §8.6 -- no
/// per-field salvage); unknown fields are ignored (forward-compat).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    /// `1` (global); any other value is a future format and wipes.
    pub version: u32,
    pub mode: Mode,
    /// Per-mode input buffer.
    pub input_md_to_html: String,
    /// Per-mode input buffer.
    pub input_html_to_md: String,
    pub active_tab: ActiveTab,
    /// Input scroll offset per mode, number ≥ 0, default `0`.
    pub scroll_md_to_html: f64,
    /// Input scroll offset per mode, number ≥ 0, default `0`.
    pub scroll_html_to_md: f64,
    /// The 15 FR-4 `pulldown-cmark` booleans.
    pub cmark: CmarkFlags,
    /// The FR-4 htmd option object (enums, 1–4 spacings, `preformatted_code`).
    pub htmd: HtmdOptions,
    /// Raw `skip_tags` field text, ≤ 200 characters.
    pub skip_tags: String,
    /// Per-mode selected sample (FR-6), default `null`.
    pub sample_md_to_html: Option<MdSample>,
    /// Per-mode selected sample (FR-6), default `null`.
    pub sample_html_to_md: Option<HtmlSample>,
}

impl Default for Workspace {
    /// FR-8's initial workspace: Markdown → HTML mode on the Rendered
    /// tab, empty inputs, zero scroll offsets, the Demo Preset, and no
    /// sample selected or preloaded.
    fn default() -> Self {
        let (cmark, htmd) = demo_preset();
        Workspace {
            version: SCHEMA_VERSION,
            mode: Mode::MdToHtml,
            input_md_to_html: String::new(),
            input_html_to_md: String::new(),
            active_tab: ActiveTab::Rendered,
            scroll_md_to_html: 0.0,
            scroll_html_to_md: 0.0,
            cmark,
            htmd,
            skip_tags: String::new(),
            sample_md_to_html: None,
            sample_html_to_md: None,
        }
    }
}

/// A stored workspace was rejected; the caller wipes storage and starts
/// from defaults (FR-8's decision, executed by
/// [`StorageController::load`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Invalid;

/// Validate a parsed workspace JSON value against FR-8's schema and
/// §8.6's rules: `version == 1`, correct types and enum variants for all
/// fields, `skip_tags` ≤ 200 characters, `ul_bullet_spacing` and
/// `ol_number_spacing` within 1–4, and scroll offsets ≥ 0. Any violation
/// rejects the whole workspace; unknown fields are ignored.
pub fn validate(v: serde_json::Value) -> Result<Workspace, Invalid> {
    let workspace: Workspace = serde_json::from_value(v).map_err(|_| Invalid)?;
    if workspace.version != SCHEMA_VERSION
        || !(1..=4).contains(&workspace.htmd.ul_bullet_spacing)
        || !(1..=4).contains(&workspace.htmd.ol_number_spacing)
        || workspace.skip_tags.chars().count() > SKIP_TAGS_MAX_CHARS
        || workspace.scroll_md_to_html < 0.0
        || workspace.scroll_html_to_md < 0.0
    {
        return Err(Invalid);
    }
    Ok(workspace)
}

/// Non-fatal banner transitions the controller surfaces to the UI (the
/// UI renders them); each is announced once per transition into its
/// state, and the next successful write clears it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerSignal {
    /// FR-8 quota guard: a write failed. Text: *"Workspace could not be
    /// saved; recent changes will be lost on reload."*
    SaveFailed,
    /// FR-8 multi-tab: another tab wrote the workspace key. Text:
    /// *"This workspace was changed in another tab; saving here will
    /// overwrite it."*
    ExternalWrite,
}

/// FR-8 startup outcome of [`StorageController::load`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadOutcome {
    /// No usable workspace (absent, corrupt, or invalid -- wiped per
    /// §8.6): start from [`Workspace::default`].
    Fresh,
    /// A valid workspace was restored; FR-2 converts the active mode's
    /// buffer once at startup.
    Restored,
    /// The crash marker was present: buffers and options are restored
    /// but nothing converts; the UI renders FR-8's recovery banner and
    /// reports the user's [`RecoveryChoice`] back.
    CrashRecovery,
}

/// FR-8's crash-recovery banner actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryChoice {
    /// "Convert restored input": the UI converts; the marker is removed
    /// only once that conversion succeeds (see
    /// [`StorageController::recovery_conversion_succeeded`]).
    ConvertRestoredInput,
    /// "Clear buffers": empty both input buffers, keep outputs and diff
    /// cleared, remove the marker, and save the workspace.
    ClearBuffers,
}

/// Timer events re-entering the controller from the shared [`Clock`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageEvent {
    /// FR-8's 500 ms text-save debounce elapsed.
    SaveDue,
}

/// FR-8's unified-write controller: owns the workspace state, debounces
/// text saves, flushes before every non-text UI interaction, guards
/// quota and multi-tab transitions, and runs the crash-marker recovery
/// flow. Storage timers are this controller's own -- the conversion
/// engine ([`crate::engine`]) is never involved.
pub struct StorageController<S: Store> {
    store: S,
    clock: Rc<dyn Clock<StorageEvent>>,
    state: Workspace,
    save_timer: Option<TimerToken>,
    save_failed_banner: bool,
    external_write_banner: bool,
}

impl<S: Store> StorageController<S> {
    /// A controller over `store` starting from [`Workspace::default`];
    /// call [`Self::load`] to restore a saved workspace.
    pub fn new(store: S, clock: Rc<dyn Clock<StorageEvent>>) -> Self {
        StorageController {
            store,
            clock,
            state: Workspace::default(),
            save_timer: None,
            save_failed_banner: false,
            external_write_banner: false,
        }
    }

    /// The current workspace state (always in memory, even when writes
    /// fail).
    pub fn workspace(&self) -> &Workspace {
        &self.state
    }

    /// FR-8 safe reads: parse and validate the stored workspace.
    /// Absent, corrupt, or invalid storage (§8.6: wrong version, bad
    /// types/enums, spacing outside 1–4, `skip_tags` over 200 chars) is
    /// wiped and replaced by defaults; a present crash marker with a
    /// valid workspace restores without converting
    /// ([`LoadOutcome::CrashRecovery`]) and keeps the marker until the
    /// user chooses a [`RecoveryChoice`].
    pub fn load(&mut self) -> LoadOutcome {
        let crashed = self.store.get(CRASH_MARKER_KEY).is_some();
        let Some(raw) = self.store.get(WORKSPACE_KEY) else {
            // Nothing persisted: FR-8's initial workspace. A lone marker
            // has nothing to recover, so it goes too.
            self.store.remove(CRASH_MARKER_KEY);
            self.state = Workspace::default();
            return LoadOutcome::Fresh;
        };
        let parsed = serde_json::from_str(&raw)
            .map_err(|_| Invalid)
            .and_then(validate);
        match parsed {
            Ok(workspace) => {
                self.state = workspace;
                if crashed {
                    LoadOutcome::CrashRecovery
                } else {
                    LoadOutcome::Restored
                }
            }
            Err(Invalid) => {
                // FR-8: wipe storage and reset to the Demo Preset with
                // empty inputs. The recovery flow restores "the validated
                // workspace"; an invalid one leaves nothing to recover,
                // so the marker is cleared with it.
                self.store.remove(WORKSPACE_KEY);
                self.store.remove(CRASH_MARKER_KEY);
                self.state = Workspace::default();
                LoadOutcome::Fresh
            }
        }
    }

    /// FR-8 debounced text save: record the latest buffer text and
    /// (re)arm the 500 ms save timer; each edit restarts the window. No
    /// synchronous write happens here.
    pub fn on_text_edit(&mut self, mode: Mode, text: &str) {
        match mode {
            Mode::MdToHtml => self.state.input_md_to_html = text.to_owned(),
            Mode::HtmlToMd => self.state.input_html_to_md = text.to_owned(),
        }
        self.arm_save_timer();
    }

    /// Persist an input scroll offset (FR-8: number ≥ 0). Scroll changes
    /// ride the same 500 ms debounce as text -- a synchronous write per
    /// scroll tick would storm localStorage.
    pub fn on_scroll(&mut self, mode: Mode, offset: f64) {
        if !offset.is_finite() || offset < 0.0 {
            return;
        }
        match mode {
            Mode::MdToHtml => self.state.scroll_md_to_html = offset,
            Mode::HtmlToMd => self.state.scroll_html_to_md = offset,
        }
        self.arm_save_timer();
    }

    /// FR-8 flush-before-write (mode toggle): merge the mode change with
    /// the latest buffers, cancel the pending text timer, write now.
    pub fn set_mode(&mut self, mode: Mode) -> Option<BannerSignal> {
        self.state.mode = mode;
        self.flush()
    }

    /// FR-8 flush-before-write (tab change).
    pub fn set_active_tab(&mut self, tab: ActiveTab) -> Option<BannerSignal> {
        self.state.active_tab = tab;
        self.flush()
    }

    /// FR-8 flush-before-write (option change): the full sidebar
    /// configuration in one synchronous write. `skip_tags` is capped at
    /// 200 characters (FR-4) so a stored workspace can never be invalid
    /// on the next load.
    pub fn set_options(
        &mut self,
        cmark: CmarkFlags,
        htmd: HtmdOptions,
        skip_tags: &str,
    ) -> Option<BannerSignal> {
        self.state.cmark = cmark;
        self.state.htmd = htmd;
        self.state.skip_tags = skip_tags.chars().take(SKIP_TAGS_MAX_CHARS).collect();
        self.flush()
    }

    /// FR-4/FR-8 flush-before-write (Reset Options to Demo Preset):
    /// restores only the sidebar configuration; text, mode, tabs,
    /// scrolls, and samples are untouched.
    pub fn reset_options(&mut self) -> Option<BannerSignal> {
        (self.state.cmark, self.state.htmd) = demo_preset();
        self.state.skip_tags.clear();
        self.flush()
    }

    /// FR-6/FR-8 flush-before-write (sample load): record the Markdown →
    /// HTML mode's selected sample (`None` = none selected).
    pub fn select_md_sample(&mut self, sample: Option<MdSample>) -> Option<BannerSignal> {
        self.state.sample_md_to_html = sample;
        self.flush()
    }

    /// FR-6/FR-8 flush-before-write (sample load): record the HTML →
    /// Markdown mode's selected sample (`None` = none selected).
    pub fn select_html_sample(&mut self, sample: Option<HtmlSample>) -> Option<BannerSignal> {
        self.state.sample_html_to_md = sample;
        self.flush()
    }

    /// FR-8 guaranteed flush (`pagehide`/`visibilitychange`) and the
    /// shared tail of every flush-before-write interaction: cancel the
    /// pending text timer and write the whole state synchronously.
    pub fn flush(&mut self) -> Option<BannerSignal> {
        if let Some(token) = self.save_timer.take() {
            self.clock.cancel(token);
        }
        self.write_now()
    }

    /// A [`Clock`] timer fired. A due (or stale -- writing current state
    /// is idempotent) `SaveDue` writes the whole workspace now.
    pub fn on_event(&mut self, event: StorageEvent) -> Option<BannerSignal> {
        match event {
            StorageEvent::SaveDue => {
                self.save_timer = None;
                self.write_now()
            }
        }
    }

    /// FR-8 multi-tab: the browser fired a `storage` event. For the
    /// workspace key this tab keeps its own state and announces
    /// [`BannerSignal::ExternalWrite`] once per transition into the
    /// externally-modified state; further external writes while the
    /// banner shows do not re-announce. This tab's next successful write
    /// clears it (last-writer-wins). Other keys are ignored.
    pub fn on_storage_event(&mut self, key: &str) -> Option<BannerSignal> {
        if key != WORKSPACE_KEY || self.external_write_banner {
            return None;
        }
        self.external_write_banner = true;
        Some(BannerSignal::ExternalWrite)
    }

    /// FR-8 crash recovery: apply the banner choice. `ClearBuffers`
    /// empties both input buffers, removes the marker, and saves now;
    /// `ConvertRestoredInput` defers marker removal to
    /// [`Self::recovery_conversion_succeeded`] ("once conversion
    /// succeeds").
    pub fn on_recovery_choice(&mut self, choice: RecoveryChoice) -> Option<BannerSignal> {
        match choice {
            RecoveryChoice::ConvertRestoredInput => None,
            RecoveryChoice::ClearBuffers => {
                self.state.input_md_to_html.clear();
                self.state.input_html_to_md.clear();
                self.store.remove(CRASH_MARKER_KEY);
                self.flush()
            }
        }
    }

    /// FR-8: the recovery conversion committed successfully; only now is
    /// the crash marker removed.
    pub fn recovery_conversion_succeeded(&mut self) {
        self.store.remove(CRASH_MARKER_KEY);
    }

    /// (Re)start the 500 ms save window, cancelling any pending one.
    fn arm_save_timer(&mut self) {
        if let Some(token) = self.save_timer.take() {
            self.clock.cancel(token);
        }
        self.save_timer = Some(self.clock.schedule(SAVE_DEBOUNCE_MS, StorageEvent::SaveDue));
    }

    /// FR-8 unified write: the whole workspace as one JSON object. A
    /// failure announces [`BannerSignal::SaveFailed`] once per
    /// transition into failure; a success clears both banners (quota and
    /// external-write).
    fn write_now(&mut self) -> Option<BannerSignal> {
        let Ok(json) = serde_json::to_string(&self.state) else {
            return None;
        };
        match self.store.set(WORKSPACE_KEY, &json) {
            Ok(()) => {
                self.save_failed_banner = false;
                self.external_write_banner = false;
                None
            }
            Err(_) if self.save_failed_banner => None,
            Err(_) => {
                self.save_failed_banner = true;
                Some(BannerSignal::SaveFailed)
            }
        }
    }
}

/// The `wasm32`-only `localStorage` adapter: thin, raw `web_sys` calls,
/// untestable natively by design. FR-8 store selection on the web:
/// `localStorage` when accessible, [`MemoryStore`] when blocked.
#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::{MemoryStore, Store, StoreError};

    /// `localStorage`-backed [`Store`]; every call is wrapped so a
    /// throwing or blocked storage surfaces as a [`StoreError`] or
    /// `None` instead of a panic (FR-8 safe reads).
    pub struct WebStore(web_sys::Storage);

    impl Store for WebStore {
        fn get(&self, key: &str) -> Option<String> {
            self.0.get_item(key).ok().flatten()
        }

        fn set(&self, key: &str, value: &str) -> Result<(), StoreError> {
            self.0.set_item(key, value).map_err(|err| {
                let name = js_sys::Error::from(err).name();
                if name == "QuotaExceededError" {
                    StoreError::QuotaExceeded
                } else {
                    StoreError::Blocked
                }
            })
        }

        fn remove(&self, key: &str) {
            let _ = self.0.remove_item(key);
        }
    }

    /// FR-8 blocked-storage fallback: pick the web store when
    /// `localStorage` is accessible, else run entirely on a
    /// [`MemoryStore`] so conversions keep working.
    pub fn store_or_fallback() -> Box<dyn Store> {
        match web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
            Some(storage) => Box::new(WebStore(storage)),
            None => Box::new(MemoryStore::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;

    /// Manual-time [`Clock`] for the storage debounce (same pattern as
    /// the engine's): `advance` moves time and yields due events in
    /// (deadline, insertion) order; cancelled timers vanish.
    struct FakeClock {
        now: Cell<u64>,
        next_id: Cell<u64>,
        timers: RefCell<BTreeMap<(u64, u64), (TimerToken, StorageEvent)>>,
    }

    impl FakeClock {
        fn new() -> Self {
            FakeClock {
                now: Cell::new(0),
                next_id: Cell::new(0),
                timers: RefCell::new(BTreeMap::new()),
            }
        }

        fn advance(&self, ms: u64) -> Vec<StorageEvent> {
            self.now.set(self.now.get() + ms);
            let mut due = Vec::new();
            while let Some(key) = {
                let timers = self.timers.borrow();
                timers.keys().find(|(at, _)| *at <= self.now.get()).copied()
            } {
                if let Some((_, event)) = self.timers.borrow_mut().remove(&key) {
                    due.push(event);
                }
            }
            due
        }

        fn live_timers(&self) -> usize {
            self.timers.borrow().len()
        }
    }

    impl Clock<StorageEvent> for FakeClock {
        fn now_ms(&self) -> u64 {
            self.now.get()
        }

        fn schedule(&self, delay_ms: u32, event: StorageEvent) -> TimerToken {
            self.next_id.set(self.next_id.get() + 1);
            let id = self.next_id.get();
            let token = TimerToken(id);
            self.timers
                .borrow_mut()
                .insert((self.now.get() + u64::from(delay_ms), id), (token, event));
            token
        }

        fn cancel(&self, token: TimerToken) {
            self.timers
                .borrow_mut()
                .retain(|_, (pending, _)| *pending != token);
        }
    }

    /// A [`MemoryStore`] with scriptable failures and a write journal
    /// (the `(key, value)` pair of every successful `set`).
    struct ScriptedStore {
        inner: MemoryStore,
        fail_with: Cell<Option<StoreError>>,
        writes: RefCell<Vec<(String, String)>>,
    }

    impl ScriptedStore {
        fn shared() -> Rc<Self> {
            Rc::new(ScriptedStore {
                inner: MemoryStore::new(),
                fail_with: Cell::new(None),
                writes: RefCell::new(Vec::new()),
            })
        }

        fn seed(&self, key: &str, value: &str) {
            Store::set(&self.inner, key, value).expect("seeding never fails");
        }

        fn fail_with(&self, error: StoreError) {
            self.fail_with.set(Some(error));
        }

        fn heal(&self) {
            self.fail_with.set(None);
        }

        fn workspace_writes(&self) -> Vec<String> {
            self.writes
                .borrow()
                .iter()
                .filter(|(key, _)| key == WORKSPACE_KEY)
                .map(|(_, value)| value.clone())
                .collect()
        }

        fn workspace_present(&self) -> bool {
            self.inner.get(WORKSPACE_KEY).is_some()
        }

        fn marker_present(&self) -> bool {
            self.inner.get(CRASH_MARKER_KEY).is_some()
        }

        fn stored_workspace(&self) -> Workspace {
            serde_json::from_str(
                &self
                    .inner
                    .get(WORKSPACE_KEY)
                    .expect("workspace key is stored"),
            )
            .expect("stored workspace JSON is valid")
        }
    }

    impl Store for Rc<ScriptedStore> {
        fn get(&self, key: &str) -> Option<String> {
            self.inner.get(key)
        }

        fn set(&self, key: &str, value: &str) -> Result<(), StoreError> {
            if let Some(error) = self.fail_with.get() {
                return Err(error);
            }
            self.writes
                .borrow_mut()
                .push((key.to_owned(), value.to_owned()));
            self.inner.set(key, value)
        }

        fn remove(&self, key: &str) {
            self.inner.remove(key);
        }
    }

    struct Harness {
        clock: Rc<FakeClock>,
        store: Rc<ScriptedStore>,
        controller: StorageController<Rc<ScriptedStore>>,
    }

    impl Harness {
        fn new() -> Self {
            let clock = Rc::new(FakeClock::new());
            let store = ScriptedStore::shared();
            Harness {
                controller: StorageController::new(store.clone(), clock.clone()),
                clock,
                store,
            }
        }

        /// Move time forward, feeding due timer events into the
        /// controller and collecting the resulting banner signals.
        fn advance(&mut self, ms: u64) -> Vec<Option<BannerSignal>> {
            self.clock
                .advance(ms)
                .into_iter()
                .map(|event| self.controller.on_event(event))
                .collect()
        }
    }

    /// A workspace differing from the default in every field, so
    /// roundtrip tests prove each field survives storage.
    fn distinct_workspace() -> Workspace {
        use htmd::options::{HeadingStyle, TranslationMode};
        let (cmark, htmd) = demo_preset();
        Workspace {
            mode: Mode::HtmlToMd,
            input_md_to_html: "# md".to_owned(),
            input_html_to_md: "<p>html</p>".to_owned(),
            active_tab: ActiveTab::Source,
            scroll_md_to_html: 12.5,
            scroll_html_to_md: 3.0,
            cmark: CmarkFlags {
                footnotes: true,
                ..cmark
            },
            htmd: HtmdOptions {
                heading_style: HeadingStyle::Setex,
                ul_bullet_spacing: 1,
                ol_number_spacing: 4,
                translation_mode: TranslationMode::Faithful,
                ..htmd
            },
            skip_tags: "img, span".to_owned(),
            sample_md_to_html: Some(MdSample::Math),
            sample_html_to_md: Some(HtmlSample::InlineHtml),
            ..Workspace::default()
        }
    }

    /// `base` with field `key` replaced by `value` (or removed).
    fn mutated(base: &Workspace, key: &str, value: serde_json::Value) -> serde_json::Value {
        let mut v = serde_json::to_value(base).expect("workspace serializes");
        let map = v.as_object_mut().expect("workspace is a JSON object");
        if value.is_null() && key.starts_with('-') {
            map.remove(&key[1..]);
        } else {
            map.insert(key.to_owned(), value);
        }
        v
    }

    // ---------- validate: §8.6 read-path cases ----------

    #[test]
    fn default_workspace_serializes_the_fr8_schema_shape() {
        let v = serde_json::to_value(Workspace::default()).expect("serializes");
        let keys: BTreeSet<String> = v.as_object().expect("object").keys().cloned().collect();
        assert_eq!(
            keys,
            [
                "version",
                "mode",
                "input_md_to_html",
                "input_html_to_md",
                "active_tab",
                "scroll_md_to_html",
                "scroll_html_to_md",
                "cmark",
                "htmd",
                "skip_tags",
                "sample_md_to_html",
                "sample_html_to_md",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect()
        );
        assert_eq!(v["version"], serde_json::json!(1));
        assert_eq!(v["mode"], serde_json::json!("md-to-html"));
        assert_eq!(v["active_tab"], serde_json::json!("rendered"));
        assert!(v["sample_md_to_html"].is_null());
        assert!(v["sample_html_to_md"].is_null());
        let cmark_keys: BTreeSet<String> = v["cmark"]
            .as_object()
            .expect("cmark object")
            .keys()
            .cloned()
            .collect();
        assert_eq!(cmark_keys.len(), 15);
        let htmd_keys: BTreeSet<String> = v["htmd"]
            .as_object()
            .expect("htmd object")
            .keys()
            .cloned()
            .collect();
        assert_eq!(htmd_keys.len(), 12);
    }

    #[test]
    fn validate_roundtrips_a_distinct_workspace() {
        let v = serde_json::to_value(distinct_workspace()).expect("serializes");
        assert_eq!(validate(v), Ok(distinct_workspace()));
    }

    #[test]
    fn validate_ignores_unknown_fields() {
        let mut v = serde_json::to_value(distinct_workspace()).expect("serializes");
        v.as_object_mut()
            .expect("object")
            .insert("future_field".to_owned(), serde_json::json!({"x": [1, 2]}));
        assert_eq!(validate(v), Ok(distinct_workspace()));
    }

    #[test]
    fn validate_rejects_version_two() {
        let v = mutated(&distinct_workspace(), "version", serde_json::json!(2));
        assert_eq!(validate(v), Err(Invalid));
    }

    #[test]
    fn validate_rejects_wrong_field_types() {
        for (key, value) in [
            ("version", serde_json::json!("1")),
            ("mode", serde_json::json!(7)),
            ("input_md_to_html", serde_json::json!(42)),
            ("active_tab", serde_json::json!("Rendered")),
            ("scroll_md_to_html", serde_json::json!("top")),
            ("cmark", serde_json::json!(true)),
            ("skip_tags", serde_json::json!(null)),
            ("sample_html_to_md", serde_json::json!(3)),
        ] {
            assert_eq!(
                validate(mutated(&distinct_workspace(), key, value)),
                Err(Invalid),
                "field `{key}` with wrong type must invalidate"
            );
        }
    }

    #[test]
    fn validate_rejects_wrong_types_inside_nested_objects() {
        let mut base = serde_json::to_value(distinct_workspace()).expect("serializes");
        base["cmark"]["tables"] = serde_json::json!("yes");
        assert_eq!(validate(base.clone()), Err(Invalid));
        base["htmd"]["ul_bullet_spacing"] = serde_json::json!("3");
        assert_eq!(validate(base.clone()), Err(Invalid));
        base["htmd"]["translation_mode"] = serde_json::json!(1);
        assert_eq!(validate(base.clone()), Err(Invalid));
        base["scroll_html_to_md"] = serde_json::json!(-0.5);
        assert_eq!(validate(base), Err(Invalid));
    }

    #[test]
    fn validate_rejects_missing_fields() {
        for key in [
            "version",
            "mode",
            "input_html_to_md",
            "cmark",
            "htmd",
            "skip_tags",
        ] {
            assert_eq!(
                validate(mutated(
                    &distinct_workspace(),
                    &format!("-{key}"),
                    serde_json::Value::Null
                )),
                Err(Invalid),
                "missing field `{key}` must invalidate"
            );
        }
        // Ruling: the nullable sample fields are the schema's only fields
        // with a stated default (`null`), so their absence reads as
        // "no sample selected" rather than invalidating the workspace.
        let v = mutated(
            &distinct_workspace(),
            "-sample_md_to_html",
            serde_json::Value::Null,
        );
        let mut expected = distinct_workspace();
        expected.sample_md_to_html = None;
        assert_eq!(validate(v), Ok(expected));
    }

    #[test]
    fn validate_rejects_invalid_enum_variants() {
        for (key, value) in [
            ("mode", serde_json::json!("md-to-md")),
            ("active_tab", serde_json::json!("preview")),
            ("sample_md_to_html", serde_json::json!("inline_html")),
            ("sample_html_to_md", serde_json::json!("basics2")),
        ] {
            assert_eq!(
                validate(mutated(&distinct_workspace(), key, value)),
                Err(Invalid),
                "enum field `{key}` with invalid variant must invalidate"
            );
        }
        // Each sample slot accepts its own valid variants; `inline_html`
        // is valid only for the HTML → Markdown slot (checked above).
        let mut v = serde_json::to_value(distinct_workspace()).expect("serializes");
        v["sample_md_to_html"] = serde_json::json!("basics");
        v["sample_html_to_md"] = serde_json::json!("math");
        assert!(validate(v).is_ok());
        let mut v = serde_json::to_value(distinct_workspace()).expect("serializes");
        v["sample_md_to_html"] = serde_json::Value::Null;
        assert!(validate(v).is_ok());
    }

    #[test]
    fn validate_rejects_spacing_outside_one_to_four() {
        for field in ["ul_bullet_spacing", "ol_number_spacing"] {
            for bad in [
                serde_json::json!(0),
                serde_json::json!(5),
                serde_json::json!(256),
            ] {
                let mut v = serde_json::to_value(distinct_workspace()).expect("serializes");
                v["htmd"][field] = bad.clone();
                assert_eq!(
                    validate(v),
                    Err(Invalid),
                    "{field} = {bad} must invalidate (§8.6 out-of-bound)"
                );
            }
            for good in [1u8, 4] {
                let mut v = serde_json::to_value(distinct_workspace()).expect("serializes");
                v["htmd"][field] = serde_json::json!(good);
                assert!(validate(v).is_ok(), "{field} = {good} is valid");
            }
        }
    }

    #[test]
    fn validate_rejects_skip_tags_over_200_chars() {
        let v = mutated(
            &distinct_workspace(),
            "skip_tags",
            serde_json::json!("a".repeat(201)),
        );
        assert_eq!(validate(v), Err(Invalid));
        let v = mutated(
            &distinct_workspace(),
            "skip_tags",
            serde_json::json!("a".repeat(200)),
        );
        assert!(validate(v).is_ok());
    }

    // ---------- MemoryStore ----------

    #[test]
    fn memory_store_roundtrips_entries() {
        let store = MemoryStore::with(WORKSPACE_KEY, "{}");
        assert_eq!(store.get(WORKSPACE_KEY), Some("{}".to_owned()));
        assert_eq!(store.get("absent"), None);
        Store::set(&store, WORKSPACE_KEY, "{\"v\":1}").expect("memory set never fails");
        assert_eq!(store.get(WORKSPACE_KEY), Some("{\"v\":1}".to_owned()));
        store.remove(WORKSPACE_KEY);
        assert_eq!(store.get(WORKSPACE_KEY), None);
    }

    // ---------- load: §8.6 storage scenarios end-to-end ----------

    #[test]
    fn load_without_workspace_starts_fresh_defaults() {
        let mut h = Harness::new();
        assert_eq!(h.controller.load(), LoadOutcome::Fresh);
        assert_eq!(*h.controller.workspace(), Workspace::default());
    }

    #[test]
    fn load_restores_valid_workspace() {
        let mut h = Harness::new();
        let saved = distinct_workspace();
        h.store.seed(
            WORKSPACE_KEY,
            &serde_json::to_string(&saved).expect("serializes"),
        );
        assert_eq!(h.controller.load(), LoadOutcome::Restored);
        assert_eq!(*h.controller.workspace(), saved);
    }

    #[test]
    fn load_wipes_corrupt_json_to_defaults() {
        let mut h = Harness::new();
        h.store.seed(WORKSPACE_KEY, "{not json at all");
        assert_eq!(h.controller.load(), LoadOutcome::Fresh);
        assert_eq!(*h.controller.workspace(), Workspace::default());
        assert!(
            !h.store.workspace_present(),
            "§8.6: corrupt payload is wiped"
        );
    }

    #[test]
    fn load_wipes_future_version() {
        let mut h = Harness::new();
        h.store.seed(
            WORKSPACE_KEY,
            &serde_json::to_string(&mutated(
                &distinct_workspace(),
                "version",
                serde_json::json!(2),
            ))
            .expect("serializes"),
        );
        assert_eq!(h.controller.load(), LoadOutcome::Fresh);
        assert_eq!(*h.controller.workspace(), Workspace::default());
        assert!(!h.store.workspace_present());
    }

    #[test]
    fn load_wipes_wrong_field_types_and_bad_enums() {
        for (key, value) in [
            ("input_html_to_md", serde_json::json!(false)),
            ("mode", serde_json::json!("md-to-md")),
            ("htmd", serde_json::json!("Fenced")),
        ] {
            let mut h = Harness::new();
            h.store.seed(
                WORKSPACE_KEY,
                &serde_json::to_string(&mutated(&distinct_workspace(), key, value))
                    .expect("serializes"),
            );
            assert_eq!(h.controller.load(), LoadOutcome::Fresh, "{key}");
            assert!(!h.store.workspace_present(), "{key}");
            assert_eq!(*h.controller.workspace(), Workspace::default(), "{key}");
        }
    }

    #[test]
    fn load_wipes_invalid_spacing_values() {
        for field in ["ul_bullet_spacing", "ol_number_spacing"] {
            for bad in [serde_json::json!(0), serde_json::json!(5)] {
                let mut h = Harness::new();
                let mut json = serde_json::to_value(distinct_workspace()).expect("serializes");
                json["htmd"][field] = bad.clone();
                h.store.seed(WORKSPACE_KEY, &json.to_string());
                assert_eq!(h.controller.load(), LoadOutcome::Fresh, "{field} = {bad}");
                assert!(!h.store.workspace_present());
            }
        }
    }

    #[test]
    fn load_wipes_oversized_skip_tags() {
        let mut h = Harness::new();
        h.store.seed(
            WORKSPACE_KEY,
            &serde_json::to_string(&mutated(
                &distinct_workspace(),
                "skip_tags",
                serde_json::json!("a".repeat(201)),
            ))
            .expect("serializes"),
        );
        assert_eq!(h.controller.load(), LoadOutcome::Fresh);
        assert!(!h.store.workspace_present());
    }

    // ---------- crash-marker recovery (FR-8 / §8.6) ----------

    #[test]
    fn load_with_marker_restores_without_converting_and_keeps_marker() {
        let mut h = Harness::new();
        let saved = distinct_workspace();
        h.store.seed(
            WORKSPACE_KEY,
            &serde_json::to_string(&saved).expect("serializes"),
        );
        h.store.seed(CRASH_MARKER_KEY, "1");
        assert_eq!(h.controller.load(), LoadOutcome::CrashRecovery);
        assert_eq!(*h.controller.workspace(), saved, "buffers restored");
        assert!(
            h.store.marker_present(),
            "FR-8: marker is kept until the user acts"
        );
    }

    #[test]
    fn load_with_marker_and_unusable_workspace_wipes_both() {
        // Ruling: the recovery flow restores "the validated workspace";
        // when validation fails there is nothing to recover, so the
        // marker is cleared with the wiped workspace.
        let mut h = Harness::new();
        h.store.seed(WORKSPACE_KEY, "{corrupt");
        h.store.seed(CRASH_MARKER_KEY, "1");
        assert_eq!(h.controller.load(), LoadOutcome::Fresh);
        assert!(!h.store.workspace_present());
        assert!(!h.store.marker_present());
    }

    #[test]
    fn load_with_marker_but_no_workspace_clears_marker() {
        let mut h = Harness::new();
        h.store.seed(CRASH_MARKER_KEY, "1");
        assert_eq!(h.controller.load(), LoadOutcome::Fresh);
        assert_eq!(*h.controller.workspace(), Workspace::default());
        assert!(!h.store.marker_present());
    }

    #[test]
    fn recovery_clear_buffers_empties_inputs_removes_marker_saves() {
        let mut h = Harness::new();
        let saved = distinct_workspace();
        h.store.seed(
            WORKSPACE_KEY,
            &serde_json::to_string(&saved).expect("serializes"),
        );
        h.store.seed(CRASH_MARKER_KEY, "1");
        h.controller.load();
        h.controller
            .on_recovery_choice(RecoveryChoice::ClearBuffers);
        assert!(!h.store.marker_present());
        let stored = h.store.stored_workspace();
        assert_eq!(stored.input_md_to_html, "");
        assert_eq!(stored.input_html_to_md, "");
        // Everything except the buffers survives.
        assert_eq!(stored.mode, saved.mode);
        assert_eq!(stored.htmd, saved.htmd);
        assert_eq!(stored.skip_tags, saved.skip_tags);
    }

    #[test]
    fn recovery_convert_removes_marker_only_after_success() {
        let mut h = Harness::new();
        h.store.seed(
            WORKSPACE_KEY,
            &serde_json::to_string(&distinct_workspace()).expect("serializes"),
        );
        h.store.seed(CRASH_MARKER_KEY, "1");
        h.controller.load();
        h.controller
            .on_recovery_choice(RecoveryChoice::ConvertRestoredInput);
        assert!(
            h.store.marker_present(),
            "FR-8: marker is removed once conversion succeeds, not before"
        );
        h.controller.recovery_conversion_succeeded();
        assert!(!h.store.marker_present());
        assert_eq!(
            h.controller.workspace().input_html_to_md,
            distinct_workspace().input_html_to_md,
            "convert-again keeps the restored buffers"
        );
    }

    // ---------- debounce & flush-before-write ----------

    #[test]
    fn five_edits_within_500ms_cause_one_write() {
        let mut h = Harness::new();
        for i in 1..=5 {
            h.controller
                .on_text_edit(Mode::MdToHtml, &format!("draft {i}"));
            h.advance(100);
        }
        assert!(
            h.store.workspace_writes().is_empty(),
            "the 500 ms window has not elapsed"
        );
        let signals = h.advance(500);
        assert_eq!(signals, vec![None]);
        let writes = h.store.workspace_writes();
        assert_eq!(writes.len(), 1, "five edits within the window coalesce");
        let stored: Workspace = serde_json::from_str(&writes[0]).expect("valid JSON");
        assert_eq!(stored.input_md_to_html, "draft 5");
    }

    #[test]
    fn option_change_immediately_after_typing_writes_text_at_once() {
        // §8.6 data-loss prevention: type, toggle an option within
        // 500 ms, refresh -- the typed text is fully preserved.
        let mut h = Harness::new();
        h.controller.on_text_edit(Mode::MdToHtml, "freshly typed");
        let (cmark, htmd) = demo_preset();
        let signal = h.controller.set_options(cmark, htmd, "");
        assert_eq!(signal, None);
        assert_eq!(h.store.workspace_writes().len(), 1);
        let stored = h.store.stored_workspace();
        assert_eq!(stored.input_md_to_html, "freshly typed");
        assert_eq!(stored.cmark, cmark);
        assert_eq!(h.advance(600), vec![], "pending timer was cancelled");
        assert_eq!(
            h.store.workspace_writes().len(),
            1,
            "no second write after the flush-before-write"
        );
    }

    #[test]
    fn every_non_text_interaction_flushes_before_write() {
        for interaction in [
            |c: &mut StorageController<Rc<ScriptedStore>>| {
                c.set_mode(Mode::HtmlToMd);
            },
            |c: &mut StorageController<Rc<ScriptedStore>>| {
                c.set_active_tab(ActiveTab::Source);
            },
            |c: &mut StorageController<Rc<ScriptedStore>>| {
                c.select_html_sample(Some(HtmlSample::Math));
            },
            |c: &mut StorageController<Rc<ScriptedStore>>| {
                c.select_md_sample(None);
            },
            |c: &mut StorageController<Rc<ScriptedStore>>| {
                c.reset_options();
            },
        ] {
            let mut h = Harness::new();
            h.controller.on_text_edit(Mode::HtmlToMd, "typed text");
            interaction(&mut h.controller);
            assert_eq!(
                h.store.workspace_writes().len(),
                1,
                "interaction writes synchronously"
            );
            assert_eq!(h.store.stored_workspace().input_html_to_md, "typed text");
            assert_eq!(h.advance(700), vec![]);
        }
    }

    #[test]
    fn set_options_caps_skip_tags_at_200_chars() {
        let mut h = Harness::new();
        let (cmark, htmd) = demo_preset();
        h.controller.set_options(cmark, htmd, &"t".repeat(250));
        assert_eq!(h.controller.workspace().skip_tags.chars().count(), 200);
        assert_eq!(h.store.stored_workspace().skip_tags.chars().count(), 200);
    }

    #[test]
    fn reset_options_restores_only_the_sidebar_configuration() {
        let mut h = Harness::new();
        h.controller.load();
        h.controller.on_text_edit(Mode::MdToHtml, "keep me");
        h.controller.set_mode(Mode::HtmlToMd);
        h.controller.reset_options();
        let ws = h.controller.workspace().clone();
        let (cmark, htmd) = demo_preset();
        assert_eq!(ws.cmark, cmark);
        assert_eq!(ws.htmd, htmd);
        assert_eq!(ws.skip_tags, "");
        assert_eq!(ws.mode, Mode::HtmlToMd, "reset leaves the mode alone");
        assert_eq!(ws.input_md_to_html, "keep me");
    }

    #[test]
    fn pagehide_flush_forces_a_pending_save() {
        let mut h = Harness::new();
        h.controller.on_text_edit(Mode::MdToHtml, "before unload");
        assert!(h.store.workspace_writes().is_empty());
        assert_eq!(h.controller.flush(), None);
        assert_eq!(h.store.workspace_writes().len(), 1);
        assert_eq!(h.clock.live_timers(), 0, "the pending save timer is gone");
        assert_eq!(h.advance(1_000), vec![]);
    }

    #[test]
    fn scroll_edits_ride_the_debounced_save() {
        let mut h = Harness::new();
        h.controller.on_scroll(Mode::MdToHtml, 42.5);
        assert!(h.store.workspace_writes().is_empty());
        h.advance(499);
        assert!(h.store.workspace_writes().is_empty());
        h.advance(1);
        assert_eq!(h.store.workspace_writes().len(), 1);
        assert_eq!(h.store.stored_workspace().scroll_md_to_html, 42.5);
    }

    // ---------- quota guard ----------

    #[test]
    fn quota_failure_signals_once_until_next_success() {
        let mut h = Harness::new();
        h.controller.load();
        h.store.fail_with(StoreError::QuotaExceeded);
        h.controller.on_text_edit(Mode::MdToHtml, "text");
        assert_eq!(h.advance(500), vec![Some(BannerSignal::SaveFailed)]);
        // Still failing: no re-announce.
        h.controller.on_text_edit(Mode::MdToHtml, "more");
        assert_eq!(h.advance(500), vec![None]);
        assert_eq!(h.controller.flush(), None);
        // The next successful write clears the state silently.
        h.store.heal();
        h.controller.on_text_edit(Mode::MdToHtml, "healed");
        assert_eq!(h.advance(500), vec![None]);
        assert_eq!(h.store.workspace_writes().len(), 1);
        // A new transition into failure announces again.
        h.store.fail_with(StoreError::QuotaExceeded);
        assert_eq!(h.controller.flush(), Some(BannerSignal::SaveFailed));
    }

    #[test]
    fn blocked_store_never_panics_and_keeps_converting_state() {
        // §8.6 blocked storage: the app (here: the controller's in-memory
        // state) keeps working when every write fails as Blocked.
        let mut h = Harness::new();
        h.store.fail_with(StoreError::Blocked);
        h.controller.load();
        h.controller.on_text_edit(Mode::MdToHtml, "typed");
        assert_eq!(
            h.controller.set_mode(Mode::HtmlToMd),
            Some(BannerSignal::SaveFailed)
        );
        assert_eq!(h.controller.set_active_tab(ActiveTab::Source), None);
        h.advance(500);
        let ws = h.controller.workspace();
        assert_eq!(ws.input_md_to_html, "typed");
        assert_eq!(ws.mode, Mode::HtmlToMd);
        assert!(h.store.workspace_writes().is_empty());
    }

    // ---------- multi-tab ----------

    #[test]
    fn external_write_announces_once_until_own_write() {
        let mut h = Harness::new();
        h.controller.load();
        let before = h.controller.workspace().clone();
        assert_eq!(
            h.controller.on_storage_event(WORKSPACE_KEY),
            Some(BannerSignal::ExternalWrite)
        );
        // However many further external writes arrive: no re-announce.
        assert_eq!(h.controller.on_storage_event(WORKSPACE_KEY), None);
        assert_eq!(h.controller.on_storage_event(WORKSPACE_KEY), None);
        assert_eq!(
            h.controller.workspace(),
            &before,
            "external writes never alter this tab's buffers"
        );
        // Own successful write clears it (and stays last-writer-wins).
        h.controller.on_text_edit(Mode::MdToHtml, "mine now");
        h.advance(500);
        assert_eq!(h.store.workspace_writes().len(), 1);
        assert_eq!(h.store.stored_workspace().input_md_to_html, "mine now");
        // After the clear, a new external write announces again.
        assert_eq!(
            h.controller.on_storage_event(WORKSPACE_KEY),
            Some(BannerSignal::ExternalWrite)
        );
    }

    #[test]
    fn storage_events_for_other_keys_are_ignored() {
        let mut h = Harness::new();
        h.controller.load();
        assert_eq!(h.controller.on_storage_event(CRASH_MARKER_KEY), None);
        assert_eq!(h.controller.on_storage_event("unrelated"), None);
    }

    #[test]
    fn external_write_banner_clears_via_a_non_text_flush_too() {
        let mut h = Harness::new();
        h.controller.load();
        h.controller.on_storage_event(WORKSPACE_KEY);
        assert_eq!(h.controller.set_mode(Mode::HtmlToMd), None);
        assert_eq!(
            h.controller.on_storage_event(WORKSPACE_KEY),
            Some(BannerSignal::ExternalWrite),
            "the successful write cleared the state"
        );
    }
}
