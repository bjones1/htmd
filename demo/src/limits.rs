//! FR-7 limit banners: the state machine deciding when each
//! `aria-live="assertive"` limit banner appears. Banners are announced
//! exactly once per *transition into* a limit state, stay visible while
//! the state persists, and clear when the input returns within the limit.

use crate::convert::Blocked;

/// FR-7 size limit, verbatim: the banner shown when the input exceeds
/// the 100,000-byte ceiling ("Input exceeds the 100,000-byte maximum
/// for client-side processing.").
pub const SIZE_LIMIT_TEXT: &str =
    "Input exceeds the 100,000-byte maximum for client-side processing.";

/// FR-7 nesting limit, verbatim: the banner shown when the input's
/// element nesting exceeds 512 levels ("Input exceeds the supported
/// nesting depth.").
pub const NESTING_LIMIT_TEXT: &str = "Input exceeds the supported nesting depth.";

/// Every banner kind, in the order banners stack in the region.
const ALL: [BannerKind; 2] = [BannerKind::SizeLimit, BannerKind::NestingLimit];

/// One FR-7 limit banner; each kind is an independent child element of
/// the single banner region (§5: concurrent banners stack, each
/// announced once when added).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerKind {
    SizeLimit,
    NestingLimit,
}

impl BannerKind {
    /// The FR-7 banner text, verbatim.
    pub fn text(self) -> &'static str {
        match self {
            BannerKind::SizeLimit => SIZE_LIMIT_TEXT,
            BannerKind::NestingLimit => NESTING_LIMIT_TEXT,
        }
    }

    fn index(self) -> usize {
        match self {
            BannerKind::SizeLimit => 0,
            BannerKind::NestingLimit => 1,
        }
    }
}

impl From<Blocked> for BannerKind {
    /// The UI feeds each [`Blocked`] conversion result straight into
    /// [`Banners::enter`] via this mapping.
    fn from(blocked: Blocked) -> Self {
        match blocked {
            Blocked::TooLarge => BannerKind::SizeLimit,
            Blocked::TooDeep => BannerKind::NestingLimit,
        }
    }
}

/// The FR-7 banner state: one slot per [`BannerKind`]. A banner shows
/// for the whole duration of the limit state and is announced exactly
/// once, on the transition into it.
#[derive(Debug, Default)]
pub struct Banners {
    /// Slot holds `true` while its limit state persists.
    active: [bool; 2],
    /// Slot holds `true` from the transition into the limit until the
    /// announcement is drained.
    announced: [bool; 2],
}

impl Banners {
    /// A quiet state: no banners active, nothing pending.
    pub fn new() -> Self {
        Self::default()
    }

    /// A blocked conversion entered the limit state. The first
    /// transition announces; repeats while the state persists do not
    /// (FR-7: "exactly once per transition into the limit", not per
    /// keystroke).
    pub fn enter(&mut self, kind: BannerKind) {
        let i = kind.index();
        if !self.active[i] {
            self.active[i] = true;
            self.announced[i] = true;
        }
    }

    /// The input returned within the limit ("processing resumes
    /// automatically"): the banner clears. An announcement not yet
    /// drained is dropped too -- it would announce a condition that no
    /// longer holds.
    pub fn exit(&mut self, kind: BannerKind) {
        let i = kind.index();
        self.active[i] = false;
        self.announced[i] = false;
    }

    /// Kinds whose banner is currently showing.
    pub fn active(&self) -> impl Iterator<Item = BannerKind> + '_ {
        ALL.into_iter().filter(move |&k| self.active[k.index()])
    }

    /// Kinds that transitioned into the limit since the last drain, in
    /// stack order; the aria-live region announces each exactly once.
    pub fn take_announced(&mut self) -> impl Iterator<Item = BannerKind> + '_ {
        let drained = std::mem::take(&mut self.announced);
        ALL.into_iter().filter(move |&k| drained[k.index()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_limit_text_matches_fr7_verbatim() {
        assert_eq!(
            SIZE_LIMIT_TEXT,
            "Input exceeds the 100,000-byte maximum for client-side processing."
        );
    }

    #[test]
    fn nesting_limit_text_matches_fr7_verbatim() {
        assert_eq!(
            NESTING_LIMIT_TEXT,
            "Input exceeds the supported nesting depth."
        );
    }

    #[test]
    fn banner_kind_text_maps_to_consts() {
        assert_eq!(BannerKind::SizeLimit.text(), SIZE_LIMIT_TEXT);
        assert_eq!(BannerKind::NestingLimit.text(), NESTING_LIMIT_TEXT);
    }

    #[test]
    fn enter_announces_exactly_once_per_transition() {
        let mut banners = Banners::new();
        banners.enter(BannerKind::SizeLimit);
        // Subsequent blocked conversions while the state persists do
        // not re-announce (FR-7: "exactly once per transition").
        banners.enter(BannerKind::SizeLimit);
        banners.enter(BannerKind::SizeLimit);
        assert_eq!(
            banners.take_announced().collect::<Vec<_>>(),
            vec![BannerKind::SizeLimit]
        );
        assert_eq!(banners.take_announced().count(), 0);
        // The banner itself stays visible while the state persists.
        assert_eq!(
            banners.active().collect::<Vec<_>>(),
            vec![BannerKind::SizeLimit]
        );
    }

    #[test]
    fn exit_clears_the_banner() {
        let mut banners = Banners::new();
        banners.enter(BannerKind::NestingLimit);
        assert_eq!(banners.take_announced().count(), 1);
        banners.exit(BannerKind::NestingLimit);
        assert_eq!(banners.active().count(), 0);
    }

    #[test]
    fn reentry_after_exit_announces_again() {
        let mut banners = Banners::new();
        banners.enter(BannerKind::SizeLimit);
        assert_eq!(banners.take_announced().count(), 1);
        banners.exit(BannerKind::SizeLimit);
        assert_eq!(banners.active().count(), 0);
        banners.enter(BannerKind::SizeLimit);
        assert_eq!(
            banners.take_announced().collect::<Vec<_>>(),
            vec![BannerKind::SizeLimit]
        );
        assert_eq!(
            banners.active().collect::<Vec<_>>(),
            vec![BannerKind::SizeLimit]
        );
    }

    #[test]
    fn both_kinds_coexist_independently() {
        let mut banners = Banners::new();
        banners.enter(BannerKind::SizeLimit);
        banners.enter(BannerKind::NestingLimit);
        banners.enter(BannerKind::SizeLimit);
        let announced = banners.take_announced().collect::<Vec<_>>();
        assert_eq!(announced.len(), 2);
        assert!(announced.contains(&BannerKind::SizeLimit));
        assert!(announced.contains(&BannerKind::NestingLimit));
        assert_eq!(
            banners.active().collect::<Vec<_>>(),
            vec![BannerKind::SizeLimit, BannerKind::NestingLimit]
        );
        // Clearing one leaves the other showing.
        banners.exit(BannerKind::SizeLimit);
        assert_eq!(
            banners.active().collect::<Vec<_>>(),
            vec![BannerKind::NestingLimit]
        );
    }

    #[test]
    fn exit_before_drain_drops_the_announcement() {
        let mut banners = Banners::new();
        banners.enter(BannerKind::SizeLimit);
        banners.exit(BannerKind::SizeLimit);
        assert_eq!(banners.take_announced().count(), 0);
        assert_eq!(banners.active().count(), 0);
    }

    #[test]
    fn exit_without_enter_is_a_noop() {
        let mut banners = Banners::new();
        banners.exit(BannerKind::NestingLimit);
        assert_eq!(banners.active().count(), 0);
        assert_eq!(banners.take_announced().count(), 0);
    }

    #[test]
    fn fresh_banners_are_quiet() {
        let mut banners = Banners::new();
        assert_eq!(banners.active().count(), 0);
        assert_eq!(banners.take_announced().count(), 0);
    }

    #[test]
    fn blocked_maps_to_banner_kinds() {
        assert_eq!(BannerKind::from(Blocked::TooLarge), BannerKind::SizeLimit);
        assert_eq!(BannerKind::from(Blocked::TooDeep), BannerKind::NestingLimit);
    }
}
