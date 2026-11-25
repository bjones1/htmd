//! The permalink format: the part of the page address after `#`.
//!
//! The fragment is never sent to the server, which is what keeps the input
//! private (spec, Permalinks). It is a list of `key=value` pairs joined by
//! `&`, using only characters which chat apps and mail clients leave alone:
//!
//! | Key | Value |
//! | --- | --- |
//! | `v` | the format version, `1`; always first |
//! | `d` | the direction's [`Choice::code`] |
//! | `i` | the input: raw DEFLATE of its UTF-8, in unpadded base64url; absent when the input is empty or too large to share |
//! | `tm`, `hs`, `hr`, `br`, `bm`, `cb`, `cf`, `ls`, `rs` | an htmd dropdown's [`Choice::code`] |
//! | `bs`, `ns` | spaces after a bullet / list number, `1`–`4` |
//! | `pw`, `nr` | preserve code whitespace / `<noscript>` as raw text, `0` or `1` |
//! | `sk` | the Skip tags field as typed, UTF-8 in unpadded base64url |
//! | `x` | every enabled extension's [`Extension::code`], joined by `.` |
//!
//! Options at their default are left out. Addresses shared today must keep
//! working, so version 1 never changes meaning: a future format gets a new
//! `v`, and the decoder keeps reading this one.
//!
//! Decoding is all-or-nothing. Anything unexpected, such as an unknown or
//! repeated key, makes the whole address invalid.

use std::collections::BTreeMap;

use crate::base64;
use crate::settings::*;

const VERSION: &str = "1";

/// Inputs larger than this when decompressed make an address invalid. It
/// bounds the memory a hand-crafted address can claim.
const MAX_DECODED_INPUT: usize = 64 << 20;

/// The result of reading an address fragment.
#[derive(Debug, PartialEq, Eq)]
pub enum Decoded {
    /// The address carries no saved state; the page shows the default
    /// example.
    NoState,
    /// The address could not be fully read.
    Invalid,
    /// The saved state.
    State(State),
}

/// Encode `state` as a fragment, without the leading `#`. With
/// `include_input` false, the input is left out, so that opening the address
/// restores an empty input.
pub fn encode(state: &State, include_input: bool) -> String {
    let mut pairs = vec![("v", VERSION.to_string())];
    pairs.push(("d", state.direction.code().to_string()));
    if include_input && !state.input.is_empty() {
        let compressed = miniz_oxide::deflate::compress_to_vec(state.input.as_bytes(), 9);
        pairs.push(("i", base64::encode(&compressed)));
    }
    push_settings(&mut pairs, &state.settings);
    join(&pairs)
}

/// Encode only the options, in the same form, for passing settings to the
/// converter.
pub fn encode_settings(settings: &Settings) -> String {
    let mut pairs = Vec::new();
    push_settings(&mut pairs, settings);
    join(&pairs)
}

/// Read a fragment produced by [`encode_settings`].
pub fn decode_settings(text: &str) -> Option<Settings> {
    let mut pairs = parse_pairs(text)?;
    let settings = take_settings(&mut pairs)?;
    pairs.is_empty().then_some(settings)
}

/// Read an address fragment, with or without its leading `#`.
pub fn decode(fragment: &str) -> Decoded {
    let fragment = fragment.strip_prefix('#').unwrap_or(fragment);
    if fragment.is_empty() {
        return Decoded::NoState;
    }
    match decode_state(fragment) {
        Some(state) => Decoded::State(state),
        None => Decoded::Invalid,
    }
}

fn decode_state(fragment: &str) -> Option<State> {
    // Some mail clients and chat apps percent-encode characters in links.
    let fragment = percent_decode(fragment)?;
    let mut pairs = parse_pairs(&fragment)?;
    if pairs.remove("v")? != VERSION {
        return None;
    }
    let direction = Direction::from_code(pairs.remove("d")?)?;
    let input = match pairs.remove("i") {
        Some(encoded) => {
            let compressed = base64::decode(encoded)?;
            let bytes =
                miniz_oxide::inflate::decompress_to_vec_with_limit(&compressed, MAX_DECODED_INPUT)
                    .ok()?;
            String::from_utf8(bytes).ok()?
        }
        None => String::new(),
    };
    let settings = take_settings(&mut pairs)?;
    pairs.is_empty().then_some(State {
        direction,
        input,
        settings,
    })
}

fn push_settings(pairs: &mut Vec<(&'static str, String)>, settings: &Settings) {
    fn choice<T: Choice + Default>(
        pairs: &mut Vec<(&'static str, String)>,
        key: &'static str,
        value: T,
    ) {
        if value != T::default() {
            pairs.push((key, value.code().to_string()));
        }
    }
    let defaults = HtmdOptions::default();
    let htmd = &settings.htmd;
    choice(pairs, "tm", htmd.translation_mode);
    choice(pairs, "hs", htmd.heading_style);
    choice(pairs, "hr", htmd.hr_style);
    choice(pairs, "br", htmd.br_style);
    choice(pairs, "bm", htmd.bullet_marker);
    if htmd.bullet_spacing != defaults.bullet_spacing {
        pairs.push(("bs", htmd.bullet_spacing.to_string()));
    }
    if htmd.number_spacing != defaults.number_spacing {
        pairs.push(("ns", htmd.number_spacing.to_string()));
    }
    choice(pairs, "cb", htmd.code_block_style);
    choice(pairs, "cf", htmd.code_fence);
    choice(pairs, "ls", htmd.link_style);
    choice(pairs, "rs", htmd.reference_style);
    if htmd.preserve_code_whitespace != defaults.preserve_code_whitespace {
        pairs.push(("pw", bool_code(htmd.preserve_code_whitespace).into()));
    }
    if htmd.noscript_as_raw_text != defaults.noscript_as_raw_text {
        pairs.push(("nr", bool_code(htmd.noscript_as_raw_text).into()));
    }
    if !htmd.skip_tags.is_empty() {
        pairs.push(("sk", base64::encode(htmd.skip_tags.as_bytes())));
    }
    if settings.extensions != Extensions::default() {
        let codes: Vec<_> = settings.extensions.iter().map(Extension::code).collect();
        pairs.push(("x", codes.join(".")));
    }
}

fn take_settings(pairs: &mut BTreeMap<&str, &str>) -> Option<Settings> {
    fn choice<T: Choice + Default>(pairs: &mut BTreeMap<&str, &str>, key: &str) -> Option<T> {
        match pairs.remove(key) {
            Some(code) => T::from_code(code),
            None => Some(T::default()),
        }
    }
    fn spacing(pairs: &mut BTreeMap<&str, &str>, key: &str, default: u8) -> Option<u8> {
        match pairs.remove(key) {
            Some(text) => {
                let value: u8 = text.parse().ok()?;
                LIST_SPACING_RANGE.contains(&value).then_some(value)
            }
            None => Some(default),
        }
    }
    fn flag(pairs: &mut BTreeMap<&str, &str>, key: &str, default: bool) -> Option<bool> {
        match pairs.remove(key) {
            Some("0") => Some(false),
            Some("1") => Some(true),
            Some(_) => None,
            None => Some(default),
        }
    }

    let defaults = HtmdOptions::default();
    let htmd = HtmdOptions {
        translation_mode: choice(pairs, "tm")?,
        heading_style: choice(pairs, "hs")?,
        hr_style: choice(pairs, "hr")?,
        br_style: choice(pairs, "br")?,
        bullet_marker: choice(pairs, "bm")?,
        bullet_spacing: spacing(pairs, "bs", defaults.bullet_spacing)?,
        number_spacing: spacing(pairs, "ns", defaults.number_spacing)?,
        code_block_style: choice(pairs, "cb")?,
        code_fence: choice(pairs, "cf")?,
        link_style: choice(pairs, "ls")?,
        reference_style: choice(pairs, "rs")?,
        preserve_code_whitespace: flag(pairs, "pw", defaults.preserve_code_whitespace)?,
        noscript_as_raw_text: flag(pairs, "nr", defaults.noscript_as_raw_text)?,
        skip_tags: match pairs.remove("sk") {
            Some(encoded) => String::from_utf8(base64::decode(encoded)?).ok()?,
            None => String::new(),
        },
    };
    let extensions = match pairs.remove("x") {
        Some(list) => {
            let mut extensions = Extensions::NONE;
            for code in list.split('.').filter(|code| !code.is_empty()) {
                let ext = Extension::from_code(code)?;
                if extensions.contains(ext) {
                    return None;
                }
                extensions.set(ext, true);
            }
            // Old-style footnotes imply footnotes; an address listing only the
            // former was not written by the playground.
            if extensions.contains(Extension::OldFootnotes)
                && !list
                    .split('.')
                    .any(|code| code == Extension::Footnotes.code())
            {
                return None;
            }
            extensions
        }
        None => Extensions::default(),
    };
    Some(Settings { htmd, extensions })
}

fn bool_code(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

fn join(pairs: &[(&str, String)]) -> String {
    pairs
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join("&")
}

/// Split `a=1&b=2` into a map, rejecting a pair without `=` and a repeated
/// key.
fn parse_pairs(text: &str) -> Option<BTreeMap<&str, &str>> {
    let mut pairs = BTreeMap::new();
    if text.is_empty() {
        return Some(pairs);
    }
    for pair in text.split('&') {
        let (key, value) = pair.split_once('=')?;
        if pairs.insert(key, value).is_some() {
            return None;
        }
    }
    Some(pairs)
}

fn percent_decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hex = text.get(index + 1..index + 3)?;
            decoded.push(u8::from_str_radix(hex, 16).ok()?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_option_changed() -> State {
        let mut extensions = Extensions::NONE;
        extensions.set(Extension::OldFootnotes, true);
        extensions.set(Extension::Wikilinks, true);
        State {
            direction: Direction::MarkdownToHtml,
            input: "<p>héllo &amp; 😀</p>\n".repeat(20),
            settings: Settings {
                htmd: HtmdOptions {
                    translation_mode: TranslationMode::Faithful,
                    heading_style: HeadingStyle::Setext,
                    hr_style: HrStyle::Underscores,
                    br_style: BrStyle::Backslash,
                    bullet_marker: BulletMarker::Dash,
                    bullet_spacing: 1,
                    number_spacing: 4,
                    code_block_style: CodeBlockStyle::Indented,
                    code_fence: CodeFence::Tildes,
                    link_style: LinkStyle::Referenced,
                    reference_style: ReferenceStyle::Shortcut,
                    preserve_code_whitespace: true,
                    noscript_as_raw_text: false,
                    skip_tags: "SCRIPT, <bad>".into(),
                },
                extensions,
            },
        }
    }

    #[test]
    fn round_trips_every_option() {
        let state = every_option_changed();
        let fragment = encode(&state, true);
        assert!(
            fragment
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_.=&".contains(&b))
        );
        assert_eq!(decode(&format!("#{fragment}")), Decoded::State(state));
    }

    #[test]
    fn defaults_are_left_out() {
        let state = State {
            input: "a".into(),
            ..State::default()
        };
        let fragment = encode(&state, true);
        assert!(fragment.starts_with("v=1&d=h&i="));
        assert_eq!(fragment.matches('&').count(), 2);
        assert_eq!(encode(&state, false), "v=1&d=h");
    }

    #[test]
    fn address_without_input_restores_an_empty_input() {
        let state = every_option_changed();
        let Decoded::State(decoded) = decode(&encode(&state, false)) else {
            panic!("invalid");
        };
        assert_eq!(decoded.input, "");
        assert_eq!(decoded.settings, state.settings);
        assert_eq!(decoded.direction, state.direction);
    }

    #[test]
    fn empty_fragment_has_no_state() {
        assert_eq!(decode(""), Decoded::NoState);
        assert_eq!(decode("#"), Decoded::NoState);
    }

    #[test]
    fn percent_encoded_addresses_still_decode() {
        let state = every_option_changed();
        let fragment = encode(&state, true).replace('=', "%3D").replace('.', "%2e");
        assert_eq!(decode(&fragment), Decoded::State(state));
    }

    #[test]
    fn invalid_addresses_are_rejected_as_a_whole() {
        for fragment in [
            "d=h",
            "v=2&d=h",
            "v=1",
            "v=1&d=x",
            "v=1&d=h&d=h",
            "v=1&d=h&zz=1",
            "v=1&d=h&bs=5",
            "v=1&d=h&bs=",
            "v=1&d=h&pw=yes",
            "v=1&d=h&tm=q",
            "v=1&d=h&i=!!",
            "v=1&d=h&i=AAAA",
            "v=1&d=h&x=tb.zz",
            "v=1&d=h&x=tb.tb",
            "v=1&d=h&x=of",
            "v=1&d=h&sk=%FF",
            "v=1&d=h&novalue",
            "v=1&d=h&",
            "v=1&d=h%2",
            "garbage",
        ] {
            assert_eq!(decode(fragment), Decoded::Invalid, "{fragment}");
        }
    }

    #[test]
    fn explicit_defaults_and_empty_extension_list_are_readable() {
        let Decoded::State(state) = decode("v=1&d=m&tm=p&bs=3&nr=1&x=") else {
            panic!("invalid");
        };
        assert_eq!(state.settings.htmd, HtmdOptions::default());
        assert_eq!(state.settings.extensions, Extensions::NONE);
        assert_eq!(state.direction, Direction::MarkdownToHtml);
    }

    /// Addresses shared today keep working: this one must always decode the
    /// same way.
    #[test]
    fn version_1_is_stable() {
        let Decoded::State(state) = decode("v=1&d=h&i=symw88i00S-wAwA&hs=s&bs=1&x=st") else {
            panic!("invalid");
        };
        assert_eq!(state.input, "<p>Hi</p>");
        assert_eq!(state.settings.htmd.heading_style, HeadingStyle::Setext);
        assert_eq!(state.settings.htmd.bullet_spacing, 1);
        assert_eq!(
            state.settings.extensions.iter().collect::<Vec<_>>(),
            [Extension::Strikethrough]
        );
    }

    #[test]
    fn settings_round_trip() {
        let settings = every_option_changed().settings;
        assert_eq!(decode_settings(&encode_settings(&settings)), Some(settings));
        assert_eq!(decode_settings(""), Some(Settings::default()));
        assert_eq!(decode_settings("v=1"), None);
    }
}
