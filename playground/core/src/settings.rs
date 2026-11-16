//! The playground's options, as described in the spec's Options section.

/// An option whose value is one of a fixed list, shown as a dropdown.
pub trait Choice: Copy + PartialEq + 'static {
    /// Every value, in the order the dropdown lists them; the first is the
    /// default.
    const ALL: &'static [Self];
    /// The value's short name in a permalink. Permalinks shared today must
    /// keep working, so a code never changes once published.
    fn code(self) -> &'static str;
    /// The text shown in the dropdown.
    fn label(self) -> &'static str;
    /// The value whose [`Choice::code`] is `code`.
    fn from_code(code: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|value| value.code() == code)
    }
}

macro_rules! choice {
    ($(#[$meta:meta])* $name:ident { $first:ident = $first_code:literal, $first_label:literal; $($variant:ident = $code:literal, $label:literal;)* }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum $name {
            #[default]
            $first,
            $($variant,)*
        }

        impl Choice for $name {
            const ALL: &'static [Self] = &[Self::$first, $(Self::$variant,)*];
            fn code(self) -> &'static str {
                match self {
                    Self::$first => $first_code,
                    $(Self::$variant => $code,)*
                }
            }
            fn label(self) -> &'static str {
                match self {
                    Self::$first => $first_label,
                    $(Self::$variant => $label,)*
                }
            }
        }
    };
}

choice!(
    /// Which converter the playground runs.
    Direction {
        HtmlToMarkdown = "h", "HTML → Markdown";
        MarkdownToHtml = "m", "Markdown → HTML";
    }
);

impl Direction {
    /// The other direction.
    pub fn flipped(self) -> Self {
        match self {
            Self::HtmlToMarkdown => Self::MarkdownToHtml,
            Self::MarkdownToHtml => Self::HtmlToMarkdown,
        }
    }

    /// The name of the input language, which labels the input pane.
    pub fn input_language(self) -> &'static str {
        match self {
            Self::HtmlToMarkdown => "HTML",
            Self::MarkdownToHtml => "Markdown",
        }
    }

    /// The name of the output language.
    pub fn output_language(self) -> &'static str {
        self.flipped().input_language()
    }
}

choice!(
    /// htmd's `TranslationMode`.
    TranslationMode {
        Pure = "p", "Pure";
        Faithful = "f", "Faithful";
    }
);

choice!(
    /// htmd's `HeadingStyle`.
    HeadingStyle {
        Atx = "a", "ATX (# Heading)";
        Setext = "s", "Setext (underlined)";
    }
);

choice!(
    /// htmd's `HrStyle`.
    HrStyle {
        Asterisks = "a", "* * *";
        Dashes = "d", "- - -";
        Underscores = "u", "_ _ _";
    }
);

choice!(
    /// htmd's `BrStyle`.
    BrStyle {
        TwoSpaces = "s", "two spaces";
        Backslash = "b", "backslash";
    }
);

choice!(
    /// htmd's `BulletListMarker`.
    BulletMarker {
        Asterisk = "a", "*";
        Dash = "d", "-";
    }
);

choice!(
    /// htmd's `CodeBlockStyle`.
    CodeBlockStyle {
        Fenced = "f", "fenced";
        Indented = "i", "indented";
    }
);

choice!(
    /// htmd's `CodeBlockFence`.
    CodeFence {
        Backticks = "b", "```";
        Tildes = "t", "~~~";
    }
);

choice!(
    /// htmd's `LinkStyle`.
    LinkStyle {
        Inline = "i", "inline";
        InlineAutolinks = "a", "inline with autolinks";
        Referenced = "r", "referenced";
    }
);

choice!(
    /// htmd's `LinkReferenceStyle`.
    ReferenceStyle {
        Full = "f", "full ([t][1])";
        Collapsed = "c", "collapsed ([t][])";
        Shortcut = "s", "shortcut ([t])";
    }
);

/// The smallest and largest number of spaces after a list marker.
pub const LIST_SPACING_RANGE: std::ops::RangeInclusive<u8> = 1..=4;

/// The options for HTML → Markdown. The defaults are the spec's, which match
/// htmd 0.5.5's own; they are spelled out here so that a later htmd changing
/// a default does not change the playground.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HtmdOptions {
    pub translation_mode: TranslationMode,
    pub heading_style: HeadingStyle,
    pub hr_style: HrStyle,
    pub br_style: BrStyle,
    pub bullet_marker: BulletMarker,
    pub bullet_spacing: u8,
    pub number_spacing: u8,
    pub code_block_style: CodeBlockStyle,
    pub code_fence: CodeFence,
    pub link_style: LinkStyle,
    pub reference_style: ReferenceStyle,
    pub preserve_code_whitespace: bool,
    pub noscript_as_raw_text: bool,
    /// The Skip tags field exactly as typed; see [`parse_skip_tags`].
    pub skip_tags: String,
}

impl Default for HtmdOptions {
    fn default() -> Self {
        Self {
            translation_mode: TranslationMode::default(),
            heading_style: HeadingStyle::default(),
            hr_style: HrStyle::default(),
            br_style: BrStyle::default(),
            bullet_marker: BulletMarker::default(),
            bullet_spacing: 3,
            number_spacing: 2,
            code_block_style: CodeBlockStyle::default(),
            code_fence: CodeFence::default(),
            link_style: LinkStyle::default(),
            reference_style: ReferenceStyle::default(),
            preserve_code_whitespace: false,
            noscript_as_raw_text: true,
            skip_tags: String::new(),
        }
    }
}

/// Clamp a list-spacing value into [`LIST_SPACING_RANGE`].
pub fn clamp_list_spacing(value: i64) -> u8 {
    value.clamp(
        i64::from(*LIST_SPACING_RANGE.start()),
        i64::from(*LIST_SPACING_RANGE.end()),
    ) as u8
}

/// The Skip tags field, split into the names passed to htmd and the entries
/// which were ignored.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkipTags {
    /// Lowercased tag names, without duplicates. html5ever lowercases HTML tag
    /// names and htmd compares them by case, so lowercasing here is what
    /// makes the field case-insensitive.
    pub names: Vec<String>,
    /// Entries which are not valid tag names, as typed.
    pub ignored: Vec<String>,
}

/// Split the Skip tags field on commas and whitespace. A valid name is an
/// ASCII letter followed by ASCII letters, digits, `-`, `_`, or `.`, which
/// covers HTML's own elements and custom elements.
pub fn parse_skip_tags(text: &str) -> SkipTags {
    let mut result = SkipTags::default();
    for entry in text
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|entry| !entry.is_empty())
    {
        let mut chars = entry.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_alphabetic())
            && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
        if valid {
            let name = entry.to_ascii_lowercase();
            if !result.names.contains(&name) {
                result.names.push(name);
            }
        } else {
            result.ignored.push(entry.to_string());
        }
    }
    result
}

/// A pulldown-cmark extension, one checkbox each.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extension {
    Tables,
    Math,
    Footnotes,
    OldFootnotes,
    Strikethrough,
    TaskLists,
    SmartPunctuation,
    HeadingAttributes,
    YamlMetadata,
    PlusesMetadata,
    Gfm,
    DefinitionLists,
    Superscript,
    Subscript,
    Wikilinks,
}

impl Extension {
    /// Every extension, in the order of the spec's extension table.
    pub const ALL: [Extension; 15] = [
        Self::Tables,
        Self::Math,
        Self::Footnotes,
        Self::OldFootnotes,
        Self::Strikethrough,
        Self::TaskLists,
        Self::SmartPunctuation,
        Self::HeadingAttributes,
        Self::YamlMetadata,
        Self::PlusesMetadata,
        Self::Gfm,
        Self::DefinitionLists,
        Self::Superscript,
        Self::Subscript,
        Self::Wikilinks,
    ];

    fn bit(self) -> u16 {
        1 << self as u16
    }

    /// The extension's short name in a permalink; never changes once
    /// published.
    pub fn code(self) -> &'static str {
        match self {
            Self::Tables => "tb",
            Self::Math => "mt",
            Self::Footnotes => "fn",
            Self::OldFootnotes => "of",
            Self::Strikethrough => "st",
            Self::TaskLists => "tl",
            Self::SmartPunctuation => "sp",
            Self::HeadingAttributes => "ha",
            Self::YamlMetadata => "ym",
            Self::PlusesMetadata => "pm",
            Self::Gfm => "gfm",
            Self::DefinitionLists => "dl",
            Self::Superscript => "sup",
            Self::Subscript => "sub",
            Self::Wikilinks => "wl",
        }
    }

    /// The checkbox label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Tables => "Tables",
            Self::Math => "Math",
            Self::Footnotes => "Footnotes",
            Self::OldFootnotes => "Old-style footnotes",
            Self::Strikethrough => "Strikethrough",
            Self::TaskLists => "Task lists",
            Self::SmartPunctuation => "Smart punctuation",
            Self::HeadingAttributes => "Heading attributes",
            Self::YamlMetadata => "YAML metadata blocks",
            Self::PlusesMetadata => "+++ metadata blocks",
            Self::Gfm => "GFM",
            Self::DefinitionLists => "Definition lists",
            Self::Superscript => "Superscript",
            Self::Subscript => "Subscript",
            Self::Wikilinks => "Wikilinks",
        }
    }

    /// The checkbox's one-line help, from pulldown-cmark's `Options` docs
    /// where they exist.
    pub fn help(self) -> &'static str {
        match self {
            Self::Tables => "GitHub-style pipe tables.",
            Self::Math => "$inline$ and $$display$$ math, shown as formula text.",
            Self::Footnotes => {
                "GitHub-compatible footnotes: [^1] references and [^1]: definitions."
            }
            Self::OldFootnotes => {
                "The older footnote syntax, with different indentation and nesting rules. Implies footnotes."
            }
            Self::Strikethrough => "~~text~~ becomes struck through.",
            Self::TaskLists => "List items starting with [ ] or [x] become checkboxes.",
            Self::SmartPunctuation => {
                "Curly quotes, en and em dashes from -- and ---, and … from ..."
            }
            Self::HeadingAttributes => "# Heading {#id .class} sets the heading's ID and classes.",
            Self::YamlMetadata => "A leading block between --- lines is metadata, not content.",
            Self::PlusesMetadata => "A leading block between +++ lines is metadata, not content.",
            Self::Gfm => "GitHub Flavored Markdown extras: blockquote tags such as > [!NOTE].",
            Self::DefinitionLists => "A term line followed by a : definition line.",
            Self::Superscript => "^text^ becomes superscript.",
            Self::Subscript => "~text~ becomes subscript; ~~text~~ stays strikethrough.",
            Self::Wikilinks => "Obsidian-style [[Page]] and [[Page|label]] links.",
        }
    }

    fn on_by_default(self) -> bool {
        // htmd writes tables and math, so the Rendered tab of HTML → Markdown
        // needs both to show its own output correctly.
        matches!(self, Self::Tables | Self::Math)
    }

    /// The extension whose short name is `code`.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|ext| ext.code() == code)
    }
}

/// The set of enabled pulldown-cmark extensions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extensions(u16);

impl Default for Extensions {
    fn default() -> Self {
        Self(
            Extension::ALL
                .into_iter()
                .filter(|ext| ext.on_by_default())
                .fold(0, |bits, ext| bits | ext.bit()),
        )
    }
}

impl Extensions {
    /// No extensions.
    pub const NONE: Self = Self(0);

    /// Whether `ext` is enabled.
    pub fn contains(self, ext: Extension) -> bool {
        self.0 & ext.bit() != 0
    }

    /// Enable or disable `ext`. Old-style footnotes require footnotes, so
    /// enabling the former enables the latter, and disabling the latter
    /// disables the former.
    pub fn set(&mut self, ext: Extension, on: bool) {
        if on {
            self.0 |= ext.bit();
            if ext == Extension::OldFootnotes {
                self.0 |= Extension::Footnotes.bit();
            }
        } else {
            self.0 &= !ext.bit();
            if ext == Extension::Footnotes {
                self.0 &= !Extension::OldFootnotes.bit();
            }
        }
    }

    /// The enabled extensions, in [`Extension::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = Extension> {
        Extension::ALL
            .into_iter()
            .filter(move |ext| self.contains(*ext))
    }
}

/// Every option the user can set.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Settings {
    pub htmd: HtmdOptions,
    pub extensions: Extensions,
}

/// Everything a permalink records.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    pub direction: Direction,
    pub input: String,
    pub settings: Settings,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skip_tags_split_on_commas_and_spaces_and_ignore_case() {
        let parsed = parse_skip_tags(" SCRIPT, style  my-el,,<b> 1x script");
        assert_eq!(parsed.names, ["script", "style", "my-el"]);
        assert_eq!(parsed.ignored, ["<b>", "1x"]);
    }

    #[test]
    fn old_footnotes_imply_footnotes() {
        let mut ext = Extensions::NONE;
        ext.set(Extension::OldFootnotes, true);
        assert!(ext.contains(Extension::Footnotes));
        ext.set(Extension::Footnotes, false);
        assert!(!ext.contains(Extension::OldFootnotes));
        ext.set(Extension::Footnotes, true);
        ext.set(Extension::OldFootnotes, false);
        assert!(ext.contains(Extension::Footnotes));
    }

    #[test]
    fn defaults_match_the_spec() {
        let defaults: Vec<_> = Extensions::default().iter().collect();
        assert_eq!(defaults, [Extension::Tables, Extension::Math]);
        let htmd = HtmdOptions::default();
        assert_eq!((htmd.bullet_spacing, htmd.number_spacing), (3, 2));
        assert!(htmd.noscript_as_raw_text);
    }

    #[test]
    fn list_spacing_clamps() {
        assert_eq!(clamp_list_spacing(-3), 1);
        assert_eq!(clamp_list_spacing(9), 4);
        assert_eq!(clamp_list_spacing(2), 2);
    }
}
