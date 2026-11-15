//! HTML → HTML canonicalization backing the FR-5 pipeline fidelity check.

mod canonicalize;

// The fidelity-panel task that consumes this re-export is the module's only
// customer; until it exists, `-D warnings` sees it as unused.
#[allow(unused_imports)]
pub use canonicalize::canonicalize;
