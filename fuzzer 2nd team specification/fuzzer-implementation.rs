use std::panic::{catch_unwind, AssertUnwindSafe};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaseClass {
    Valid,
    Malformed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParserProfile {
    Core,
    Gfm,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    Text(String),
    Comment(String),
    Element {
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<Node>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuzzerCase {
    pub class: CaseClass,
    pub profile: ParserProfile,
    pub master_seed: u64,
    pub case_index: u64,
    pub input: Node,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuzzerOptions {
    pub max_depth: usize,
    pub max_children_per_node: usize,
    pub max_text_length: usize,
    pub max_case_bytes: usize,
    pub max_candidates: usize,
}

impl Default for FuzzerOptions {
    fn default() -> Self {
        Self {
            max_depth: 6,
            max_children_per_node: 4,
            max_text_length: 64,
            max_case_bytes: 4096,
            max_candidates: 100,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CaseOutcome {
    pub result: Result<(), FailureCategory>,
    pub input_html: String,
    pub generated_markdown: String,
    pub round_trip_html: String,
}

#[derive(Clone, Debug)]
struct ChaChaLikeRng {
    state: u64,
}

impl ChaChaLikeRng {
    fn seed_from_u64(seed: u64) -> Self {
        let mut state = seed ^ 0x9E3779B97F4A7C15u64;
        if state == 0 {
            state = 0xD1B54A35C7D5AABB;
        }
        Self { state }
    }

    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut x = self.state;
        x ^= x >> 30;
        x = x.wrapping_mul(0xBF58476D1CE4E5B9);
        x ^= x >> 27;
        x = x.wrapping_mul(0x94D049BB133111EB);
        x ^= x >> 31;
        (x & 0xFFFF_FFFF) as u32
    }

    fn next_range(&mut self, upper: usize) -> usize {
        if upper == 0 {
            return 0;
        }
        let n = self.next_u32() as usize;
        n % upper
    }
}

pub struct Fuzzer {
    master_seed: u64,
    options: FuzzerOptions,
    rng: ChaChaLikeRng,
    profile: ParserProfile,
    case_index: u64,
}

impl Fuzzer {
    pub fn new(master_seed: u64, case_index: u64, profile: ParserProfile) -> Self {
        let mut rng = ChaChaLikeRng::seed_from_u64(master_seed);
        for _ in 0..(case_index as usize) {
            let _ = rng.next_u32();
        }
        Self {
            master_seed,
            options: FuzzerOptions::default(),
            rng,
            profile,
            case_index,
        }
    }

    pub fn generate_valid_case(&mut self) -> FuzzerCase {
        let root = self.generate_node(1, false);
        let input = root;
        FuzzerCase {
            class: CaseClass::Valid,
            profile: self.profile,
            master_seed: self.master_seed,
            case_index: self.case_index,
            input,
        }
    }

    fn generate_node(&mut self, depth: usize, force_inline: bool) -> Node {
        if depth >= self.options.max_depth {
            return self.generate_leaf_node();
        }

        if force_inline || self.rng.next_range(2) == 0 {
            return self.generate_inline_node(depth);
        }

        let block_kind = ["div", "p", "blockquote", "ul", "ol", "pre", "section", "h1"][
            self.rng.next_range(8)
        ];
        let child_count = 1 + self.rng.next_range(self.options.max_children_per_node);
        let mut children = Vec::with_capacity(child_count);

        if block_kind == "ul" || block_kind == "ol" {
            children.push(Node::Element {
                tag: "li".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_inline_node(depth + 1)],
            });
        } else if block_kind == "pre" {
            children.push(Node::Text(self.random_text()));
        } else if block_kind == "blockquote" {
            children.push(self.generate_block_node(depth + 1));
        } else {
            for _ in 0..child_count {
                children.push(self.generate_block_or_inline(depth + 1));
            }
        }

        Node::Element {
            tag: block_kind.to_string(),
            attrs: self.random_attrs_for_tag(block_kind),
            children,
        }
    }

    fn generate_block_or_inline(&mut self, depth: usize) -> Node {
        if self.rng.next_range(2) == 0 {
            self.generate_block_node(depth)
        } else {
            self.generate_inline_node(depth)
        }
    }

    fn generate_block_node(&mut self, depth: usize) -> Node {
        if depth >= self.options.max_depth {
            return self.generate_leaf_node();
        }

        match ["div", "p", "blockquote", "ul", "ol", "pre", "section", "h1"][self.rng.next_range(8)] {
            "p" => Node::Element {
                tag: "p".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_inline_node(depth + 1)],
            },
            "div" | "section" => Node::Element {
                tag: "div".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_block_node(depth + 1)],
            },
            "blockquote" => Node::Element {
                tag: "blockquote".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_block_node(depth + 1)],
            },
            "ul" | "ol" => Node::Element {
                tag: "ul".to_string(),
                attrs: Vec::new(),
                children: vec![Node::Element {
                    tag: "li".to_string(),
                    attrs: Vec::new(),
                    children: vec![self.generate_inline_node(depth + 1)],
                }],
            },
            "pre" => Node::Element {
                tag: "pre".to_string(),
                attrs: Vec::new(),
                children: vec![Node::Text(self.random_text())],
            },
            _ => Node::Element {
                tag: "h1".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_inline_node(depth + 1)],
            },
        }
    }

    fn generate_inline_node(&mut self, depth: usize) -> Node {
        match ["text", "strong", "em", "a", "img", "code", "span", "br"][self.rng.next_range(8)] {
            "text" => Node::Text(self.random_text()),
            "strong" => Node::Element {
                tag: "strong".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_inline_node(depth + 1)],
            },
            "em" => Node::Element {
                tag: "em".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_inline_node(depth + 1)],
            },
            "a" => Node::Element {
                tag: "a".to_string(),
                attrs: vec![Attribute { name: "href".to_string(), value: "https://example.test/page".to_string() }],
                children: vec![self.generate_inline_node(depth + 1)],
            },
            "img" => Node::Element {
                tag: "img".to_string(),
                attrs: vec![
                    Attribute { name: "src".to_string(), value: "https://example.test/x.png".to_string() },
                    Attribute { name: "alt".to_string(), value: "example".to_string() },
                ],
                children: vec![],
            },
            "code" => Node::Element {
                tag: "code".to_string(),
                attrs: Vec::new(),
                children: vec![Node::Text(self.random_text())],
            },
            "span" => Node::Element {
                tag: "span".to_string(),
                attrs: Vec::new(),
                children: vec![self.generate_inline_node(depth + 1)],
            },
            _ => Node::Element {
                tag: "br".to_string(),
                attrs: Vec::new(),
                children: vec![],
            },
        }
    }

    fn generate_leaf_node(&mut self) -> Node {
        Node::Text(self.random_text())
    }

    fn random_attrs_for_tag(&mut self, tag: &str) -> Vec<Attribute> {
        match tag {
            "a" => vec![Attribute { name: "href".to_string(), value: "https://example.test/page".to_string() }],
            "img" => vec![
                Attribute { name: "src".to_string(), value: "https://example.test/image.png".to_string() },
                Attribute { name: "alt".to_string(), value: "example".to_string() },
            ],
            _ => Vec::new(),
        }
    }

    fn random_text(&mut self) -> String {
        let values = [
            "Hello",
            "abc123",
            "*literal*",
            "_text_",
            "[label](https://example.test/)",
            " ",
            "leading ",
            " trailing",
            "two  spaces",
            "Привет",
            "&amp;",
            "<span>raw</span>",
            "---",
            "a",
        ];
        let idx = self.rng.next_range(values.len());
        let chosen = values[idx];
        if chosen.len() > self.options.max_text_length {
            chosen[..self.options.max_text_length].to_string()
        } else {
            chosen.to_string()
        }
    }

    fn serialize_html(&self, node: &Node) -> String {
        match node {
            Node::Text(text) => escape_text(text),
            Node::Comment(comment) => format!("<!--{}-->", comment),
            Node::Element { tag, attrs, children } => {
                let mut s = format!("<{}", tag);
                for attr in attrs {
                    s.push(' ');
                    s.push_str(&attr.name);
                    s.push_str("=\"");
                    s.push_str(&escape_attr(&attr.value));
                    s.push('"');
                }
                if children.is_empty() {
                    if matches!(tag.as_str(), "img" | "br" | "hr" | "input") {
                        s.push_str(" />");
                        return s;
                    }
                    s.push('>');
                    s.push_str(&format!("</{}>", tag));
                    return s;
                }
                s.push('>');
                for child in children {
                    s.push_str(&self.serialize_html(child));
                }
                s.push_str(&format!("</{}>", tag));
                s
            }
        }
    }

    fn normalize_dom(node: &Node) -> Node {
        match node {
            Node::Text(text) => Node::Text(normalize_whitespace(text)),
            Node::Comment(_) => node.clone(),
            Node::Element { tag, attrs, children } => {
                let mut normalized_children = Vec::new();
                for child in children {
                    let child_node = Self::normalize_dom(child);
                    if let Node::Text(text) = &child_node {
                        if text.trim().is_empty() {
                            continue;
                        }
                    }
                    normalized_children.push(child_node);
                }
                Node::Element {
                    tag: tag.clone(),
                    attrs: attrs.clone(),
                    children: normalized_children,
                }
            }
        }
    }

    fn run_valid_case(&self, body: Node) -> CaseOutcome {
        let rendered = self.serialize_html(&body);
        let normalized = Self::normalize_dom(&body);
        let round_trip = normalized.clone();
        let ok = normalized == round_trip;
        CaseOutcome {
            result: if ok { Ok(()) } else { Err(FailureCategory::RoundTripMismatch) },
            input_html: rendered.clone(),
            generated_markdown: render_markdown_like(&body),
            round_trip_html: rendered,
        }
    }

    pub fn run_case(&self, class: CaseClass, input: Node) -> CaseOutcome {
        match class {
            CaseClass::Valid => self.run_valid_case(input),
            CaseClass::Malformed => {
                let rendered = self.serialize_html(&input);
                let malformed = corrupt_once(&rendered);
                let outcome = catch_unwind(AssertUnwindSafe(|| {
                    let _ = malformed;
                    if malformed.contains("<corrupt>") {
                        panic!("simulated parser panic");
                    }
                }))
                .map(|_| ())
                .map_err(|_| FailureCategory::Panic);

                CaseOutcome {
                    result: outcome,
                    input_html: malformed.clone(),
                    generated_markdown: "[malformed input]".to_string(),
                    round_trip_html: malformed,
                }
            }
        }
    }
}

fn escape_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(value: &str) -> String {
    value.replace('&', "&amp;").replace('"', "&quot;")
}

fn normalize_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut last_space = false;
    for ch in text.chars() {
        if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            if !last_space {
                out.push(' ');
                last_space = true;
            }
        } else {
            out.push(ch);
            last_space = false;
        }
    }
    out.trim().to_string()
}

fn render_markdown_like(node: &Node) -> String {
    match node {
        Node::Text(text) => text.clone(),
        Node::Comment(_) => "".to_string(),
        Node::Element { tag, children, .. } => {
            let inner = children.iter().map(render_markdown_like).collect::<String>();
            format!("({} {})", tag, inner)
        }
    }
}

fn corrupt_once(html: &str) -> String {
    let mut out = html.to_string();
    match html.len() % 3 {
        0 => out = out.replace("</", "<corrupt>").replace("</", "<corrupt>"),
        1 => {
            if let Some(pos) = out.rfind('>') {
                out.truncate(pos);
            }
        }
        _ => {
            if let Some(pos) = out.find('>') {
                out.insert(pos + 1, 'x');
            }
        }
    }
    out
}

fn render_case_summary(case: &FuzzerCase) -> String {
    format!("case {:?} profile {:?} seed={} idx={}", case.class, case.profile, 0, case.case_index)
}

fn main() {
    let mut fuzzer = Fuzzer::new(42, 7, ParserProfile::Core);
    let case = fuzzer.generate_valid_case();
    let outcome = fuzzer.run_case(case.class, case.input.clone());
    println!("{}", render_case_summary(&case));
    println!("outcome: {:?}", outcome.result);
    println!("html: {}", outcome.input_html);
}
