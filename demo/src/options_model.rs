//! Sidebar option model: pulldown-cmark flags, htmd options, the
//! `skip_tags` field parser, and the demo preset (spec FR-4).

use htmd::options::{
    BrStyle, BulletListMarker, CodeBlockFence, CodeBlockStyle, HeadingStyle, HrStyle,
    LinkReferenceStyle, LinkStyle, TranslationMode,
};
use serde::{Deserialize, Serialize};

/// The 15 `pulldown_cmark::Options::ENABLE_*` flags of pulldown-cmark 0.13,
/// one sidebar checkbox each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CmarkFlags {
    pub tables: bool,
    pub footnotes: bool,
    pub strikethrough: bool,
    pub task_lists: bool,
    pub smart_punctuation: bool,
    pub heading_attributes: bool,
    pub math: bool,
    pub yaml_style_metadata_blocks: bool,
    pub pluses_delimited_metadata_blocks: bool,
    pub old_footnotes: bool,
    pub gfm: bool,
    pub definition_list: bool,
    pub superscript: bool,
    pub subscript: bool,
    pub wikilinks: bool,
}

impl CmarkFlags {
    /// The equivalent `pulldown_cmark` options bitset.
    pub fn to_pulldown(self) -> pulldown_cmark::Options {
        let flags = [
            (self.tables, pulldown_cmark::Options::ENABLE_TABLES),
            (self.footnotes, pulldown_cmark::Options::ENABLE_FOOTNOTES),
            (
                self.strikethrough,
                pulldown_cmark::Options::ENABLE_STRIKETHROUGH,
            ),
            (self.task_lists, pulldown_cmark::Options::ENABLE_TASKLISTS),
            (
                self.smart_punctuation,
                pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION,
            ),
            (
                self.heading_attributes,
                pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES,
            ),
            (self.math, pulldown_cmark::Options::ENABLE_MATH),
            (
                self.yaml_style_metadata_blocks,
                pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS,
            ),
            (
                self.pluses_delimited_metadata_blocks,
                pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS,
            ),
            (
                self.old_footnotes,
                pulldown_cmark::Options::ENABLE_OLD_FOOTNOTES,
            ),
            (self.gfm, pulldown_cmark::Options::ENABLE_GFM),
            (
                self.definition_list,
                pulldown_cmark::Options::ENABLE_DEFINITION_LIST,
            ),
            (
                self.superscript,
                pulldown_cmark::Options::ENABLE_SUPERSCRIPT,
            ),
            (self.subscript, pulldown_cmark::Options::ENABLE_SUBSCRIPT),
            (self.wikilinks, pulldown_cmark::Options::ENABLE_WIKILINKS),
        ];
        let mut options = pulldown_cmark::Options::empty();
        for (enabled, flag) in flags {
            if enabled {
                options.insert(flag);
            }
        }
        options
    }
}

// The `htmd` option enums live in the library crate, which does not
// depend on serde. These remote-derive mirrors plug into the fields of
// [`HtmdOptions`] via `#[serde(with = ...)]` (serde's supported way to
// (de)serialize foreign types for foreign traits), so FR-8's stored
// schema can embed `HtmdOptions` without duplicating its field list or
// touching the library crate. Unit variants serialize as their names,
// e.g. `"Atx"`, matching FR-4's option spellings.
#[derive(Serialize, Deserialize)]
#[serde(remote = "HeadingStyle")]
enum HeadingStyleDef {
    Atx,
    Setex,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "HrStyle")]
enum HrStyleDef {
    Dashes,
    Asterisks,
    Underscores,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "BrStyle")]
enum BrStyleDef {
    TwoSpaces,
    Backslash,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CodeBlockStyle")]
enum CodeBlockStyleDef {
    Indented,
    Fenced,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CodeBlockFence")]
enum CodeBlockFenceDef {
    Tildes,
    Backticks,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "BulletListMarker")]
enum BulletListMarkerDef {
    Asterisk,
    Dash,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "LinkStyle")]
enum LinkStyleDef {
    Inlined,
    InlinedPreferAutolinks,
    Referenced,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "LinkReferenceStyle")]
enum LinkReferenceStyleDef {
    Full,
    Collapsed,
    Shortcut,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TranslationMode")]
enum TranslationModeDef {
    Pure,
    Faithful,
}

/// The 12 options of `htmd::options::Options`, one sidebar control each;
/// `Default` equals the crate defaults.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HtmdOptions {
    #[serde(with = "HeadingStyleDef")]
    pub heading_style: HeadingStyle,
    #[serde(with = "HrStyleDef")]
    pub hr_style: HrStyle,
    #[serde(with = "BrStyleDef")]
    pub br_style: BrStyle,
    #[serde(with = "LinkStyleDef")]
    pub link_style: LinkStyle,
    #[serde(with = "LinkReferenceStyleDef")]
    pub link_reference_style: LinkReferenceStyle,
    #[serde(with = "CodeBlockStyleDef")]
    pub code_block_style: CodeBlockStyle,
    #[serde(with = "CodeBlockFenceDef")]
    pub code_block_fence: CodeBlockFence,
    #[serde(with = "BulletListMarkerDef")]
    pub bullet_list_marker: BulletListMarker,
    pub ul_bullet_spacing: u8,
    pub ol_number_spacing: u8,
    pub preformatted_code: bool,
    #[serde(with = "TranslationModeDef")]
    pub translation_mode: TranslationMode,
}

impl Default for HtmdOptions {
    fn default() -> Self {
        Self::from(htmd::options::Options::default())
    }
}

impl From<htmd::options::Options> for HtmdOptions {
    fn from(options: htmd::options::Options) -> Self {
        Self {
            heading_style: options.heading_style,
            hr_style: options.hr_style,
            br_style: options.br_style,
            link_style: options.link_style,
            link_reference_style: options.link_reference_style,
            code_block_style: options.code_block_style,
            code_block_fence: options.code_block_fence,
            bullet_list_marker: options.bullet_list_marker,
            ul_bullet_spacing: options.ul_bullet_spacing,
            ol_number_spacing: options.ol_number_spacing,
            preformatted_code: options.preformatted_code,
            translation_mode: options.translation_mode,
        }
    }
}

impl HtmdOptions {
    /// The equivalent `htmd` conversion options.
    pub fn to_htmd_options(self) -> htmd::options::Options {
        htmd::options::Options {
            heading_style: self.heading_style,
            hr_style: self.hr_style,
            br_style: self.br_style,
            link_style: self.link_style,
            link_reference_style: self.link_reference_style,
            code_block_style: self.code_block_style,
            code_block_fence: self.code_block_fence,
            bullet_list_marker: self.bullet_list_marker,
            ul_bullet_spacing: self.ul_bullet_spacing,
            ol_number_spacing: self.ol_number_spacing,
            preformatted_code: self.preformatted_code,
            translation_mode: self.translation_mode,
        }
    }
}

/// The raw `skip_tags` field is capped at this many characters before
/// parsing, matching the persistence validation limit (spec FR-4/FR-8).
const SKIP_TAGS_MAX_CHARS: usize = 200;

/// Parse the sidebar's comma-separated `skip_tags` field: split on commas,
/// trim, lowercase, then drop empty parts, duplicates, and entries not
/// matching `[a-z][a-z0-9-]*`. Input past 200 characters is ignored.
/// An empty result means "skip nothing"; the caller must not pass it to
/// htmd's `skip_tags`, which panics on an empty slice.
pub fn parse_skip_tags(raw: &str) -> Vec<String> {
    let capped: String = raw.chars().take(SKIP_TAGS_MAX_CHARS).collect();
    let mut tags: Vec<String> = Vec::new();
    for part in capped.split(',') {
        let tag = part.trim().to_ascii_lowercase();
        if is_tag_name(&tag) && !tags.contains(&tag) {
            tags.push(tag);
        }
    }
    tags
}

fn is_tag_name(tag: &str) -> bool {
    let mut chars = tag.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// The FR-4 "Reset Options to Demo Preset" state: the 7 common cmark flags
/// on (the remaining 8 off) and htmd options at crate defaults with
/// `translation_mode: Pure`. The `skip_tags` field is cleared to "".
pub fn demo_preset() -> (CmarkFlags, HtmdOptions) {
    (
        CmarkFlags {
            tables: true,
            footnotes: true,
            strikethrough: true,
            task_lists: true,
            smart_punctuation: true,
            heading_attributes: true,
            math: true,
            ..CmarkFlags::default()
        },
        HtmdOptions::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_tags_empty_and_whitespace_only_yield_nothing() {
        assert!(parse_skip_tags("").is_empty());
        assert!(parse_skip_tags("   ").is_empty());
        assert!(parse_skip_tags(" , ,, ").is_empty());
    }

    #[test]
    fn skip_tags_trims_and_lowercases() {
        assert_eq!(parse_skip_tags("IMG"), vec!["img"]);
        assert_eq!(parse_skip_tags("  Div , SPAN "), vec!["div", "span"]);
    }

    #[test]
    fn skip_tags_drops_duplicates_keeping_first_occurrence() {
        assert_eq!(parse_skip_tags("img, IMG, div,div"), vec!["img", "div"]);
    }

    #[test]
    fn skip_tags_drops_invalid_names() {
        assert_eq!(
            parse_skip_tags("1bad, -x, img, a-b2, , 9, div-"),
            vec!["img", "a-b2", "div-"]
        );
    }

    #[test]
    fn skip_tags_ignores_input_past_200_chars() {
        let raw = format!("{},zzz", "b".repeat(199));
        assert_eq!(parse_skip_tags(&raw), vec!["b".repeat(199)]);
    }

    #[test]
    fn demo_preset_is_seven_common_flags_and_crate_defaults_with_pure() {
        let (flags, htmd_opts) = demo_preset();
        assert_eq!(
            flags,
            CmarkFlags {
                tables: true,
                footnotes: true,
                strikethrough: true,
                task_lists: true,
                smart_punctuation: true,
                heading_attributes: true,
                math: true,
                ..CmarkFlags::default()
            }
        );
        assert_eq!(
            htmd_opts,
            HtmdOptions::from(htmd::options::Options::default())
        );
        assert_eq!(htmd_opts.translation_mode, TranslationMode::Pure);
    }

    #[test]
    fn preset_to_pulldown_is_exactly_the_seven_common_flags() {
        let (flags, _) = demo_preset();
        let expected = pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_FOOTNOTES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS
            | pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION
            | pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES
            | pulldown_cmark::Options::ENABLE_MATH;
        assert_eq!(flags.to_pulldown(), expected);
    }

    #[test]
    fn all_on_to_pulldown_is_all_fifteen_flags() {
        let all_on = CmarkFlags {
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
        let expected = pulldown_cmark::Options::ENABLE_TABLES
            | pulldown_cmark::Options::ENABLE_FOOTNOTES
            | pulldown_cmark::Options::ENABLE_STRIKETHROUGH
            | pulldown_cmark::Options::ENABLE_TASKLISTS
            | pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION
            | pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES
            | pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
            | pulldown_cmark::Options::ENABLE_OLD_FOOTNOTES
            | pulldown_cmark::Options::ENABLE_MATH
            | pulldown_cmark::Options::ENABLE_GFM
            | pulldown_cmark::Options::ENABLE_DEFINITION_LIST
            | pulldown_cmark::Options::ENABLE_SUPERSCRIPT
            | pulldown_cmark::Options::ENABLE_SUBSCRIPT
            | pulldown_cmark::Options::ENABLE_WIKILINKS;
        assert_eq!(all_on.to_pulldown(), expected);
    }

    #[test]
    fn defaults_are_all_flags_off_and_crate_defaults() {
        assert_eq!(
            CmarkFlags::default().to_pulldown(),
            pulldown_cmark::Options::empty()
        );
        assert_eq!(
            HtmdOptions::default().to_htmd_options(),
            htmd::options::Options::default()
        );
    }

    #[test]
    fn htmd_options_map_every_field() {
        let model = HtmdOptions {
            heading_style: HeadingStyle::Setex,
            hr_style: HrStyle::Underscores,
            br_style: BrStyle::Backslash,
            link_style: LinkStyle::Referenced,
            link_reference_style: LinkReferenceStyle::Shortcut,
            code_block_style: CodeBlockStyle::Indented,
            code_block_fence: CodeBlockFence::Tildes,
            bullet_list_marker: BulletListMarker::Dash,
            ul_bullet_spacing: 1,
            ol_number_spacing: 4,
            preformatted_code: true,
            translation_mode: TranslationMode::Faithful,
        };
        let o = model.to_htmd_options();
        assert_eq!(o.heading_style, HeadingStyle::Setex);
        assert_eq!(o.hr_style, HrStyle::Underscores);
        assert_eq!(o.br_style, BrStyle::Backslash);
        assert_eq!(o.link_style, LinkStyle::Referenced);
        assert_eq!(o.link_reference_style, LinkReferenceStyle::Shortcut);
        assert_eq!(o.code_block_style, CodeBlockStyle::Indented);
        assert_eq!(o.code_block_fence, CodeBlockFence::Tildes);
        assert_eq!(o.bullet_list_marker, BulletListMarker::Dash);
        assert_eq!(o.ul_bullet_spacing, 1);
        assert_eq!(o.ol_number_spacing, 4);
        assert!(o.preformatted_code);
        assert_eq!(o.translation_mode, TranslationMode::Faithful);
    }
}
