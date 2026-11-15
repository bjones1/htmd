//! HTML → HTML canonicalization and word-level diff backing the FR-5
//! pipeline fidelity check.

mod canonicalize;
mod diff;

// The fidelity-panel task that consumes these re-exports is the module's only
// customer; until it exists, `-D warnings` sees it as unused.
#[allow(unused_imports)]
pub use canonicalize::canonicalize;
#[allow(unused_imports)]
pub use diff::{DiffToken, WordDiff, word_diff};
