//! Word-level diff over canonicalized HTML strings for the FR-5 fidelity
//! panel.

use similar::{ChangeTag, DiffableStr, TextDiff};

/// §6:213 — after the common token prefix and suffix are stripped, a
/// middle longer than 5,000 tokens on either side skips the diff ("Diff
/// too large to display").
const MAX_MIDDLE_TOKENS: usize = 5_000;

/// Outcome of diffing the two canonicalized strings.
#[derive(Debug, Clone, PartialEq)]
pub enum WordDiff {
    /// The inputs are equal after word tokenization (FR-5:131).
    Inclusive,
    /// Either middle exceeds 5,000 tokens, so the diff is skipped and the
    /// panel shows both canonical strings instead (§6:213).
    TooLarge,
    /// The changed middle as interleaved tokens, with the common token
    /// prefix and suffix already stripped. Never empty.
    Changes(Vec<DiffToken>),
}

/// One token of the changed middle: shared text, text only in the
/// original, or text only in the round-tripped output.
#[derive(Debug, Clone, PartialEq)]
pub enum DiffToken {
    /// Present in both inputs.
    Equal(String),
    /// Present only in the original.
    Removed(String),
    /// Present only in the round-tripped output.
    Inserted(String),
}

/// Word-level diff of two canonicalized HTML strings (FR-5:131): tokenize
/// both sides with the same `similar` word tokenizer, strip the longest
/// common token prefix and suffix, skip the diff when either middle
/// exceeds 5,000 tokens (§6:213), and otherwise return the middle's
/// tokens. No `similar` deadline is configured: `std::time::Instant`
/// panics on `wasm32-unknown-unknown`.
// The fidelity-panel task that calls this is the module's only consumer;
// until it exists, `-D warnings` sees it as dead code.
#[allow(dead_code)]
pub fn word_diff(original: &str, roundtripped: &str) -> WordDiff {
    let old = original.tokenize_words();
    let new = roundtripped.tokenize_words();

    // Longest common token prefix and suffix, by token content; the two
    // pointers may meet but never cross, so both middles are well-formed.
    let min_len = old.len().min(new.len());
    let mut prefix = 0;
    while prefix < min_len && old[prefix] == new[prefix] {
        prefix += 1;
    }
    let mut suffix = 0;
    while suffix < min_len - prefix && old[old.len() - 1 - suffix] == new[new.len() - 1 - suffix] {
        suffix += 1;
    }

    let old_middle = &old[prefix..old.len() - suffix];
    let new_middle = &new[prefix..new.len() - suffix];

    if old_middle.len() > MAX_MIDDLE_TOKENS || new_middle.len() > MAX_MIDDLE_TOKENS {
        return WordDiff::TooLarge;
    }
    if old_middle.is_empty() && new_middle.is_empty() {
        return WordDiff::Inclusive;
    }

    // A maximal strip cannot leave both middles nonempty and fully equal
    // (the prefix would have continued), so the result always carries at
    // least one Removed or Inserted token.
    let tokens = TextDiff::from_slices(old_middle, new_middle)
        .iter_all_changes()
        .map(|change| match change.tag() {
            ChangeTag::Equal => DiffToken::Equal(change.value().to_string()),
            ChangeTag::Delete => DiffToken::Removed(change.value().to_string()),
            ChangeTag::Insert => DiffToken::Inserted(change.value().to_string()),
        })
        .collect();
    WordDiff::Changes(tokens)
}

#[cfg(test)]
mod tests {
    use super::super::canonicalize;
    use super::DiffToken::{Equal, Inserted, Removed};
    use super::WordDiff;
    use super::word_diff;

    /// FR-5:131 — tokenization-equal inputs produce no changes.
    #[test]
    fn identical_strings_are_inclusive() {
        assert_eq!(word_diff("", ""), WordDiff::Inclusive);
        assert_eq!(word_diff("alpha beta", "alpha beta"), WordDiff::Inclusive);
    }

    /// §8.7:298/300 — forms differing only by safe variations canonicalize
    /// to the same string, so the diff sees no changes.
    #[test]
    fn canonically_equivalent_forms_are_inclusive() {
        assert_eq!(
            word_diff(
                &canonicalize(r#"<P CLASS="b a">x</P>"#),
                &canonicalize(r#"<p class="a b">x</p>"#),
            ),
            WordDiff::Inclusive
        );
    }

    /// FR-5:131 — a single replaced word is exactly one Removed and one
    /// Inserted token; the shared context is stripped as prefix/suffix.
    #[test]
    fn single_word_change_yields_one_removed_and_one_inserted() {
        assert_eq!(
            word_diff("alpha beta gamma", "alpha delta gamma"),
            WordDiff::Changes(vec![Removed("beta".into()), Inserted("delta".into())])
        );
    }

    /// §6:213 — fully divergent inputs whose middles exceed 5,000 tokens
    /// on both sides skip the diff.
    #[test]
    fn fully_divergent_inputs_past_the_cap_are_too_large() {
        let original = "alpha ".repeat(6_000);
        let roundtripped = "omega ".repeat(6_000);
        assert_eq!(word_diff(&original, &roundtripped), WordDiff::TooLarge);
    }

    /// §6:213 — the strip runs before the cap, so a one-word change inside
    /// a 10,000-word shared context diffs instead of being skipped, and
    /// the result stays minimal.
    #[test]
    fn change_inside_large_shared_context_stays_small() {
        let text = |flipped: &str| {
            (0..10_000)
                .map(|i| {
                    if i == 5_000 {
                        flipped.to_string()
                    } else {
                        format!("w{i}")
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert_eq!(
            word_diff(&text("flip"), &text("flop")),
            WordDiff::Changes(vec![Removed("flip".into()), Inserted("flop".into())])
        );
    }

    /// Whitespace runs are tokens (FR-5:131), so a spacing-only difference
    /// changes whitespace tokens only.
    #[test]
    fn whitespace_only_difference_yields_only_whitespace_tokens() {
        assert_eq!(
            word_diff("a b", "a  b"),
            WordDiff::Changes(vec![Removed(" ".into()), Inserted("  ".into())])
        );
    }

    /// Tokens shared inside the changed region survive as Equal context
    /// between the Removed/Inserted runs.
    #[test]
    fn equal_context_inside_the_changed_region_is_kept() {
        assert_eq!(
            word_diff("a x b x c", "a y b y c"),
            WordDiff::Changes(vec![
                Removed("x".into()),
                Inserted("y".into()),
                Equal(" ".into()),
                Equal("b".into()),
                Equal(" ".into()),
                Removed("x".into()),
                Inserted("y".into()),
            ])
        );
    }

    /// §6:213 — the cap trips only when a middle *exceeds* 5,000 tokens;
    /// exactly 5,000 on one side still diffs.
    #[test]
    fn middle_of_exactly_5000_tokens_is_not_skipped() {
        let words = |n: usize| {
            (0..n)
                .map(|i| format!("w{i}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let original = format!(" {}", words(2_500));
        let roundtripped = format!("q {} r", words(2_498));
        match word_diff(&original, &roundtripped) {
            WordDiff::Changes(tokens) => assert!(
                tokens
                    .iter()
                    .any(|token| matches!(token, Removed(_) | Inserted(_)))
            ),
            other => panic!("expected Changes at the 5,000-token boundary, got {other:?}"),
        }
    }
}
