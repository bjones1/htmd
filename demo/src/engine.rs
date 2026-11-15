//! FR-2 live adaptive debounce: the state machine deciding *when* a
//! conversion runs and *which* run's result commits. The engine never
//! converts anything itself -- conversion results and iframe load events
//! arrive from the caller (the wasm/UI layer), which also supplies the
//! [`Clock`] that production code backs with `web_sys` timers.

// The UI task that drives these inputs is this module's only consumer;
// until it exists, `-D warnings` sees the items as dead code.
#![allow(dead_code)]

use std::rc::Rc;

/// FR-2's immediate-conversion ceiling: inputs at or below this many UTF-8
/// bytes skip the debounce and convert on the next animation frame.
const IMMEDIATE_MAX_BYTES: usize = 2_000;

/// FR-2's debounce window, measured from the last edit of a large input.
const DEBOUNCE_MS: u32 = 250;

/// FR-2's forced deadline, measured from the first edit of the current
/// batch ("the first edit not yet included in a dispatched run").
const FORCED_DEADLINE_MS: u64 = 900;

/// FR-7's client-side processing ceiling. Over it conversion halts; the
/// banner and the explicit output clear belong to [`Engine::on_limit_entered`].
const MAX_INPUT_BYTES: usize = 100_000;

/// Time source and timer registry. Production code wraps `web_sys`
/// timers; tests use a manual clock.
pub trait Clock {
    /// Milliseconds since an arbitrary origin.
    fn now_ms(&self) -> u64;
    /// Fire `event` once, `delay_ms` from now; cancelled tokens never fire.
    fn schedule(&self, delay_ms: u32, event: EngineEvent) -> TimerToken;
    /// Retract a scheduled event.
    fn cancel(&self, token: TimerToken);
}

/// Opaque handle to a scheduled [`Clock`] event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerToken(u64);

/// Timer-fired events re-entering the engine via [`Engine::on_event`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineEvent {
    /// The next-animation-frame tick of the ≤ 2 kB immediate path.
    Immediate,
    /// FR-2's 250 ms debounce elapsed since the last edit.
    Debounce,
    /// FR-2's forced deadline elapsed for the pending batch.
    Deadline,
}

/// The two conversion modes of FR-2's input isolation, each with its own
/// buffer and output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionMode {
    MarkdownToHtml,
    HtmlToMarkdown,
}

impl ConversionMode {
    fn opposite(self) -> Self {
        match self {
            ConversionMode::MarkdownToHtml => ConversionMode::HtmlToMarkdown,
            ConversionMode::HtmlToMarkdown => ConversionMode::MarkdownToHtml,
        }
    }
}

/// What the engine tells the UI to do; actions apply to the active mode's
/// output pane at the time the caller executes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Run the conversion now (synchronously) for sequence number `seq`,
    /// then report back via [`Engine::on_conversion_ready`].
    DispatchConversion { seq: u64 },
    /// Clear the preview, Source text, and fidelity panel (FR-2: `""` is
    /// assigned to `iframe.srcdoc` without waiting for a load event).
    ClearOutput,
}

/// FR-2's commit gate: the run's sequence number either still matches and
/// the output is written, or the run is stale and writes nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommitOutcome {
    /// `seq` is current: write preview, Source, and fidelity together.
    Commit,
    /// The sequence moved on: write nothing.
    Stale,
}

struct ModeState {
    buffer: String,
    /// FR-2 output validity: unchanged buffer and options since the commit
    /// that produced the output.
    output_valid: bool,
}

/// Timers of FR-2's pending batch. Dispatch or cancellation clears them;
/// the next edit starts a new batch.
struct Batch {
    first_edit_ms: u64,
    immediate: Option<TimerToken>,
    debounce: Option<TimerToken>,
    deadline: Option<TimerToken>,
}

impl Batch {
    fn immediate(now_ms: u64, clock: &dyn Clock) -> Self {
        Batch {
            first_edit_ms: now_ms,
            immediate: Some(clock.schedule(0, EngineEvent::Immediate)),
            debounce: None,
            deadline: None,
        }
    }

    fn debounced(now_ms: u64, clock: &dyn Clock) -> Self {
        Batch {
            first_edit_ms: now_ms,
            immediate: None,
            debounce: Some(clock.schedule(DEBOUNCE_MS, EngineEvent::Debounce)),
            deadline: Some(clock.schedule(FORCED_DEADLINE_MS as u32, EngineEvent::Deadline)),
        }
    }
}

/// FR-2 scheduling engine. Pure state: it advances sequence numbers, owns
/// the debounce/deadline timers of the pending batch, tracks the single
/// in-flight and single queued run, and gates commits by sequence number.
pub struct Engine {
    clock: Rc<dyn Clock>,
    seq: u64,
    mode: ConversionMode,
    modes: [ModeState; 2],
    batch: Option<Batch>,
    /// FR-2: a run is in flight from dispatch until the iframe fires `load`
    /// for that commit.
    in_flight: Option<u64>,
    /// FR-2: at most one queued run, latest-input-wins.
    queued: Option<u64>,
    /// Sequence number of the current commit; anchors iframe load matching.
    committed: Option<u64>,
}

impl Engine {
    pub fn new(clock: Rc<dyn Clock>) -> Self {
        Engine {
            clock,
            seq: 0,
            mode: ConversionMode::MarkdownToHtml,
            modes: std::array::from_fn(|_| ModeState {
                buffer: String::new(),
                output_valid: false,
            }),
            batch: None,
            in_flight: None,
            queued: None,
            committed: None,
        }
    }

    /// The current FR-2 sequence number.
    pub fn seq(&self) -> u64 {
        self.seq
    }

    /// The mode whose buffer edits and options currently apply to.
    pub fn mode(&self) -> ConversionMode {
        self.mode
    }

    /// FR-2 "every buffer change (typing, ...)": advance the sequence and
    /// classify the input adaptively by UTF-8 byte length -- ≤ 2 kB
    /// converts on the next animation frame, larger inputs debounce for
    /// 250 ms under the 900 ms forced deadline. Empty input clears the
    /// output immediately (§8.6) without dispatching or arming timers.
    pub fn on_edit(&mut self, text: &str) -> Vec<Action> {
        let active = self.active_index();
        self.modes[active].buffer = text.to_owned();
        self.input_trigger()
    }

    /// FR-2 buffer overwrite (sample load, Reload Sample, Undo, "Use
    /// Output as Input"): write `mode`'s buffer, make it the active mode,
    /// cancel the pending batch's timers, and bypass the debounce -- the
    /// fresh buffer converts on the next animation frame whatever its size.
    pub fn on_buffer_overwrite(&mut self, mode: ConversionMode, text: &str) -> Vec<Action> {
        self.mode = mode;
        let target = index_of(mode);
        self.modes[target].buffer = text.to_owned();
        self.modes[target].output_valid = false;
        self.immediate_trigger()
    }

    /// FR-2 "every option change" (ordinary sidebar toggles and
    /// `skip_tags` text edits): advance the sequence, invalidate the
    /// inactive mode's output, and reconvert the active mode on the same
    /// adaptive schedule as an edit of its buffer. The Reset button is not
    /// this input -- FR-4 makes it immediate ([`Engine::on_reset`]).
    pub fn on_options_change(&mut self) -> Vec<Action> {
        let inactive = index_of(self.mode.opposite());
        self.modes[inactive].output_valid = false;
        self.input_trigger()
    }

    /// FR-4 Reset (demo-spec.md:113): an option change "followed by
    /// immediate reconversion". Advances the sequence and invalidates
    /// outputs like any option change, but bypasses the FR-2 adaptive
    /// debounce -- the active buffer reconverts on the next animation
    /// frame whatever its size class.
    pub fn on_reset(&mut self) -> Vec<Action> {
        let inactive = index_of(self.mode.opposite());
        self.modes[inactive].output_valid = false;
        self.immediate_trigger()
    }

    /// FR-2 mode switch: advance the sequence, cancel both timers of the
    /// pending batch so it never commits into the other mode's pane, and
    /// apply output validity -- a valid target shows its output unchanged
    /// and converts nothing; an invalid target is cleared and its own
    /// buffer converts immediately.
    pub fn on_mode_switch(&mut self) -> Vec<Action> {
        self.advance_seq();
        self.cancel_batch();
        self.mode = self.mode.opposite();
        let target = self.active_index();
        if self.modes[target].output_valid {
            return Vec::new();
        }
        let len = self.modes[target].buffer.len();
        if len == 0 || len > MAX_INPUT_BYTES {
            return self.clear_active_output();
        }
        let clock = Rc::clone(&self.clock);
        self.batch = Some(Batch::immediate(clock.now_ms(), &*clock));
        vec![Action::ClearOutput]
    }

    /// FR-2 "entering an FR-7 limit": advance the sequence, cancel pending
    /// timers and batch state, and clear the output. The banner is Task 7's.
    pub fn on_limit_entered(&mut self) -> Vec<Action> {
        self.advance_seq();
        self.clear_active_output()
    }

    /// A [`Clock`] timer fired. A live batch timer either dispatches the
    /// conversion or, if a current run is still in flight, coalesces into
    /// FR-2's single queued run. Events for cancelled or stale timers are
    /// ignored.
    pub fn on_event(&mut self, event: EngineEvent) -> Vec<Action> {
        let role_live = match (&self.batch, event) {
            (Some(batch), EngineEvent::Immediate) => batch.immediate.is_some(),
            (Some(batch), EngineEvent::Debounce) => batch.debounce.is_some(),
            (Some(batch), EngineEvent::Deadline) => batch.deadline.is_some(),
            (None, _) => false,
        };
        if !role_live {
            return Vec::new();
        }
        if self.in_flight == Some(self.seq) {
            self.queued = Some(self.seq);
            self.cancel_batch();
            return Vec::new();
        }
        // A stale in-flight run was already released by `advance_seq`; the
        // slot is free, so dispatch without waiting for its load event.
        let seq = self.seq;
        self.dispatch_now(seq)
    }

    /// FR-2 commit gate: one synchronous step writing preview, Source, and
    /// fidelity together, or nothing if the run is stale.
    pub fn on_conversion_ready(&mut self, seq: u64) -> CommitOutcome {
        if seq == self.seq && self.in_flight == Some(seq) {
            self.committed = Some(seq);
            self.modes[self.active_index()].output_valid = true;
            CommitOutcome::Commit
        } else {
            CommitOutcome::Stale
        }
    }

    /// FR-2: the iframe fired `load`. It ends the in-flight run only when
    /// it matches the current commit (stale loads are ignored) and then
    /// dispatches the queued run, if any.
    pub fn on_iframe_load(&mut self, seq: u64) -> Vec<Action> {
        if self.in_flight == Some(seq) && self.committed == Some(seq) {
            self.in_flight = None;
            if let Some(queued) = self.queued.take() {
                return self.dispatch_now(queued);
            }
        }
        Vec::new()
    }

    fn active_index(&self) -> usize {
        index_of(self.mode)
    }

    fn advance_seq(&mut self) {
        self.seq += 1;
        // FR-2 sequence rule: discarding older in-flight and queued runs
        // releases the in-flight slot immediately, so newly triggered runs
        // dispatch without waiting for an obsolete load event.
        if self.in_flight.is_some_and(|s| s < self.seq) {
            self.in_flight = None;
        }
        if self.queued.is_some_and(|s| s < self.seq) {
            self.queued = None;
        }
    }

    /// Shared pipeline of the seq-advancing input triggers (edits, option
    /// changes) after the buffer mirror was updated.
    fn input_trigger(&mut self) -> Vec<Action> {
        self.advance_seq();
        let active = self.active_index();
        self.modes[active].output_valid = false;
        let len = self.modes[active].buffer.len();
        if len == 0 {
            return self.clear_active_output();
        }
        if len > MAX_INPUT_BYTES {
            // FR-7: conversion halts over the ceiling; the pending batch
            // must not dispatch. The banner and output clear arrive via
            // `on_limit_entered`.
            self.cancel_batch();
            return Vec::new();
        }
        self.schedule_batch();
        Vec::new()
    }

    /// Shared pipeline of the immediate (non-debounced) triggers -- FR-2
    /// buffer overwrites and FR-4 Reset -- after the buffer mirror was
    /// updated: advance the sequence and dispatch on the next animation
    /// frame, replacing any pending batch whatever its size class.
    fn immediate_trigger(&mut self) -> Vec<Action> {
        self.advance_seq();
        let active = self.active_index();
        self.modes[active].output_valid = false;
        let len = self.modes[active].buffer.len();
        if len == 0 {
            return self.clear_active_output();
        }
        self.cancel_batch();
        if len > MAX_INPUT_BYTES {
            // FR-7: conversion halts over the ceiling (see `input_trigger`).
            return Vec::new();
        }
        let clock = Rc::clone(&self.clock);
        self.batch = Some(Batch::immediate(clock.now_ms(), &*clock));
        Vec::new()
    }

    /// FR-2 adaptive classification of the active buffer: ≤ 2 kB rides the
    /// next animation frame; larger input debounces 250 ms after the last
    /// edit under the forced deadline measured from the batch's first
    /// edit. Further edits reset only the 250 ms timer, never the deadline.
    fn schedule_batch(&mut self) {
        let clock = Rc::clone(&self.clock);
        let now = clock.now_ms();
        let small = self.modes[self.active_index()].buffer.len() <= IMMEDIATE_MAX_BYTES;
        if small {
            match &mut self.batch {
                Some(batch) if batch.immediate.is_some() => {}
                Some(batch) => {
                    if let Some(token) = batch.debounce.take() {
                        clock.cancel(token);
                    }
                    if let Some(token) = batch.deadline.take() {
                        clock.cancel(token);
                    }
                    batch.immediate = Some(clock.schedule(0, EngineEvent::Immediate));
                }
                None => self.batch = Some(Batch::immediate(now, &*clock)),
            }
        } else {
            match &mut self.batch {
                Some(batch) => {
                    if let Some(token) = batch.immediate.take() {
                        clock.cancel(token);
                    }
                    if let Some(token) = batch.debounce.take() {
                        clock.cancel(token);
                    }
                    if batch.deadline.is_none() {
                        let elapsed = now.saturating_sub(batch.first_edit_ms);
                        let delay = FORCED_DEADLINE_MS.saturating_sub(elapsed) as u32;
                        batch.deadline = Some(clock.schedule(delay, EngineEvent::Deadline));
                    }
                    batch.debounce = Some(clock.schedule(DEBOUNCE_MS, EngineEvent::Debounce));
                }
                None => self.batch = Some(Batch::debounced(now, &*clock)),
            }
        }
    }

    fn dispatch_now(&mut self, seq: u64) -> Vec<Action> {
        self.in_flight = Some(seq);
        // FR-2: dispatch clears both timers of that batch; the next edit
        // starts a new batch.
        self.cancel_batch();
        vec![Action::DispatchConversion { seq }]
    }

    fn cancel_batch(&mut self) {
        if let Some(batch) = self.batch.take() {
            if let Some(token) = batch.immediate {
                self.clock.cancel(token);
            }
            if let Some(token) = batch.debounce {
                self.clock.cancel(token);
            }
            if let Some(token) = batch.deadline {
                self.clock.cancel(token);
            }
        }
    }

    fn clear_active_output(&mut self) -> Vec<Action> {
        self.cancel_batch();
        self.modes[self.active_index()].output_valid = false;
        // `""` goes to iframe.srcdoc without waiting for a load event, so
        // no commit anchor remains for load matching.
        self.committed = None;
        vec![Action::ClearOutput]
    }
}

fn index_of(mode: ConversionMode) -> usize {
    match mode {
        ConversionMode::MarkdownToHtml => 0,
        ConversionMode::HtmlToMarkdown => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeMap;

    /// Manual-time [`Clock`]: `advance` moves time, `pop_due` yields due
    /// events in (deadline, insertion) order, cancelled timers vanish.
    struct FakeClock {
        inner: RefCell<Inner>,
    }

    struct Inner {
        now: u64,
        next_id: u64,
        timers: BTreeMap<(u64, u64), (u64, EngineEvent)>,
    }

    impl FakeClock {
        fn new() -> Self {
            FakeClock {
                inner: RefCell::new(Inner {
                    now: 0,
                    next_id: 0,
                    timers: BTreeMap::new(),
                }),
            }
        }

        fn advance(&self, ms: u64) {
            self.inner.borrow_mut().now += ms;
        }

        fn pop_due(&self) -> Option<EngineEvent> {
            let mut inner = self.inner.borrow_mut();
            let key = *inner.timers.keys().find(|(at, _)| *at <= inner.now)?;
            inner.timers.remove(&key).map(|(_, event)| event)
        }

        fn live_timers(&self) -> usize {
            self.inner.borrow().timers.len()
        }
    }

    impl Clock for FakeClock {
        fn now_ms(&self) -> u64 {
            self.inner.borrow().now
        }

        fn schedule(&self, delay_ms: u32, event: EngineEvent) -> TimerToken {
            let mut inner = self.inner.borrow_mut();
            inner.next_id += 1;
            let id = inner.next_id;
            let at = inner.now + u64::from(delay_ms);
            inner.timers.insert((at, id), (id, event));
            TimerToken(id)
        }

        fn cancel(&self, token: TimerToken) {
            self.inner
                .borrow_mut()
                .timers
                .retain(|_, (id, _)| *id != token.0);
        }
    }

    struct Harness {
        clock: Rc<FakeClock>,
        engine: Engine,
        now: u64,
        suspended: Vec<u64>,
    }

    impl Harness {
        fn new() -> Self {
            let clock = Rc::new(FakeClock::new());
            Harness {
                engine: Engine::new(clock.clone()),
                clock,
                now: 0,
                suspended: Vec::new(),
            }
        }

        fn advance(&mut self, ms: u64) -> Vec<Action> {
            self.now += ms;
            self.clock.advance(ms);
            let mut actions = Vec::new();
            while let Some(event) = self.clock.pop_due() {
                actions.extend(self.engine.on_event(event));
            }
            actions
        }

        fn dispatch_seqs(actions: &[Action]) -> Vec<u64> {
            actions
                .iter()
                .filter_map(|a| match a {
                    Action::DispatchConversion { seq } => Some(*seq),
                    _ => None,
                })
                .collect()
        }

        /// §8.6's test-only suspension point: hold the dispatched run for
        /// `seq` before its commit, mirroring the wasm hook's async delay.
        fn suspend(&mut self, seq: u64) {
            self.suspended.push(seq);
        }

        fn resume(&mut self, seq: u64) -> CommitOutcome {
            self.suspended.retain(|s| *s != seq);
            self.engine.on_conversion_ready(seq)
        }

        fn big_text() -> String {
            "x".repeat(2_500)
        }
    }

    #[test]
    fn immediate_path_dispatches_on_next_frame() {
        let mut h = Harness::new();
        assert!(h.engine.on_edit("hello").is_empty());
        // The forced deadline never applies to the ≤ 2 kB immediate path:
        // exactly one timer is armed.
        assert_eq!(h.clock.live_timers(), 1);
        let actions = h.advance(0);
        assert_eq!(Harness::dispatch_seqs(&actions), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        assert!(h.engine.on_iframe_load(1).is_empty());
        assert_eq!(h.clock.live_timers(), 0);
    }

    #[test]
    fn rapid_small_edits_coalesce_into_one_dispatch() {
        let mut h = Harness::new();
        h.engine.on_edit("a");
        h.engine.on_edit("ab");
        h.engine.on_edit("abc");
        let actions = h.advance(0);
        assert_eq!(Harness::dispatch_seqs(&actions), vec![3]);
        assert_eq!(h.engine.on_conversion_ready(3), CommitOutcome::Commit);
    }

    #[test]
    fn debounce_fires_250ms_after_typing_stops() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text());
        assert_eq!(h.clock.live_timers(), 2);
        assert!(h.advance(249).is_empty());
        let actions = h.advance(1);
        assert_eq!(Harness::dispatch_seqs(&actions), vec![1]);
    }

    #[test]
    fn further_edits_reset_only_the_250ms_timer() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text()); // t=0: debounce@250, deadline@900
        assert!(h.advance(200).is_empty());
        h.engine.on_edit(&Harness::big_text()); // t=200: debounce→450
        assert!(h.advance(50).is_empty()); // t=250: cancelled timer stays silent
        assert!(h.advance(199).is_empty()); // t=449
        let actions = h.advance(1); // t=450
        assert_eq!(Harness::dispatch_seqs(&actions), vec![2]);
        // The 900 ms deadline was cleared by the dispatch, never extended.
        assert!(h.advance(450).is_empty());
        assert_eq!(h.clock.live_timers(), 0);
    }

    #[test]
    fn forced_deadline_dispatches_by_900ms_under_continuous_edits() {
        let mut h = Harness::new();
        let text = Harness::big_text();
        let mut dispatches = Vec::new();
        for _ in 0..30 {
            // edits at t=0..=2900, every 100 ms, for three seconds
            h.engine.on_edit(&text);
            for seq in Harness::dispatch_seqs(&h.advance(100)) {
                dispatches.push((h.now, seq));
                assert_eq!(h.engine.on_conversion_ready(seq), CommitOutcome::Commit);
                h.engine.on_iframe_load(seq);
            }
        }
        // Every batch dispatches exactly at its 900 ms forced deadline,
        // carrying the latest sequence number.
        assert_eq!(dispatches, vec![(900, 9), (1800, 18), (2700, 27)]);
    }

    #[test]
    fn dispatch_clears_both_batch_timers() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text());
        assert_eq!(Harness::dispatch_seqs(&h.advance(250)), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        // The unexpired 900 ms deadline was cleared: no double dispatch.
        assert!(h.advance(650).is_empty());
        assert_eq!(h.clock.live_timers(), 0);
    }

    #[test]
    fn suspended_run_writes_nothing_once_sequence_advanced() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text());
        assert_eq!(Harness::dispatch_seqs(&h.advance(250)), vec![1]);
        h.suspend(1); // §8.6: hold run 1 before its commit
        h.engine.on_edit(&Harness::big_text()); // t=250: newer input, seq→2
        assert_eq!(Harness::dispatch_seqs(&h.advance(250)), vec![2]);
        assert_eq!(h.engine.on_conversion_ready(2), CommitOutcome::Commit);
        assert_eq!(h.resume(1), CommitOutcome::Stale); // resumes, writes nothing
        assert!(h.engine.on_iframe_load(2).is_empty());
    }

    #[test]
    fn latest_input_during_suspension_runs_once_and_commits() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text());
        assert_eq!(Harness::dispatch_seqs(&h.advance(250)), vec![1]);
        h.suspend(1);
        h.engine.on_edit(&Harness::big_text()); // t=250
        h.advance(10);
        h.engine.on_edit(&Harness::big_text()); // t=260: debounce→510
        let actions = h.advance(250); // t=510
        assert_eq!(Harness::dispatch_seqs(&actions), vec![3]);
        assert_eq!(h.engine.on_conversion_ready(3), CommitOutcome::Commit);
        assert_eq!(h.resume(1), CommitOutcome::Stale);
        assert_eq!(h.clock.live_timers(), 0); // no additional runs accumulate
    }

    #[test]
    fn size_limit_crossing_during_suspension_does_not_refill_cleared_output() {
        let mut h = Harness::new();
        h.engine.on_edit("small");
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![1]);
        h.suspend(1);
        let oversized = "x".repeat(100_001);
        assert!(h.engine.on_edit(&oversized).is_empty()); // no dispatch armed
        let actions = h.engine.on_limit_entered(); // FR-7 crossing clears output
        assert_eq!(actions, vec![Action::ClearOutput]);
        assert!(h.advance(5_000).is_empty()); // conversion stays halted
        assert_eq!(h.resume(1), CommitOutcome::Stale); // no refill on resume
        h.engine.on_edit("back under");
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![4]); // resumes
    }

    #[test]
    fn mode_switch_cancels_pending_batch_timers() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text());
        assert_eq!(h.clock.live_timers(), 2);
        let actions = h.engine.on_mode_switch(); // target buffer empty: clear only
        assert_eq!(actions, vec![Action::ClearOutput]);
        assert_eq!(h.clock.live_timers(), 0);
        assert!(h.advance(1_000).is_empty()); // the batch never commits anywhere
    }

    #[test]
    fn switch_to_valid_mode_shows_output_and_converts_nothing() {
        let mut h = Harness::new();
        h.engine.on_edit("hello");
        let actions = h.advance(0);
        assert_eq!(Harness::dispatch_seqs(&actions), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        h.engine.on_iframe_load(1);
        assert_eq!(
            h.engine.on_mode_switch(),
            vec![Action::ClearOutput] // to HtmlToMarkdown, never used
        );
        let back = h.engine.on_mode_switch(); // MarkdownToHtml output still valid
        assert!(back.is_empty());
        assert!(h.advance(1_000).is_empty());
    }

    #[test]
    fn option_change_invalidates_inactive_mode_reconverted_on_switch() {
        let mut h = Harness::new();
        h.engine.on_edit("hello"); // MarkdownToHtml buffer
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        h.engine.on_iframe_load(1);
        h.engine
            .on_buffer_overwrite(ConversionMode::HtmlToMarkdown, &Harness::big_text());
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![2]); // no debounce
        assert_eq!(h.engine.on_conversion_ready(2), CommitOutcome::Commit);
        h.engine.on_iframe_load(2);
        // Option change reconverts the active mode (HtmlToMarkdown, > 2 kB).
        h.engine.on_options_change();
        assert_eq!(Harness::dispatch_seqs(&h.advance(250)), vec![3]);
        assert_eq!(h.engine.on_conversion_ready(3), CommitOutcome::Commit);
        h.engine.on_iframe_load(3);
        // Switching to MarkdownToHtml: invalidated by the option change, so
        // it is cleared and immediately reconverts its own buffer.
        let actions = h.engine.on_mode_switch();
        assert_eq!(actions, vec![Action::ClearOutput]);
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![4]);
        assert_eq!(h.engine.on_conversion_ready(4), CommitOutcome::Commit);
    }

    #[test]
    fn reset_on_large_buffer_dispatches_immediately() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text()); // arms debounce@250 + deadline@900
        assert_eq!(h.clock.live_timers(), 2);
        assert!(h.engine.on_reset().is_empty());
        assert_eq!(h.clock.live_timers(), 1); // both replaced by a next-frame timer
        let actions = h.advance(0); // no 250 ms wait, per FR-4:113
        assert_eq!(Harness::dispatch_seqs(&actions), vec![2]);
        assert_eq!(h.engine.on_conversion_ready(2), CommitOutcome::Commit);
    }

    #[test]
    fn option_change_on_large_buffer_still_debounces() {
        let mut h = Harness::new();
        h.engine.on_edit(&Harness::big_text()); // seq 1, debounce@250
        h.engine.on_options_change(); // seq 2, debounce reset to 250
        assert_eq!(h.clock.live_timers(), 2); // debounce + deadline, no frame timer
        assert!(h.advance(0).is_empty()); // the split is real, not a blanket bypass
        let actions = h.advance(250);
        assert_eq!(Harness::dispatch_seqs(&actions), vec![2]);
    }

    #[test]
    fn empty_input_clears_output_without_dispatch() {
        let mut h = Harness::new();
        h.engine.on_edit("hello");
        let actions = h.advance(0);
        assert_eq!(Harness::dispatch_seqs(&actions), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        h.engine.on_iframe_load(1);
        assert_eq!(h.engine.on_edit(""), vec![Action::ClearOutput]);
        assert_eq!(h.clock.live_timers(), 0);
        assert!(h.advance(1_000).is_empty());
        assert!(h.engine.on_iframe_load(1).is_empty()); // stale load after clear
    }

    #[test]
    fn buffer_overwrite_bypasses_debounce() {
        let mut h = Harness::new();
        let actions = h
            .engine
            .on_buffer_overwrite(ConversionMode::MarkdownToHtml, &Harness::big_text());
        assert!(actions.is_empty());
        assert_eq!(h.clock.live_timers(), 1); // next frame only, no 250 ms wait
        let fired = h.advance(0);
        assert_eq!(Harness::dispatch_seqs(&fired), vec![1]);
    }

    #[test]
    fn oversized_input_schedules_no_conversion() {
        let mut h = Harness::new();
        let oversized = "x".repeat(100_001);
        assert!(h.engine.on_edit(&oversized).is_empty());
        assert_eq!(h.clock.live_timers(), 0);
        assert!(h.advance(5_000).is_empty());
    }

    #[test]
    fn stale_iframe_load_is_ignored() {
        let mut h = Harness::new();
        h.engine.on_edit("hello");
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        h.engine.on_edit(&Harness::big_text()); // seq→2 releases run 1's slot
        assert_eq!(Harness::dispatch_seqs(&h.advance(250)), vec![2]);
        assert_eq!(h.engine.on_conversion_ready(2), CommitOutcome::Commit);
        assert!(h.engine.on_iframe_load(1).is_empty()); // old write's load
        assert_eq!(h.engine.in_flight, Some(2)); // run 2 unaffected
        assert!(h.engine.on_iframe_load(2).is_empty());
        assert_eq!(h.engine.in_flight, None);
    }

    #[test]
    fn forced_tick_while_run_in_flight_coalesces_into_single_queued_run() {
        let mut h = Harness::new();
        h.engine.on_edit("hello");
        assert_eq!(Harness::dispatch_seqs(&h.advance(0)), vec![1]);
        assert_eq!(h.engine.on_conversion_ready(1), CommitOutcome::Commit);
        // White-box: run 1 still holds the slot (its load has not fired)
        // while a pending batch's timers come due. Public inputs cannot
        // produce this -- every trigger advances the sequence and releases
        // the slot -- but FR-2 mandates coalescing for the production
        // wiring, where load timing comes from the real iframe.
        let clock = h.clock.clone();
        h.engine.batch = Some(Batch {
            first_edit_ms: h.now,
            immediate: None,
            debounce: Some(clock.schedule(10, EngineEvent::Debounce)),
            deadline: None,
        });
        assert!(h.advance(10).is_empty()); // queued, not dispatched
        assert_eq!(h.engine.queued, Some(1));
        h.engine.batch = Some(Batch {
            first_edit_ms: h.now,
            immediate: None,
            debounce: None,
            deadline: Some(clock.schedule(10, EngineEvent::Deadline)),
        });
        assert!(h.advance(10).is_empty()); // the forced tick coalesces too
        assert_eq!(h.engine.queued, Some(1));
        let actions = h.engine.on_iframe_load(1); // run finished → queued runs
        assert_eq!(Harness::dispatch_seqs(&actions), vec![1]);
        assert_eq!(h.engine.queued, None);
    }
}
