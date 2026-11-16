//! Settings, presets, and the permalink format, shared by the playground's
//! UI (`app`) and its converter (`engine`). The behavior is specified in
//! `pulldown-cmark-htmd-demo-2/htmddemo2-spec.md`.

mod base64;
pub mod codec;
pub mod presets;
pub mod settings;

/// Inputs larger than this (as UTF-8) pause conversion (spec, Limits and
/// failures).
pub const LARGE_INPUT_BYTES: usize = 1 << 20;

/// The longest page address the playground writes (spec, Permalinks).
pub const MAX_ADDRESS_LENGTH: usize = 32_768;
