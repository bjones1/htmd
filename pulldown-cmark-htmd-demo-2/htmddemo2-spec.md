htmd and pulldown-cmark playground
==================================

This document specifies what the playground does, as seen by its user. It does
not prescribe how the code is organized.

Purpose
-------

The playground is a web page for trying out two converters, in either
direction:

* **HTML → Markdown**, using htmd.
* **Markdown → HTML**, using
  [pulldown-cmark](https://pulldown-cmark.github.io/pulldown-cmark/).

It is modeled on the [CommonMark dingus](https://spec.commonmark.org/dingus/):
the user types on one side and sees the result on the other, updated as they
type. Both converters run in the browser; there is no server.

Page layout
-----------

From top to bottom:

1. A header with the title "htmd playground", the htmd and pulldown-cmark
   versions the page was built from, and links to the source repository and to
   htmd on docs.rs and crates.io.
2. A **direction switch** with two positions, "HTML → Markdown" (the default)
   and "Markdown → HTML".
3. The **options panel** (see Options).
4. The **input pane** on the left and the **output pane** on the right, each
   half the width. On *narrow screens* (under 800 px wide) the panes stack,
   input first.
5. A **status line** under the output pane.

### Direction switch

Changing the direction moves the current output into the input pane and
converts it the other way, so one click round-trips the text. The options are
kept. Undo of this change is not required.

The direction switch and **Copy** always act on output that is up to date with
the input, even if the user is still inside the typing pause. The exceptions
are under Limits and failures.

### Input pane

* A single plain-text editor, labelled "HTML" or "Markdown" to match the
  direction.
* An **Examples** menu, which replaces the input with a preset for the current
  direction (see Examples), and a **Clear** button, which empties the input.
  If the input is not empty and differs from every preset, both ask the user
  to confirm first.

### Output pane

The output pane has tabs:

| Tab | HTML → Markdown | Markdown → HTML |
| --- | --- | --- |
| **Source** (default) | the Markdown text | the HTML text |
| **Rendered** | the Markdown, converted back to HTML by pulldown-cmark and shown as a page | the HTML, shown as a page |
| **Original** | the input HTML, shown as a page | (tab not shown) |

In HTML → Markdown mode, comparing **Rendered** with **Original** shows how well
the HTML survived the round trip.

The selected tab stays selected across a direction switch; if it disappears,
**Source** is selected. While conversion is paused (see Limits and failures),
every tab keeps its last content.

The **Source** tab has a **Copy** button. If the browser does not allow
copying, the text is shown selected, with a message asking the user to copy it
manually.

### Status line

The status line shows the time the last conversion took, in milliseconds, and
the length of the output in characters (Unicode code points). The time covers
only the conversion shown in **Source**, not rendering the other tabs. It also
shows the notices described under Limits and failures and under Permalinks.

Conversion
----------

The output updates after the user pauses typing for about a tenth of a second,
and immediately after any change to an option.

Options
-------

The options panel shows a group of controls for each converter. It starts
collapsed on narrow screens and expanded otherwise; the user can expand or
collapse it. A **Reset to defaults** button restores every option
below without changing the input. Each control has a one-line help text, based
on the library's documentation where it exists.

### htmd options

These are shown only in HTML → Markdown mode. The defaults are those in the
table below, which match htmd 0.5.5's own defaults; if a later htmd changes a
default, the table still applies.

| Option | Control | Choices | Default | Shown |
| --- | --- | --- | --- | --- |
| Translation mode | dropdown | Pure, Faithful | Pure | always |
| Heading style | dropdown | ATX (`# Heading`), Setext (underlined) | ATX | always |
| Horizontal rule | dropdown | `* * *`, `- - -`, `_ _ _` | `* * *` | always |
| Line break | dropdown | two spaces, backslash | two spaces | disabled in Faithful mode, which always writes `<br>` |
| Bullet marker | dropdown | `*`, `-` | `*` | always |
| Spaces after bullet | number | 1–4 | 3 | always |
| Spaces after list number | number | 1–4 | 2 | always |
| Code block style | dropdown | fenced, indented | fenced | always |
| Code fence | dropdown | ` ``` `, `~~~` | ` ``` ` | only when code block style is fenced |
| Link style | dropdown | inline, inline with autolinks, referenced | inline | always |
| Reference style | dropdown | full (`[t][1]`), collapsed (`[t][]`), shortcut (`[t]`) | full | only when link style is referenced |
| Preserve code whitespace | checkbox | | off | always |
| Treat `<noscript>` content as raw text | checkbox | | on | always |
| Skip tags | text field | tag names, separated by commas or spaces | empty | always |

Pure mode always writes plain Markdown, dropping what Markdown cannot express
(such as attributes). Faithful mode keeps such HTML as raw HTML in the Markdown.

**Skip tags** removes the listed elements together with their content. Names
are case-insensitive; anything that is not a valid tag name is ignored, and the
ignored entries are listed under the field.

A number outside 1–4 is clamped to that range, and an emptied number field
returns to its last value when it loses focus.

A value that is hidden or disabled is kept, not reset, and applies again when
its control reappears.

### pulldown-cmark options

These are shown in both directions, since both use pulldown-cmark (see the tab
table). There is one checkbox for each pulldown-cmark extension:

| Extension | Default |
| --- | --- |
| Tables | on |
| Math | on |
| Footnotes | off |
| Old-style footnotes | off |
| Strikethrough | off |
| Task lists | off |
| Smart punctuation | off |
| Heading attributes | off |
| YAML metadata blocks | off |
| `+++` metadata blocks | off |
| GFM | off |
| Definition lists | off |
| Superscript | off |
| Subscript | off |
| Wikilinks | off |

Tables and math are on by default because htmd writes both: with them off, the
**Rendered** tab of HTML → Markdown would show its own output incorrectly.
Math is shown as its formula text, not typeset.
Old-style footnotes requires footnotes: checking old-style footnotes also checks
footnotes, and unchecking footnotes also unchecks old-style footnotes.

Examples
--------

Each direction has its own presets, and **Basic** is the first in each menu.
The HTML **Basic** preset is the *default example*: the page opens with it
when the address carries no saved state.

HTML → Markdown:

| Preset | Contains |
| --- | --- |
| Basic | a paragraph with `<strong>` and `<em>` text |
| Headings | `<h1>`–`<h3>` and paragraphs |
| Lists | nested bullet lists, and a numbered list starting at 3 |
| Links and images | a link whose text is its URL, a titled link, an image with alt text |
| Code | inline code with spaces at both ends, and a Rust code block |
| Tables | a table with a header row |
| Blockquotes | nested blockquotes |
| Math | inline and display math spans |
| Article | a few kilobytes of freely redistributable article text with headings, lists, links, a rule, a line break, and a code block, and no `<script>` or `<style>` |
| Edge cases | empty elements, entities, runs of whitespace, comments, `<script>`, `<style>`, and `<noscript>` |

Markdown → HTML: Basic, a short sample of plain CommonMark, then one preset for
each pulldown-cmark extension, named after its checkbox and in the order of the
extension table, each holding a short sample that uses that extension. Picking
an extension's preset also checks that
extension, so the preset always shows its feature. No other option changes,
except as the footnotes rule under Options requires.

Expected results
----------------

These rows are the reference behavior. Each was produced with htmd 0.5.5 and
pulldown-cmark 0.13.4, with every option at its default unless the Settings
column says otherwise. `⏎` is a newline and `␣` a space. A trailing newline at
the end of the output is ignored. The `SCRIPT` row also relies on the
playground's **Skip tags** rule: htmd itself matches tag names by case.

### HTML → Markdown

| Input | Settings | Output |
| --- | --- | --- |
| `<h1>Hi</h1>` | | `#␣Hi` |
| `<h1>Hi</h1>` | Setext | `Hi⏎==` |
| `<p><strong>a</strong> and <em>b</em></p>` | | `**a**␣and␣*b*` |
| `<ul><li>a</li><li>b</li></ul>` | | `*␣␣␣a⏎*␣␣␣b` |
| `<ul><li>a</li><li>b</li></ul>` | `-` marker, 1 space | `-␣a⏎-␣b` |
| `<ol start="3"><li>a</li></ol>` | | `3.␣␣a` |
| `<ol start="3"><li>a</li></ol>` | 1 space after list number | `3.␣a` |
| `<hr>` | | `*␣*␣*` |
| `<hr>` | `- - -` | `-␣-␣-` |
| `<p>a<br>b</p>` | | `a␣␣⏎b` |
| `<p>a<br>b</p>` | backslash | `a\⏎b` |
| `<p>a<br>b</p>` | Faithful | `a<br>b` |
| `<p><a href="https://e.com">t</a></p>` | | `[t](https://e.com)` |
| `<p><a href="https://e.com">t</a></p>` | referenced | `[t][1]⏎⏎[1]:␣https://e.com` |
| `<p><a href="https://e.com">t</a></p>` | referenced, collapsed | `[t][]⏎⏎[t]:␣https://e.com` |
| `<p><a href="https://e.com">t</a></p>` | referenced, shortcut | `[t]⏎⏎[t]:␣https://e.com` |
| `<p><a href="https://e.com">https://e.com</a></p>` | inline with autolinks | `<https://e.com>` |
| `<p><img src="i.png" alt="cat"></p>` | | `![cat](i.png)` |
| `<pre><code class="language-rust">fn x() {}</code></pre>` | | ` ```rust⏎fn x() {}⏎``` ` |
| `<pre><code class="language-rust">fn x() {}</code></pre>` | `~~~` fence | `~~~rust⏎fn x() {}⏎~~~` |
| `<pre><code>fn x() {}</code></pre>` | indented | `␣␣␣␣fn x() {}` |
| `<p><code> a </code></p>` | | `` `a` `` |
| `<p><code> a </code></p>` | preserve code whitespace | `` `␣␣a␣␣` `` |
| `<table><tr><th>a</th></tr><tr><td>b</td></tr></table>` | | `\|␣a␣\|⏎\|␣-␣\|⏎\|␣b␣\|` |
| `<p><span class="math math-inline">x^2</span></p>` | | `$x^2$` |
| `<p><span class="math math-display">x^2</span></p>` | | `$$x^2$$` |
| `<p class="x">a</p>` | | `a` |
| `<p class="x">a</p>` | Faithful | `<p class="x">a</p>` |
| `<p>a</p><script>alert(1)</script>` | | `a⏎⏎alert(1)` |
| `<p>a</p><script>alert(1)</script>` | skip tags `SCRIPT` | `a` |
| `<p>a</p><noscript><b>x</b></noscript>` | | `a⏎⏎\<b>x\</b>` |
| `<p>a</p><noscript><b>x</b></noscript>` | `<noscript>` as raw text off | `a⏎⏎**x**` |

### Markdown → HTML

| Input | Settings | Output |
| --- | --- | --- |
| `#␣Hi` | | `<h1>Hi</h1>` |
| `$x^2$` | | `<p><span class="math math-inline">x^2</span></p>` |
| `$$x^2$$` | | `<p><span class="math math-display">x^2</span></p>` |
| `$x^2$` | math off | `<p>$x^2$</p>` |
| `\|␣a␣\|⏎\|---\|⏎\|␣b␣\|` | | a table with header cell `a` and body cell `b` |
| `~~a~~` | | `<p>~~a~~</p>` |
| `~~a~~` | strikethrough | `<p><del>a</del></p>` |
| `-␣[x]␣a` | task lists | a list item with a checked, disabled checkbox |
| `"a"␣--␣b` | smart punctuation | `<p>“a” – b</p>` |

### Round trip

Converting `#␣Hi⏎⏎$x^2$` to HTML and switching direction gives back
`#␣Hi⏎⏎$x^2$`.

Permalinks
----------

The page address records the direction, the input, and every option that
differs from its default. None of it is sent to the server. Opening the
address in a new tab restores them
exactly; the converted output can differ only if the page was rebuilt with a
newer htmd or pulldown-cmark.

The address is at most 32,768 characters long, so that links survive chat apps
and mail clients. When the input does not fit, the address keeps only the
direction and options, and a notice says the input is too large to share.
Opening such an address restores the direction and options with an empty
input.

The address updates only after the user changes the input, the direction or
an option; opening the page does not rewrite it. Nothing the user does in the
playground adds an entry to the browser history; only editing the address by
hand does.

* A **Share** button copies the address. It first brings the address up to
  date with the latest keystrokes. It has the same manual-copy fallback as
  **Copy**, with the address shown selected. When the input is too large
  to share, it shows that notice instead of copying.
* An address the page cannot fully read is invalid as a whole; it is never
  partly applied. The page then shows the default example and a notice saying
  the link was invalid. The address is left unchanged until the next update
  described above.
* Editing the address by hand, or using Back and Forward, loads the state it
  describes; an address with no saved state loads the default example. The
  loaded state replaces any keystrokes still inside the typing pause.
* Addresses shared today keep working after the playground changes.

Limits and failures
-------------------

* **Large input.** Above 1 MiB of input (measured as UTF-8 text), conversion
  pauses and the status line offers a "Convert anyway" button. The output shows
  the last result until then. Once pressed, the button applies until the input
  next drops to 1 MiB or below. If a direction switch or an address puts more
  than 1 MiB of text into the input, conversion pauses in the same way, with
  an empty output.
* **Stale output.** While conversion is paused, and after a crash, the
  direction switch and **Copy** are unavailable, because the output does not
  match the input.
* **Crashes.** If a converter fails or crashes, the page does not freeze
  silently. This includes input nested so deeply that a converter runs out of
  stack. A message replaces the output pane, with a **Copy input** button, a
  **Reload** link, and a **Start over** link that loads the page without the
  saved address (in case the saved input itself causes the crash). The input
  stays editable, but nothing converts again until the page is reloaded.

Constraints
-----------

* **Safety.** After the site loads, the page makes no network requests.
  Nothing the user enters, and nothing a converter produces from it, runs
  scripts, loads anything from the network, navigates the page, or changes the
  look of the rest of the playground. This applies to the **Rendered** and
  **Original** tabs in particular: `<script>alert(1)</script>` never runs,
  `<img src="https://example.com/x.png">` shows as a broken image without a
  request, and clicking a link does nothing.
* **Browsers:** current Chrome, Firefox, and Safari, on desktop and phone.
* **Accessibility:** every control works from the keyboard, has a visible focus
  style and a label, and meets WCAG AA contrast. Each rendered tab has a
  title. Status changes are announced to screen readers.
* **Themes:** the page, including the rendered tabs, follows the system's light
  or dark setting.
* **Size and speed:** the downloaded page, with all its files, is under 1 MB
  with gzip, and converting the Article example to Markdown takes under 10 ms
  (median of 10 runs) in desktop Chrome.
* **Build:** the site is built with Leptos 0.8 and published to GitHub Pages
  at `https://dkaimekinmasters.github.io/paradigms-htmd/` on every push to
  `main`.
* **The library is untouched.** Adding the playground does not change htmd's
  code, its published package, or its existing build, lint, and test results.

Acceptance criteria
-------------------

* Every option visibly changes the result of at least one preset, in
  **Source** or **Rendered**.
* Switching direction twice on the default example gives back the original
  HTML, ignoring whitespace between tags and a trailing newline.

Out of scope
------------

User accounts, analytics, saving work anywhere other than the page address,
custom htmd element handlers, syntax highlighting, and a conversion that never
finishes.
