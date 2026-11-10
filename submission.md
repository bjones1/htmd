# Project 2 draft submission: htmd subscript 2

Team: Pavel Shegay, Viacheslav Sokolov, Arsen Zhanserkenov. Arsen Sailau did
not take part in this submission.

## Contributions

- Pavel Shegay: section frame, introduction, and escaping of a literal `~` in
  text.
- Viacheslav Sokolov: table rows and explanation for when `<sub>` can be
  written as `~a~` and when it must stay raw HTML (inside a word, next to
  punctuation, whitespace at the edges of the content).
- Arsen Zhanserkenov: empty and whitespace-only `<sub>`, nesting, and the
  Approach steps.

## Build check

### Viacheslav Sokolov

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Verify that you can
compile and run the project in the current git repo.` produced:

> **Verified successfully.** Using Cargo from `C:\Users\Admin\.cargo\bin`
> because it wasn't on `PATH`:
>
> - `cargo build` — passed
> - `cargo clippy -- --deny warnings` — passed
> - `cargo test` — passed: 267 tests, 1 ignored
> - `cargo run --example simple` — ran successfully
>
> The working tree remains clean.

### Pavel Shegay

TODO

### Arsen Zhanserkenov

TODO

## Review rounds

### Viacheslav Sokolov

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 4 (1 accepted, 3 rejected) | Reworded the sentence on why in-word subscripts need HTML so it no longer claims that nothing can fix them. |

### Pavel Shegay

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | TODO | TODO |

### Arsen Zhanserkenov

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | TODO | TODO |

## Last review

### Viacheslav Sokolov

Prompting Copilot Fusion (GPT-6 Luna), Standard, with `Review only lines
745-774 of translation_approach.md (the table rows I added to the Subscript
section and the three explanation paragraphs below the table). Check them
against the pulldown-cmark subscript extension (Options::ENABLE_SUBSCRIPT) and
CommonMark left/right-flanking delimiter rules. Do not edit any files. List any
factual errors, unclear wording, or missing cases as numbered findings.`
produced the following output; strikethrough indicates LLM output that I
deemed incorrect or irrelevant.

1. ~~The negative example `a ~b~c` is not a valid failure case under
   CommonMark/pulldown-cmark subscript rules. The opener is left-flanking
   because it is preceded by a space, and the closer is right-flanking even
   when followed by a letter, so this should be treated as a valid subscript,
   not as a literal-text case. The prose currently says it "fails for the same
   reason on one side," which contradicts the delimiter-flanking model.~~
   (pulldown-cmark 0.13.4 renders `a ~b~c` as literal text.)
2. ~~The rule sentence is oversimplified and not the actual algorithm: "A
   single `~` opens a subscript only if the character before it is whitespace,
   punctuation, or the start of the line, and closes it only if the character
   after it is whitespace, punctuation, or the end of the line" is not the
   CommonMark left/right-flanking rule. The real test is both sides of the
   delimiter run together: opener/closer validity depends on
   left-flanking/right-flanking, not on a one-sided check of just the adjacent
   character. This matters because cases like `x.~2~` are valid even though
   the closer is not followed by whitespace/punctuation.~~ (In `x.~2~` the
   closer is followed by the end of the line, which the rule covers; for a
   single `~` the stated rule gives the same result as the flanking rules.)
3. The explanation "Nothing written next to the tildes can fix this" is too
   absolute and inconsistent with the surrounding point that the parser looks
   at the characters outside the element. In fact, changing the neighboring
   characters can make the same content valid: `H ~2~ O` is a valid subscript,
   while `H~2~O` is not. The document is describing the outside-neighbor test
   correctly, but then states the opposite too strongly.
4. ~~The section is missing a key positive case that shows the real boundary
   behavior: examples such as `a ~b~c` or `x.~2~` are exactly the cases
   CommonMark uses to demonstrate that the check is on the characters outside
   the delimiters, not on the content inside. Right now the doc only presents
   the blocking cases and omits the corresponding valid "one-side letter,
   other-side space/punctuation" cases that explain the rule.~~ (Based on
   finding 1; `x.~2~` is already a row of the table.)

### Pavel Shegay

TODO

### Arsen Zhanserkenov

TODO
