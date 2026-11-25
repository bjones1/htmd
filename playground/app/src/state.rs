//! The playground's state and the rules for when it converts and when it
//! writes the address (spec, Conversion, Permalinks, and Limits and
//! failures).
//!
//! The input `<textarea>` itself holds the input text, read only when
//! converting, so a keystroke in a large input costs nothing until the typing
//! pause ends.

use std::time::Duration;

use htmd_playground_core::LARGE_INPUT_BYTES;
use htmd_playground_core::codec::{self, Decoded};
use htmd_playground_core::presets;
use htmd_playground_core::settings::{Direction, Settings, State};
use leptos::html;
use leptos::prelude::*;

use crate::{address, engine};

const TYPING_PAUSE: Duration = Duration::from_millis(100);

/// The output pane's tabs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tab {
    Source,
    Rendered,
    Original,
}

impl Tab {
    /// The tabs shown in `direction`.
    pub fn all(direction: Direction) -> &'static [Tab] {
        match direction {
            Direction::HtmlToMarkdown => &[Tab::Source, Tab::Rendered, Tab::Original],
            Direction::MarkdownToHtml => &[Tab::Source, Tab::Rendered],
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Tab::Source => "Source",
            Tab::Rendered => "Rendered",
            Tab::Original => "Original",
        }
    }
}

/// The result of the last conversion.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Output {
    /// The converted text, shown in Source.
    pub source: String,
    /// The HTML shown in Rendered.
    pub rendered_html: String,
    /// The HTML shown in Original (HTML → Markdown only).
    pub original_html: String,
    /// The time the conversion shown in Source took, absent when nothing has
    /// been converted.
    pub millis: Option<f64>,
    /// The length of `source` in Unicode code points.
    pub chars: usize,
}

/// Whether the output matches the input.
#[derive(Clone, Debug, PartialEq)]
pub enum Phase {
    /// The output is up to date, or will be when the typing pause ends.
    Current,
    /// The input is over [`LARGE_INPUT_BYTES`]; the output is from an earlier
    /// input, or empty.
    Paused,
    /// A converter crashed with this message; nothing converts again.
    Crashed(String),
}

/// A notice about the page address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressNotice {
    /// The address could not be read, so the default example is shown.
    InvalidLink,
    /// The input did not fit in the address, which holds only the direction
    /// and options.
    TooLargeToShare,
}

/// The playground's state. Every field is a handle, so the struct is `Copy`
/// and event handlers capture it directly.
#[derive(Clone, Copy)]
pub struct Playground {
    pub direction: RwSignal<Direction>,
    pub settings: RwSignal<Settings>,
    pub input: NodeRef<html::Textarea>,
    pub output: RwSignal<Output>,
    pub phase: RwSignal<Phase>,
    pub tab: RwSignal<Tab>,
    pub address_notice: RwSignal<Option<AddressNotice>>,
    /// A one-off message, such as the result of copying.
    pub message: RwSignal<Option<String>>,
    /// The address shown selected when copying the link failed.
    pub share_fallback: RwSignal<Option<String>>,
    /// Set by Convert anyway; cleared once the input is no longer large.
    convert_anyway: StoredValue<bool>,
    pending: StoredValue<Option<TimeoutHandle>>,
}

impl Playground {
    pub fn new() -> Self {
        Self {
            direction: RwSignal::new(Direction::default()),
            settings: RwSignal::new(Settings::default()),
            input: NodeRef::new(),
            output: RwSignal::new(Output::default()),
            phase: RwSignal::new(Phase::Current),
            tab: RwSignal::new(Tab::Source),
            address_notice: RwSignal::new(None),
            message: RwSignal::new(None),
            share_fallback: RwSignal::new(None),
            convert_anyway: StoredValue::new(false),
            pending: StoredValue::new(None),
        }
    }

    /// The input text.
    pub fn input_text(&self) -> String {
        self.input
            .get_untracked()
            .map(|input| input.value())
            .unwrap_or_default()
    }

    fn set_input_text(&self, text: &str) {
        if let Some(input) = self.input.get_untracked() {
            input.set_value(text);
        }
    }

    /// Whether the output may not match the input, which disables the
    /// direction switch and Copy (spec, Limits and failures).
    pub fn is_stale(&self) -> bool {
        self.phase.with(|phase| *phase != Phase::Current)
    }

    fn state(&self) -> State {
        State {
            direction: self.direction.get_untracked(),
            input: self.input_text(),
            settings: self.settings.get_untracked(),
        }
    }

    fn cancel_pending(&self) {
        if let Some(handle) = self.pending.try_update_value(Option::take).flatten() {
            handle.clear();
        }
    }

    /// Convert the current input now.
    fn convert(&self) {
        self.cancel_pending();
        if matches!(self.phase.get_untracked(), Phase::Crashed(_)) {
            return;
        }
        let input = self.input_text();
        if input.len() <= LARGE_INPUT_BYTES {
            self.convert_anyway.set_value(false);
        } else if !self.convert_anyway.get_value() {
            self.phase.set(Phase::Paused);
            return;
        }
        let direction = self.direction.get_untracked();
        let settings = self.settings.get_untracked();
        match convert(direction, input, &settings) {
            Ok(output) => {
                self.output.set(output);
                self.phase.set(Phase::Current);
            }
            Err(message) => self.phase.set(Phase::Crashed(message)),
        }
    }

    /// Record the current state in the address.
    pub fn write_address(&self) {
        let fits = address::write(&self.state());
        self.address_notice
            .set((!fits).then_some(AddressNotice::TooLargeToShare));
        self.message.set(None);
        self.share_fallback.set(None);
    }

    /// The user typed: convert and update the address after the typing
    /// pause.
    pub fn input_changed(&self) {
        self.cancel_pending();
        let playground = *self;
        let handle = set_timeout_with_handle(
            move || {
                playground.pending.set_value(None);
                playground.convert();
                playground.write_address();
            },
            TYPING_PAUSE,
        );
        self.pending.set_value(handle.ok());
    }

    /// Change the options, then convert and update the address at once.
    pub fn update_settings(&self, change: impl FnOnce(&mut Settings)) {
        self.settings.update(change);
        self.convert();
        self.write_address();
    }

    /// Finish any conversion still waiting for the typing pause to end.
    pub fn flush(&self) {
        if self.pending.with_value(Option::is_some) {
            self.convert();
            self.write_address();
        }
    }

    /// Replace the input on the user's behalf (Examples, Clear).
    pub fn replace_input(&self, text: &str) {
        self.cancel_pending();
        self.set_input_text(text);
        self.convert();
        self.write_address();
    }

    /// Press Convert anyway.
    pub fn convert_anyway(&self) {
        self.convert_anyway.set_value(true);
        self.convert();
    }

    /// Move the output into the input and convert it the other way. Returns
    /// whether the direction changed.
    pub fn switch_direction(&self) -> bool {
        self.flush();
        if self.is_stale() {
            return false;
        }
        let text = self.output.with_untracked(|output| output.source.clone());
        self.show(self.direction.get_untracked().flipped(), &text);
        self.convert();
        self.write_address();
        true
    }

    /// Show `text` as the input in `direction`, with an empty output until it
    /// converts.
    fn show(&self, direction: Direction, text: &str) {
        self.cancel_pending();
        self.direction.set(direction);
        if !Tab::all(direction).contains(&self.tab.get_untracked()) {
            self.tab.set(Tab::Source);
        }
        self.set_input_text(text);
        self.output.set(Output::default());
        self.convert_anyway.set_value(false);
        if self.phase.get_untracked() == Phase::Paused {
            self.phase.set(Phase::Current);
        }
    }

    /// Load the state the address describes. Called on opening the page and
    /// on `hashchange` (a hand edit, Back, or Forward); neither writes the
    /// address.
    pub fn load_from_address(&self) {
        let (state, notice) = match codec::decode(&address::fragment()) {
            Decoded::State(state) => (state, None),
            Decoded::NoState => (default_state(), None),
            Decoded::Invalid => (default_state(), Some(AddressNotice::InvalidLink)),
        };
        self.settings.set(state.settings);
        self.show(state.direction, &state.input);
        self.address_notice.set(notice);
        self.message.set(None);
        self.share_fallback.set(None);
        self.convert();
    }
}

fn default_state() -> State {
    State {
        input: presets::default_example().to_string(),
        ..State::default()
    }
}

fn now() -> f64 {
    window()
        .performance()
        .map(|performance| performance.now())
        .unwrap_or_default()
}

fn convert(direction: Direction, input: String, settings: &Settings) -> Result<Output, String> {
    let start = now();
    let output = match direction {
        Direction::HtmlToMarkdown => engine::html_to_markdown(&input, settings)?,
        Direction::MarkdownToHtml => engine::markdown_to_html(&input, settings)?,
    };
    let millis = now() - start;
    let (rendered_html, original_html) = match direction {
        Direction::HtmlToMarkdown => (engine::markdown_to_html(&output, settings)?, input),
        Direction::MarkdownToHtml => (output.clone(), String::new()),
    };
    Ok(Output {
        chars: output.chars().count(),
        source: output,
        rendered_html,
        original_html,
        millis: Some(millis),
    })
}
