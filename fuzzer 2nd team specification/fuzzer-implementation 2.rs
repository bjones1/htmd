use htmd::{
    HtmlToMarkdown,
    options::{BulletListMarker, CodeBlockStyle, HeadingStyle, Options, TranslationMode},
};
use markup5ever_rcdom::{Handle, NodeData};
use pulldown_cmark::{Options as MarkdownOptions, Parser};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::Serialize;
use std::{
    fs, io,
    panic::{AssertUnwindSafe, catch_unwind},
    path::Path,
};

pub const GENERATOR_VERSION: u32 = 2;
pub const TEXT_POOL_VERSION: u32 = 2;
pub const MAX_TREE_DEPTH: usize = 6;
pub const MAX_CHILDREN: usize = 4;
pub const MAX_TEXT_SCALARS: usize = 64;
pub const MAX_FRAGMENT_BYTES: usize = 4096;
pub const MAX_CANDIDATES: usize = 100;
pub const DEFAULT_TIMEOUT_SECS: u64 = 5;
pub const DEFAULT_MEMORY_LIMIT_MIB: usize = 512;
pub const DEFAULT_SEEDS: [u64; 5] = [1, 2, 3, 42, 20_260_301];
const HTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum CaseClass {
    Valid,
    Malformed,
}

fn within_byte_limit(fragment: &str, limit: usize) -> bool {
    fragment.len() <= limit
}

fn first_fitting_candidate<T>(
    max_bytes: usize,
    max_candidates: usize,
    mut next: impl FnMut() -> Result<(T, String), GeneratorError>,
) -> Result<(T, String), GeneratorError> {
    for _ in 0..max_candidates {
        let (candidate, fragment) = next()?;
        if within_byte_limit(&fragment, max_bytes) {
            return Ok((candidate, fragment));
        }
    }
    Err(GeneratorError::CandidateLimitExceeded {
        max_bytes,
        attempts: max_candidates,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum ParserProfile {
    Core,
    Gfm,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Text(String),
    Element {
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<Node>,
    },
    Fragment(Vec<Node>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FuzzerCase {
    pub class: CaseClass,
    pub profile: Option<ParserProfile>,
    pub master_seed: u64,
    pub case_index: u64,
    pub input: Node,
    pub source_html: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FuzzerOptions {
    pub max_depth: usize,
    pub max_children: usize,
    pub max_text_scalars: usize,
    pub max_fragment_bytes: usize,
    pub max_candidates: usize,
    pub timeout_secs: u64,
    pub memory_limit_mib: usize,
}

impl Default for FuzzerOptions {
    fn default() -> Self {
        Self {
            max_depth: MAX_TREE_DEPTH,
            max_children: MAX_CHILDREN,
            max_text_scalars: MAX_TEXT_SCALARS,
            max_fragment_bytes: MAX_FRAGMENT_BYTES,
            max_candidates: MAX_CANDIDATES,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
            memory_limit_mib: DEFAULT_MEMORY_LIMIT_MIB,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureCategory {
    ExpectedError,
    UnexpectedError,
    Panic,
    Timeout,
    ResourceLimit,
    NonDeterminism,
    OutOfScope,
    RoundTripMismatch,
    KnownLoss,
    StaleAllowlistEntry,
    IsolationSetupFailure,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CaseOutcome {
    pub category: Option<FailureCategory>,
    pub input_html: String,
    pub generated_markdown: Option<String>,
    pub round_trip_html: Option<String>,
    pub first_difference: Option<String>,
    pub normalized_source: Option<DomNode>,
    pub normalized_round_trip: Option<DomNode>,
    pub detail: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum DomNode {
    Text(String),
    Comment(String),
    Element {
        namespace: String,
        tag: String,
        attrs: Vec<(String, String)>,
        children: Vec<DomNode>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeneratorError {
    InvalidTree(String),
    CandidateLimitExceeded { max_bytes: usize, attempts: usize },
}

impl std::fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidTree(message) => write!(f, "invalid generated tree: {message}"),
            Self::CandidateLimitExceeded {
                max_bytes,
                attempts,
            } => write!(
                f,
                "no candidate fit within {max_bytes} bytes after {attempts} attempts"
            ),
        }
    }
}

impl std::error::Error for GeneratorError {}

struct CaseRng(ChaCha8Rng);

impl CaseRng {
    fn new(seed: u64) -> Self {
        Self(ChaCha8Rng::seed_from_u64(seed))
    }

    fn choose(&mut self, upper: usize) -> usize {
        assert!(upper > 0, "cannot choose from an empty set");
        self.0
            .random_range(0..u32::try_from(upper).expect("choice count fits in u32"))
            as usize
    }

    fn one_to(&mut self, upper: usize) -> usize {
        1 + self.choose(upper)
    }
}

pub struct Fuzzer {
    master_seed: u64,
    case_index: u64,
    profile: ParserProfile,
    options: FuzzerOptions,
    rng: CaseRng,
}

impl Fuzzer {
    pub fn new(master_seed: u64, case_index: u64, profile: ParserProfile) -> Self {
        Self::with_options(master_seed, case_index, profile, FuzzerOptions::default())
    }

    pub fn with_options(
        master_seed: u64,
        case_index: u64,
        profile: ParserProfile,
        options: FuzzerOptions,
    ) -> Self {
        Self {
            master_seed,
            case_index,
            profile,
            options,
            rng: CaseRng::new(master_seed.wrapping_add(case_index)),
        }
    }

    pub fn options(&self) -> &FuzzerOptions {
        &self.options
    }

    pub fn generate_valid_case(&mut self) -> Result<FuzzerCase, GeneratorError> {
        if self.options.max_depth == 0
            || self.options.max_children == 0
            || self.options.max_candidates == 0
        {
            return Err(GeneratorError::InvalidTree(
                "depth, child, and candidate limits must be positive".to_string(),
            ));
        }
        self.rng = CaseRng::new(self.master_seed.wrapping_add(self.case_index));
        let case_index = self.case_index;
        let (input, source_html) = first_fitting_candidate(
            self.options.max_fragment_bytes,
            self.options.max_candidates,
            || {
                let input = self.generate_tree();
                validate_tree(&input, &self.options, self.profile).map_err(|error| {
                    GeneratorError::InvalidTree(format!("case {case_index}: {error}"))
                })?;
                let source_html = serialize_html(&input);
                Ok((input, source_html))
            },
        )?;
        self.case_index = self.case_index.wrapping_add(1);
        Ok(FuzzerCase {
            class: CaseClass::Valid,
            profile: Some(self.profile),
            master_seed: self.master_seed,
            case_index,
            input,
            source_html,
        })
    }

    pub fn generate_malformed_case(&mut self) -> Result<FuzzerCase, GeneratorError> {
        let mut case = self.generate_valid_case()?;
        case.class = CaseClass::Malformed;
        case.profile = None;
        case.source_html = corrupt_once(&case.source_html, &mut self.rng);
        Ok(case)
    }

    fn generate_tree(&mut self) -> Node {
        let count = self
            .rng
            .one_to(8)
            .max(usize::from(self.profile == ParserProfile::Gfm) + 1);
        let mut children = Vec::with_capacity(count);
        children.push(self.coverage_fixture(self.case_index));

        if self.profile == ParserProfile::Gfm {
            children.push(self.generate_translatable_table());
        }

        while children.len() < count {
            children.push(self.generate_block(1, false));
        }

        Node::Fragment(children)
    }

    fn coverage_fixture(&mut self, case_index: u64) -> Node {
        let cycle = if self.profile == ParserProfile::Gfm {
            65
        } else {
            54
        };
        let index = (case_index % cycle as u64) as usize;
        let mut text = || Node::Text(self.random_text());
        match index % cycle {
            0 => element("blockquote", vec![element("p", vec![text()])]),
            1 => element("ul", vec![element("li", vec![text()])]),
            2 => element("ol", vec![element("li", vec![text()])]),
            3..=8 => element(&format!("h{}", index % 6 + 1), vec![text()]),
            9 => element("hr", vec![]),
            10 => element("p", vec![text()]),
            11 => element("pre", vec![element("code", vec![text()])]),
            12 => element(
                "p",
                vec![element_with_attrs(
                    "a",
                    vec![text()],
                    vec![attr("href", "https://example.test/page")],
                )],
            ),
            13 => element(
                "p",
                vec![element_with_attrs(
                    "a",
                    vec![text()],
                    vec![
                        attr("href", "https://example.test/page"),
                        attr("title", "sample"),
                    ],
                )],
            ),
            14 => element(
                "p",
                vec![element_with_attrs(
                    "a",
                    vec![text()],
                    vec![
                        attr("href", "https://example.test/page"),
                        attr("id", "sample"),
                    ],
                )],
            ),
            15 => element(
                "p",
                vec![element_with_attrs(
                    "img",
                    vec![],
                    vec![
                        attr("src", "https://example.test/image.png"),
                        attr("alt", "image"),
                    ],
                )],
            ),
            16 => element("p", vec![element("code", vec![text()])]),
            17 => element("p", vec![element("strong", vec![text()])]),
            18 => element("p", vec![element("b", vec![text()])]),
            19 => element("p", vec![element("i", vec![text()])]),
            20 => element("p", vec![element("em", vec![text()])]),
            21 => element("p", vec![element("br", vec![])]),
            22 => element("p", vec![element("span", vec![text()])]),
            23 => element("div", vec![element("p", vec![text()])]),
            24 => element("section", vec![element("p", vec![text()])]),
            25 => element("p", vec![element("q", vec![text()])]),
            26 => element(
                "ul",
                vec![element(
                    "li",
                    vec![element_with_attrs(
                        "input",
                        vec![],
                        vec![attr("id", "input")],
                    )],
                )],
            ),
            27 => element_with_attrs(
                "ol",
                vec![element("li", vec![text()])],
                vec![attr("start", "2")],
            ),
            28 => element_with_attrs(
                "ul",
                vec![element("li", vec![text()])],
                vec![attr("id", "list")],
            ),
            29 => {
                let language = if case_index % 2 == 0 {
                    "language-rust"
                } else {
                    "language-py3"
                };
                element(
                    "pre",
                    vec![element_with_attrs(
                        "code",
                        vec![text()],
                        vec![attr("class", language)],
                    )],
                )
            }
            30 => element("pre", vec![element("span", vec![text()])]),
            31 => element_with_attrs("p", vec![text()], vec![attr("class", "paragraph")]),
            32 => element_with_attrs("h2", vec![text()], vec![attr("title", "heading")]),
            33 => element("ul", vec![element("li", vec![text()])]),
            34 => element("p", vec![element("b", vec![text()])]),
            35 => element("p", vec![element("i", vec![text()])]),
            36 => element(
                "p",
                vec![element_with_attrs(
                    "img",
                    vec![],
                    vec![attr("src", "image.png")],
                )],
            ),
            37 => element_with_attrs(
                "section",
                vec![element("p", vec![text()])],
                vec![attr("id", "section")],
            ),
            38 => element_with_attrs(
                "p",
                vec![element("q", vec![text()])],
                vec![attr("class", "quote")],
            ),
            39 => element(
                "ul",
                vec![element_with_attrs(
                    "li",
                    vec![text()],
                    vec![attr("class", "item")],
                )],
            ),
            40 => element_with_attrs("p", vec![element("br", vec![])], vec![attr("id", "break")]),
            41 => element(
                "ul",
                vec![element_with_attrs(
                    "li",
                    vec![text()],
                    vec![attr("id", "item")],
                )],
            ),
            42 => element_with_attrs(
                "blockquote",
                vec![element("p", vec![text()])],
                vec![attr("class", "quote")],
            ),
            43 => element_with_attrs("hr", vec![], vec![attr("id", "rule")]),
            44 => element_with_attrs(
                "div",
                vec![element("p", vec![text()])],
                vec![attr("id", "container")],
            ),
            45 => element_with_attrs(
                "pre",
                vec![Node::Text(self.random_text())],
                vec![attr("id", "pre")],
            ),
            46 => element(
                "p",
                vec![element_with_attrs(
                    "strong",
                    vec![text()],
                    vec![attr("class", "bold")],
                )],
            ),
            47 => element(
                "p",
                vec![element_with_attrs(
                    "b",
                    vec![text()],
                    vec![attr("class", "bold")],
                )],
            ),
            48 => element(
                "p",
                vec![element_with_attrs(
                    "i",
                    vec![text()],
                    vec![attr("class", "italic")],
                )],
            ),
            49 => element(
                "p",
                vec![element_with_attrs(
                    "em",
                    vec![text()],
                    vec![attr("class", "emphasis")],
                )],
            ),
            50 => element(
                "p",
                vec![element_with_attrs(
                    "code",
                    vec![text()],
                    vec![attr("class", "code")],
                )],
            ),
            51 => element(
                "p",
                vec![element_with_attrs(
                    "span",
                    vec![text()],
                    vec![attr("class", "span")],
                )],
            ),
            52 => element_with_attrs(
                "p",
                vec![element("q", vec![text()])],
                vec![attr("title", "quote")],
            ),
            53 if self.profile == ParserProfile::Gfm => element(
                "ul",
                vec![element(
                    "li",
                    vec![element_with_attrs(
                        "input",
                        vec![],
                        vec![attr("type", "checkbox")],
                    )],
                )],
            ),
            53 => element(
                "ul",
                vec![element_with_attrs(
                    "li",
                    vec![element_with_attrs(
                        "input",
                        vec![],
                        vec![attr("class", "input")],
                    )],
                    vec![],
                )],
            ),
            54 => table_node(true, false),
            55 => table_node(false, true),
            56 => table_with_cell_attribute("td"),
            57 => table_with_cell_attribute("th"),
            58 => table_with_attribute(),
            59 => table_with_part_attribute("caption", "class", "caption"),
            60 => table_with_part_attribute("tfoot", "id", "footer"),
            61 => table_with_part_attribute("thead", "id", "head"),
            62 => table_with_part_attribute("tbody", "id", "body"),
            63 => table_with_part_attribute("tr", "class", "row"),
            _ => table_with_part_attribute("td", "title", "cell"),
        }
    }

    fn generate_block(&mut self, depth: usize, _in_list: bool) -> Node {
        if depth + 1 >= self.options.max_depth {
            return element("p", vec![Node::Text(self.random_text())]);
        }

        let choices: &[&str] = if self.profile == ParserProfile::Gfm {
            &[
                "blockquote",
                "ul",
                "ol",
                "h1",
                "h2",
                "h3",
                "h4",
                "h5",
                "h6",
                "hr",
                "p",
                "pre",
                "div",
                "section",
                "table",
            ]
        } else {
            &[
                "blockquote",
                "ul",
                "ol",
                "h1",
                "h2",
                "h3",
                "h4",
                "h5",
                "h6",
                "hr",
                "p",
                "pre",
                "div",
                "section",
            ]
        };
        let tag = choices[self.rng.choose(choices.len())];

        match tag {
            "hr" => element("hr", vec![]),
            "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let count = self.rng.one_to(self.options.max_children);
                let children = (0..count)
                    .map(|_| self.generate_inline(depth + 1, false, false))
                    .collect();
                element(tag, children)
            }
            "ul" | "ol" => {
                let count = self.rng.one_to(self.options.max_children);
                let items = (0..count)
                    .map(|_| self.generate_list_item(depth + 1))
                    .collect();
                let attrs = if tag == "ol" && self.rng.choose(2) == 1 {
                    vec![attr("start", &self.rng.one_to(4).to_string())]
                } else {
                    Vec::new()
                };
                element_with_attrs(tag, items, attrs)
            }
            "blockquote" | "div" | "section" => {
                let count = self.rng.one_to(self.options.max_children);
                let children = (0..count)
                    .map(|_| self.generate_block(depth + 1, false))
                    .collect();
                let attrs = self.optional_extra_attribute();
                element_with_attrs(tag, children, attrs)
            }
            "pre" => {
                if self.rng.choose(5) < 4 {
                    let code_count = self.rng.one_to(self.options.max_children);
                    let texts = (0..code_count)
                        .map(|_| Node::Text(self.random_text()))
                        .collect();
                    let code_attrs = if self.rng.choose(3) == 0 {
                        vec![attr("class", "language-rust")]
                    } else {
                        Vec::new()
                    };
                    element_with_attrs(
                        "pre",
                        vec![element_with_attrs("code", texts, code_attrs)],
                        self.optional_extra_attribute(),
                    )
                } else {
                    let fallback = if self.rng.choose(2) == 0 {
                        vec![Node::Text(self.random_text())]
                    } else {
                        vec![element("span", vec![Node::Text(self.random_text())])]
                    };
                    element_with_attrs("pre", fallback, self.optional_extra_attribute())
                }
            }
            "table" if depth + 4 <= self.options.max_depth => self.generate_translatable_table(),
            "table" => element("p", vec![Node::Text(self.random_text())]),
            _ => self.generate_list_item(depth),
        }
    }

    fn generate_list_item(&mut self, depth: usize) -> Node {
        let mut children = vec![self.generate_inline(depth + 1, false, false)];
        if depth + 1 < self.options.max_depth && self.rng.choose(4) == 0 {
            children.push(self.generate_block(depth + 1, true));
        }
        let attrs = self.optional_extra_attribute();
        if self.rng.choose(4) == 0 {
            let input_attrs = if self.profile == ParserProfile::Gfm {
                vec![attr("type", "checkbox")]
            } else {
                vec![attr("id", "input")]
            };
            children.push(element_with_attrs("input", vec![], input_attrs));
        }
        element_with_attrs("li", children, attrs)
    }

    fn generate_translatable_table(&mut self) -> Node {
        let columns = self.rng.one_to(self.options.max_children);
        let body_rows = self.rng.one_to(self.options.max_children);
        let header = element(
            "thead",
            vec![element(
                "tr",
                (0..columns)
                    .map(|_| element("th", vec![Node::Text(self.random_text())]))
                    .collect(),
            )],
        );
        let body = element(
            "tbody",
            (0..body_rows)
                .map(|_| {
                    element(
                        "tr",
                        (0..columns)
                            .map(|_| element("td", vec![Node::Text(self.random_text())]))
                            .collect(),
                    )
                })
                .collect(),
        );
        element("table", vec![header, body])
    }

    fn generate_inline(&mut self, depth: usize, inside_anchor: bool, inside_code: bool) -> Node {
        if depth >= self.options.max_depth || inside_code {
            return Node::Text(self.random_text());
        }
        let choices: &[&str] = if inside_anchor {
            &[
                "text", "strong", "b", "i", "em", "span", "code", "br", "img",
            ]
        } else {
            &[
                "text", "strong", "b", "i", "em", "a", "img", "code", "span", "br", "q",
            ]
        };
        match choices[self.rng.choose(choices.len())] {
            "text" => Node::Text(self.random_text()),
            tag @ ("strong" | "b" | "i" | "em" | "span" | "q") => element_with_attrs(
                tag,
                vec![self.generate_inline(depth + 1, inside_anchor, false)],
                self.optional_extra_attribute(),
            ),
            "a" => element_with_attrs(
                "a",
                vec![self.generate_inline(depth + 1, true, false)],
                vec![attr("href", "https://example.test/page")],
            ),
            "img" => element_with_attrs(
                "img",
                vec![],
                vec![
                    attr("src", "https://example.test/image.png"),
                    attr("alt", "image"),
                ],
            ),
            "code" => element("code", vec![Node::Text(self.random_text())]),
            "br" => element("br", vec![]),
            _ => Node::Text(self.random_text()),
        }
    }

    fn optional_extra_attribute(&mut self) -> Vec<Attribute> {
        if self.rng.choose(3) == 0 {
            match self.rng.choose(3) {
                0 => vec![attr("id", "sample")],
                1 => vec![attr("class", "sample")],
                _ => vec![attr("title", "sample")],
            }
        } else {
            Vec::new()
        }
    }

    fn random_text(&mut self) -> String {
        const TEXT_CLASSES: &[&[&str]] = &[
            &["Hello", "abc123", "abcdefghijklmnopqrstuvwxyz012345"],
            &[
                "*literal*",
                "_text_",
                "`code`",
                "#",
                "[",
                "]",
                "<",
                ">",
                "|",
                "\\",
            ],
            &[
                "- item", "+ item", "> quote", "1. item", "* item", "---", "- - -", "***", "___",
            ],
            &[
                "[label](https://example.test/)",
                "![alt](image.png)",
                "<span>raw</span>",
                "~~deleted~~",
            ],
            &["| cell |", "- [ ] task", "$x^2$", "$$x^2$$"],
            &["&", "\r\n", "\u{a0}"],
            &[
                "",
                " ",
                "   ",
                "\t",
                "\r\n",
                " leading",
                "trailing ",
                "two  spaces",
            ],
            &["Привет", "«quoted»", "—dash—", "e\u{301}", "😀"],
        ];
        let class = TEXT_CLASSES[self.rng.choose(TEXT_CLASSES.len())];
        class[self.rng.choose(class.len())].to_string()
    }
}

fn element(tag: &str, children: Vec<Node>) -> Node {
    Node::Element {
        tag: tag.to_string(),
        attrs: Vec::new(),
        children,
    }
}

fn element_with_attrs(tag: &str, children: Vec<Node>, attrs: Vec<Attribute>) -> Node {
    Node::Element {
        tag: tag.to_string(),
        attrs,
        children,
    }
}

fn attr(name: &str, value: &str) -> Attribute {
    Attribute {
        name: name.to_string(),
        value: value.to_string(),
    }
}

fn table_node(with_caption: bool, with_footer: bool) -> Node {
    let mut children = Vec::new();
    if with_caption {
        children.push(element("caption", vec![Node::Text("caption".to_string())]));
    }
    children.push(element(
        "thead",
        vec![element(
            "tr",
            vec![element("th", vec![Node::Text("head".to_string())])],
        )],
    ));
    children.push(element(
        "tbody",
        vec![element(
            "tr",
            vec![element("td", vec![Node::Text("cell".to_string())])],
        )],
    ));
    if with_footer {
        children.push(element(
            "tfoot",
            vec![element(
                "tr",
                vec![element("td", vec![Node::Text("footer".to_string())])],
            )],
        ));
    }
    element("table", children)
}

fn table_with_cell_attribute(cell_tag: &str) -> Node {
    let mut table = table_node(false, false);
    if let Node::Element { children, .. } = &mut table {
        add_attribute_to_first_cell(children, cell_tag, attr("colspan", "2"));
    }
    table
}

fn table_with_attribute() -> Node {
    let table = table_node(false, false);
    if let Node::Element { children, .. } = table {
        element_with_attrs("table", children, vec![attr("id", "table")])
    } else {
        unreachable!("table_node always returns an element")
    }
}

fn table_with_part_attribute(tag: &str, name: &str, value: &str) -> Node {
    let mut table = table_node(tag == "caption", tag == "tfoot");
    if let Node::Element { children, .. } = &mut table {
        add_attribute_to_first_element(children, tag, attr(name, value));
    }
    table
}

fn add_attribute_to_first_element(nodes: &mut [Node], tag: &str, attribute: Attribute) -> bool {
    for node in nodes {
        if let Node::Element {
            tag: node_tag,
            attrs,
            children,
        } = node
        {
            if node_tag == tag {
                attrs.push(attribute);
                return true;
            }
            if add_attribute_to_first_element(children, tag, attribute.clone()) {
                return true;
            }
        }
    }
    false
}

fn add_attribute_to_first_cell(nodes: &mut [Node], cell_tag: &str, attribute: Attribute) -> bool {
    for node in nodes {
        if let Node::Element {
            tag,
            attrs,
            children,
        } = node
        {
            if tag == cell_tag {
                attrs.push(attribute);
                return true;
            }
            if add_attribute_to_first_cell(children, cell_tag, attribute.clone()) {
                return true;
            }
        }
    }
    false
}

pub fn validate_tree(
    root: &Node,
    options: &FuzzerOptions,
    profile: ParserProfile,
) -> Result<(), GeneratorError> {
    let Node::Fragment(children) = root else {
        return Err(GeneratorError::InvalidTree(
            "the generated root must be a fragment".to_string(),
        ));
    };
    if !(1..=8).contains(&children.len()) {
        return Err(GeneratorError::InvalidTree(
            "a body fragment must contain 1 to 8 top-level blocks".to_string(),
        ));
    }
    if profile == ParserProfile::Gfm && !children.iter().any(contains_translatable_table) {
        return Err(GeneratorError::InvalidTree(
            "every GFM case requires a translatable table".to_string(),
        ));
    }
    for child in children {
        validate_node(child, 1, None, false, options, profile)?;
        if let Node::Element { tag, .. } = child
            && !is_block_tag(tag)
        {
            return Err(GeneratorError::InvalidTree(format!(
                "top-level element {tag} is not a block"
            )));
        }
    }
    Ok(())
}

fn validate_node(
    node: &Node,
    depth: usize,
    parent: Option<&str>,
    in_anchor: bool,
    options: &FuzzerOptions,
    profile: ParserProfile,
) -> Result<(), GeneratorError> {
    if depth > options.max_depth {
        return Err(GeneratorError::InvalidTree(format!(
            "tree exceeds the maximum depth at depth {depth}"
        )));
    }
    match node {
        Node::Fragment(_) => Err(GeneratorError::InvalidTree(
            "fragments are permitted only at the root".to_string(),
        )),
        Node::Text(text) => {
            if text.chars().count() > options.max_text_scalars {
                return Err(GeneratorError::InvalidTree(
                    "text node exceeds the scalar-value limit".to_string(),
                ));
            }
            Ok(())
        }
        Node::Element {
            tag,
            attrs,
            children,
        } => {
            if !permitted_tag(tag, profile) {
                return Err(GeneratorError::InvalidTree(format!(
                    "element {tag} is not in the generator inventory"
                )));
            }
            if children.len() > options.max_children && !is_void_tag(tag) {
                return Err(GeneratorError::InvalidTree(format!(
                    "{tag} exceeds the child-count limit"
                )));
            }
            if is_void_tag(tag) && !children.is_empty() {
                return Err(GeneratorError::InvalidTree(format!(
                    "void element {tag} has children"
                )));
            }
            if tag == "a" && in_anchor {
                return Err(GeneratorError::InvalidTree(
                    "anchors may not be nested".to_string(),
                ));
            }
            validate_attributes(tag, attrs, parent, profile)?;
            for child in children {
                validate_node(
                    child,
                    depth + 1,
                    Some(tag),
                    in_anchor || tag == "a",
                    options,
                    profile,
                )?;
            }
            validate_content_model(tag, children, profile)?;
            Ok(())
        }
    }
}

fn validate_content_model(
    tag: &str,
    children: &[Node],
    profile: ParserProfile,
) -> Result<(), GeneratorError> {
    let tags: Vec<&str> = children
        .iter()
        .filter_map(|node| match node {
            Node::Element { tag, .. } => Some(tag.as_str()),
            Node::Text(_) => None,
            Node::Fragment(_) => Some("#fragment"),
        })
        .collect();
    let all_inline = children.iter().all(|node| match node {
        Node::Text(_) => true,
        Node::Element { tag, .. } => !is_block_tag(tag) && tag != "input",
        Node::Fragment(_) => false,
    });
    let valid = match tag {
        "ul" | "ol" => tags.iter().all(|child| *child == "li"),
        "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => all_inline,
        "pre" => children.iter().all(|child| match child {
            Node::Text(_) => true,
            Node::Element { tag, children, .. } if tag == "code" => {
                children.iter().all(|child| matches!(child, Node::Text(_)))
            }
            Node::Element { tag, .. } => !is_void_tag(tag),
            Node::Fragment(_) => false,
        }),
        "code" => children.iter().all(|child| matches!(child, Node::Text(_))),
        "a" | "strong" | "b" | "i" | "em" | "span" | "q" => all_inline,
        "table" => tags
            .iter()
            .all(|child| matches!(*child, "caption" | "thead" | "tbody" | "tfoot" | "tr")),
        "thead" | "tbody" | "tfoot" => tags.iter().all(|child| *child == "tr"),
        "tr" => tags.iter().all(|child| matches!(*child, "th" | "td")),
        "th" | "td" | "caption" => all_inline,
        "blockquote" | "div" | "section" => children.iter().all(|child| match child {
            Node::Text(text) => text.chars().all(|ch| ch.is_ascii_whitespace()),
            Node::Element { tag, .. } => is_block_tag(tag),
            Node::Fragment(_) => false,
        }),
        "li" => children.iter().all(|child| match child {
            Node::Text(_) => true,
            Node::Element { .. } => true,
            Node::Fragment(_) => false,
        }),
        "input" if profile == ParserProfile::Core => true,
        _ => true,
    };
    if valid {
        Ok(())
    } else {
        Err(GeneratorError::InvalidTree(format!(
            "children violate the {tag} content model"
        )))
    }
}

fn validate_attributes(
    tag: &str,
    attrs: &[Attribute],
    parent: Option<&str>,
    profile: ParserProfile,
) -> Result<(), GeneratorError> {
    let mut names = std::collections::BTreeSet::new();
    for attribute in attrs {
        if !names.insert(attribute.name.as_str()) {
            return Err(GeneratorError::InvalidTree(format!(
                "duplicate {} attribute on {tag}",
                attribute.name
            )));
        }
        let valid = match (tag, attribute.name.as_str()) {
            ("a", "href") => valid_url_or_path(&attribute.value),
            ("img", "src") => valid_url_or_path(&attribute.value),
            ("img", "alt") => attribute.value.is_empty() || is_plain_value(&attribute.value),
            ("ol", "start") => attribute
                .value
                .parse::<u32>()
                .is_ok_and(|n| (1..=4).contains(&n)),
            ("code", "class") if parent == Some("pre") => {
                matches!(attribute.value.as_str(), "language-rust" | "language-py3")
            }
            ("td" | "th", "colspan") => matches!(attribute.value.as_str(), "1" | "2"),
            ("input", "type") => {
                profile == ParserProfile::Gfm
                    && attribute.value == "checkbox"
                    && parent == Some("li")
            }
            (_, "id" | "class" | "title") => {
                attribute.value.is_empty() || is_plain_value(&attribute.value)
            }
            _ => false,
        };
        if !valid {
            return Err(GeneratorError::InvalidTree(format!(
                "attribute {} is not valid for {tag}",
                attribute.name
            )));
        }
    }
    if tag == "a" && !attrs.iter().any(|a| a.name == "href") {
        return Err(GeneratorError::InvalidTree(
            "generated hyperlinks require href".to_string(),
        ));
    }
    if tag == "img" && !attrs.iter().any(|a| a.name == "src") {
        return Err(GeneratorError::InvalidTree(
            "generated images require src".to_string(),
        ));
    }
    Ok(())
}

fn valid_url_or_path(value: &str) -> bool {
    !value.is_empty()
        && (value.starts_with("https://example.test/")
            || (!value.starts_with('/')
                && value
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '/' | '.' | '_' | '-'))))
}

fn is_plain_value(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
}

fn permitted_tag(tag: &str, profile: ParserProfile) -> bool {
    const COMMON: &[&str] = &[
        "blockquote",
        "ul",
        "ol",
        "li",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "hr",
        "p",
        "pre",
        "a",
        "img",
        "code",
        "strong",
        "b",
        "i",
        "em",
        "br",
        "span",
        "div",
        "section",
        "q",
        "input",
    ];
    const TABLE: &[&str] = &[
        "table", "caption", "thead", "tbody", "tfoot", "tr", "td", "th",
    ];
    COMMON.contains(&tag) || (profile == ParserProfile::Gfm && TABLE.contains(&tag))
}

fn contains_translatable_table(node: &Node) -> bool {
    match node {
        Node::Fragment(children) => children.iter().any(contains_translatable_table),
        Node::Text(_) => false,
        Node::Element {
            tag,
            attrs,
            children,
        } => {
            (tag == "table" && attrs.is_empty() && is_translatable_table_parts(children))
                || children.iter().any(contains_translatable_table)
        }
    }
}

fn is_translatable_table_parts(children: &[Node]) -> bool {
    !children.is_empty()
        && children.iter().all(|child| match child {
            Node::Element {
                tag,
                attrs,
                children,
            } if matches!(tag.as_str(), "thead" | "tbody") && attrs.is_empty() => {
                !children.is_empty()
                    && children.iter().all(|row| match row {
                        Node::Element {
                            tag,
                            attrs,
                            children,
                        } if tag == "tr" && attrs.is_empty() => {
                            !children.is_empty()
                                && children.iter().all(|cell| match cell {
                                    Node::Element {
                                        tag,
                                        attrs,
                                        children,
                                    } if matches!(tag.as_str(), "th" | "td")
                                        && attrs.is_empty() =>
                                    {
                                        children.iter().all(|text| matches!(text, Node::Text(_)))
                                    }
                                    _ => false,
                                })
                        }
                        _ => false,
                    })
            }
            _ => false,
        })
}

fn is_void_tag(tag: &str) -> bool {
    matches!(tag, "img" | "br" | "hr" | "input")
}

fn is_block_tag(tag: &str) -> bool {
    const BLOCK_ELEMENTS: &[&str] = &[
        "address",
        "article",
        "aside",
        "base",
        "basefont",
        "blockquote",
        "body",
        "caption",
        "center",
        "col",
        "colgroup",
        "dd",
        "details",
        "dialog",
        "dir",
        "div",
        "dl",
        "dt",
        "fieldset",
        "figcaption",
        "figure",
        "footer",
        "form",
        "frame",
        "frameset",
        "h1",
        "h2",
        "h3",
        "h4",
        "h5",
        "h6",
        "head",
        "header",
        "hr",
        "html",
        "iframe",
        "legend",
        "li",
        "link",
        "main",
        "menu",
        "menuitem",
        "nav",
        "noframes",
        "ol",
        "optgroup",
        "option",
        "p",
        "param",
        "pre",
        "script",
        "search",
        "section",
        "style",
        "summary",
        "table",
        "tbody",
        "td",
        "textarea",
        "tfoot",
        "th",
        "thead",
        "title",
        "tr",
        "track",
        "ul",
    ];
    BLOCK_ELEMENTS.contains(&tag)
}

pub fn serialize_html(root: &Node) -> String {
    let mut output = String::new();
    serialize_node(root, &mut output);
    output
}

fn serialize_node(node: &Node, output: &mut String) {
    match node {
        Node::Text(text) => escape_text(text, output),
        Node::Fragment(children) => {
            for child in children {
                serialize_node(child, output);
            }
        }
        Node::Element {
            tag,
            attrs,
            children,
        } => {
            output.push('<');
            output.push_str(tag);
            for attribute in attrs {
                output.push(' ');
                output.push_str(&attribute.name);
                output.push_str("=\"");
                escape_attribute(&attribute.value, output);
                output.push('"');
            }
            output.push('>');
            for child in children {
                serialize_node(child, output);
            }
            if !is_void_tag(tag) {
                output.push_str("</");
                output.push_str(tag);
                output.push('>');
            }
        }
    }
}

fn escape_text(text: &str, output: &mut String) {
    for ch in text.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            _ => output.push(ch),
        }
    }
}

fn escape_attribute(value: &str, output: &mut String) {
    for ch in value.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '"' => output.push_str("&quot;"),
            _ => output.push(ch),
        }
    }
}

fn corrupt_once(html: &str, rng: &mut CaseRng) -> String {
    let mode = rng.choose(3);
    match mode {
        0 => {
            if let Some(start) = html.rfind("</")
                && let Some(relative_end) = html[start..].find('>')
            {
                let mut malformed = html.to_string();
                malformed.replace_range(start..=start + relative_end, "");
                return malformed;
            }
            html.trim_end_matches('>').to_string()
        }
        1 => {
            if let Some(start) = html.find("</")
                && let Some(relative_end) = html[start..].find('>')
            {
                let mut malformed = html.to_string();
                malformed.replace_range(start..=start + relative_end, "</unknown>");
                return malformed;
            }
            format!("{html}<")
        }
        _ => {
            if let Some(start) = html.rfind('<')
                && let Some(end) = html[start..].find('>')
                && start + end + 1 < html.len()
            {
                return html[..start + end].to_string();
            }
            format!("{html}<unfinished")
        }
    }
}

fn faithful_options() -> Options {
    Options {
        heading_style: HeadingStyle::Atx,
        bullet_list_marker: BulletListMarker::Asterisk,
        code_block_style: CodeBlockStyle::Fenced,
        translation_mode: TranslationMode::Faithful,
        ..Options::default()
    }
}

fn markdown_options(profile: ParserProfile) -> MarkdownOptions {
    match profile {
        ParserProfile::Core => MarkdownOptions::empty(),
        ParserProfile::Gfm => {
            MarkdownOptions::ENABLE_TABLES
                | MarkdownOptions::ENABLE_STRIKETHROUGH
                | MarkdownOptions::ENABLE_TASKLISTS
        }
    }
}

pub fn run_case(case: &FuzzerCase) -> CaseOutcome {
    let guarded = catch_unwind(AssertUnwindSafe(|| run_case_inner(case)));
    match guarded {
        Ok(Ok(outcome)) => outcome,
        Ok(Err(message)) => CaseOutcome {
            category: Some(FailureCategory::UnexpectedError),
            input_html: case.source_html.clone(),
            generated_markdown: None,
            round_trip_html: None,
            first_difference: None,
            normalized_source: None,
            normalized_round_trip: None,
            detail: Some(message),
        },
        Err(_) => CaseOutcome {
            category: Some(FailureCategory::Panic),
            input_html: case.source_html.clone(),
            generated_markdown: None,
            round_trip_html: None,
            first_difference: None,
            normalized_source: None,
            normalized_round_trip: None,
            detail: Some("conversion or oracle panicked".to_string()),
        },
    }
}

fn run_case_inner(case: &FuzzerCase) -> Result<CaseOutcome, String> {
    let options = faithful_options();
    let converter = HtmlToMarkdown::builder().options(options).build();
    let first_markdown = converter
        .convert(&case.source_html)
        .map_err(|error| error.to_string())?;
    let second_markdown = converter
        .convert(&case.source_html)
        .map_err(|error| error.to_string())?;

    if first_markdown != second_markdown {
        return Ok(CaseOutcome {
            category: Some(FailureCategory::NonDeterminism),
            input_html: case.source_html.clone(),
            generated_markdown: Some(first_markdown),
            round_trip_html: None,
            first_difference: None,
            normalized_source: None,
            normalized_round_trip: None,
            detail: Some("two conversions produced different Markdown".to_string()),
        });
    }

    let Some(profile) = case.profile else {
        converter
            .html_to_tree(&case.source_html)
            .map_err(|error| error.to_string())?;
        return Ok(CaseOutcome {
            category: None,
            input_html: case.source_html.clone(),
            generated_markdown: Some(first_markdown),
            round_trip_html: None,
            first_difference: None,
            normalized_source: None,
            normalized_round_trip: None,
            detail: None,
        });
    };

    let mut round_trip_html = String::new();
    pulldown_cmark::html::push_html(
        &mut round_trip_html,
        Parser::new_ext(&first_markdown, markdown_options(profile)),
    );
    let source_tree = body_children(&converter, &case.source_html)?;
    let round_trip_tree = body_children(&converter, &round_trip_html)?;
    if contains_foreign_namespace(&source_tree) || contains_foreign_namespace(&round_trip_tree) {
        return Ok(CaseOutcome {
            category: Some(FailureCategory::OutOfScope),
            input_html: case.source_html.clone(),
            generated_markdown: Some(first_markdown),
            round_trip_html: Some(round_trip_html),
            first_difference: None,
            normalized_source: None,
            normalized_round_trip: None,
            detail: Some("a generated or rendered DOM contains a non-HTML namespace".to_string()),
        });
    }
    let normalized_source = normalize_tree(source_tree);
    let normalized_round_trip = normalize_tree(round_trip_tree);
    let differs = normalized_source != normalized_round_trip;
    Ok(CaseOutcome {
        category: differs.then_some(FailureCategory::RoundTripMismatch),
        input_html: case.source_html.clone(),
        generated_markdown: Some(first_markdown),
        round_trip_html: Some(round_trip_html),
        first_difference: differs
            .then(|| first_difference(&normalized_source, &normalized_round_trip, "$")),
        normalized_source: Some(normalized_source),
        normalized_round_trip: Some(normalized_round_trip),
        detail: None,
    })
}

fn body_children(converter: &HtmlToMarkdown, html: &str) -> Result<Vec<DomNode>, String> {
    let document = converter
        .html_to_tree(html)
        .map_err(|error| error.to_string())?;
    let html_node = document
        .children
        .borrow()
        .iter()
        .find(|node| is_element(node, "html"))
        .cloned()
        .ok_or_else(|| "oracle setup failure: HTML document element not found".to_string())?;
    let body = html_node
        .children
        .borrow()
        .iter()
        .find(|node| is_element(node, "body"))
        .cloned()
        .ok_or_else(|| "oracle setup failure: HTML body element not found".to_string())?;
    let owned = body
        .children
        .borrow()
        .iter()
        .map(to_dom_node)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(owned.into_iter().flatten().collect())
}

fn is_element(handle: &Handle, tag: &str) -> bool {
    matches!(
        &handle.data,
        NodeData::Element { name, .. }
            if name.ns.as_ref() == HTML_NAMESPACE && name.local.as_ref() == tag
    )
}

fn to_dom_node(handle: &Handle) -> Result<Option<DomNode>, String> {
    match &handle.data {
        NodeData::Text { contents } => Ok(Some(DomNode::Text(contents.borrow().to_string()))),
        NodeData::Comment { contents } => Ok(Some(DomNode::Comment(contents.to_string()))),
        NodeData::Element { name, attrs, .. } => {
            let attrs = attrs
                .borrow()
                .iter()
                .map(|attribute| {
                    (
                        format!("{}:{}", attribute.name.ns, attribute.name.local),
                        attribute.value.to_string(),
                    )
                })
                .collect::<Vec<_>>();
            let children = handle
                .children
                .borrow()
                .iter()
                .map(to_dom_node)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();
            Ok(Some(DomNode::Element {
                namespace: name.ns.to_string(),
                tag: name.local.to_string().to_ascii_lowercase(),
                attrs,
                children,
            }))
        }
        NodeData::Document => {
            let children = handle
                .children
                .borrow()
                .iter()
                .map(to_dom_node)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect();
            Ok(Some(DomNode::Element {
                namespace: String::new(),
                tag: "#document".to_string(),
                attrs: Vec::new(),
                children,
            }))
        }
        NodeData::Doctype { .. } | NodeData::ProcessingInstruction { .. } => Ok(None),
    }
}

fn contains_foreign_namespace(nodes: &[DomNode]) -> bool {
    nodes.iter().any(|node| match node {
        DomNode::Element {
            namespace,
            children,
            ..
        } => {
            (!namespace.is_empty() && namespace != HTML_NAMESPACE)
                || contains_foreign_namespace(children)
        }
        DomNode::Text(_) | DomNode::Comment(_) => false,
    })
}

fn normalize_tree(nodes: Vec<DomNode>) -> DomNode {
    DomNode::Element {
        namespace: String::new(),
        tag: "#body".to_string(),
        attrs: Vec::new(),
        children: normalize_children(nodes, None),
    }
}

fn normalize_children(mut children: Vec<DomNode>, parent: Option<&str>) -> Vec<DomNode> {
    for child in &mut children {
        let DomNode::Element {
            namespace,
            tag,
            attrs,
            children: nested,
        } = child
        else {
            if let DomNode::Text(text) = child {
                *text = collapse_ascii_whitespace(text);
            }
            continue;
        };
        attrs.sort();
        if tag == "ol" {
            attrs.retain(|(name, value)| !(name.ends_with(":start") && value == "1"));
        }
        if tag == "textarea" {
            continue;
        }
        if tag == "pre" {
            normalize_pre(attrs, nested);
            continue;
        }
        *nested = normalize_children(std::mem::take(nested), Some(tag));
        let _ = namespace;
    }

    remove_block_boundary_whitespace(&mut children);
    canonicalize_br_boundaries(&mut children);
    canonicalize_inline_boundaries(&mut children);
    if parent.is_none_or(is_block_tag) {
        trim_inline_run_edges(&mut children);
    }
    if parent == Some("li") {
        children = wrap_list_inline_runs(children);
    }
    merge_text_siblings(children)
}

fn normalize_pre(attrs: &mut Vec<(String, String)>, children: &mut [DomNode]) {
    attrs.sort();
    if children.len() != 1 {
        return;
    }
    let DomNode::Element {
        tag,
        attrs: code_attrs,
        children: code_children,
        ..
    } = &mut children[0]
    else {
        return;
    };
    if tag != "code" {
        return;
    }
    if let Some(DomNode::Text(text)) = code_children.last_mut() {
        if text.ends_with("\r\n") {
            text.truncate(text.len() - 2);
        } else if text.ends_with('\n') {
            text.pop();
        }
    }
    code_attrs.sort();
    let pre_language = take_single_language_class(attrs);
    let code_language = take_single_language_class(code_attrs);
    match (pre_language, code_language) {
        (Some((pre_key, pre_class)), Some((code_key, code_class))) if pre_class == code_class => {
            code_attrs.push((code_key, code_class));
            let _ = pre_key;
        }
        (Some((key, class)), None) => code_attrs.push((key, class)),
        (pre, code) => {
            if let Some((key, class)) = pre {
                attrs.push((key, class));
            }
            if let Some((key, class)) = code {
                code_attrs.push((key, class));
            }
        }
    }
    attrs.sort();
    code_attrs.sort();
}

fn take_single_language_class(attrs: &mut Vec<(String, String)>) -> Option<(String, String)> {
    let index = attrs
        .iter()
        .position(|(name, value)| name.ends_with(":class") && value.starts_with("language-"))?;
    let (key, class) = attrs.remove(index);
    if class.split_ascii_whitespace().count() == 1 {
        Some((key, class))
    } else {
        attrs.push((key, class));
        None
    }
}

fn collapse_ascii_whitespace(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_whitespace = false;
    for ch in text.chars() {
        if ch.is_ascii_whitespace() {
            if !in_whitespace {
                result.push(' ');
            }
            in_whitespace = true;
        } else {
            result.push(ch);
            in_whitespace = false;
        }
    }
    result
}

fn remove_block_boundary_whitespace(children: &mut Vec<DomNode>) {
    let mut index = 0;
    while index < children.len() {
        let remove = match &children[index] {
            DomNode::Text(text) if text.chars().all(|ch| ch.is_ascii_whitespace()) => {
                let before_block = index == 0 || is_block_node(&children[index - 1]);
                let after_block =
                    index + 1 == children.len() || is_block_node(&children[index + 1]);
                before_block || after_block
            }
            _ => false,
        };
        if remove {
            children.remove(index);
        } else {
            index += 1;
        }
    }
}

fn is_block_node(node: &DomNode) -> bool {
    matches!(node, DomNode::Element { tag, .. } if is_block_tag(tag))
}

fn canonicalize_br_boundaries(children: &mut [DomNode]) {
    let mut index = 0;
    while index < children.len() {
        if matches!(&children[index], DomNode::Element { tag, .. } if tag == "br") {
            if index > 0 {
                trim_last_text(&mut children[index - 1], false);
            }
            if index + 1 < children.len() {
                trim_first_text(&mut children[index + 1], true);
            }
        }
        index += 1;
    }
}

fn canonicalize_inline_boundaries(children: &mut Vec<DomNode>) {
    let mut index = 0;
    while index + 1 < children.len() {
        if is_inline_node(&children[index]) && is_inline_node(&children[index + 1]) {
            let left_space = trim_last_text(&mut children[index], false);
            let right_space = trim_first_text(&mut children[index + 1], true);
            if left_space || right_space {
                children.insert(index + 1, DomNode::Text(" ".to_string()));
                index += 2;
                continue;
            }
        }
        index += 1;
    }
}

fn trim_inline_run_edges(children: &mut [DomNode]) {
    let mut start = 0;
    while start < children.len() {
        while start < children.len()
            && (!is_inline_node(&children[start])
                || matches!(&children[start], DomNode::Element { tag, .. } if tag == "br"))
        {
            start += 1;
        }
        let mut end = start;
        while end < children.len()
            && is_inline_node(&children[end])
            && !matches!(&children[end], DomNode::Element { tag, .. } if tag == "br")
        {
            end += 1;
        }
        if end > start {
            trim_first_text(&mut children[start], true);
            trim_last_text(&mut children[end - 1], false);
        }
        start = end.saturating_add(1);
    }
}

fn is_inline_node(node: &DomNode) -> bool {
    !is_block_node(node)
}

fn trim_first_text(node: &mut DomNode, leading: bool) -> bool {
    match node {
        DomNode::Text(text) => trim_text_boundary(text, leading),
        DomNode::Element { children, .. } => {
            for child in children {
                if trim_first_text(child, leading) {
                    return true;
                }
            }
            false
        }
        DomNode::Comment(_) => false,
    }
}

fn trim_last_text(node: &mut DomNode, leading: bool) -> bool {
    match node {
        DomNode::Text(text) => trim_text_boundary(text, leading),
        DomNode::Element { children, .. } => {
            for child in children.iter_mut().rev() {
                if trim_last_text(child, leading) {
                    return true;
                }
            }
            false
        }
        DomNode::Comment(_) => false,
    }
}

fn trim_text_boundary(text: &mut String, leading: bool) -> bool {
    let old_len = text.len();
    if leading {
        let trim = text
            .char_indices()
            .take_while(|(_, ch)| ch.is_ascii_whitespace())
            .map(|(index, ch)| index + ch.len_utf8())
            .last()
            .unwrap_or(0);
        text.drain(..trim);
    } else {
        let trim_start = text
            .char_indices()
            .rev()
            .take_while(|(_, ch)| ch.is_ascii_whitespace())
            .map(|(index, _)| index)
            .last()
            .unwrap_or(text.len());
        text.truncate(trim_start);
    }
    text.len() != old_len
}

fn wrap_list_inline_runs(children: Vec<DomNode>) -> Vec<DomNode> {
    let mut output = Vec::new();
    let mut run = Vec::new();
    for child in children {
        if is_inline_node(&child) {
            run.push(child);
        } else {
            if !run.is_empty() {
                output.push(canonical_paragraph(std::mem::take(&mut run)));
            }
            output.push(child);
        }
    }
    if !run.is_empty() {
        output.push(canonical_paragraph(run));
    }
    output
}

fn canonical_paragraph(children: Vec<DomNode>) -> DomNode {
    DomNode::Element {
        namespace: HTML_NAMESPACE.to_string(),
        tag: "p".to_string(),
        attrs: Vec::new(),
        children,
    }
}

fn merge_text_siblings(children: Vec<DomNode>) -> Vec<DomNode> {
    let mut output: Vec<DomNode> = Vec::with_capacity(children.len());
    for child in children {
        match child {
            DomNode::Text(text) if text.is_empty() => {}
            DomNode::Text(text) => {
                if let Some(DomNode::Text(previous)) = output.last_mut() {
                    previous.push_str(&text);
                } else {
                    output.push(DomNode::Text(text));
                }
            }
            other => output.push(other),
        }
    }
    output
}

fn first_difference(left: &DomNode, right: &DomNode, path: &str) -> String {
    if left == right {
        return path.to_string();
    }
    match (left, right) {
        (
            DomNode::Element {
                namespace: left_ns,
                tag: left_tag,
                attrs: left_attrs,
                children: left_children,
            },
            DomNode::Element {
                namespace: right_ns,
                tag: right_tag,
                attrs: right_attrs,
                children: right_children,
            },
        ) => {
            if left_ns != right_ns {
                return format!("{path}.namespace");
            }
            if left_tag != right_tag {
                return format!("{path}.tag");
            }
            if left_attrs != right_attrs {
                return format!("{path}.attributes");
            }
            let common = left_children.len().min(right_children.len());
            for index in 0..common {
                if left_children[index] != right_children[index] {
                    return first_difference(
                        &left_children[index],
                        &right_children[index],
                        &format!("{path}.children[{index}]"),
                    );
                }
            }
            format!("{path}.children[{common}]")
        }
        _ => path.to_string(),
    }
}

pub fn write_failure_artifact(
    case: &FuzzerCase,
    outcome: &CaseOutcome,
    options: &FuzzerOptions,
    path: impl AsRef<Path>,
) -> io::Result<()> {
    let lockfile_identity = fs::read("Cargo.lock")
        .map(|bytes| format!("fnv1a64:{:016x}", fnv1a64(&bytes)))
        .unwrap_or_else(|_| "unavailable".to_string());
    let artifact = serde_json::json!({
        "master_seed": case.master_seed,
        "case_index": case.case_index,
        "class": case.class,
        "parser_profile": case.profile,
        "original_html": case.source_html,
        "effective_converter_options": format!("{:?}", faithful_options()),
        "generator_version": GENERATOR_VERSION,
        "text_pool_version": TEXT_POOL_VERSION,
        "generator_configuration": options,
        "versions": {
            "htmd": env!("CARGO_PKG_VERSION"),
            "html5ever": "0.39",
            "pulldown-cmark": "0.13.4",
            "rand": "0.9.2",
            "rand_chacha": "0.9.0"
        },
        "htmd_source_commit": option_env!("GIT_COMMIT").unwrap_or("unknown"),
        "lockfile_identity": lockfile_identity,
        "generated_markdown": outcome.generated_markdown,
        "round_trip_html": outcome.round_trip_html,
        "normalized_source_dom": outcome.normalized_source,
        "normalized_round_trip_dom": outcome.normalized_round_trip,
        "first_differing_path": outcome.first_difference,
        "failure_category": outcome.category,
        "detail": outcome.detail,
        "memory_limit_mode": "not enforced by this in-process harness",
        "memory_limit_mib": options.memory_limit_mib,
        "timeout_seconds": options.timeout_secs
    });
    fs::write(path, serde_json::to_vec_pretty(&artifact)?)
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_cases_replay_from_their_seed_and_index() {
        for profile in [ParserProfile::Core, ParserProfile::Gfm] {
            let mut sequence = Fuzzer::new(42, 0, profile);
            for index in 0..100 {
                let sequential = sequence.generate_valid_case().unwrap();
                let isolated = Fuzzer::new(42, index, profile)
                    .generate_valid_case()
                    .unwrap();
                assert_eq!(sequential.case_index, index);
                assert_eq!(sequential.master_seed, 42);
                assert_eq!(sequential.input, isolated.input);
                assert_eq!(sequential.source_html, isolated.source_html);
            }
        }
    }

    #[test]
    fn valid_generation_obeys_bounds_and_gfm_profile() {
        for profile in [ParserProfile::Core, ParserProfile::Gfm] {
            let case = Fuzzer::new(0, 0, profile).generate_valid_case().unwrap();
            validate_tree(&case.input, &FuzzerOptions::default(), profile).unwrap();
            assert!(case.source_html.len() <= MAX_FRAGMENT_BYTES);
            assert_eq!(case.profile, Some(profile));
        }
    }

    #[test]
    fn escaping_is_single_pass_and_void_tags_have_no_end_tag() {
        let input = Node::Fragment(vec![
            element("p", vec![Node::Text("&<>\r\n".to_string())]),
            element("br", vec![]),
        ]);
        assert_eq!(serialize_html(&input), "<p>&amp;&lt;&gt;\r\n</p><br>");
    }

    #[test]
    fn malformed_generation_changes_the_serialized_source() {
        let mut fuzzer = Fuzzer::new(7, 0, ParserProfile::Core);
        let valid = fuzzer.generate_valid_case().unwrap();
        let malformed = fuzzer.generate_malformed_case().unwrap();
        assert_eq!(malformed.class, CaseClass::Malformed);
        assert_eq!(malformed.profile, None);
        assert_ne!(valid.source_html, malformed.source_html);
    }

    #[test]
    fn oracle_normalizes_list_wrappers_and_empty_text() {
        let bare = normalize_tree(vec![DomNode::Element {
            namespace: HTML_NAMESPACE.to_string(),
            tag: "li".to_string(),
            attrs: Vec::new(),
            children: vec![
                DomNode::Text("".to_string()),
                DomNode::Text(" item ".to_string()),
            ],
        }]);
        let wrapped = normalize_tree(vec![DomNode::Element {
            namespace: HTML_NAMESPACE.to_string(),
            tag: "li".to_string(),
            attrs: Vec::new(),
            children: vec![canonical_paragraph(vec![DomNode::Text("item".to_string())])],
        }]);
        assert_eq!(bare, wrapped);
    }

    #[test]
    fn oracle_canonicalizes_inline_and_br_whitespace_boundaries() {
        let wrapped_left = normalize_children(
            vec![
                DomNode::Text("a".to_string()),
                DomNode::Element {
                    namespace: HTML_NAMESPACE.to_string(),
                    tag: "em".to_string(),
                    attrs: Vec::new(),
                    children: vec![DomNode::Text(" b".to_string())],
                },
            ],
            None,
        );
        assert_eq!(
            wrapped_left,
            vec![
                DomNode::Text("a ".to_string()),
                DomNode::Element {
                    namespace: HTML_NAMESPACE.to_string(),
                    tag: "em".to_string(),
                    attrs: Vec::new(),
                    children: vec![DomNode::Text("b".to_string())],
                },
            ]
        );

        let around_br = normalize_children(
            vec![
                DomNode::Text("a ".to_string()),
                DomNode::Element {
                    namespace: HTML_NAMESPACE.to_string(),
                    tag: "br".to_string(),
                    attrs: Vec::new(),
                    children: Vec::new(),
                },
                DomNode::Text(" b".to_string()),
            ],
            None,
        );
        assert_eq!(
            around_br,
            vec![
                DomNode::Text("a".to_string()),
                DomNode::Element {
                    namespace: HTML_NAMESPACE.to_string(),
                    tag: "br".to_string(),
                    attrs: Vec::new(),
                    children: Vec::new(),
                },
                DomNode::Text("b".to_string()),
            ]
        );
    }

    #[test]
    fn oracle_preserves_opaque_text_and_applies_code_block_exceptions() {
        let opaque = DomNode::Element {
            namespace: HTML_NAMESPACE.to_string(),
            tag: "textarea".to_string(),
            attrs: Vec::new(),
            children: vec![DomNode::Text(" a  b\r\n".to_string())],
        };
        assert_eq!(normalize_children(vec![opaque.clone()], None), vec![opaque]);

        let pre = DomNode::Element {
            namespace: HTML_NAMESPACE.to_string(),
            tag: "pre".to_string(),
            attrs: vec![(":class".to_string(), "language-rust".to_string())],
            children: vec![DomNode::Element {
                namespace: HTML_NAMESPACE.to_string(),
                tag: "code".to_string(),
                attrs: vec![(":class".to_string(), "language-rust".to_string())],
                children: vec![DomNode::Text("x\r\n\r\n".to_string())],
            }],
        };
        let normalized = normalize_children(vec![pre], None);
        let DomNode::Element {
            attrs, children, ..
        } = &normalized[0]
        else {
            panic!("expected pre")
        };
        assert!(attrs.is_empty());
        let DomNode::Element {
            attrs: code_attrs,
            children: code_children,
            ..
        } = &children[0]
        else {
            panic!("expected code")
        };
        assert_eq!(code_attrs.len(), 1);
        assert_eq!(code_children, &vec![DomNode::Text("x\r\n".to_string())]);
    }

    #[test]
    fn oracle_ignores_start_one_and_sorts_attribute_sets() {
        let list = DomNode::Element {
            namespace: HTML_NAMESPACE.to_string(),
            tag: "ol".to_string(),
            attrs: vec![
                (":title".to_string(), "t".to_string()),
                (":start".to_string(), "1".to_string()),
            ],
            children: Vec::new(),
        };
        assert_eq!(
            normalize_children(vec![list], None),
            vec![DomNode::Element {
                namespace: HTML_NAMESPACE.to_string(),
                tag: "ol".to_string(),
                attrs: vec![(":title".to_string(), "t".to_string())],
                children: Vec::new(),
            }]
        );
    }

    #[test]
    fn wrapping_case_seed_is_replayable_at_u64_boundary() {
        let mut sequence = Fuzzer::new(u64::MAX, u64::MAX, ParserProfile::Core);
        let first = sequence.generate_valid_case().unwrap();
        let second = sequence.generate_valid_case().unwrap();
        assert_eq!(first.case_index, u64::MAX);
        assert_eq!(second.case_index, 0);
        assert_eq!(
            second.input,
            Fuzzer::new(u64::MAX, 0, ParserProfile::Core)
                .generate_valid_case()
                .unwrap()
                .input
        );
    }

    #[test]
    fn validator_accepts_depth_six_and_rejects_depth_seven() {
        fn nested_divs(count: usize) -> Node {
            let mut node = element("p", vec![Node::Text("x".to_string())]);
            for _ in 0..count {
                node = element("div", vec![node]);
            }
            Node::Fragment(vec![node])
        }

        assert!(
            validate_tree(
                &nested_divs(4),
                &FuzzerOptions::default(),
                ParserProfile::Core
            )
            .is_ok()
        );
        assert!(
            validate_tree(
                &nested_divs(5),
                &FuzzerOptions::default(),
                ParserProfile::Core
            )
            .is_err()
        );
    }

    #[test]
    fn validator_rejects_content_model_and_void_element_violations() {
        let invalid_paragraph = Node::Fragment(vec![element(
            "p",
            vec![element("div", vec![element("p", vec![])])],
        )]);
        let invalid_void = Node::Fragment(vec![element(
            "p",
            vec![element("br", vec![Node::Text("child".to_string())])],
        )]);
        assert!(
            validate_tree(
                &invalid_paragraph,
                &FuzzerOptions::default(),
                ParserProfile::Core
            )
            .is_err()
        );
        assert!(
            validate_tree(
                &invalid_void,
                &FuzzerOptions::default(),
                ParserProfile::Core
            )
            .is_err()
        );
    }

    #[test]
    fn byte_limit_rejects_without_truncating_or_retrying_forever() {
        assert!(within_byte_limit(&"x".repeat(4096), 4096));
        assert!(!within_byte_limit(&"x".repeat(4097), 4096));

        let options = FuzzerOptions {
            max_fragment_bytes: 1,
            max_candidates: 100,
            ..FuzzerOptions::default()
        };
        let error = Fuzzer::with_options(0, 0, ParserProfile::Core, options)
            .generate_valid_case()
            .unwrap_err();
        assert_eq!(
            error,
            GeneratorError::CandidateLimitExceeded {
                max_bytes: 1,
                attempts: 100
            }
        );
    }

    #[test]
    fn oversized_candidates_are_discarded_without_truncation() {
        let mut candidates = [
            ("oversized", "12345".to_string()),
            ("accepted", "1234".to_string()),
        ]
        .into_iter();
        let (candidate, fragment) = first_fitting_candidate(4, 2, || {
            let (candidate, fragment) = candidates.next().unwrap();
            Ok((candidate, fragment))
        })
        .unwrap();
        assert_eq!(candidate, "accepted");
        assert_eq!(fragment, "1234");
    }

    #[test]
    fn actual_converter_and_dom_oracle_run_on_a_predefined_input() {
        let case = FuzzerCase {
            class: CaseClass::Valid,
            profile: Some(ParserProfile::Core),
            master_seed: 1,
            case_index: 0,
            input: element("p", vec![Node::Text("Hello".to_string())]),
            source_html: "<p>Hello</p>".to_string(),
        };
        let outcome = run_case(&case);
        assert_eq!(outcome.category, None, "{outcome:?}");
        assert_eq!(outcome.generated_markdown.as_deref(), Some("Hello"));
    }

    #[test]
    fn generated_cases_reach_the_real_converter_and_dom_oracle() {
        for profile in [ParserProfile::Core, ParserProfile::Gfm] {
            let mut fuzzer = Fuzzer::new(42, 0, profile);
            for _ in 0..20 {
                let case = fuzzer.generate_valid_case().unwrap();
                let outcome = run_case(&case);
                assert!(outcome.generated_markdown.is_some(), "{outcome:?}");
                assert!(
                    !matches!(
                        outcome.category,
                        Some(FailureCategory::UnexpectedError | FailureCategory::Panic)
                    ),
                    "{outcome:?}"
                );
            }
        }
    }

    #[test]
    fn translatable_and_html_fallback_examples_follow_faithful_handlers() {
        let converter = HtmlToMarkdown::builder()
            .options(faithful_options())
            .build();
        let translated = [
            ("<a href=\"https://example.test/\">link</a>", "<a"),
            ("<img src=\"image.png\" alt=\"image\">", "<img"),
            ("<ol start=\"2\"><li>x</li></ol>", "<ol"),
            (
                "<pre><code class=\"language-rust\">fn main()</code></pre>",
                "<pre",
            ),
            ("<strong>x</strong>", "<strong"),
            (
                "<table><thead><tr><th>head</th></tr></thead><tbody><tr><td>cell</td></tr></tbody></table>",
                "<table",
            ),
        ];
        for (input, raw_tag) in translated {
            let markdown = converter.convert(input).unwrap();
            assert!(!markdown.contains(raw_tag), "{input}: {markdown}");
        }

        let serialized = [
            "<a href=\"https://example.test/\" id=\"x\">link</a>",
            "<img src=\"image.png\" alt=\"image\" class=\"x\">",
            "<ol class=\"x\"><li>x</li></ol>",
            "<pre id=\"x\"><code>text</code></pre>",
            "<strong class=\"x\">x</strong>",
            "<table><caption>caption</caption><thead><tr><th>head</th></tr></thead><tbody><tr><td>cell</td></tr></tbody></table>",
            "<span>x</span>",
            "<br>",
            "<div>x</div>",
            "<section><p>x</p></section>",
            "<q>x</q>",
            "<input id=\"x\">",
        ];
        for input in serialized {
            let markdown = converter.convert(input).unwrap();
            assert!(markdown.contains('<'), "{input}: {markdown}");
        }
    }

    #[test]
    fn generator_self_tests_are_reproducible_for_one_hundred_cases() {
        for profile in [ParserProfile::Core, ParserProfile::Gfm] {
            let mut first = Fuzzer::new(0, 0, profile);
            let mut second = Fuzzer::new(0, 0, profile);
            for _ in 0..100 {
                let first_case = first.generate_valid_case().unwrap();
                let second_case = second.generate_valid_case().unwrap();
                assert_eq!(first_case.source_html, second_case.source_html);
            }
        }
    }

    #[test]
    fn default_seeds_generate_each_valid_and_malformed_class() {
        for seed in DEFAULT_SEEDS {
            for profile in [ParserProfile::Core, ParserProfile::Gfm] {
                let mut valid = Fuzzer::new(seed, 0, profile);
                let mut coverage = String::new();
                for _ in 0..1000 {
                    let case = valid.generate_valid_case().unwrap();
                    assert_eq!(case.class, CaseClass::Valid);
                    assert_eq!(case.profile, Some(profile));
                    assert!(case.source_html.len() <= MAX_FRAGMENT_BYTES);
                    coverage.push_str(&case.source_html);
                }
                for tag in [
                    "blockquote",
                    "ul",
                    "ol",
                    "li",
                    "h1",
                    "h2",
                    "h3",
                    "h4",
                    "h5",
                    "h6",
                    "hr",
                    "p",
                    "pre",
                    "a",
                    "img",
                    "code",
                    "strong",
                    "b",
                    "i",
                    "em",
                    "br",
                    "span",
                    "div",
                    "section",
                    "q",
                    "input",
                ] {
                    assert!(
                        coverage.contains(&format!("<{tag}")),
                        "{seed} {profile:?} {tag}"
                    );
                }
                if profile == ParserProfile::Core {
                    assert!(!coverage.contains("<table"), "{seed}");
                } else {
                    for tag in [
                        "table", "caption", "thead", "tbody", "tfoot", "tr", "td", "th",
                    ] {
                        assert!(coverage.contains(&format!("<{tag}")), "{seed} {tag}");
                    }
                    assert!(coverage.contains("colspan=\"2\""), "{seed}");
                    assert!(coverage.contains("type=\"checkbox\""), "{seed}");
                }
            }

            let mut malformed = Fuzzer::new(seed, 0, ParserProfile::Core);
            for _ in 0..1000 {
                let case = malformed.generate_malformed_case().unwrap();
                assert_eq!(case.class, CaseClass::Malformed);
                assert_eq!(case.profile, None);
                assert!(case.source_html.len() <= MAX_FRAGMENT_BYTES);
            }
        }
    }
}
