# YAML Metadata Block Support

YAML metadata block support translates a leading
`<script type="application/yaml">` element into CommonMark YAML-style
frontmatter. It does **not** parse, validate, or reformat YAML syntax.

The feature aligns with `pulldown-cmark`'s
`ENABLE_YAML_STYLE_METADATA_BLOCKS` representation: the upstream Markdown
extension produces `<script type="application/yaml">` around the metadata
payload. Everything not described here is unchanged, including ordinary HTML,
Markdown, and non-YAML script handling.

## 1. Configuration, Options, and Handler Precedence

1. YAML metadata block support is opt-in. `Options` exposes
   `yaml_metadata_blocks: bool`, with a default of `false`.
   `HtmlToMarkdownBuilder::yaml_metadata_blocks(bool)` sets the same option.
   `HtmlToMarkdownBuilder::options(Options)` also uses the option value it
   receives.
2. A value of `false` disables recognition and leaves every script element on
   the existing conversion path. A value of `true` enables recognition for
   eligible leading YAML `<script>` elements only.
3. YAML metadata handling applies identically in
   `TranslationMode::Pure` and `TranslationMode::Faithful`: eligibility and
   payload formatting do not depend on the translation mode.
4. In `TranslationMode::Pure`, a recognized metadata block is emitted as
   Markdown frontmatter. In `TranslationMode::Faithful`, the feature emits the
   same frontmatter rather than retaining the original `<script>` tag.
5. Handlers registered for `"script"` have precedence over the built-in YAML
   metadata handler. The last `.add_handler(vec!["script"], ...)` registration
   is selected first; a previous script handler is reached only if the later
   handler calls `fallback`.
6. `.skip_tags(vec!["script"])` is an ordinary script handler registration:
   it selects the skip behavior and suppresses both metadata conversion and
   payload output for `"script"` elements.
7. A custom script handler may inspect the element attributes and decide
   independently whether to handle, skip, or fall back. If it returns by
   falling back, the built-in YAML recognition and the existing script behavior
   remain available as the next handler in the chain.
8. Untyped `<script>` elements and scripts whose `type` value is any value
   other than `application/yaml` retain their existing behavior. The option
   does not change them in either translation mode.

## 2. HTML Recognition and DOM Placement

1. Recognition requires an element whose tag name is `script`.
2. The `type` attribute is the only attribute considered for recognition.
   `id`, `src`, `data-*`, class, and all other attributes are ignored.
3. The `type` attribute value must equal `application/yaml` after trimming
   ASCII whitespace (`' '`, `'\t'`, `'\n'`, `'\r'`, and `'\f'`) from both
   ends. Comparison is ASCII-case-insensitive, so
   `Application/YAML` and `APPLICATION/YAML` are valid.
4. A missing `type` attribute does not match. A value such as
   `application/yaml; charset=utf-8` does not match.
5. A YAML metadata candidate is recognized only if it is the first
   non-whitespace node in the input document or fragment. Whitespace-only text
   nodes do not affect leading placement. Structural wrappers such as
   `html`, `head`, and `body` do not affect the placement test.
6. `html5ever` may hoist a leading `<script>` element from the input fragment
   into the parsed `<head>`. Leading placement is determined by pre-order
   depth-first traversal of the parsed tree, descending transparently through
   `html`, `head`, and `body` wrapper elements without treating them as
   content nodes. A script hoisted into `head` is therefore leading only when
   no preceding meaningful node appears before it in pre-order traversal.
7. Comments and processing instructions are meaningful nodes for placement.
   A doctype is not document content and does not make a later script
   non-leading. Other element nodes and non-whitespace text nodes make a
   YAML script non-leading.
8. At most one YAML script is recognized as metadata. The first eligible
   script in document order, if present, is the sole candidate. If a custom
   `"script"` handler or `.skip_tags(vec!["script"])` handles or suppresses
   that candidate without calling `fallback`, its candidacy is consumed; a
   second YAML `<script>` is never promoted to metadata, even when only
   whitespace separates it from the first.
9. Any YAML `<script>` that follows content, is inside a `p`,
   `blockquote`, list, table, or other container, or follows the first YAML
   script is not metadata and uses standard `<script>` handling.

## 3. Payload Decoding and Markdown Output Formatting

1. The payload is the text content between the opening and closing
   `<script>` tags. No YAML syntax is parsed or reformatted.
2. HTML entity decoding is not performed. Literal `&`, `<`, `>`,
   `&amp;`, `&lt;`, and `&gt;` in script text remain text as represented by
   `html5ever`; neither the metadata block nor the standard script path
   decodes them.
3. Closing-sequence decoding is reversible and limited to the escaped
   representations. Scan the payload left-to-right. In a case-insensitive
   `</script` sequence encoded as `<\/script` (where `\/` replaces `/`),
   decode `\/` back to `/`. Separately, decode each pair `\\` back to one
   `\`. All other characters, including ordinary `/`, `<`, `>`, and `&`,
   remain unchanged.
4. The line-ending normalization order is CRLF (`\r\n`) to LF (`\n`), then
   CR (`\r`) to LF (`\n`). This produces `\n` for every output line ending.
5. Payload whitespace is otherwise preserved. The implementation does not
   strip indentation, leading blank lines, trailing blank lines, or ordinary
   payload whitespace.
6. After line-ending normalization, check whether the payload ends with a YAML
   closing marker line: specifically, the normalized payload equals `"..."` or
   `"...\n"`, or ends with `"\n..."` or `"\n...\n"`. If so, strip the trailing
   `"..."` or `"...\n"` marker sequence while preserving any `\n` that
   immediately preceded `...`, and emit `---` as the closing delimiter. (For
   example, `"\ntitle: Demo\n\n...\n"` becomes `"\ntitle: Demo\n\n"`, emitting
   `"---\ntitle: Demo\n\n---"`; `"..."` or `"...\n"` becomes empty and emits
   `"---\n---"` per rule 7.)
7. If no payload and no normalized `...` closer remain, emit exactly
   `---\n---`.
8. For a non-empty payload, emit `---` on the opening line, the payload
   without any newly added payload whitespace, and `---` on a line by itself
   as the closing delimiter. Do not add a payload-leading newline that is not
   present in the payload, and do not add a payload-trailing newline that is
   not present in the payload; add one line ending only when needed to place
   the closing delimiter on a separate line.
9. A trailing blank line in the payload remains a blank line before the
   closing delimiter. For example, payload `title: Demo\n\n` emits
   `---\ntitle: Demo\n\n---`.
10. The closing `---` delimiter is separated from following Markdown content
    by exactly one blank line: emit `---\n\n` and then the following content.
    There is no extra blank line at the end of a document when no content
    follows.

## 4. Edge Cases, Round-Trip Invariants, and Examples

1. Leading whitespace-only text nodes before the YAML script do not displace
   it. A whitespace-only document is empty and emits no metadata.
2. Empty documents, documents containing no script, and documents containing
   only untyped or non-YAML scripts keep their existing output.
3. Payload text containing `&`, `<`, or `>` is emitted literally and is not
   entity-decoded or escaped.
4. A literal embedded `</SCRIPT>` cannot appear as raw script text because
   `html5ever` interprets it as a closing tag. The reversible escaped form
   `<\/SCRIPT>` is preserved through HTML parsing and decodes to
   `</SCRIPT>` in the metadata payload.
5. Trailing blank lines belong to the payload unless they are the part of a
   normalized `...` closer. The closing `---` remains after those lines.
6. Multiple scripts are handled independently: only the first eligible YAML
   script becomes metadata; every later script, including a second YAML
   script, uses standard handling.

The round-trip contract is:

1. `pulldown-cmark` with `ENABLE_YAML_STYLE_METADATA_BLOCKS` recognizes
   frontmatter and represents it as
   `<script type="application/yaml">` plus its payload.
2. `htmd` recognizes that leading representation, subject to Configuration,
   Recognition, and Handler rules in this specification.
3. `htmd` emits a Markdown frontmatter block framed by `---` delimiters.
4. The payload itself is not parsed or reserialized; supported line-ending
   and reversible closing-sequence normalization are the only payload changes.

### Examples

#### Standard metadata

**Input (HTML):**

```html
<script type="application/yaml">
title: Example
author: Ada
</script>
<p>Body</p>
```

**Output (Markdown):**

```markdown
---
title: Example
author: Ada
---

Body
```

The first node is the YAML script, and the payload is preserved while the
following Markdown body is separated by one blank line.

#### Empty block

**Input (HTML):**

```html
<script type="application/yaml"></script>
```

**Output (Markdown):**

```markdown
---
---
```

An empty payload has no extra blank line or payload whitespace.

#### Leading whitespace and special characters

**Input (HTML):**

```html

<script type=" APPLICATION/YAML " id="metadata">
symbols: & < >
</script>
```

**Output (Markdown):**

```markdown
---
symbols: & < >
---
```

The leading whitespace-only text node is ignored, ASCII whitespace and
case around the type value are accepted, and `& < >` remain unchanged.

#### Escaped closing sequence and backslashes

**Input (HTML):**

```html
<script type="application/yaml">
example: <\/SCRIPT>
path: C:\\temp
</script>
```

**Output (Markdown):**

```markdown
---
example: </SCRIPT>
path: C:\temp
---
```

The escaped closing sequence and doubled backslashes are decoded reversibly;
ordinary YAML text is not otherwise changed.

#### Trailing blank line and `...` closer

**Input (HTML):**

```html
<script type="application/yaml">
title: Demo

...
</script>
```

**Output (Markdown):**

```markdown
---
title: Demo

---
```

The blank line before `...` is preserved payload whitespace, while the final
`...` line is normalized to the closing `---` delimiter.

#### Trailing payload blank lines

**Input (HTML):**

```html
<script type="application/yaml">title: Demo

</script>
```

**Output (Markdown):**

```markdown
---
title: Demo

---
```

The two line endings after `Demo` leave one blank line before the closing
delimiter.

#### Non-leading YAML script

**Input (HTML):**

```html
<p>Before</p>
<script type="application/yaml">title: NotMetadata</script>
```

**Output (Markdown) in Pure mode:**

```markdown
Before

title: NotMetadata
```

The script is not leading, so it falls back to standard script handling. In
Faithful mode, the script remains serialized as HTML instead.

#### Second YAML script

**Input (HTML):**

```html
<script type="application/yaml">title: First</script>

<script type="application/yaml">title: Second</script>
```

**Output (Markdown) in Pure mode:**

```markdown
---
title: First
---

title: Second
```

The first YAML script is metadata; the second script is not metadata and uses
standard script handling.

#### Untyped and JavaScript scripts

**Input (HTML):**

```html
<script>console.log("a");</script>
<script type="text/javascript">console.log("b");</script>
<script type="application/json">{"a":1}</script>
```

**Output (Markdown) in Pure mode:**

```markdown
console.log("a");

console.log("b");

{"a":1}
```

These scripts are not YAML metadata and retain the existing script behavior,
including any configured script handlers and faithful-mode serialization.
