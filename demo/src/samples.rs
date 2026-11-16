//! FR-6 sample documents: the three fixed-content samples shipped in the
//! binary and the pure selection/undo state behind the Samples buttons.
//! Buffer writes themselves belong to the conversion engine
//! ([`crate::engine::Engine::on_buffer_overwrite`]); this module decides
//! *which* mode's buffer a click or reload replaces, with what text, and
//! what the one-shot Undo restores.

use crate::engine::ConversionMode;
use crate::storage::{HtmlSample, MdSample};

/// Task brief's size guidance: each sample variant stays within ~1 kB.
#[allow(dead_code)] // asserted by the unit tests; not read at runtime
const MAX_VARIANT_BYTES: usize = 1024;

/// One FR-6 sample: fixed content with one variant per supported mode.
pub struct Sample {
    pub id: SampleId,
    /// Sidebar button label.
    pub label: &'static str,
    /// The Markdown → HTML variant, when the sample supports that mode.
    pub md_variant: Option<&'static str>,
    /// The HTML → Markdown variant, when the sample supports that mode.
    pub html_variant: Option<&'static str>,
}

impl Sample {
    /// The variant loaded into `mode`'s buffer; `None` when FR-6 does not
    /// give this sample a variant for that mode.
    pub const fn variant(&self, mode: ConversionMode) -> Option<&'static str> {
        match mode {
            ConversionMode::MarkdownToHtml => self.md_variant,
            ConversionMode::HtmlToMarkdown => self.html_variant,
        }
    }

    /// Whether the sample has a variant for `mode` (FR-6's availability
    /// matrix: basics and math in both modes, inline HTML in HTML →
    /// Markdown only).
    pub const fn available_in(&self, mode: ConversionMode) -> bool {
        match mode {
            ConversionMode::MarkdownToHtml => self.md_variant.is_some(),
            ConversionMode::HtmlToMarkdown => self.html_variant.is_some(),
        }
    }
}

/// Identifies one of the three FR-6 samples; doubles as the value behind
/// the persistence mapping ([`Self::as_md_sample`]/[`Self::as_html_sample`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleId {
    /// CommonMark basics (both modes).
    Basics,
    /// Inline HTML (HTML → Markdown only).
    InlineHtml,
    /// Math (both modes).
    Math,
}

impl SampleId {
    /// The sample's row in [`SAMPLES`].
    pub fn sample(self) -> &'static Sample {
        &SAMPLES[match self {
            SampleId::Basics => 0,
            SampleId::InlineHtml => 1,
            SampleId::Math => 2,
        }]
    }

    /// The persistence value of FR-8's `sample_md_to_html` field;
    /// `None` for samples without a Markdown variant.
    pub fn as_md_sample(self) -> Option<MdSample> {
        match self {
            SampleId::Basics => Some(MdSample::Basics),
            SampleId::InlineHtml => None,
            SampleId::Math => Some(MdSample::Math),
        }
    }

    /// The persistence value of FR-8's `sample_html_to_md` field.
    pub fn as_html_sample(self) -> Option<HtmlSample> {
        match self {
            SampleId::Basics => Some(HtmlSample::Basics),
            SampleId::InlineHtml => Some(HtmlSample::InlineHtml),
            SampleId::Math => Some(HtmlSample::Math),
        }
    }

    /// Reverse of [`Self::as_md_sample`] (FR-8 workspace restore).
    pub fn from_md_sample(sample: MdSample) -> Self {
        match sample {
            MdSample::Basics => SampleId::Basics,
            MdSample::Math => SampleId::Math,
        }
    }

    /// Reverse of [`Self::as_html_sample`] (FR-8 workspace restore).
    pub fn from_html_sample(sample: HtmlSample) -> Self {
        match sample {
            HtmlSample::Basics => SampleId::Basics,
            HtmlSample::InlineHtml => SampleId::InlineHtml,
            HtmlSample::Math => SampleId::Math,
        }
    }
}

/// The three FR-6 samples, in sidebar order (§5's sketch: Basics, HTML,
/// Math). The Markdown variants are written in `htmd`'s own output style
/// so a round trip through "Use Output as Input" comes back unchanged,
/// and each HTML variant is exactly what `pulldown-cmark` renders for its
/// Markdown variant under the fixed FR-5 round-trip flags (pinned by
/// test).
pub static SAMPLES: [Sample; 3] = [
    Sample {
        id: SampleId::Basics,
        label: "Basics",
        md_variant: Some(
            "# CommonMark basics\n\n\
             Some *emphasis*, **strong emphasis**, and a [link](https://example.com).\n\n\
             ## Lists\n\n\
             *   one\n\
             *   two\n\
             \x20   *   nested\n\n\
             1.  first\n\
             2.  second\n\n\
             ## Table\n\n\
             | Left | Right |\n\
             | ---- | ----- |\n\
             | a    | b     |\n\n\
             ## Fenced code\n\n\
             ```rust\n\
             fn main() {\n\
             \x20   println!(\"Hello, htmd!\");\n\
             }\n\
             ```\n\n\
             > A block quote with `inline code`.\n",
        ),
        html_variant: Some(
            "<h1>CommonMark basics</h1>\n\
             <p>Some <em>emphasis</em>, <strong>strong emphasis</strong>, and a <a href=\"https://example.com\">link</a>.</p>\n\
             <h2>Lists</h2>\n\
             <ul>\n\
             <li>one</li>\n\
             <li>two\n\
             <ul>\n\
             <li>nested</li>\n\
             </ul>\n\
             </li>\n\
             </ul>\n\
             <ol>\n\
             <li>first</li>\n\
             <li>second</li>\n\
             </ol>\n\
             <h2>Table</h2>\n\
             <table><thead><tr><th>Left</th><th>Right</th></tr></thead><tbody>\n\
             <tr><td>a</td><td>b</td></tr>\n\
             </tbody></table>\n\
             <h2>Fenced code</h2>\n\
             <pre><code class=\"language-rust\">fn main() {\n\
             \x20   println!(\"Hello, htmd!\");\n\
             }\n\
             </code></pre>\n\
             <blockquote>\n\
             <p>A block quote with <code>inline code</code>.</p>\n\
             </blockquote>\n",
        ),
    },
    Sample {
        id: SampleId::InlineHtml,
        label: "Inline HTML",
        md_variant: None,
        html_variant: Some(
            "<h2>Inline HTML</h2>\n\
             <p>Text with <span class=\"hl\">attributes</span> and a line<br>break.</p>\n\
             <div class=\"note\" data-kind=\"raw\">\n\
             \x20 <p>A raw HTML <em>block</em> with a <a href=\"https://example.com/docs\" title=\"Docs\">link</a>.</p>\n\
             </div>\n",
        ),
    },
    Sample {
        id: SampleId::Math,
        label: "Math",
        md_variant: Some(
            "# Math\n\n\
             Inline math $E = mc^2$ in a sentence, plus a display formula:\n\n\
             $$\\int_0^1 x^2 \\, dx = \\tfrac{1}{3}$$\n",
        ),
        html_variant: Some(
            "<h1>Math</h1>\n\
             <p>Inline math <span class=\"math math-inline\">E = mc^2</span> in a sentence, plus a display formula:</p>\n\
             <p><span class=\"math math-display\">\\int_0^1 x^2 \\, dx = \\tfrac{1}{3}</span></p>\n",
        ),
    },
];

/// What a sample click or Reload Sample tells the UI to do (FR-6:140-142).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadAction {
    /// The mode whose buffer receives `text` — always the sample's own
    /// mode, never the other one's.
    pub target: ConversionMode,
    /// True when the click switches the active mode (the cross-mode click
    /// of FR-6:140).
    pub switches_mode: bool,
    /// The variant to write into the target buffer.
    pub text: &'static str,
    /// Whether the load offers the FR-6 Undo affordance: the replaced
    /// buffer was non-empty, the same rule as "Use Output as Input"
    /// (FR-2:62).
    pub undo_offered: bool,
}

/// What a successful Undo tells the UI to do (FR-6:141): restore the
/// replaced buffer of `mode` and nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UndoAction {
    /// The mode whose buffer the Undo writes back.
    pub mode: ConversionMode,
    /// The buffer contents displaced by the load.
    pub text: String,
}

/// The pending Undo, if any: the buffer displaced by the latest load.
#[derive(Debug, Clone, PartialEq, Eq)]
struct UndoSlot {
    mode: ConversionMode,
    previous: String,
}

/// FR-6 sample selection state: which sample each mode has selected, plus
/// the single Undo snapshot of the latest load. Pure logic — buffers and
/// conversions live in the engine; persistence lives in storage.
pub struct SampleState {
    md_selection: Option<MdSample>,
    html_selection: Option<HtmlSample>,
    undo: Option<UndoSlot>,
}

impl SampleState {
    /// FR-8:154's initial state: no sample selected or preloaded, in
    /// either mode.
    pub fn new() -> Self {
        SampleState {
            md_selection: None,
            html_selection: None,
            undo: None,
        }
    }

    /// FR-8's `sample_md_to_html` value to persist for the Markdown →
    /// HTML mode (`None` = none selected).
    pub fn md_selection(&self) -> Option<MdSample> {
        self.md_selection
    }

    /// FR-8's `sample_html_to_md` value to persist for the HTML →
    /// Markdown mode (`None` = none selected).
    pub fn html_selection(&self) -> Option<HtmlSample> {
        self.html_selection
    }

    /// Seed the selections from a restored FR-8 workspace. Undo history
    /// starts empty: Undo is offered per load (FR-6:140), never across a
    /// page reload.
    pub fn restore(&mut self, md: Option<MdSample>, html: Option<HtmlSample>) {
        self.md_selection = md;
        self.html_selection = html;
        self.undo = None;
    }

    /// FR-6:140 sample click: load `sample`'s variant for the current
    /// mode into that mode's buffer. When the sample has no variant for
    /// the current mode (Inline HTML clicked in Markdown → HTML), the
    /// click switches to the sample's mode and loads it into that mode's
    /// buffer — the other mode's buffer and selection stay untouched.
    /// `target_buffer` is the *target* mode's current buffer, which
    /// decides whether Undo is offered (non-empty, per FR-2:62's rule).
    /// The load becomes the target mode's selected sample.
    pub fn load(
        &mut self,
        current_mode: ConversionMode,
        sample: SampleId,
        target_buffer: &str,
    ) -> LoadAction {
        // A sample always has an HTML variant (FR-6's matrix), so the
        // fallback mode is the one conversion mode it can load into.
        let target = if sample.sample().available_in(current_mode) {
            current_mode
        } else {
            other_mode(current_mode)
        };
        let switches_mode = target != current_mode;
        self.apply_load(target, sample, switches_mode, true, target_buffer)
    }

    /// FR-6:142 Reload Sample: load the mode's selected sample (its
    /// default, CommonMark basics, when none is selected) into the
    /// current buffer, offering the same Undo. Never switches mode and
    /// never changes the selection — a reload with nothing selected
    /// leaves it unselected.
    pub fn reload_sample(&mut self, mode: ConversionMode, current_buffer: &str) -> LoadAction {
        let sample = match mode {
            ConversionMode::MarkdownToHtml => self.md_selection.map(SampleId::from_md_sample),
            ConversionMode::HtmlToMarkdown => self.html_selection.map(SampleId::from_html_sample),
        }
        .unwrap_or(SampleId::Basics);
        self.apply_load(mode, sample, false, false, current_buffer)
    }

    /// FR-6:140-141 Undo: restore the buffer displaced by the latest
    /// load, exactly once per load. The selection stays as the load left
    /// it, so a subsequent Reload Sample loads that selected sample.
    pub fn undo(&mut self) -> Option<UndoAction> {
        self.undo.take().map(|slot| UndoAction {
            mode: slot.mode,
            text: slot.previous,
        })
    }

    /// Drop the pending Undo. FR-2:62's toast lifetime — the next buffer
    /// edit, mode switch, "Use Output as Input", or the toast's close
    /// button — is the UI's to report; none of those events pass through
    /// this module.
    pub fn dismiss_undo(&mut self) {
        self.undo = None;
    }

    /// Shared tail of load and reload: arm the one-shot Undo from the
    /// displaced buffer and describe the buffer write. Only a click
    /// (`records_selection`) makes the sample the mode's selection —
    /// reload reads the selection, it must not write it, so the basics
    /// fallback stays unselected (FR-6:142 loads "that mode's default if
    /// none is selected" without selecting it).
    fn apply_load(
        &mut self,
        target: ConversionMode,
        sample: SampleId,
        switches_mode: bool,
        records_selection: bool,
        target_buffer: &str,
    ) -> LoadAction {
        let text = sample
            .sample()
            .variant(target)
            .expect("every sample loads into its own mode");
        if records_selection {
            match target {
                ConversionMode::MarkdownToHtml => {
                    self.md_selection = sample.as_md_sample();
                }
                ConversionMode::HtmlToMarkdown => {
                    self.html_selection = sample.as_html_sample();
                }
            }
        }
        self.undo = (!target_buffer.is_empty()).then(|| UndoSlot {
            mode: target,
            previous: target_buffer.to_owned(),
        });
        LoadAction {
            target,
            switches_mode,
            text,
            undo_offered: self.undo.is_some(),
        }
    }
}

fn other_mode(mode: ConversionMode) -> ConversionMode {
    match mode {
        ConversionMode::MarkdownToHtml => ConversionMode::HtmlToMarkdown,
        ConversionMode::HtmlToMarkdown => ConversionMode::MarkdownToHtml,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::{convert_html_to_md, convert_md_to_html};
    use crate::fidelity::canonicalize;
    use crate::options_model::{CmarkFlags, HtmdOptions};
    use htmd::options::TranslationMode;
    use serde_json::json;

    /// FR-5:118's forward options: crate defaults with `translation_mode`
    /// forced to Faithful.
    fn faithful_defaults() -> HtmdOptions {
        HtmdOptions {
            translation_mode: TranslationMode::Faithful,
            ..Default::default()
        }
    }

    /// FR-5:120's fixed round-trip renderer flags, promoted to
    /// [`CmarkFlags::tables_math`].
    fn tables_math() -> CmarkFlags {
        CmarkFlags::tables_math()
    }

    fn variant(id: SampleId, mode: ConversionMode) -> &'static str {
        id.sample().variant(mode).expect("variant exists")
    }

    #[test]
    fn new_state_selects_nothing() {
        // FR-8:154: no sample selected or preloaded.
        let mut state = SampleState::new();
        assert_eq!(state.md_selection(), None);
        assert_eq!(state.html_selection(), None);
        assert_eq!(state.undo(), None);
    }

    #[test]
    fn load_writes_current_mode_buffer_and_selects() {
        // FR-6:140: clicking a sample with a variant for the current mode
        // loads it into the current buffer and selects it for that mode.
        let mut state = SampleState::new();
        let action = state.load(
            ConversionMode::MarkdownToHtml,
            SampleId::Basics,
            "typed markdown",
        );
        assert_eq!(action.target, ConversionMode::MarkdownToHtml);
        assert!(!action.switches_mode);
        assert_eq!(
            action.text,
            variant(SampleId::Basics, ConversionMode::MarkdownToHtml)
        );
        assert!(action.undo_offered);
        assert_eq!(state.md_selection(), Some(MdSample::Basics));
        assert_eq!(state.html_selection(), None);
    }

    #[test]
    fn cross_mode_click_switches_mode_and_touches_only_html_buffer() {
        // FR-6:140 / §8.5:267: Inline HTML clicked in Markdown → HTML
        // switches to HTML → Markdown and replaces only the HTML buffer.
        let mut state = SampleState::new();
        let action = state.load(
            ConversionMode::MarkdownToHtml,
            SampleId::InlineHtml,
            "whatever the html buffer held",
        );
        assert_eq!(action.target, ConversionMode::HtmlToMarkdown);
        assert!(action.switches_mode);
        assert_eq!(
            action.text,
            variant(SampleId::InlineHtml, ConversionMode::HtmlToMarkdown)
        );
        // Only the HTML mode's selection changed; the Markdown mode's
        // buffer and selection are untouched.
        assert_eq!(state.md_selection(), None);
        assert_eq!(state.html_selection(), Some(HtmlSample::InlineHtml));
    }

    #[test]
    fn inline_html_click_from_html_mode_does_not_switch() {
        // FR-6:140's switch fires only when the variant is absent; in the
        // sample's own mode the click is an ordinary same-mode load.
        let mut state = SampleState::new();
        let action = state.load(ConversionMode::HtmlToMarkdown, SampleId::InlineHtml, "");
        assert_eq!(action.target, ConversionMode::HtmlToMarkdown);
        assert!(!action.switches_mode);
        assert_eq!(state.html_selection(), Some(HtmlSample::InlineHtml));
    }

    #[test]
    fn undo_restores_buffer_only_and_selection_survives() {
        // FR-6:141 / §8.5:267: Undo restores the replaced buffer and
        // intentionally leaves the newly selected sample unchanged.
        let mut state = SampleState::new();
        state.load(
            ConversionMode::MarkdownToHtml,
            SampleId::Math,
            "typed markdown",
        );
        assert_eq!(
            state.undo(),
            Some(UndoAction {
                mode: ConversionMode::MarkdownToHtml,
                text: "typed markdown".to_owned(),
            })
        );
        assert_eq!(state.md_selection(), Some(MdSample::Math));
        assert_eq!(state.html_selection(), None);
    }

    #[test]
    fn undo_is_once_per_load() {
        let mut state = SampleState::new();
        state.load(ConversionMode::MarkdownToHtml, SampleId::Basics, "typed");
        assert!(state.undo().is_some());
        assert_eq!(state.undo(), None);
    }

    #[test]
    fn undo_offered_only_when_replaced_buffer_was_non_empty() {
        // FR-6:140 offers "the same 'Undo' affordance as 'Use Output as
        // Input'"; FR-2:62 shows that toast only for a non-empty target.
        let mut state = SampleState::new();
        let action = state.load(ConversionMode::MarkdownToHtml, SampleId::Basics, "");
        assert!(!action.undo_offered);
        assert_eq!(state.undo(), None);
    }

    #[test]
    fn later_load_supersedes_the_undo_snapshot() {
        let mut state = SampleState::new();
        state.load(ConversionMode::HtmlToMarkdown, SampleId::Basics, "first");
        state.load(ConversionMode::HtmlToMarkdown, SampleId::Math, "second");
        // Undo restores what the *latest* load displaced.
        assert_eq!(
            state.undo(),
            Some(UndoAction {
                mode: ConversionMode::HtmlToMarkdown,
                text: "second".to_owned(),
            })
        );
        assert_eq!(state.html_selection(), Some(HtmlSample::Math));
    }

    #[test]
    fn reload_loads_selected_sample_without_changing_anything_else() {
        // FR-6:142 / §8.5:269: Reload restores the mode's selected sample,
        // offers Undo, and never changes mode or selection.
        let mut state = SampleState::new();
        state.load(
            ConversionMode::MarkdownToHtml,
            SampleId::Math,
            "edited text",
        );
        let action = state.reload_sample(ConversionMode::MarkdownToHtml, "further edits");
        assert_eq!(action.target, ConversionMode::MarkdownToHtml);
        assert!(!action.switches_mode);
        assert_eq!(
            action.text,
            variant(SampleId::Math, ConversionMode::MarkdownToHtml)
        );
        assert!(action.undo_offered);
        assert_eq!(
            state.undo(),
            Some(UndoAction {
                mode: ConversionMode::MarkdownToHtml,
                text: "further edits".to_owned(),
            })
        );
        assert_eq!(state.md_selection(), Some(MdSample::Math));
    }

    #[test]
    fn reload_defaults_to_basics_when_nothing_is_selected() {
        // FR-6:141: each mode's default sample is CommonMark basics. The
        // fallback loads basics without selecting it — FR-8:154's fresh
        // workspace stays sample-less until a click.
        let mut state = SampleState::new();
        let action = state.reload_sample(ConversionMode::HtmlToMarkdown, "");
        assert_eq!(action.target, ConversionMode::HtmlToMarkdown);
        assert!(!action.switches_mode);
        assert_eq!(
            action.text,
            variant(SampleId::Basics, ConversionMode::HtmlToMarkdown)
        );
        assert!(!action.undo_offered);
        assert_eq!(state.html_selection(), None);
        assert_eq!(state.md_selection(), None);
    }

    #[test]
    fn restore_seeds_selections_from_the_workspace() {
        // FR-8: a restored workspace replays its persisted selections;
        // no Undo survives a reload.
        let mut state = SampleState::new();
        state.restore(Some(MdSample::Math), Some(HtmlSample::InlineHtml));
        assert_eq!(state.md_selection(), Some(MdSample::Math));
        assert_eq!(state.html_selection(), Some(HtmlSample::InlineHtml));
        assert_eq!(state.undo(), None);
    }

    #[test]
    fn availability_matrix_matches_fr6() {
        // FR-6:135-137: basics — both modes; inline HTML — HTML →
        // Markdown only; math — both modes.
        let both = [
            ConversionMode::MarkdownToHtml,
            ConversionMode::HtmlToMarkdown,
        ];
        for id in [SampleId::Basics, SampleId::Math] {
            for mode in both {
                assert!(id.sample().available_in(mode), "{id:?} in {mode:?}");
                assert!(id.sample().variant(mode).is_some());
            }
        }
        assert!(
            SampleId::InlineHtml
                .sample()
                .available_in(ConversionMode::HtmlToMarkdown)
        );
        assert!(
            !SampleId::InlineHtml
                .sample()
                .available_in(ConversionMode::MarkdownToHtml)
        );
        assert_eq!(
            SampleId::InlineHtml
                .sample()
                .variant(ConversionMode::MarkdownToHtml),
            None
        );
    }

    #[test]
    fn variants_stay_within_the_size_budget() {
        // Task brief: each variant ≤ ~1 kB.
        assert_eq!(SAMPLES.len(), 3);
        for sample in &SAMPLES {
            for mode in both_modes() {
                if let Some(text) = sample.variant(mode) {
                    assert!(
                        text.len() <= MAX_VARIANT_BYTES,
                        "{} variant is {} bytes",
                        sample.label,
                        text.len()
                    );
                }
            }
        }
    }

    fn both_modes() -> [ConversionMode; 2] {
        [
            ConversionMode::MarkdownToHtml,
            ConversionMode::HtmlToMarkdown,
        ]
    }

    #[test]
    fn selections_map_onto_the_persisted_workspace_fields() {
        // FR-8:172: `sample_md_to_html` is "basics"/"math", and
        // `sample_html_to_md` is "basics"/"inline_html"/"math" — the
        // selections must serialize as exactly those strings.
        let mut state = SampleState::new();
        state.load(ConversionMode::MarkdownToHtml, SampleId::Math, "");
        state.load(ConversionMode::HtmlToMarkdown, SampleId::InlineHtml, "");
        let workspace = crate::storage::Workspace {
            sample_md_to_html: state.md_selection(),
            sample_html_to_md: state.html_selection(),
            ..Default::default()
        };
        let stored = serde_json::to_value(&workspace).unwrap();
        assert_eq!(stored["sample_md_to_html"], json!("math"));
        assert_eq!(stored["sample_html_to_md"], json!("inline_html"));
        // And the mapping round-trips back through the storage enums.
        assert_eq!(
            SampleId::from_md_sample(MdSample::Math).as_md_sample(),
            Some(MdSample::Math)
        );
        assert_eq!(
            SampleId::from_html_sample(HtmlSample::InlineHtml).as_html_sample(),
            Some(HtmlSample::InlineHtml)
        );
        assert_eq!(SampleId::InlineHtml.as_md_sample(), None);
    }

    #[test]
    fn basics_variants_are_pulldown_consistent() {
        // The HTML variant is exactly what pulldown-cmark renders for the
        // Markdown variant under the FR-5 round-trip flags, so the pair
        // cannot drift apart.
        let rendered = convert_md_to_html(
            variant(SampleId::Basics, ConversionMode::MarkdownToHtml),
            &tables_math(),
        );
        assert_eq!(
            rendered,
            variant(SampleId::Basics, ConversionMode::HtmlToMarkdown)
        );
    }

    #[test]
    fn math_variants_are_pulldown_consistent() {
        let rendered = convert_md_to_html(
            variant(SampleId::Math, ConversionMode::MarkdownToHtml),
            &tables_math(),
        );
        assert_eq!(
            rendered,
            variant(SampleId::Math, ConversionMode::HtmlToMarkdown)
        );
    }

    #[test]
    fn basics_round_trips_losslessly_in_both_modes() {
        // FR-6:135 + §8.7:307: the basics sample is zero-diff golden
        // corpus — HTML → Markdown → HTML canonicalizes identically —
        // and the Markdown side re-renders identically after its own
        // round trip.
        let html = variant(SampleId::Basics, ConversionMode::HtmlToMarkdown);
        let md = convert_html_to_md(html, &faithful_defaults(), &[]).unwrap();
        let round_tripped = convert_md_to_html(&md, &tables_math());
        assert_eq!(canonicalize(html), canonicalize(&round_tripped));

        let md_variant = variant(SampleId::Basics, ConversionMode::MarkdownToHtml);
        let rendered = convert_md_to_html(md_variant, &tables_math());
        let back = convert_html_to_md(&rendered, &faithful_defaults(), &[]).unwrap();
        let re_rendered = convert_md_to_html(&back, &tables_math());
        assert_eq!(canonicalize(&rendered), canonicalize(&re_rendered));
    }

    #[test]
    fn math_round_trips_losslessly_in_both_modes() {
        // FR-6:137: `$`/`$$` formulas round-trip while ENABLE_MATH is on.
        let html = variant(SampleId::Math, ConversionMode::HtmlToMarkdown);
        let md = convert_html_to_md(html, &faithful_defaults(), &[]).unwrap();
        assert!(md.contains('$'));
        let round_tripped = convert_md_to_html(&md, &tables_math());
        assert_eq!(canonicalize(html), canonicalize(&round_tripped));

        let md_variant = variant(SampleId::Math, ConversionMode::MarkdownToHtml);
        let rendered = convert_md_to_html(md_variant, &tables_math());
        let back = convert_html_to_md(&rendered, &faithful_defaults(), &[]).unwrap();
        let re_rendered = convert_md_to_html(&back, &tables_math());
        assert_eq!(canonicalize(&rendered), canonicalize(&re_rendered));
    }

    #[test]
    fn inline_html_sample_shows_the_mode_difference() {
        // FR-6:136: attributes are lost and visible in Pure mode,
        // preserved in Faithful mode.
        let html = variant(SampleId::InlineHtml, ConversionMode::HtmlToMarkdown);
        let pure = convert_html_to_md(html, &HtmdOptions::default(), &[]).unwrap();
        assert!(!pure.contains("class="));
        let faithful = convert_html_to_md(html, &faithful_defaults(), &[]).unwrap();
        assert!(faithful.contains(r#"<span class="hl">"#));
        assert!(faithful.contains("<br>"));
        assert!(faithful.contains(r#"<div class="note" data-kind="raw">"#));
    }

    #[test]
    fn fidelity_round_trip_renders_with_fixed_flags_not_the_sidebar() {
        // FR-5:120: even with every sidebar cmark flag enabled, the
        // round-trip side renders with exactly ENABLE_TABLES |
        // ENABLE_MATH, so text pulldown-cmark would rewrite under other
        // flags (smart punctuation's dots and dashes) cannot become a
        // false diff. Rendering the same Markdown with the sidebar's
        // flags *would* diverge, which is what the fixed flags prevent.
        use crate::fidelity::{WordDiff, word_diff};
        let html = r#"<p>Wait... he said "ok" -- really?</p>"#;
        let md = convert_html_to_md(html, &faithful_defaults(), &[]).unwrap();

        let fixed = convert_md_to_html(&md, &tables_math());
        assert_eq!(
            word_diff(&canonicalize(html), &canonicalize(&fixed)),
            WordDiff::Inclusive
        );

        let sidebar = CmarkFlags {
            tables: true,
            footnotes: true,
            strikethrough: true,
            task_lists: true,
            smart_punctuation: true,
            heading_attributes: true,
            math: true,
            yaml_style_metadata_blocks: true,
            pluses_delimited_metadata_blocks: true,
            old_footnotes: true,
            gfm: true,
            definition_list: true,
            superscript: true,
            subscript: true,
            wikilinks: true,
        };
        let rewritten = convert_md_to_html(&md, &sidebar);
        assert!(
            matches!(
                word_diff(&canonicalize(html), &canonicalize(&rewritten)),
                WordDiff::Changes(_)
            ),
            "the control: sidebar flags really do rewrite this text"
        );
    }
}
