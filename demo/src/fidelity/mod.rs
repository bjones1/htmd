//! HTML → HTML canonicalization and word-level diff backing the FR-5
//! pipeline fidelity check.

mod canonicalize;
mod diff;

pub use canonicalize::canonicalize;
pub use diff::{DiffToken, WordDiff, word_diff};
