//! The Examples menu (spec, Examples).

use crate::settings::{Direction, Extension};

/// One entry in the Examples menu.
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    pub name: &'static str,
    pub text: &'static str,
    /// The extension this preset demonstrates; picking the preset enables it.
    pub extension: Option<Extension>,
}

const fn html(name: &'static str, text: &'static str) -> Preset {
    Preset {
        name,
        text,
        extension: None,
    }
}

/// The HTML → Markdown presets. The first is the default example.
pub const HTML_PRESETS: &[Preset] = &[
    html(
        "Basic",
        "<p>Converters turn <strong>bold</strong> and <em>emphasized</em> HTML into Markdown.</p>\n",
    ),
    html(
        "Headings",
        "<h1>Main title</h1>\n<p>An introduction.</p>\n<h2>A section</h2>\n<p>Some text in the section.</p>\n<h3>A subsection</h3>\n<p>More detail.</p>\n",
    ),
    html(
        "Lists",
        "<ul>\n  <li>Fruit\n    <ul>\n      <li>Apples</li>\n      <li>Pears</li>\n    </ul>\n  </li>\n  <li>Vegetables\n    <ul>\n      <li>Carrots</li>\n    </ul>\n  </li>\n</ul>\n<ol start=\"3\">\n  <li>Third step</li>\n  <li>Fourth step</li>\n  <li>Fifth step</li>\n</ol>\n",
    ),
    html(
        "Links and images",
        "<p>Visit <a href=\"https://example.com\">https://example.com</a> for details.</p>\n<p>Read the <a href=\"https://commonmark.org/help/\" title=\"Markdown reference\">CommonMark help</a> page.</p>\n<p><img src=\"https://example.com/cat.png\" alt=\"A sleeping cat\"></p>\n",
    ),
    html(
        "Code",
        "<p>Call <code>  convert(html)  </code> to get Markdown.</p>\n<pre><code class=\"language-rust\">fn main() {\n    let md = htmd::convert(\"&lt;h1&gt;Hi&lt;/h1&gt;\").unwrap();\n    println!(\"{md}\");\n}\n</code></pre>\n",
    ),
    html(
        "Tables",
        "<table>\n  <thead>\n    <tr><th>Crate</th><th>Converts</th></tr>\n  </thead>\n  <tbody>\n    <tr><td>htmd</td><td>HTML to Markdown</td></tr>\n    <tr><td>pulldown-cmark</td><td>Markdown to HTML</td></tr>\n  </tbody>\n</table>\n",
    ),
    html(
        "Blockquotes",
        "<blockquote>\n  <p>The outer quote.</p>\n  <blockquote>\n    <p>A quote inside the quote.</p>\n  </blockquote>\n  <p>Back in the outer quote.</p>\n</blockquote>\n",
    ),
    html(
        "Math",
        "<p>The area of a circle is <span class=\"math math-inline\">\\pi r^2</span>.</p>\n<p><span class=\"math math-display\">\\sum_{k=1}^{n} k = \\frac{n(n+1)}{2}</span></p>\n",
    ),
    html("Article", ARTICLE),
    html(
        "Edge cases",
        "<p></p>\n<p>Empty inline elements: [<em></em>] [<strong></strong>] [<span></span>]</p>\n<p>Entities: &amp; &lt;tag&gt; &quot;quoted&quot; &copy; &nbsp;non-breaking&nbsp; &#x1F600; &unknown;</p>\n<p>Runs     of\n\n   whitespace\t\tand\ttabs.</p>\n<!-- A comment, which is dropped. -->\n<p>Markdown characters: *stars* _underscores_ # hash [brackets] `backticks`</p>\n<script>console.log(\"script content\");</script>\n<style>p { color: red; }</style>\n<noscript><p>Shown when <b>scripting</b> is off.</p></noscript>\n",
    ),
];

const ARTICLE: &str = r#"<article>
<h1>Plain Text That Formats Itself</h1>
<p class="lead">Lightweight markup languages let people write structured documents with nothing more than a text editor. This short article looks at why they caught on and how converters move text between them and HTML.</p>
<h2>Why lightweight markup?</h2>
<p>Before such languages became common, writers who wanted headings, lists, and links on the web had two choices: write HTML by hand or use a visual editor. Hand-written HTML is precise but noisy, because every paragraph needs an opening and a closing tag. Visual editors hide the tags but make it hard to see what a document <em>really</em> contains.</p>
<p>Lightweight markup takes a middle path. It borrows conventions people already used in plain-text email:</p>
<ul>
<li>A line of text followed by a blank line is a paragraph.</li>
<li>Words wrapped in asterisks are emphasized.</li>
<li>Lines that start with a dash or a number form a list.
<ul>
<li>Indented lines nest inside the item above them.</li>
</ul>
</li>
</ul>
<p>The result reads naturally even before it is rendered, which is the property its users value most.</p>
<h2>From one format to another</h2>
<p>Most sites still store and serve HTML, so tools that translate in both directions are useful. A <strong>Markdown to HTML</strong> converter, such as <a href="https://github.com/pulldown-cmark/pulldown-cmark">pulldown-cmark</a>, parses the text into events and writes tags for them. An <strong>HTML to Markdown</strong> converter, such as <a href="https://crates.io/crates/htmd" title="htmd on crates.io">htmd</a>, walks the parsed document tree and writes the matching Markdown for each element.</p>
<p>The second direction is harder. HTML can express things Markdown cannot, such as attributes, colors, or arbitrary nesting, so a converter must decide what to keep and what to drop:</p>
<ol>
<li>Translate everything Markdown can express.</li>
<li>Drop the rest, or keep it as raw HTML inside the Markdown.</li>
<li>Escape characters which Markdown would otherwise read as syntax.</li>
</ol>
<hr>
<h2>A small example</h2>
<p>Here is how a program might convert a fragment of HTML:</p>
<pre><code class="language-rust">use htmd::HtmlToMarkdown;

fn main() {
    let converter = HtmlToMarkdown::new();
    let markdown = converter.convert("&lt;p&gt;Hello, &lt;em&gt;world&lt;/em&gt;!&lt;/p&gt;").unwrap();
    assert_eq!(markdown, "Hello, *world*!");
}
</code></pre>
<p>Round trips are a good test of a converter: convert HTML to Markdown, convert the Markdown back, and compare the two pages. Differences point to features that one side cannot express.</p>
<h2>Further reading</h2>
<p>The <a href="https://spec.commonmark.org/">CommonMark specification</a> defines the syntax precisely, with hundreds of examples.<br>Its companion, the <a href="https://spec.commonmark.org/dingus/">dingus</a>, converts text as you type.</p>
<p><small>This article was written for the htmd playground and is dedicated to the public domain (CC0 1.0).</small></p>
</article>
"#;

const fn markdown(name: &'static str, text: &'static str, extension: Option<Extension>) -> Preset {
    Preset {
        name,
        text,
        extension,
    }
}

/// The Markdown → HTML presets: Basic, then one per extension in
/// [`Extension::ALL`] order.
pub const MARKDOWN_PRESETS: &[Preset] = &[
    markdown(
        "Basic",
        "# A heading\n\nA paragraph with *emphasis*, **strong text**, `code`, and a [link](https://commonmark.org).\n\n* A list item\n* Another item\n\n> A blockquote.\n\n```\nA code block.\n```\n",
        None,
    ),
    markdown(
        "Tables",
        "| Crate | Converts | Stars |\n| --- | :---: | ---: |\n| htmd | HTML → Markdown | 1 |\n| pulldown-cmark | Markdown → HTML | 2 |\n",
        Some(Extension::Tables),
    ),
    markdown(
        "Math",
        "The area of a circle is $\\pi r^2$.\n\n$$\\sum_{k=1}^{n} k = \\frac{n(n+1)}{2}$$\n",
        Some(Extension::Math),
    ),
    markdown(
        "Footnotes",
        "A sentence with a footnote.[^note]\n\n[^note]: The footnote's text.\n",
        Some(Extension::Footnotes),
    ),
    markdown(
        "Old-style footnotes",
        "A defined footnote[^1] and an undefined one[^2].\n\n[^1]:\n\n    With old-style footnotes, this indented text\n    is a code block after the footnote.\n",
        Some(Extension::OldFootnotes),
    ),
    markdown(
        "Strikethrough",
        "This is ~~wrong~~ right.\n",
        Some(Extension::Strikethrough),
    ),
    markdown(
        "Task lists",
        "- [x] Write the spec\n- [x] Review it\n- [ ] Ship the playground\n",
        Some(Extension::TaskLists),
    ),
    markdown(
        "Smart punctuation",
        "\"Double\" and 'single' quotes -- an en dash --- an em dash... and an ellipsis.\n",
        Some(Extension::SmartPunctuation),
    ),
    markdown(
        "Heading attributes",
        "# A heading {#custom-id .highlight}\n\nThe heading has an ID and a class.\n",
        Some(Extension::HeadingAttributes),
    ),
    markdown(
        "YAML metadata blocks",
        "---\ntitle: An example\nauthor: Someone\n---\n\nThe block above is metadata.\n",
        Some(Extension::YamlMetadata),
    ),
    markdown(
        "+++ metadata blocks",
        "+++\ntitle = \"An example\"\nauthor = \"Someone\"\n+++\n\nThe block above is metadata.\n",
        Some(Extension::PlusesMetadata),
    ),
    markdown(
        "GFM",
        "> [!NOTE]\n> A GitHub-style note.\n\n> [!WARNING]\n> A warning.\n",
        Some(Extension::Gfm),
    ),
    markdown(
        "Definition lists",
        "Markdown\n: A lightweight markup language.\n\nHTML\n: The language of web pages.\n",
        Some(Extension::DefinitionLists),
    ),
    markdown(
        "Superscript",
        "This sentence has a ^raised^ word.\n",
        Some(Extension::Superscript),
    ),
    markdown(
        "Subscript",
        "This sentence has a ~lowered~ word.\n",
        Some(Extension::Subscript),
    ),
    markdown(
        "Wikilinks",
        "See [[Main Page]] or [[Help:Contents|the help pages]].\n",
        Some(Extension::Wikilinks),
    ),
];

/// The presets offered in `direction`.
pub fn presets(direction: Direction) -> &'static [Preset] {
    match direction {
        Direction::HtmlToMarkdown => HTML_PRESETS,
        Direction::MarkdownToHtml => MARKDOWN_PRESETS,
    }
}

/// The page's content when the address carries no saved state.
pub fn default_example() -> &'static str {
    HTML_PRESETS[0].text
}

/// Whether replacing `input` needs the user's confirmation: it is not empty
/// and differs from every preset.
pub fn replacing_needs_confirmation(input: &str) -> bool {
    !input.is_empty()
        && !HTML_PRESETS
            .iter()
            .chain(MARKDOWN_PRESETS)
            .any(|preset| preset.text == input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_markdown_preset_per_extension_in_order() {
        assert_eq!(MARKDOWN_PRESETS[0].name, "Basic");
        let extensions: Vec<_> = MARKDOWN_PRESETS[1..]
            .iter()
            .map(|preset| preset.extension.unwrap())
            .collect();
        assert_eq!(extensions, Extension::ALL);
        for preset in &MARKDOWN_PRESETS[1..] {
            assert_eq!(preset.name, preset.extension.unwrap().label());
        }
    }

    #[test]
    fn article_meets_the_spec() {
        assert!((2_000..10_000).contains(&ARTICLE.len()));
        for tag in ["<h2>", "<ul>", "<a ", "<hr>", "<br>", "<pre>"] {
            assert!(ARTICLE.contains(tag), "{tag}");
        }
        assert!(!ARTICLE.contains("<script") && !ARTICLE.contains("<style"));
    }

    #[test]
    fn confirmation_only_for_custom_input() {
        assert!(!replacing_needs_confirmation(""));
        assert!(!replacing_needs_confirmation(MARKDOWN_PRESETS[3].text));
        assert!(replacing_needs_confirmation("custom"));
    }
}
