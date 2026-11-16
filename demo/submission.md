Project 2 submission
====================

Team members are Muzanbekova Kamila, Danial Baitakov, Justin Gray, and Nurlybek
Aimagambetov. The specification consists of the file
[demo-spec.md](demo-spec.md).

Draft submission
----------------

### Contributions

* Muzanbekova Kamila: Initiated the repository fork, created the critical
  demo-spec.md and review.md files, added teammates to the repository to enable
  collaboration, and began the initial code review process for the project
  specification. Completed the specification analysis and performance audit up
  to and including point B6, made the corresponding edits to the `review.md`
  file, and moved the irrelevant points into the specification. Round 1-4
  review.
* Danial Baitakov: Finished the review work from B7 through L10, applying each
  finding to the spec and marking it resolved in review.md. Added the artifacts
  the professor asked for (contribution list, review history, compile
  verification output) and ran the round-5 review cycle until nothing was left
  open. Also ran the round-6 review and added five more findings from a
  spec-quality checklist, then left round 6 open so a teammate could take it.
* Justin Gray: Ran the round-8 and round-9 spec reviews with Claude Opus 5.5
  and fixed every medium finding from both, including the fixed fidelity
  round-trip options, a synchronous commit step, per-mode output validity,
  parse-based preview stripping, and nesting depth measured on the
  `html_to_tree` tree. Ran the round-10 review (9 new findings, left open),
  recorded rounds 8–10 in this file, and reformatted it.
* Nurlybek Aimagambetov: Reviewed and addressed some of the round 6 review
  findings, clarified typing protection and added acceptance coverage case,
  corrected the nesting-limit explanation

### Compiler test

The following statements summarize LLM output on each team member's computer,
based on the prompt "Verify that you can compile and run the project in the
current git repo."

#### Muzanbekova Kamila

### LLM Verification Output

<strong>Prompt applied:</strong> <em>"Verify that you can compile and run the
project in the current git repo."</em>

<strong>LLM Response:</strong> Verified. The repo builds, lints, tests, and runs
cleanly.

| Check                           | Command                          | Result                                                     |
| ------------------------------- | -------------------------------- | ---------------------------------------------------------- |
| **Build**                       | `cargo build`                    | Finished dev profile                                       |
| **Lint**                        | `cargo clippy -- -deny warnings` | No warnings                                                |
| **Tests**                       | `cargo test`                     | 266 passed, 1 ignored (the table-generator test), 0 failed |
| <strong>Run (examples)</strong> | `cargo run --example table`      | Printed converted Markdown                                 |
| <strong>Run (examples)</strong> | `cargo run --example simple`     | Ran after rebuild                                          |

<em>Environment Note:</em> The first run of `target/debug/examples/simple.exe`
was temporarily blocked by Windows Smart App Control ("Application Control
Policy blocked this file", os error 4551) due to an unsigned binary reputation
check. A fresh `rustc hello-world` and the `table` example executed correctly.
Rebuilding `simple` under a different debug profile resolved the signature
prompt, allowing the execution to bypass the environment restriction and
successfully pass all internal code assertions. No repository source files were
modified during this execution check (`git status` remained clean)

#### Danial Baitakov

<strong>Prompt applied:</strong> <em>"Verify that you can compile and run the
project in the current git repo."</em> — run by GLM-5.3 (OpenCode) on Linux,
repository at commit `bb9161d`.

<strong>LLM Response:</strong> Verified. The repo builds, lints, tests, and runs
cleanly.

| Check                           | Command                           | Result                                                                                           |
| ------------------------------- | --------------------------------- | ------------------------------------------------------------------------------------------------ |
| **Build**                       | `cargo build`                     | Finished dev profile                                                                             |
| **Lint**                        | `cargo clippy -- --deny warnings` | No warnings                                                                                      |
| **Tests**                       | `cargo test`                      | 267 passed, 1 ignored (the table-generator test), 0 failed                                       |
| <strong>Run (examples)</strong> | `cargo run --example table`       | Printed the converted Markdown table                                                             |
| <strong>Run (examples)</strong> | `cargo run --example simple`      | Self-checking example (assertions only, no printed output); exited 0 with all assertions passing |

<em>Environment note:</em> Linux; no environment restrictions encountered; no
repository source files were modified during this check.

#### Nurlybek Aimagambetov

<strong>Prompt applied:</strong> <em>"Verify that you can compile and run the
project in the current git repo."</em>

<strong>LLM Response:</strong> Verified: the library compiles and runs through
its test suite.

* cargo build — passed
* cargo clippy -- --deny warnings — passed
* cargo test — passed, including doctests; one test intentionally ignored The
  default target directory produced filesystem errors. All checks passed using
  CARGO\_TARGET\_DIR=\$env:TEMP\\htmd-verification-target. No source files
  changed.

#### Justin Gray

Compiler verification pending; the member has rejoined the team for the final submission and will record the verification output here.


### Spec review rounds

The following table summarizes the design and review iterations for the `htmd /
pulldown-cmark live demo` specification. It tracks the document's evolution from
initial conceptual sketches to a production-ready specification optimized for
WebAssembly runtime stability and browser security.

| Round | Findings              | Main changes made                                                                                                                                                                                                                                                                                                                                                                                                                            |
| ----- | --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1** | **18**                | Narrowed the scope to strict one-way conversion modes, established a compressed WASM asset budget, and defined initial verifiable acceptance criteria.                                                                                                                                                                                                                                                                                       |
| **2** | **26**                | Resolved diff pipeline contradictions, added the missing Section 5 UI sketch, formalized the localStorage validation schema, and eliminated runtime panics on empty tag inputs.                                                                                                                                                                                                                                                              |
| **3** | **12**                | Enforced total preview isolation using a sandboxed `<iframe>` with a strict Content Security Policy to eliminate XSS risks and hidden third-party network requests.                                                                                                                                                                                                                                                                          |
| **4** | **38**                | Verified all settings against `htmd 0.5.5` and `pulldown-cmark 0.13.4`, expanded the option mapping to 15 flags, resolved reset semantics, and added deep DOM nesting bounds.                                                                                                                                                                                                                                                                |
| **5** | **12**                | Made performance and size budgets measurable (artifact, command, profile), defined the demo build contract and CI workflow, normalized `skip_tags`, hardened the iframe/panic-hook/debounce contracts, and renumbered sections; then reconciled long-task vs compute budgets, unified storage version handling, added the persisted-state schema, moved the nesting guard to a streaming pre-scan, and pinned the 2 kB threshold.            |
| **6** | **11**                | Open, awaiting processing: six LLM findings (missing axe-core CI step, underspecified nesting pre-scan, unadjudicable forced-tick latency rule, `&nbsp;` exemption inconsistency, missing acceptance criteria for "Use Output as Input"/"Reload Sample", preset naming variance) plus five spec-quality checklist findings (word tokenizer definition, code-point vs locale sorting, NFC normalization, first-run state, banner byte units). |
| **7** | **8**                 | <p>Addressed some findings from the previous round. A follow-up review identified 8 new non-critical issues, some of which I fixed while dismissing the rest as not applicable</p>                                                                                                                                                                                                                                                           |
| **8** | **13**                | Fixed all 7 new medium findings: fixed `TABLES \| MATH` options for the fidelity round trip (the Demo Preset's smart punctuation produced false diffs), a defined main-thread execution model for the concurrency rules, `textContent` for every user-derived view, blocked preview self-navigation, a keyboard-reachable Undo toast, banners for quota failures and multi-tab writes, and a benchmark corpus with adversarial bounds. The 6 new low findings and the open round-6/7 findings remain.                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| **9** | **13**                | Fixed all 4 new medium findings (stale runs painting the preview, mode switches not advancing the sequence number, preview stripping without a parse or an iterative walk, the multi-tab banner re-announcing every ~500 ms) and all 7 open round-6/7 medium findings, among them nesting depth measured on the `html_to_tree` tree, code-point sorting, a safe serialization root, edge-whitespace trimming, CSS `white-space` preservation, CDATA handling, and per-mode output validity. Most low findings were fixed too; R9-L3 (in part), R9-L7, R9-L8 (units), and R9-L9 remain open. |
| **10** | **9**                | Open, awaiting processing: four medium findings (the pipeline stalls when a run never reloads the iframe, every ordinary code block shows a false diff, `<noscript>` and `<template>` bypass preview stripping, a panicking input can lock the user in a crash loop) and five low findings (`skip_tags` keystrokes treated as option changes, unmeasured wasm stack headroom at 512 levels, fragment vs document parse mismatch, a per-mode fidelity-panel wording error, Undo not restoring the selected sample). |

---

#### Round Context \& Analysis

1. <strong>Round 1 (Conceptual Scope):</strong> Replaced unstable two-way sync
   ideas with a deterministic "Use Output as Input" feature, split per-mode
   state variables, and introduced performance budgets evaluated via the
   browser's Performance API.
2. <strong>Round 2 (Pipeline Logic):</strong> Designed a word-level text diff
   built strictly on canonicalized HTML fragments using `html5ever`, dropped
   unreachable library error wrappers, and fixed concurrency timers to ensure
   typing does not wipe pending changes.
3. <strong>Round 3 (Browser Security \& Sandboxing):</strong> Blocked raw HTML
   passthrough vulnerabilities, isolated event handling vectors (`onclick`,
   `onerror`), explicitly mapped the `pulldown-cmark` bitflags, and unified the
   100 kB limit using UTF-8 byte arrays.
4. <strong>Round 4 (Crate Alignment \& Edge Cases):</strong> Expanded the UI
   flags to match all 15 bitflags available in `pulldown-cmark 0.13.4`,
   protected the recursive WASM stack walker against deeply nested elements, and
   separated "Reset Options" from "Reset Sample" logic.
5. <strong>Round 5 (Measurability \& Buildout):</strong> Tied every budget to an
   artifact and CI command (`gzip -6` size check, `performance.measure`
   latency), defined the standalone-crate build contract with a `path`
   dependency on `htmd`, added the persisted-state schema and a streaming
   nesting guard, and reconciled the 100 ms compute vs 350–400 ms end-to-end
   latency criteria.
6. <strong>Round 6 (Open):</strong> A fresh-context review produced six findings
   (axe-core missing from the CI step list, underspecified nesting pre-scan,
   unadjudicable forced-tick latency rule, `&nbsp;` exemption inconsistency,
   missing acceptance bullets for "Use Output as Input"/"Reload Sample", preset
   naming variance); a specification-quality checklist pass added five more
   (word tokenization for the diff, code-point vs locale-aware sorting, NFC
   normalization, first-run state, banner byte units). All 11 remain open for
   processing.
7. <strong>Round 7 (Various issues):</strong> Fixed some of the issues in
   previous round review (Preset naming variance, omitted axe-core from CI,
   first run state), improved typing protection wording and added coverage.
8. <strong>Round 8 (Verification against htmd source):</strong> A review checked
   the spec against `src/dom_walker.rs`, `src/util/text.rs`, and the round-trip
   test helpers. It confirmed that R6-M1, R6-M3, R6-L3, R6-L5, R7-M7, and R7-L1
   are resolved, and it raised 13 new findings (7 medium, 6 low). All 7 medium
   findings were fixed:
   * The fidelity round trip now uses fixed `ENABLE_TABLES | ENABLE_MATH`.
     `htmd` escapes only `` \ & * _ ` [ ] $ ``, so the Demo Preset's smart
     punctuation, strikethrough, and heading attributes rewrote ordinary text
     (`don't` → `don’t`).
   * Conversion runs synchronously on the main thread. "In flight" now means
     dispatch through commit, and crossing a limit discards stale runs.
   * `textContent` now covers the fidelity columns, the diff, banners, and
     toasts, not only the Source tab.
   * Preview links can no longer navigate the sandboxed iframe to a remote page:
     the demo injects `<base target="_blank">` and strips `target` attributes
     and prefetch `<link>` elements.
   * The Undo toast is no longer dismissed by focus movement, so keyboard users
     can reach it.
   * Failed storage writes and writes from another tab now show non-fatal
     banners instead of losing data silently.
   * The 100 ms budget is measured against `demo/bench/corpus/`. Adversarial
     inputs are bounded by a 5,000-token diff cap and a 2 MB output display
     limit.

   Each fix has matching acceptance criteria. The 6 low findings (`skip_tags`
   vs SVG case, optional end tags in the nesting scan, `&nbsp;` normalization,
   untested browser floors, Trunk/`wasm-opt` pinning, editorial) remain open,
   as do the earlier R6/R7 findings.
9. <strong>Round 9 (Round-8 regressions and carried-over findings):</strong> A
   review against the source and the open round-6–8 findings raised 13 new
   findings (4 medium, 9 low), and the fixes were applied in the same pass:
   * A run's commit is one synchronous step that writes the preview, Source
     text, and fidelity panel together or not at all, and every buffer change,
     option change, and mode switch advances the sequence number.
   * Preview stripping parses the output with `html5ever`, walks it
     iteratively, and removes every `<base>`, `<meta>`, and `<link>` element and
     every `target` attribute.
   * Nesting depth is measured on the tree from `html_to_tree`, and the same
     tree is passed to `tree_to_markdown`.
   * Each mode has its own output with a validity rule, and samples have
     per-mode variants persisted in two new storage fields.
   * Canonicalization trims block-edge whitespace, preserves inline
     `white-space` styles, sorts by code point, and serializes the synthetic
     `<html>` element's children, never the document node.

   Still open: R9-L3 (in part), R9-L7, R9-L8 (units), R9-L9, and round 8's
   browser-floor and editorial findings.
10. <strong>Round 10 (Open):</strong> A fresh review of the post-round-9 spec
    raised 9 new findings (4 medium, 5 low), listed in full under "Last LLM
    review" below. None has been applied to the spec yet.

### Last LLM review

Prompting **Claude Opus 5.5** (Claude Code, fresh context) with <em>"review
@demo/demo-spec.md"</em> produced the following round-10 output. The review
checked the post-round-9 specification against `src/lib.rs`,
`src/dom_walker.rs`, `src/element_handler/pre.rs`, `tests/common/mod.rs`,
`tests/code_tests.rs`, and the round-9 notes in `review.md`. Cargo is not
installed on the reviewing machine, so the `html5ever` and `markup5ever_rcdom`
sources were not opened; claims resting on them are marked as unverified. No
finding was deemed incorrect or irrelevant, so nothing is struck through; all 9
findings remain open for processing.

#### What checks out

* FR-4's 12 `htmd` options, their variants, and their defaults still match
  `src/options.rs`, and the 15-flag count for `pulldown-cmark` 0.13.4 is right.
* `convert` is exactly `html_to_tree` followed by `tree_to_markdown`
  (`src/lib.rs:148–214`), so FR-7's parse-once nesting check is implementable.
  `html_to_tree` uses `parse_document`, which is consistent with "children of
  `<head>` and `<body>` are depth 1".

#### Blocking issues

None.

#### Medium-severity issues

* <strong>R10-M1 — A conversion that never reloads the iframe stalls the
  pipeline for good (FR-2, execution model):</strong> a run counts as in flight
  until the iframe fires `load` for its commit, but a stale run writes nothing,
  a run whose output is cleared (empty input, the 2 MB limit, an `Err` banner, a
  size or nesting limit) may not set `srcdoc`, and a mode switch can discard a
  run still waiting for `load`. None of these gets a `load`, so the queued run
  never starts and conversion stops for the rest of the session; the 8.6
  concurrency test hits the stale case directly. <em>Fix: a run that writes
  nothing, or does not navigate the iframe, ends at commit; define what
  clearing the preview does to `srcdoc`; ignore a `load` that does not match
  the current commit.</em>
* <strong>R10-M2 — Every ordinary code block shows a false diff (FR-5,
  preserved subtrees):</strong> CommonMark code blocks always end with a line
  ending, so `pulldown-cmark` renders `<pre><code>x\n</code></pre>`, while the
  usual input `<pre><code>x</code></pre>` converts to ```` ```\nx\n``` ````
  (`tests/code_tests.rs`). Inside a preserved subtree nothing trims the extra
  `\n`, so the corpus's real-world page shows a change on every code block, and
  the basics sample passes 8.7's zero-diff test only if its HTML happens to
  include the `\n`. <em>Fix: on both sides, remove one trailing U+000A from the
  last text inside each `<pre>` (a single trailing newline does not render);
  add a golden for `<pre><code>x</code></pre>`.</em>
* <strong>R10-M3 — `<noscript>` and `<template>` bypass preview stripping
  (FR-1; 8.6 "Preview navigation" and "Preview stripping"):</strong> the
  sandboxed preview has scripting disabled, so the browser parses `<noscript>`
  contents as markup, but `html5ever` parses with scripting enabled by default,
  so the walk sees them as text and strips nothing. If the serializer writes
  that text raw (believed but unverified), `<noscript><a href="https://x.example"
  target="_self">click</a></noscript>` loads `x.example` into the preview, and
  `<link rel=preconnect>`, which CSP does not cover, gets through the same way.
  `RcDom` stores `<template>` contents apart from its children, so the walk
  misses them too, while browsers make `<template shadowrootmode="open">` a
  live shadow root. <em>Fix: remove `<noscript>` and `<template>` elements whole
  in the preview walk, or parse and serialize with scripting off; add both
  cases to the 8.6 stripping test.</em>
* <strong>R10-M4 — A panicking input can lock the user in a crash loop (FR-7,
  FR-8):</strong> the flush before "Use Output as Input" or a sample load, the
  500 ms text save, and the `pagehide` flush when the user clicks reload (as the
  overlay instructs) can all persist the crashing input. Startup converts the
  restored buffer once, so it panics again, and the only way out is clearing
  site data by hand. <em>Fix: have the panic hook and the
  `error`/`unhandledrejection` listeners stop all further storage writes and
  set a crash-marker key; when the marker is present at startup, restore the
  workspace without converting and show a banner offering to convert or
  clear.</em>

#### Low-severity / editorial

* <strong>R10-L1 — Typing in `skip_tags` counts as an option change (FR-4,
  FR-8):</strong> on a 100 kB input every keystroke runs a full conversion and a
  synchronous save of the whole workspace, and FR-8 does not say whether a
  stored `skip_tags` over 200 characters is invalid. <em>Fix: debounce the
  field like text input and state the stored-length rule.</em>
* <strong>R10-L2 — Wasm stack headroom at 512 levels is unmeasured
  (FR-7):</strong> the default stack is 1 MiB and each level runs about six to
  eight frames (`walk_node` → `walk_element` → handler → `walk_children`).
  <em>Fix: run the 512-level test with the most stack-hungry element (nested
  lists, tables, or blockquotes) in both translation modes and the release
  build, or set the stack size explicitly.</em>
* <strong>R10-L3 — The fidelity panel and the converter parse the input
  differently (FR-5):</strong> the panel parses the original as a fragment, but
  `htmd` parses it as a document (`src/lib.rs:148`), so for full-page input such
  as the corpus page the doctype, `<html>`/`<body>` attributes, and `<head>`
  contents are handled differently on the two sides. <em>Fix: canonicalize
  from the tree `htmd` converted, or state this as a limitation.</em>
* <strong>R10-L4 — FR-2 gives each mode its own fidelity panel, but FR-5 limits
  the panel to HTML → Markdown mode.</strong> <em>Fix: editorial.</em>
* <strong>R10-L5 — Undo after a sample load does not restore the selected
  sample (FR-6):</strong> a later Reload Sample loads the sample the user just
  undid. <em>Fix: state whether this is intended.</em>

Still open from round 9: R9-L3 (the `strip = true` trade-off and the
release-build panic trigger), R9-L7 (no time ceiling for "neither hangs"),
R9-L8 (byte-unit conventions), R9-L9 (the Pure preset hides the fidelity check
by default), and round 8's browser-floor and editorial findings.

<strong>Total new findings: 9 (blocking 0, medium 4, low 5).</strong>

Final submission
----------------

### Muzanbekova Kamila

GitHub userid: kmuzanbekova

#### Contributions

* initializing the repo/spec/review files, resolving Round 10 and 11 spec review findings, adding the benchmark corpus and demo README/deploy, and running Implementation Review Round 3

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **10** | **9** (4 medium, 5 low) | Resolved all 9 findings: fixed iframe `load` pipeline stalls (R10-M1), trimmed trailing `<pre>` newlines in diff canonicalization (R10-M2), stripped `<noscript>` and `<template>` in preview sanitization (R10-M3), added crash-loop recovery markers (R10-M4), and addressed all 5 low-severity items (R10-L1–R10-L5). |
| **11** | **10** (6 medium, 4 low) | Resolved all 10 findings: 8 fixed (`R11-M1`, `R11-M2`, `R11-M4`, `R11-M5`, `R11-M6`, `R11-L2`, `R11-L3`, `R11-L4`) — aligned FR-1 with all 15 `pulldown-cmark` sidebar flags, added `<noscript>` and `<template>` removal to the FR-1 preview walk, pinned `htmd` options in the FR-5 fidelity check, retained the crash marker until user recovery action, clarified the test suspension point in FR-2, specified Source output copying for "Use Output as Input", defined output/diff clearing on `Err`, and fixed editorial typos — and 2 adjudicated: `R11-M3` became moot once **bjones1**'s "Allow all available pulldown-cmark flags" ruling made all 15 flags sidebar-exposed and persisted, and `R11-L1` is resolved by §8.6's "no user-supplied `target` attribute" wording. |


#### Last specification review

**LLM and prompt used:** Prompting **Big Pickle** (OpenCode agent, fresh context) with *"Review @demo/demo-spec.md, writing findings to @demo/review.md"*

## Round 11 — open findings (awaiting processing)

Prompting **Big Pickle** (OpenCode agent, fresh context) with <em>"review
@demo/demo-spec.md"</em> produced the following round-11 output. The review
checked the post-round-10 specification (commit `52e8094`) against
`src/options.rs`, `src/lib.rs`, `src/element_handler/code.rs`,
`src/element_handler/element_util.rs`, `src/element_handler/pre.rs`,
`src/element_handler/span.rs`, `translation_approach.md`, and the pinned
`html5ever` 0.39.0, `markup5ever_rcdom` 0.39.0+unofficial, and
`pulldown-cmark` 0.13.4 sources in the Cargo registry. The runtime probe was
blocked by Windows Application Control (`os error 4551`), so behavior was
established by source inspection; `cargo build`, `cargo clippy`, and
`cargo test` were not run. No previously resolved finding is re-raised
unchanged; R11-M2 and R11-M5 address gaps left by the round-10 fixes, and
R10-M1 remains open under carried-over items.

#### What checks out

* The 12 `htmd` options, their variants, and their defaults still match
  `src/options.rs`, including `preformatted_code: false` and
  `translation_mode: Pure`.
* `HtmlToMarkdown::html_to_tree` followed by `tree_to_markdown` is exactly
  `convert` (`src/lib.rs:148–214`), so FR-7's parse-once depth check is
  implementable.
* `markup5ever_rcdom`'s serializer iterates `children` and panics on a
  `Document` node; `TraversalScope::IncludeNode` and `ChildrenOnly(None)`
  behave as FR-5 describes, and void elements serialize without a
  self-closing slash (`lib.rs:623–678`).
* `similar::TextDiff::from_words` splits on words and whitespace and needs no
  optional `similar` feature, so FR-5's tokenizer choice is implementable.
* The fixed `ENABLE_TABLES | ENABLE_MATH` round-trip options avoid the
  smart-punctuation false diffs FR-5 describes, and `htmd`'s span handler
  recognizes only its two math `class` values, so it adds no unrelated
  rewrites.
* The `skip_tags` empty-list panic is real
  (`src/element_handler/mod.rs:396–400`), and FR-4's "omit the call" rule
  avoids it.
* Round 10's trailing-newline rule for `<pre>` and its `skip_tags` debounce
  now match the source and the stated behavior.

#### Blocking issues

None.

#### Medium-severity issues
[fixed]
* ~~<strong>R11-M1 — FR-1 enables all 15 `pulldown-cmark` flags, contradicting FR-4 and the Demo Preset (FR-1 line 37; FR-4 lines 74–89 and 108; 8.5 line 260):</strong> FR-1 says Markdown → HTML uses "all 15  `Options::ENABLE_*` flags listed in FR-4", but FR-4 exposes only 7 and  fixes the other 8 off; Reset enables those 7 and disables the remaining 8.  Enabling all 15 would make `ENABLE_GFM`, the metadata-block flags,definition lists, superscript, subscript, and wikilinks change output with  no sidebar control, and the first conversion after a reset would not match FR-1. FR-4 also lists the 8 fixed-off flags as ordinary bullets in the same option list, so they read as controls. <em>Fix: say FR-1 uses the 7 exposed flags with their sidebar/preset values and that the other 8 are never enabled; move the 8 into a clearly labeled "fixed off" paragraph or drop them from the option list.</em>~~
[fixed]
*~~ <strong>R11-M2 — The preview still does not remove `<noscript>` or `<template>`, and the new template rationale is inaccurate (FR-1 lines 46–47; 8.6 line 286):</strong> The round-10 fix added `<noscript>` and  `<template>` to the explanation and the 8.6 test but not to the removal  list, which still contains only `<base>`, `<meta>`, `<link>`, and  `target`. `html5ever`'s tree builder defaults scripting_enabled: true` (`tree_builder/mod.rs:47–75`), so `<noscript>...</noscript>` is parsed as  one text node and the walk sees no `<meta>`/`<link>` inside it. The serializer also defaults `scripting_enabled: true`, and its `write_text`  writes `<noscript>` text raw (`serialize/mod.rs:214–227`), so  `<noscript><meta http-equiv="refresh" ...></noscript>` or  `<noscript><a target="_self" ...></noscript>` is emitted unescaped; the  sandboxed preview, which parses with scripting disabled, turns it back  into live markup, defeating the removal. Separately, `RcDom` stores  `<template>` contents in `template_contents`, not `children`; the FR-1 walk  has no rule for that field and the serializer ignores it  (`markup5ever_rcdom-0.39.0+unofficial/lib.rs:642–658`), so the contents are  dropped from the preview rather than becoming a live shadow root, while the  `<template>` element itself remains and 8.6 still requires its absence.  8.6 also misspells `<nonscript>`, so that check does not exercise the real  element. <em>Fix: remove `<noscript>` and `<template>` whole in the  preview walk (or parse with scripting disabled so noscript contents are  visible to the walk, but still remove both elements); correct the rationale  to say template contents are dropped by the serializer; spell `<noscript>`  in 8.6; add a `<noscript>` case carrying `<meta http-equiv="refresh">` and  a `<template shadowrootmode="open">` case with contents.</em>~~
* ~~<strong>R11-M3 — The persisted `cmark` object stores all 15 flags although
  FR-4 fixes 8 off (FR-8 line 165; FR-4 lines 74–89):</strong> The schema
  lists `yaml_style_metadata_blocks`, `pluses_delimited_metadata_blocks`,
  `old_footnotes`, `gfm`, `definition_list`, `superscript`, `subscript`, and
  `wikilinks`, and validation only checks that the values are booleans. A
  stored `gfm: true` is therefore accepted; if the app applies it, it enables
  an out-of-scope behavior with no sidebar control, and `old_footnotes: true`
  also sets the `ENABLE_FOOTNOTES` bit
  (`pulldown-cmark-0.13.4/src/lib.rs:715`), turning footnotes on even when
  `footnotes` is false. If the app instead ignores the 8, they are dead
  persisted state that can disagree with the UI and with Reset. Round 6
  recorded this object as holding the 7 exposed flags, so this is a
  regression in the schema. <em>Fix: store only the 7 exposed flags; or
  validate/force the other 8 to `false` on read and state that they are
  ignored. Update the schema list and the 8.6 invalid-payload tests.</em>~~
Adjudication: moot: bjones1's Nov 11 PR comment ("Allow all available pulldown-cmark flags") superseded it; all 15 flags are sidebar-exposed (FR-4) and persisted (FR-8), so none is fixed-off state.
[fixed]
* ~~<strong>R11-M4 — The fidelity round trip leaves the non-`translation_mode` `htmd` options unstated (FR-5 lines 114–116; 8.7 line 300):</strong> FR-5  fixes the `pulldown-cmark` options for the HTML → Markdown → HTML round trip but requires only `translation_mode: Faithful` from`htmd`; it does  not say whether `heading_style`, `link_style`, `preformatted_code`, and the  other sidebar options apply to the fidelity conversion or whether it uses  fixed crate defaults. 8.7 pins only `translation_mode: Faithful` and the  fixed round-trip options, so its golden expectations are not reproducible.  The choice is observable: `preformatted_code` changes inline-code handling  (`src/element_handler/code.rs:141–149`, `223–244`); with `true`, a newline  in a `<code>` element becomes a space and the span stays a code span, while  the default serializes the element as raw HTML. That difference survives  the round trip into the canonical HTML and changes the diff. <em>Fix:  state the exact `htmd` option set used for the fidelity conversion (all  sidebar options, or crate defaults plus `Faithful`) and pin every one in  8.7.</em>~~
[fixed]
~~* <strong>R11-M5 — The crash marker is removed before the user chooses a recovery action (FR-8 line 156; FR-2 line 59; 8.6 line 287):</strong> On startup with `htmd_cmark_live_crashed` present, FR-8 removes the marker, restores the buffers, and shows the recovery banner without converting. If  the user reloads again before choosing `Convert restored input` or `Clear buffers`, the marker is gone, so startup runs the initial conversion on the  crashing input, panics, and shows the overlay again; the recovery state  survives only one reload and the 8.6 test, which reloads once, passes  anyway. <em>Fix: keep the marker until the user clears the buffers or a  conversion of the restored input succeeds, and clear it in those actions  rather than at startup. Also state whether `Clear buffers clears the output and diff as well as the inputs.</em>~~
[fixed]
~~* <strong>R11-M6 — The 8.6 concurrency test introduces an async gap the execution model does not have (FR-2 line 55; 8.6 line 271):</strong> FR-2 says conversion runs synchronously on the main thread and a run's commit is one synchronous step; at most one run is in flight and one is queued. 8.6 slows a conversion with a "300 ms async delay between compute and commit" and has newer input arrive during the delay. A synchronous compute/commit pair cannot yield at that point; if the test hook yields, the run is asynchronous in a way FR-2 does not define. With one in-flight run and one queued run, the newer input cannot produce a "newer output" while the delayed run is still in flight — it can only be queued — so the stale run has nothing newer to overwrite. If the hook busy-waits instead, no input event can arrive. <em>Fix: put the delay at a defined async boundary (before dispatch, or on the iframe `load` path) and test the queue/sequence behavior there, or explicitly define a test-only suspension point in FR-2 and state that the stale run writes nothing and the queued run then commits.</em>~~

#### Low-severity / editorial

* ~~<strong>R11-L1 — 8.6's "no `target` attribute" is unsatisfiable with the
  injected `<base target="_blank">` (FR-1 line 47; 8.6 line 286):</strong>
  8.6 requires the injected `srcdoc` to contain no `target` attribute, but
  FR-1 deliberately injects `<base target="_blank">` after the CSP `<meta>`.
  If the test inspects the final `srcdoc`, as written, it always finds that
  target. <em>Fix: require no user-supplied `target` attribute and exclude the
  demo's own `<base>`, or inspect the content before the base is
  injected.</em>~~
Adjudication: resolved by the current 8.6 wording "no user-supplied `target` attribute (excluding the demo's own injected `<base target="_blank">`)" (demo-spec.md line 291).
[fixed]
~~* <strong>R11-L2 — "Use Output as Input" does not identify which output view it copies (FR-2 line 61; FR-3 lines 66–69; 8.5 line 263):</strong> The output pane has a Source tab (the generated Markdown or HTML string) and a Rendered tab (the live preview; in HTML → Markdown it is HTML re-rendered from the Markdown). "Copies the current output" and 8.5's "active output" do not say whether the copied value is the Source string or the Rendered DOM, nor whether the active tab matters. In HTML → Markdown, copying the  Rendered HTML into the opposite (Markdown → HTML) buffer is a different action from copying the Source Markdown into it. <em>Fix: state that the generated Source string is copied regardless of the active tab, or define the tab-dependent behavior.</em>~~
[fixed]
~~* <strong>R11-L3 — The `Err` path leaves the output and diff state undefined (FR-7 line 146):</strong> The future-proof wrapper's `Err` branch shows a  non-fatal banner that clears on the next input change, but it does not say  whether the output and diff are cleared, left stale, or marked invalid. If  they remain, "Use Output as Input" can copy the previous input's result  while the current input failed, and the banner can disappear on the next keystroke without a successful conversion. <em>Fix: on `Err`, clear the output/diff or mark them invalid and disable "Use Output as Input"; state  whether the banner survives until a successful conversion.</em>~~
[fixed]
~~* <strong>R11-L4 — Editorial defects, mostly from the round-10 edits:</strong> line 37 has a double space in "with  all"; line 106 reads "f the parsed  list"; line 118 runs `).*Accepted limitation:*` together without a space or line break and calls fragment-parser wrappers "Automated"; line 144 has "then then halts"; line 286 misspells `<nonscript>` and its example  `<noscript><a href="https://x.example" target="_self"> and <template shadowrootmode="open">` has unclosed tags; line 300 has "the CommonMark-basics sample is part" where the subject is plural. <em>Fix: correct the spelling, spacing, and agreement.</em>~~

Still open from earlier rounds: R10-M1 (a run that writes nothing or clears
the preview never fires the `load` that ends the in-flight state), R9-L3,
R9-L7, R9-L8 (unit convention), R9-L9, and round 8's browser-floor and
editorial findings. R10-M2, R10-M4, R10-L1, R10-L2, R10-L3, R10-L4, and
R10-L5 are treated as resolved by the round-10 edits; R10-M3 is only partly
resolved (see R11-M2).

**Total new findings: 10 (blocking 0, medium 6, low 4).**

Validation: specification and source inspection only. No source or
specification files were changed by this review. `cargo build`, `cargo
clippy -- --deny warnings`, and `cargo test` were not run; a runtime probe was
attempted but blocked by Windows Application Control (`os error 4551`).

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **IR3** | **8** (2 medium, 6 low) | Reviewed the post-IR2 additions (benchmark corpus, `demo/README.md`, `demo-ci.yaml`): the corpus matches §6 byte-for-byte against the shipped samples, and the build, size gate, and Pages deploy are correct. 7 fixed — README now states the `wasm-bindgen-test` browser suite is not yet available (IR3-M2); §6 amended to describe `real-page.html` as a synthetic real-world-style page (IR3-L1); manifest depth corrected to 9 by FR-7's measure (IR3-L2) and its line-ending note aligned with `.gitattributes` (IR3-L3); README Clippy (`--all-targets`) and CI-trigger descriptions corrected (IR3-L4, IR3-L5); CI no longer cancels in-progress runs on `main`, protecting Pages deploys (IR3-L6). 1 adjudicated — the browser/Playwright/axe-core/fuzz/benchmark half of Deliverable 3 (IR3-M1) is blocked on Nurlybek's pending browser tests. Zero blocking. |

#### Last implementation review

**LLM and prompt used:** Prompting **Claude Opus 5.5** (`claude-opus-5-5`, Claude Code in VS Code, fresh context) with the implementation-review round-3 prompt quoted verbatim below.

## Implementation review round 3

```text
You are a fresh-context LLM reviewer performing Implementation Review Round 3 for the htmd / pulldown-cmark live demo (demo/). Implementation Review Rounds 1 and 2 (## Implementation review round 1 and ## Implementation review round 2 in demo/review.md) already ran and converged; since then, demo/bench/corpus/, demo/README.md, and .github/workflows/demo-ci.yaml were added.
Your task (execute all steps in order):
1. Perform the Review: Read demo/demo-spec.md (the authoritative spec) and inspect the implementation under demo/ (src/, index.html, Trunk.toml, Cargo.toml, bench/corpus/, README.md) as well as .github/workflows/demo-ci.yaml and the recent rounds in demo/review.md (do not re-litigate previously struck/resolved findings). Note that Windows Smart App Control (os error 4551) blocks local Cargo execution on this machine, so perform source/spec inspection without running local builds.
2. Record Verbatim in demo/review.md: Append a new section titled ## Implementation review round 3 to demo/review.md matching the exact structure of ## Implementation review round 1 and ## Implementation review round 2:
	• Quote this entire prompt verbatim in a fenced code block at the top.
	• State reviewer identity and scope.
	• Include subsections: #### What checks out, #### Blocking issues, #### Medium-severity issues, #### Low-severity / editorial (using IDs IR3-B#, IR3-M#, IR3-L# with file:line references and concrete fixes, or None. if none exist), and a total count summary.
3. Fix or Strikethrough Every Finding: For every finding raised in Round 3:
	• If it is a genuine defect in demo/, demo/README.md, demo/bench/corpus/, or .github/workflows/demo-ci.yaml, fix it in code/docs and mark it **Status:** fixed (...) in demo/review.md.
	• If a finding is non-applicable, blocked by a known constraint (e.g., pending Nurlybek's browser/Playwright tests), or superseded, wrap the finding in ~~strikethrough~~ and add an Adjudication: note explaining why.
4. Update demo/submission.md: In demo/submission.md, under my section (### Muzanbekova Kamila) ONLY:
	• Replace (Table showing implementation review rounds) under #### Implementation review rounds with a Markdown table summarizing **IR3** (finding counts by severity and main changes/resolutions made, matching the table format in Danial's section).
	• Replace (LLM and prompt used) and (LLM output, possibly with strikethrough) under #### Last implementation review with the LLM model used, the verbatim prompt, and the verbatim Round 3 review section (including any strikethroughs/resolutions) from demo/review.md.
	• Verify that Step 4's earlier fixes in my section are intact (one-sentence #### Contributions, #### Specification review rounds containing only rows **10** and **11**, and **11** stating all 10 findings were resolved).
Preserve existing line endings (CRLF) when editing demo/review.md and demo/submission.md. Do not modify any other teammate's section in demo/submission.md.
```

Reviewed by **Claude Opus 5.5** (`claude-opus-5-5`, Claude Code in VS Code, fresh
context) with the round-3 implementation-review prompt quoted above. Scope: the
artifacts added after IR2 converged — `demo/bench/corpus/` (four documents plus
the manifest), `demo/README.md`, and `.github/workflows/demo-ci.yaml` — checked
against `demo/demo-spec.md` §6, §7 Deliverables 2–4, and §8.1/§8.3, plus a
cross-check of those artifacts against `demo/src/` (`samples.rs`, `convert.rs`),
`index.html`, `Trunk.toml`, `Cargo.toml`, and `.gitattributes`. IR1/IR2
findings and their rulings were not re-litigated. Evidence is source and spec
inspection only: Windows Smart App Control (`os error 4551`) blocks local Cargo
execution, so `cargo build`/`clippy`/`test`, `trunk`, and the gzip size check
were not run. Corpus claims were checked with a standalone Python script
(byte counts, sample repetition, element depth).

#### What checks out

* **Corpus contents match §6:211 exactly.** `demo/bench/corpus/` holds exactly
  four documents plus `README.md`. `basics-scaled.md` is byte-for-byte the
  `SAMPLES[0]` Markdown variant (samples.rs:116–135) joined 60 times by a blank
  line (19,859 bytes); `math-scaled.md` is the Math Markdown variant
  (samples.rs:182–186) ×182 (20,019 bytes); `inline-html.html` is the Inline
  HTML variant (samples.rs:171–177) ×85 (20,060 bytes) — all three ≈20 kB.
  `real-page.html` is 70,545 bytes (inside 50–100 kB), well-formed (every
  non-void element explicitly closed, explicit `<tbody>`), with 16 sections,
  tables, `<pre>` blocks, and definition lists. Every byte count in the
  manifest matches `wc -c` on the working tree.
* **Corpus bytes are stable across platforms.** `.gitattributes` marks
  `demo/bench/corpus/* -text`; `git ls-files --eol` shows `i/lf w/lf` for all
  five files, so the recorded sizes hold on Windows checkouts too (the
  manifest's prose said otherwise — IR3-L3).
* **CI covers §8.1 and the build half of Deliverable 3.** demo-ci.yaml runs
  `cargo clippy --locked --target wasm32-unknown-unknown -- -D warnings`
  (:58–59), native `cargo test --locked` (:61–62), `trunk build --release`
  (:71–72), and the gzip size check with `gzip -6 -c demo/dist/*.wasm | wc -c`
  failing at `>= 1572864` (:74–90, a strict `<` limit as §6:217 and §8.1:240
  require), guarded against zero or multiple `.wasm` files. Trunk is pinned
  (`TRUNK_VERSION: "0.22.1"`), and `Trunk.toml` `[tools]` pins
  `wasm-bindgen = "0.2.130"` (matching Cargo.toml) and
  `wasm-opt = "version_131"`; `data-wasm-opt="z"` is on the rust link
  (index.html:7). The demo crate's own lockfile is enforced (`--locked`, plus
  `cargo fetch --locked` before Trunk's unlocked build).
* **Deliverable 4 (Pages) is correct.** The release build passes
  `--public-url "/${{ github.event.repository.name }}/"`, the project-site
  path §7:232 requires; `upload-pages-artifact@v3` uploads `demo/dist`; the
  `deploy` job runs only on pushes to `main`, with exactly
  `pages: write` + `id-token: write` and the `github-pages` environment. The
  shipped build never enables `demo-panic-test`.
* **The root-repo gates stay independent.** The workflow lives in its own file,
  runs from `working-directory: demo`, and caches `demo` separately;
  `.github/workflows/ci.yaml` is untouched (§7:231, §8.1:238).
* **demo/README.md (Deliverable 2)** gives native and wasm instructions:
  prerequisites (the wasm target, Trunk pinned to CI's 0.22.1, Trunk-managed
  `wasm-bindgen`/`wasm-opt` with the version-match caveat), native
  `cargo test --locked`, CI's exact Clippy line, `trunk serve`, the
  `--public-url /htmd/` release build with the reason, the size check
  reproduced exactly as CI runs it, and `demo-panic-test` usage with the
  never-deploy rule (FR-7:148). Accuracy defects are IR3-M2, IR3-L4, IR3-L5.

#### Blocking issues

None.

#### Medium-severity issues

* ~~**IR3-M1 — demo-ci.yaml implements only the build half of Deliverable 3,
  and nothing consumes the benchmark corpus (.github/workflows/demo-ci.yaml:34–90;
  demo-spec.md:211, 231, 248):**~~ §7:231 also requires the
  `wasm-bindgen-test` suite on headless Chromium in debug and release (at
  minimum the FR-7 nesting cases under `--release`), the `demo-panic-test`
  crash-recovery runs in both profiles, a malformed-input fuzz corpus, the
  Playwright matrix (Chromium, Firefox, WebKit), and an axe-core scan failing
  CI on violations; §8.3:248 requires every corpus document to be measured via
  `performance.measure` (the `htmd:convert` measure from IR1-L4). None of these
  jobs exists, and no harness reads `demo/bench/corpus/`. <em>Fix: add a
  `browser` job (wasm-bindgen-test debug + `--release`, with and without
  `demo-panic-test`), a Playwright job running the three-browser matrix, the
  axe-core scan, the fuzz corpus, and a corpus benchmark asserting
  `htmd:convert` < 100 ms per document, and make `deploy` depend on it.</em>
  Adjudication: blocked by a known constraint — these suites are the browser
  and Playwright tests still pending from Nurlybek, and cannot be written or
  verified here (local Cargo execution is blocked by `os error 4551`).
  The CI build half and the corpus they need are in place; the README now says
  the browser suite is not yet available (IR3-M2), so no document claims
  otherwise.
* ~~**IR3-M2 — README documents a browser test suite that does not exist
  (demo/README.md:79–98, 131):**~~ the "Browser tests" section gives
  `cargo test --target wasm32-unknown-unknown` commands for a
  `wasm-bindgen-test` suite on headless Chromium, but `demo/Cargo.toml` has no
  `wasm-bindgen-test` dev-dependency and `demo/src/` has no
  `#[wasm_bindgen_test]` case or `wasm_bindgen_test_configure!(run_in_browser)`.
  A reader following the section gets no browser run (plain `#[test]`
  functions do not run under `wasm-bindgen-test-runner` in a browser), and the
  "Panic testing" `cargo test … --features demo-panic-test` line inherits the
  problem. <em>Fix: state that the suite is not yet available and that CI does
  not run it, keep the commands as the intended invocation, and flag the panic
  `cargo test` line.</em>
  **Status:** fixed (demo/README.md:85–90 opens the section with "Not yet
  available": no `wasm-bindgen-test` dev-dependency or cases exist, the browser
  suite, Playwright matrix, and axe-core scan are still to be added, CI does
  not run them, and the commands apply once the suite lands; the Panic testing
  section now notes that its `cargo test` line needs that suite).

#### Low-severity / editorial

* ~~**IR3-L1 — `real-page.html` is synthetic, but §6:211 requires "a saved
  real-world page" (demo-spec.md:211; demo/bench/corpus/README.md:20–21):**~~
  the manifest discloses that the page was written to resemble a
  documentation site rather than saved from the web, so the corpus departs from
  the spec's letter. The substance — a full-page document (doctype, head,
  nav, form, tables, code, lists, definition lists) inside 50–100 kB — is met,
  and a synthetic page avoids redistributing third-party content. <em>Fix:
  replace the file with a saved, redistributable page, or amend §6 under
  Deliverable 5 (spec updated to match the implementation).</em>
  **Status:** fixed (Deliverable-5 spec amendment: demo-spec.md:211 now reads
  "a real-world-style page, 50–100 kB: a synthetic documentation page whose
  markup mirrors a typical documentation site, recorded as synthetic in the
  manifest").
* ~~**IR3-L2 — the manifest's "maximum element depth is 11" uses a different
  convention from FR-7 (demo/bench/corpus/README.md:13):**~~ the deepest path is
  `html > body > div > main > article > section > ul > li > ul > li > a`;
  11 counts `<html>` and `<body>`, but FR-7:146 and `max_element_depth`
  (convert.rs:69–93) put children of `<body>` at depth 1, giving 9. A reader
  checking the corpus against the 512 limit sees an off-by-two number.
  <em>Fix: state 9 by FR-7's measure.</em>
  **Status:** fixed (manifest now reads "Maximum element depth is 9 by FR-7's
  measure (children of `<body>` at depth 1), well under the 512 limit").
* ~~**IR3-L3 — the manifest's line-ending note contradicts `.gitattributes`
  (demo/bench/corpus/README.md:15–18):**~~ it says a Windows checkout with
  `core.autocrlf=true` rewrites the corpus to CRLF and tells readers to measure
  committed bytes, but `.gitattributes` marks `demo/bench/corpus/* -text`, so
  Git never converts them (`git ls-files --eol`: `w/lf`). <em>Fix: say the
  files are `-text` and the sizes hold in every checkout.</em>
  **Status:** fixed (the note now cites the `-text` attribute and states the
  sizes hold in every checkout, including Windows with `core.autocrlf=true`).
* ~~**IR3-L4 — README claims plain `cargo clippy` lints the unit tests
  (demo/README.md:64–65):**~~ "`cargo clippy --locked -- -D warnings` checks
  the native build, which also compiles the unit tests' `cfg(test)` code" is
  wrong: without `--all-targets` (or `--tests`) Clippy checks only the library
  and binary targets, not `cfg(test)` code. <em>Fix: give
  `cargo clippy --locked --all-targets -- -D warnings`.</em>
  **Status:** fixed (demo/README.md:63–68 now says plain `cargo clippy` skips
  test targets and gives the `--all-targets` command).
* ~~**IR3-L5 — README understates the workflow's triggers
  (demo/README.md:140; .github/workflows/demo-ci.yaml:6–21):**~~ the README says
  the workflow "runs on changes under `demo/`", but its `paths` filters also
  include the root `src/**`, `Cargo.toml`, and the workflow file — correctly,
  since the demo depends on the root crate by path. <em>Fix: list all
  trigger paths.</em>
  **Status:** fixed (demo/README.md:151–152 now names `demo/`, the root `htmd`
  library's `src/` and `Cargo.toml`, and the workflow itself).
* ~~**IR3-L6 — `cancel-in-progress: true` can cancel a Pages deploy mid-way
  (.github/workflows/demo-ci.yaml:28–30):**~~ the concurrency group is per ref,
  so two quick pushes to `main` cancel the first run, including a `deploy` job
  already publishing; GitHub's Pages starter workflows avoid cancelling
  production deployments for this reason. <em>Fix: cancel only off `main`:
  `cancel-in-progress: ${{ github.ref != 'refs/heads/main' }}`.</em>
  **Status:** fixed (demo-ci.yaml:30–31 now cancels superseded runs only off
  `main`, with a one-line comment giving the reason).

**Total new findings: 8 (blocking 0, medium 2, low 6).**

Reviewer's judgment: the corpus, README, and workflow are sound in substance
— the corpus matches §6 byte-for-byte against the shipped samples, and the
build, size gate, and Pages deploy implement §7 Deliverables 3–4's build half
and §8.1 correctly. The only substantive gap (IR3-M1) is the browser/Playwright
half of Deliverable 3, which waits on Nurlybek's tests; the rest are
documentation-accuracy fixes and one deploy-safety setting.

Validation: source and spec inspection only; `cargo build`, `cargo clippy`,
`cargo test`, `trunk`, and the gzip check were not run (Windows Smart App
Control, `os error 4551`). Corpus sizes, sample repetition, and element depth
were verified with a standalone Python script. Files changed by the fixes:
`demo/README.md`, `demo/bench/corpus/README.md`,
`.github/workflows/demo-ci.yaml`, and `demo/demo-spec.md` (§6:211); no Rust
source changed.

**Implementation review round 3 resolution:** 8 findings — 7 fixed (IR3-M2
README browser-suite status; IR3-L1 §6 corpus amendment; IR3-L2/L3 manifest
depth and line-ending notes; IR3-L4/L5 README Clippy and trigger accuracy;
IR3-L6 deploy-safe concurrency) and 1 adjudicated (IR3-M1, blocked on
Nurlybek's pending browser/Playwright tests). Zero blocking; no findings remain
open from IR3 except the adjudicated browser-test work.

### Danial Baitakov

GitHub userid: exekerey

#### Contributions

* Authored the specification fixes from the instructor's PR feedback, drove specification review rounds 12–14 to convergence, and implemented the complete demo application (conversion engine, fidelity pipeline, storage, preview sandbox, and UI) through implementation-review convergence with LLM assistance.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **12** | **5** (2 medium, 3 low) | All 5 fixed and the stale round-8/9 items closed by adjudication: `<noscript>`/`<template>` whole-element removal added to FR-1's preview walk (R12-M1); Long-Tasks-API monitoring scoped to Chromium in §6/§8.3 (R12-M2); the 2 MB output cap defined to clear both tabs and the fidelity panel and disable "Use Output as Input" (R12-L1); the wasm test suite run in debug and release profiles with the FR-7 nesting cases `--release` (R12-L2); the benchmark corpus defined as four named files plus a `README.md` size manifest (R12-L3). |
| **13** | **4** (2 medium, 2 low) | All 4 fixed: the 400 ms end-to-end and 100 ms compute budgets scoped to corpus-scale documents under a superseding 1,000 ms adversarial ceiling (R13-M1); Deliverable 3's crash-recovery cases wired to a `demo-panic-test` CI build (R13-M2); FR-5 reworded to "the preset's `htmd` options mirror the crate defaults" (R13-L1); the panic overlay placed outside the inert application root (R13-L2). |
| **14** | **2** (2 low) | Both trivial lows fixed: the deliberate-panic verification runs with `demo-panic-test` in both debug and release profiles (R14-L1) and §6's focus-restoration clause scoped to non-terminal errors (R14-L2). Convergence — zero blocking, zero medium, no findings open from rounds 1–14. |

#### Last specification review

**LLM and prompt used:** Prompting **GLM-5.3** (OpenCode agent, fresh context) with the round-14 specification-review prompt quoted verbatim below.

## Round 14 — open findings (awaiting processing)

```text
You are a fresh-context LLM reviewer performing Round 14 of an iterative specification review for a university assignment. Rounds 1–13 already ran; round 13's four findings were just resolved (timing scoping, demo-panic-test CI build, FR-5 preset wording, FR-7 overlay placement). Round 13's reviewer judged the remaining issues near-trivial. Your job: verify the round-13 fixes landed correctly and find anything still wrong. If only trivial or contradictory findings remain, say so plainly — that is the expected outcome and a legitimate result.

## The document under review

Read `/home/danial/htmd/demo/demo-spec.md` in full (~314 lines). It specifies a browser demo app ("pulldown-cmark / htmd live demo") to be implemented as a standalone Rust crate at `demo/` depending on this repo's `htmd` crate.

## What to check it against

The spec must be implementable exactly as written, against the real crate and dependency APIs available locally:

1. The htmd crate (`/home/danial/htmd/src/`): `src/lib.rs` (`convert`, `HtmlToMarkdown`, builder `.options()`/`.skip_tags()`/`.build()`, `.convert()`, `.html_to_tree()`, `.tree_to_markdown()`), `src/options.rs` (12 fields, `Pure`/`Faithful`, spacing ranges), `src/element_handler/` (skip semantics, empty-`skip_tags` panic guard).
2. Pinned versions in `/home/danial/htmd/Cargo.toml` and §6: html5ever 0.39, markup5ever_rcdom 0.39, pulldown-cmark 0.13.4, `similar`, Leptos csr, Trunk.
3. Internal consistency: FR-1…FR-8 vs §8 acceptance criteria; numbers consistent; no contradictions — especially around the round-12/13 edited text (removal list, Long-Tasks scoping, output-cap clearing, corpus definition, 400 ms/1,000 ms timing scoping, demo-panic-test feature, overlay placement).
4. Implementability in wasm.

Do NOT review style preferences. Do NOT re-litigate struck/resolved findings in `/home/danial/htmd/demo/review.md` (read rounds 10–13 first).

## Output

Append to `/home/danial/htmd/demo/review.md` a section `## Round 14 — open findings (awaiting processing)` mirroring the established round format exactly (prompt quoted verbatim in a fenced block, reviewer identity, "What checks out", "Blocking issues", "Medium-severity issues", "Low-severity / editorial", IDs `R14-B#`/`R14-M#`/`R14-L#` with line refs and concrete fixes).

CRLF WARNING: working tree is CRLF — append with line-ending preservation (e.g. Python `open(path, "a", newline="\r\n")`); verify `git diff --stat demo/review.md` shows only appended lines.

Then commit ONLY review.md: `git add demo/review.md && git commit -m "Spec review round 14: N findings (X blocking, Y medium, Z low)"` (real numbers). Do NOT modify demo-spec.md. Do NOT push.

Report back ONLY (under 12 lines): counts by severity, one-line summary of any blocking/medium findings, confirmation the four round-13 fixes landed correctly, whether any finding repeats a struck/resolved one, your explicit judgment whether all remaining findings are trivial/contradictory, the commit SHA, and confirmation demo-spec.md is untouched.
```

Reviewed by **GLM-5.3** (OpenCode agent, fresh context) with the round-14 prompt quoted
above. The review re-verified all four round-13 fixes in the current text (308 lines,
commit `ff027d1`) before hunting for new ones, against `src/lib.rs`, `src/options.rs`,
the `src/element_handler/` inventory and its panic guard, and the root `Cargo.toml`.
`similar` is not a dependency of this repository, so FR-5's `from_words` claims rest on
rounds 11–13's Cargo-registry verification.

#### What checks out

* All four round-13 fixes landed correctly: the 400 ms end-to-end budget is now scoped
  to corpus-scale documents with adversarial inputs explicitly bounded by the
  1,000 ms ceiling, which §8.3 states supersedes the 400/100 ms budgets, and §8.3's
  long-task exemption grants scheduled runs "the full applicable ceiling" (§6 line
  215; §8.3 lines 250 and 252; §8.5 line 264); Deliverable 3 (line 231) runs the §8.6
  crash-recovery cases against a `demo-panic-test` build that is never enabled in the
  shipped Pages build; FR-5 (line 116) now reads "the preset's `htmd` options mirror
  the crate defaults"; FR-7 (line 148) places the overlay outside the inert
  application root so it stays operable and focusable.
* FR-4's 12 options, variants, defaults, and 1–4 spacing clamps still match
  `src/options.rs` (fields 4–22, defaults 27–40, clamp docs 12–19); the Faithful-mode
  `br_style` note matches `src/options.rs:63–65`.
* The builder surface FR-4/FR-7 rely on exists as described: `.options()`
  (`src/lib.rs:241`), `.skip_tags()` (248), `.build()` (285), `convert` (212),
  `html_to_tree` (148, `parse_document`), and `tree_to_markdown` (170, returns
  `String`); the empty-slice panic guard is the `assert!(!tags.is_empty())` at
  `src/element_handler/mod.rs:400`, which FR-4's omit-the-call rule avoids.
* `Cargo.toml` matches §6's pins: htmd 0.5.5, html5ever 0.39, markup5ever_rcdom 0.39,
  pulldown-cmark 0.13.4. The handler inventory (no `del`/`s`, checkbox `input`,
  `sup`/`sub`, or `dl`/`dt`/`dd` handlers; tables in `table*.rs`/`td_th.rs`/`tr.rs`,
  math in `span.rs`) supports FR-5 line 120's "the only extensions htmd targets"
  claim, so the fixed `ENABLE_TABLES | ENABLE_MATH` round-trip set remains sound.
* Numbers stay consistent everywhere they interact (2,000/2,001; 100,000/100,001;
  250/900 ms; 500 ms; 512/600; 5,000 tokens; 2,000,000; 1,572,864; 1500 ms; 200
  characters; `version: 1`), and the FR-8 schema's `cmark` (15 fields) and `htmd`
  (12 fields) objects match FR-4's sidebar exactly.
* Wasm constraints hold: no filesystem, thread, or `Instant` assumptions remain
  (§6 line 213 bans `similar`'s deadline for exactly that reason), and every required
  walk (preview strip, canonicalization, depth measurement) is specified iterative.

#### Blocking issues

None.

#### Medium-severity issues

None.

#### Low-severity / editorial

* ~~<strong>R14-L1 — FR-7's "verified with deliberate panics in debug and release
  builds" still has no wired verification in either profile (FR-7 line 148 vs
  Deliverable 3 line 231):</strong>~~ R13-M2's fix wired the §8.6 crash-recovery
  Playwright cases to a `demo-panic-test` build, but that build's profile is
  unspecified, and no other CI step enables the feature — the wasm-bindgen-test step
  runs both profiles feature-less. The second clause of R13-M2's finding (the
  hook-before-abort verification "in debug and release builds") therefore still has
  no CI home. This is the unapplied half of R13-M2's suggested fix, not a re-opening
  of the half that landed. <em>Fix: one clause in Deliverable 3 — the deliberate-panic
  hook verification runs in the wasm-bindgen-test step with
  `--features demo-panic-test` in both debug and release profiles (or the Playwright
  crash-recovery build runs in both profiles).</em>
  **Status:** fixed (Deliverable 3: the crash-recovery cases run with the
  `demo-panic-test` feature enabled in both debug and release profiles).
* ~~<strong>R14-L2 — "focus … is restored after errors resolve" is vacuous (§6 line
  223):</strong>~~ the only error path that moves focus is the panic overlay, which is
  terminal — the application root is `inert`, all controls are permanently disabled,
  and recovery requires a manual reload (FR-7 lines 148–149). Non-fatal banners
  (size/nesting limits, the wrapper `Err`, quota, multi-tab) never take focus, so no
  focus-moving error ever "resolves"; the clause cannot be satisfied, falsified, or
  tested. <em>Fix: drop the clause, or define the restoring path (e.g., "non-fatal
  errors never move focus; the overlay path ends in a reload").</em>
  **Status:** fixed (§6: focus is restored after a non-terminal error resolves;
  the panic overlay is terminal until a manual reload).

**Total new findings: 2 (blocking 0, medium 0, low 2).**

Reviewer's judgment: only trivial findings remain, which is the expected outcome at
this stage. Both lows are one-clause fixes — R14-L1 completes a fix whose primary
half already landed, and R14-L2 deletes an untestable clause. No structural,
correctness, implementability, wasm-feasibility, or numeric-consistency problem
remains in the specification.

Validation: repository and source inspection only; no runtime probe was run. No
source or specification files were changed by this review; `cargo build`, `cargo
clippy`, and `cargo test` were not run (no code under review changed).

**Round 14 resolution:** both trivial findings fixed (Deliverable 3 dual-profile
demo-panic-test runs; §6 focus clause scoped to non-terminal errors). The
specification review converged at round 14: zero blocking, zero medium, two
trivial lows — fixed. No findings remain open from rounds 1–14.

**Post-round-14 amendment (controller ruling):** §6/FR-7 release-profile strip wording corrected to strip=false + wasm-opt — Task 0 empirically verified rustc strip=true breaks wasm-opt 116–133 (memory.copy/target_features); recorded for transparency after convergence.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **IR1** | **8** (1 blocking, 3 medium, 4 low) | All 8 fixed: the JS trap listeners write the crash marker before revealing the overlay (IR1-B1); 2 MB output-cap tests added (IR1-M1); §8.7:306 diff-precision pipeline tests added (IR1-M2); the preserved-subtree style check scans every `;`-separated declaration (IR1-M3); the zero-delay path uses `requestAnimationFrame` (IR1-L1); mixed-shape nesting tests added (IR1-L2); `send_wrapper` added to §6's dependency list (IR1-L3); `performance.mark`/`measure` instrumentation added around conversion (IR1-L4). |
| **IR2** | **5** (5 low) | Verified all eight IR1 fixes landed without regression (suite 164 → 169), then resolved all 5 low findings — 3 fixed (IR2-L2 §8.6:275 shape conversions, IR2-L5 parse-`Err` propagation, IR2-L3 spec amendment rewording §6/§8.3's amplification vector to table cells) and 2 resolved by controller ruling (IR2-L1, IR2-L4). Convergence — zero blocking, zero medium, no findings open from IR1–IR2. |

#### Last implementation review

**LLM and prompt used:** Prompting **GLM-5.3** (OpenCode agent, fresh context) with the implementation-review round-2 prompt quoted verbatim below.

## Implementation review round 2 — open findings (awaiting processing)

```text
You are a fresh-context LLM reviewer performing Implementation Review Round 2 for a university assignment. Round 1 (section `## Implementation review round 1` in `/home/danial/htmd/demo/review.md`) found 8 issues, all just fixed (resolution block at that section's end; commits 002381a + d9cd3fa). Your job: (1) verify every IR1 fix landed correctly, (2) find anything still wrong in the implementation vs the spec. If only trivial findings remain, say so plainly — that is a legitimate outcome (the loop stops there).

## What to review

Same scope as round 1: `demo/demo-spec.md` (authority; spec review is closed) vs everything under `/home/danial/htmd/demo/`. The implementation is a standalone Leptos CSR crate wrapping the `htmd` crate: src/{options_model,convert,fidelity/{canonicalize,diff},engine,storage,limits,panic_hook,samples,preview,app,main}.rs + index.html + Trunk.toml + Cargo.toml + inline tests.

## Method

1. IR1 verification first — each finding (B1 trap-path crash marker; M1 2MB cap tests; M2 §8.7:306 pipeline tests; M3 style-declaration scan-all ruling; L1 rAF zero-delay; L2 mixed-shape nesting; L3 send_wrapper in §6; L4 performance marks): read the fix, confirm it addresses the finding and matches the spec clause. Known parked items (skip unless load-bearing): mode-switch limit-banner re-evaluation; banner-slot theoretical reassignment; canonicalize/preview serializer duplication; basics sample superset.
2. Then a fresh pass over FR-1…FR-8 clause-level compliance and §8 coverage, focusing where round 1 did NOT look (it mapped FRs broadly — you go deeper on the interaction seams: engine↔storage timer interplay, app.rs state ownership, recovery flow edge cases, sample/undo interplay with limits, the fidelity panel's data flow end-to-end, index.html CSP vs iframe sandbox interaction).
3. Known implementer concern to adjudicate: §8.3:249's "reference-link amplification" premise is wrong for pulldown-cmark 0.13.6 (link-ref expansion fuel caps output near input size; the cap tests amplify via empty table cells ~34×/`|` instead). Decide: is the acceptance criterion (cap behavior) still satisfiable and tested (yes, via table amplification), and does the spec's example wording need an amendment note, or is it fine as illustrative?
4. Tests: run `cargo test --manifest-path demo/Cargo.toml` only if a specific doubt needs evidence (fast suite; never --ignored). Root gates from /home/danial/htmd. No trunk --release runs (slow; last measured gzip 389,845).

## Output

Append to `/home/danial/htmd/demo/review.md` a section `## Implementation review round 2 — open findings (awaiting processing)` in the established format (prompt verbatim in fenced block, reviewer identity, "What checks out" incl. per-fix IR1 verification, "Blocking issues", "Medium-severity issues", "Low-severity / editorial", IDs `IR2-B#`/`IR2-M#`/`IR2-L#`, file:line, concrete fixes).

CRLF WARNING: append with line-ending preservation; verify `git diff --stat demo/review.md` shows only appended lines.

Commit ONLY review.md: `git add demo/review.md && git commit -m "Implementation review round 2: N findings (X blocking, Y medium, Z low)"`. No other files. No push.

Report back ONLY (under 12 lines): IR1 fix verification summary (all landed? any regression?); counts by severity; one-line per blocking/medium; your adjudication of the §8.3:249 amplification wording; explicit judgment whether remaining findings are trivial; commit SHA; confirmation only review.md changed.
```

Reviewed by **GLM-5.3** (OpenCode agent, fresh context) with the round-2 implementation-review
prompt quoted above. The review re-verified all eight IR1 fixes in HEAD (commits `002381a` +
`d9cd3fa`), then made a fresh clause-level pass over FR-1…FR-8 and §8 focused on the interaction
seams round 1 did not cover, against the real `htmd` crate, the pinned `tachys` 0.2.19 and
`pulldown-cmark` 0.13.4/0.13.6 sources in the local Cargo registry, and the built `demo/dist/`.
Evidence runs: `cargo test --manifest-path demo/Cargo.toml` — **169 passed, 0 failed, 2 ignored**
(the slow depth proofs stayed ignored); `gzip -6 -c demo/dist/*.wasm | wc -c` — **389,845 bytes**
(24.8% of the 1,572,864-byte ceiling). Parked items from the round-1 prompt were left unreported.

#### What checks out

**IR1 verification — all eight fixes landed; no regression.**

* **IR1-B1 (trap-path crash marker) — fixed in its load-bearing half.** `index.html:241-247`:
  `trapCaught()` writes `htmd_cmark_live_crashed` = `"1"` inside try/catch *before* revealing the
  overlay, from both the `error` and `unhandledrejection` listeners; the key and value match
  `storage.rs:26` and the Rust hook's write (panic_hook.rs:105-109), and FR-8:160's load path
  checks presence only — so reload recovery now works for traps that bypass the Rust hook. The
  unimplemented remainder of FR-7:149 (the write halt) and the resolution note's rationale are
  IR2-L1 below.
* **IR1-M1 (2 MB cap tests) — fixed.** `output_over_two_mb_cap_clears_both_tabs_and_disables_use_output`
  (app.rs:1820-1851) asserts the amplification actually crosses 2,000,000 bytes, the cap banner
  shows, both tabs are empty, the empty-output rule disables "Use Output as Input", and nothing
  stale commits on a re-run; `output_just_under_the_two_mb_cap_still_displays` (app.rs:1855-1879)
  pins the just-under boundary (>1.8 MB, no banner). Both ran green in the suite. The
  amplification vector's deviation from §8.3:249's wording is recorded in the test and adjudicated
  as IR2-L3.
* **IR1-M2 (§8.7:306 pipeline tests) — fixed.** `cjk_and_combining_mark_pipeline_zero_diff` and
  `white_space_style_span_pipeline_nonzero_diff` (canonicalize.rs:549-592) run the inputs through
  the real `convert_html_to_md` → `convert_md_to_html(CmarkFlags::tables_math())` → canonicalize →
  `word_diff` pipeline, asserting `Inclusive` for the CJK/U+0301 cases and `Changes(_)` for the
  styled span — exactly the suggested shape.
* **IR1-M3 (style-declaration scan-all) — fixed with a sound ruling.**
  `declares_non_normal_white_space` (canonicalize.rs:377-393) now scans every `;`-separated
  declaration with the *last* `white-space` occurrence winning per the CSS cascade — which is also
  how the browser actually applies the declaration block, so preserved-subtree detection matches
  rendering; `style_attr_detection_rules` (canonicalize.rs:753-774) pins
  `color:red; white-space:pre` → preserved, `white-space:pre; white-space:normal` → not.
* **IR1-L1 (rAF zero-delay) — fixed.** `WebClock::schedule` routes 0 ms events to
  `requestAnimationFrame` and positive delays to `setTimeout` (clock.rs:114-131); `cancel`
  retracts either handle kind (clock.rs:143-155). Matches FR-2:52's pinned primitive.
* **IR1-L2 (mixed-shape nesting) — fixed in substance.**
  `six_hundred_deep_mixed_shapes_measure_and_reject` (convert.rs:158-171) drives blockquote /
  ul / li / table / tr / td cycles through the real parse and asserts the walk's exact measured
  depth (602) plus the TooDeep rejection. The specific conversion shapes §8.6:275 names remain
  untested natively — IR2-L2.
* **IR1-L3 (send_wrapper in §6) — fixed.** demo-spec.md:219 now lists `send_wrapper` (0.6) among
  the direct dependencies (commit `d9cd3fa`); `Cargo.toml:21` and the lockfile (0.6.0) match.
* **IR1-L4 (performance marks) — fixed.** app.rs:275-307/329-334 brackets
  `convert_and_commit` with `htmd:convert:start` / `htmd:convert:end` marks and the
  `htmd:convert` measure, spanning dispatch through commit including canonicalization and diff —
  §6:211's exact scope — guarded for availability and skipped off-wasm, with the names documented
  in-code as the CI contract.
* No regressions: the suite grew 164 → 169 with the five fix tests; the size budget holds
  (389,845 bytes); the hygiene ledger is unchanged (exactly 5 targeted commented
  `allow(dead_code)` — storage.rs:25,40,75; panic_hook.rs:48; samples.rs:12; no `unsafe`; the few
  non-test `unwrap`/`expect` remain invariant-guarded with justifying comments, including the two
  new ones in clock.rs).

**Fresh pass — interaction seams round 1 did not cover.**

* **Engine ↔ storage timer interplay:** two independent `WebClock` registries re-enter `Core`
  through `LateCore`, which is filled after `Core` exists but before any timer can arm (app.rs:917-945);
  one-shot closures self-deregister before delivering, so re-entrant `schedule`/`cancel` from a
  handler sees consistent state (clock.rs:105-109); the storage clock and every app-side storage
  call site check `panic_hook::storage_halted()` (app.rs:936-938, 552, 627, 643, 678, 690, 741,
  776, 803, 831, 848, 868, 1010); all borrows are statement-scoped, so no re-entrancy deadlock is
  reachable.
* **app.rs state ownership:** reactive state lives in `Copy` `Ui` signals; non-reactive modules in
  `Core` behind one `SendWrapper<Rc<RefCell<Core>>>` opened only on the main thread; handlers
  re-fetch through the `CORE` thread_local instead of capturing, keeping closures `Fn` + `Send`.
  Every buffer mutation reaches the storage controller's in-memory state (`on_text_edit` on every
  path), so no data loss is possible regardless of debounce timing — B4's concern is closed by
  construction.
* **Recovery-flow edges:** the marker is kept until the user acts and removed only on a successful
  recovery conversion (storage.rs:485-501, app.rs:431-436); an oversized restored buffer halts with
  the banner and keeps `recovery_converting` pending (tested, app.rs:1979-1996); a lone marker
  with no workspace is removed (storage.rs:327-333); an invalid workspace takes the marker with it
  (storage.rs:346-355); "Clear buffers" empties both buffers, removes the marker, saves, and
  mirrors the engine for both modes (app.rs:865-880). One letter-compliant edge remains — IR2-L4.
* **Sample/undo × limits:** every programmatic buffer overwrite that can carry arbitrary size
  ("Use Output as Input", Undo, recovery conversion) runs `overwrite_with_ceiling_check` — the
  eager 100,000-byte gate with banner, halt, and cleared target output — pinned by three tests
  (app.rs:1933-2022); sample loads are ≤ ~1 kB so they cannot cross a limit; the cross-mode
  sample click flushes storage via `set_mode` and the selection flush, so nothing is lost.
* **Fidelity panel end-to-end:** the panel is computed inside the commit gate — output
  `srcdoc`, Source text, and fidelity are written in one synchronous step behind the run's
  sequence number (app.rs:376-414); the round-trip side renders under exactly
  `ENABLE_TABLES | ENABLE_MATH` with a negative control proving sidebar flags would diverge
  (samples.rs:738-780); both sides canonicalize through the same function; Pure mode and the Err
  path clear the panel. The identical-output corner was analyzed and is benign: Leptos memo
  equality can skip the `srcdoc` rewrite so no `load` fires for that sequence, but every batch arm
  follows `advance_seq`, which releases the stale in-flight slot (engine.rs:327-338), so no
  permanent stall is reachable — confirmed against tachys 0.2.19, whose property `rebuild` writes
  unconditionally whenever it does run (`html/property.rs:313-318`).
* **CSP × sandbox interaction:** `sandbox=""` is a static attribute (no tokens), the CSP
  `<meta>` sits inside the generated `srcdoc` ahead of the demo's own `<base target="_blank">`
  (preview.rs:17-42, pinned byte-exact by test), `style-src 'unsafe-inline'` covers pulldown's
  table-alignment inline styles, and the parent document needs no CSP of its own (its only inline
  script is the static bootstrap). `prop:srcdoc` assigns the DOM property; the Source tab and all
  banners/toasts render through text-content macros.
* **Dependencies/§6:** `Cargo.toml` matches §6:219's list exactly (including `send_wrapper` 0.6);
  the lockfile carries exactly one `html5ever` 0.39.0 and one `markup5ever_rcdom` 0.39.0+unofficial,
  the same versions the root crate uses.

#### Blocking issues

None.

#### Medium-severity issues

None.

#### Low-severity / editorial

* ~~**IR2-L1 — FR-7:149's "disable further workspace writes" is still unimplemented on the JS trap
  path, and the recorded justification's premise is inaccurate (index.html:234-247; app.rs:930-945;
  storage.rs:561-580).**~~ A trapped wasm instance is not a dead runtime: a trap (panic-abort
  `unreachable`, stack exhaustion) surfaces as a per-call `RuntimeError` and the instance remains
  callable, so an already-armed 500 ms `SaveDue` timer re-enters wasm after the trap and writes —
  `STORAGE_HALTED` is set only by the Rust panic hook. The outcome is benign (the write persists
  exactly the state FR-8:160's recovery flow restores, and it never touches the marker, so no
  crash loop), but the spec clause is unmet and the index.html comment asserts a false
  impossibility. <em>Fix: set a `window.__htmdStorageHalted` flag in `trapCaught()` and make
  `WebStore::set` return `Err(StoreError::Blocked)` while that flag (or the marker key) is
  present; or keep the code and record the deviation in the Deliverable-5 spec update, rewording
  the comment to the true rationale (marker-gated reload recovery makes a JS-side write halt
  unnecessary).</em>
  **Status:** resolved-by-ruling (controller ruling: the JS trap listener cannot halt the Rust storage controller — its halted flag is Rust-side state only the panic hook sets, unreachable from JavaScript — so the write-halt half of FR-7:149 is impossible from the trap path, and the marker alone drives FR-8's recovery, which is sufficient. No behavior change; the index.html comment now states the true rationale — these handlers do run after a trap (a per-call RuntimeError, not a dead runtime), but the halt is impossible from here — replacing the inaccurate “dead runtime” premise).
* ~~**IR2-L2 — §8.6:275's named conversion shapes still have no native test (convert.rs:136-171).**~~
  The IR1-L2 fix proved the depth walk on a 600-deep mixed blockquote/list/table tree, but the
  criterion also names 600 unclosed `<p>` tags, 600 `<br>` siblings, and a ≤ 512-level
  list/blockquote/table stack converted in both `Pure` and `Faithful` — none of those `Ok`
  conversions exists anywhere in the suite (the release-wasm runs are Deliverable 3's, but these
  are cheap native conversions). <em>Fix: add the three conversions next to
  `five_hundred_twelve_deep_input_converts`, one asserting `Ok` for each shape and one
  Faithful-mode 512-deep stack.</em>
  **Status:** fixed (convert.rs gains §8.6:275's shapes beside five_hundred_twelve_deep_input_converts: six_hundred_unclosed_paragraphs_convert and six_hundred_br_siblings_convert assert Ok for the two flat shapes, and five_hundred_twelve_deep_stack_converts_in_both_modes builds a measured-exactly-512-deep blockquote/ul/li/table/tr/td stack (73 seven-level cycles plus a trailing blockquote) and asserts Ok in both Pure and Faithful modes).
* ~~**IR2-L3 — §8.3:249's "reference-link amplification" recipe is unsatisfiable on every parser
  version the spec considered and needs a Deliverable-5 amendment note (demo-spec.md:214, 249).**~~
  Verified in the local registry: pulldown-cmark initializes `link_ref_expansion_limit =
  max(text.len(), 100_000)` and consumes `url.len() + title.len()` of fuel per expansion
  (`pulldown-cmark-0.13.6/src/parse.rs:300, 376-378`; 0.13.4 has the same guard at parse.rs:288),
  so a ≤ 100,000-byte input cannot emit anywhere near 2 MB of reference-link output — the spec's
  example was never executable. The acceptance criterion's *substance* (the cap trips, banner
  shows, both tabs and the fidelity panel clear, "Use Output as Input" disables, boundary
  behavior) is satisfiable and now tested via aligned empty table cells (~34× per `|`: 1,200 rows
  → 2,053,756 bytes; 1,100 → 1,882,756, just under). <em>Adjudication: keep the criterion, amend
  the wording — replace "reference-link amplification" with table-cell (or generic output)
  amplification in §6:214 and §8.3:249 so the CI team's checklist recipe is executable.</em>
  **Status:** fixed (spec amendment: §6:214 and §8.3:249 now name empty table-cell amplification — ~34× per `|` — as the 2 MB-cap vector and record the mechanism: pulldown-cmark 0.13.x's link-reference expansion fuel (`max(len, 100_000)` bytes, consumed by each expansion's URL length) caps reference-link output near the input size, so the old recipe was never executable. Deliverable 5's submission notes never mentioned the recipe — grep finds no “reference-link” outside review.md and the amended spec — so no mirror was needed).
* ~~**IR2-L4 — the recovery banner and crash marker have no exit through normal editing
  (app.rs:844-880, 983-985; storage.rs:485-501).**~~ FR-8:160 defines marker removal only for the
  two banner actions; if the user instead types (or toggles an option) after a crash-restore,
  conversions run and commit successfully, but `recovery_converting` is armed only by
  `on_recovery_convert` — so the marker stays set, the banner (with its now-stale "restored
  without converting" text) remains, and every reload re-enters the recovery flow until one of
  the two buttons is clicked. Letter-compliant, but the crash-loop hazard the marker guards is
  gone once restored input converts successfully. <em>Fix: arm `recovery_converting` at
  startup on `LoadOutcome::CrashRecovery` so the first successful commit clears the marker (and
  dismiss the banner with it), or record the banner-only exit in the Deliverable-5 update.</em>
  **Status:** resolved-by-ruling (FR-8:160's words do not support a marker exit on a successful edit-commit: they bind removal to the two banner actions — “`Convert restored input` … removes the marker once conversion succeeds” and “`Clear buffers` … removes the marker” — while the startup clause says “keep the marker in localStorage”; nothing licenses an exit on a plain edit-path commit, so the banner-only exit is letter-compliant and the behavior stands unchanged).
* ~~**IR2-L5 — FR-7:150's "future-proof" wrapper is not actually forward-compatible
  (convert.rs:54).**~~ `html_to_tree(...).expect("infallible parse")` would panic — and
  `panic = "abort"` turns that into the terminal crash overlay — on a future htmd that returns an
  `Err`, instead of taking the specified non-fatal `Err` banner path. Untestable today (htmd
  0.5.5's parse has no reachable `Err`), so trivial, but the wrapper's stated purpose would not
  survive contact with the future it guards against. <em>Fix: propagate a parse error into the
  existing `Err` handling (a new `Blocked` variant or equivalent) whenever htmd grows one.</em>
  **Status:** fixed (the `expect("infallible parse")` at convert.rs:54 is gone: `html_to_tree`'s error now maps to a new `Blocked::ParseFailed`, which flows through the existing Err handling — a third `BannerKind::ConversionError` (FR-7:150's non-fatal error banner; the spec fixes no verbatim text for it) enters once per transition, clears the output, and exits on the next successful commit for the same mode. Unreachable on htmd 0.5.5, where html5ever reports parse errors without failing, so the suite pins the Blocked→BannerKind mapping instead).

**Total new findings: 5 (blocking 0, medium 0, low 5).**

Reviewer's judgment: all eight IR1 fixes landed without regression, and the remaining findings
are minor — two narrow state-machine edges with benign outcomes and one-line fixes or spec notes
(IR2-L1, IR2-L4), one cheap coverage add (IR2-L2), one one-line spec amendment (IR2-L3), and one
untestable-until-htmd-changes nit (IR2-L5). No structural, correctness, security, or consequential
spec-noncompliance remains; the implementation review loop can reasonably stop after one small
cleanup pass (or here, if the team prefers to fold L1/L3/L4 into the Deliverable-5 spec update
and L2/L5 into a follow-up commit).

Validation: `cargo test --manifest-path demo/Cargo.toml` (169 passed, 0 failed, 2 ignored) and
`gzip -6 -c demo/dist/*.wasm | wc -c` (389,845 bytes) were run for evidence; the `tachys` 0.2.19
and `pulldown-cmark` 0.13.4/0.13.6 sources were read from the local Cargo registry; `trunk` was
not run and no root-repo gate was needed (no Rust source changed). No file other than
`demo/review.md` was modified by this review.

**Implementation review round 2 resolution:** all 5 low findings resolved (2 fixed, 2 by ruling, 1 spec amendment). The implementation review converged at round 2: zero blocking, zero medium. No findings remain open from IR1–IR2.

### Nurlybek Aimagambetov

GitHub userid: nurlybek-aimagambetov

#### Contributions

(Nurlybek's contributions)

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **15** | **2** (0 blocking, 1 medium, 1 low) | None, the issues don't seem applicable |

#### Last specification review

**LLM and prompt used:** **GPT-6.1 Sol Extra High**, with the exact prompt quoted in the Round 15 review below.

## Round 15 — open findings

```text
review demo/demo-spec.md.
```

Reviewed by **GPT-6.1 Sol Extra High** for **Nurlybek Aimagambetov** with the
exact prompt quoted above. Scope: the complete current `demo/demo-spec.md`,
checked for internal consistency against its acceptance criteria and the
repository's `src/options.rs`, conversion API, and relevant demo code. Earlier
review findings were checked to avoid repeating resolved issues.

#### What checks out

* FR-4's 12 `htmd` options, variants, and defaults match `src/options.rs`.
  The Faithful-mode `br_style` exception and the empty-`skip_tags` guard agree
  with the crate's API and implementation.
* FR-7's parse-once nesting check is supported by the public `html_to_tree`
  and `tree_to_markdown` methods. The size-limit boundary and output-clearing
  rules agree with the corresponding §8.6 acceptance cases.
* The Round 14 dual-profile crash-recovery requirement remains in Deliverable
  3, with `demo-panic-test` excluded from the shipped Pages build. The preview
  removal list and the user-supplied-attribute exclusions match §8.6.

#### Blocking issues

None.

#### Medium-severity issues
<s>
* **R15-M1 — Canonicalization deletes significant spaces between top-level
  inline elements (`demo/demo-spec.md:128`; §8.7:303,306).** FR-5 drops every
  whitespace-only text node that is a child of the fragment root, regardless
  of its neighbors. Consequently, `<span>a</span> <span>b</span>` and
  `<span>a</span><span>b</span>` both canonicalize to the latter string, even
  though their default rendering reads `a b` and `ab`, respectively. The
  fidelity check can therefore hide a real word-boundary loss. The existing
  `whitespace_only_text_dropped_at_fragment_root` test in
  `demo/src/fidelity/canonicalize.rs:724` confirms that the demo follows this
  specification rule; it is a specification defect rather than an accidental
  implementation deviation. **Suggested fix:** drop fragment-root whitespace
  at the outer edges or beside block elements, while retaining one collapsed
  space between inline siblings. Add a negative golden case requiring these
  two fragments to remain distinct; keep the existing trailing-block-newline
  and empty-fragment goldens.</s>

#### Low-severity / editorial
<s>
* **R15-L1 — Reset's banner-preservation promise conflicts with the storage
  banner lifecycle (`demo/demo-spec.md:113,176–178,265`).** FR-4 says Reset
  does not clear or change error banners, and §8.5 requires banners to remain
  untouched. However, FR-8 requires an immediate write on an option action
  and requires the next successful write to clear a prior save-failure
  banner. If a write fails transiently and Reset's write succeeds, the same
  banner must both remain and disappear. **Suggested fix:** say Reset does
  not dismiss banners directly, while its save and reconversion follow the
  normal banner lifecycles. Apply that qualification to §8.5 and add an
  acceptance case for Reset after a transient storage failure.
</s>
**Total new findings: 2 (blocking 0, medium 1, low 1).**

Reviewer's judgment: the specification is detailed and largely consistent,
but the fidelity normalization rule can mask a visible text change. The other
finding is a narrow contradiction in the Reset wording. Both findings remain
open; this review records suggested changes without modifying the spec.

Validation: `cargo build`, `cargo clippy -- --deny warnings`, and `cargo test`
all passed. The targeted command `cargo test --manifest-path demo/Cargo.toml
whitespace_only_text_dropped_at_fragment_root` also passed (1 test), confirming
the current whitespace rule. These checks do not establish browser rendering,
accessibility, or performance compliance; no browser suite was run. Only
`demo/review.md` and Nurlybek's review entries in `demo/submission.md` were
updated.

#### Implementation review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **IR4** | **6 new** (0 blocking, 5 medium, 1 low); IR3-M1 partly outstanding |  |

#### Last implementation review

**LLM and prompt used:** **OpenAI Codex (GPT-6)**, with the exact prompt quoted in the Implementation review round 4 below.

## Implementation review round 4

```text
Review the implementation in demo/ by comparing it to demo/demo-spec.md.
```

Reviewed by **OpenAI Codex (GPT-6)** for **Nurlybek Aimagambetov** with the
exact prompt quoted above. Scope: the current implementation under `demo/`
(`src/`, `index.html`, Cargo/Trunk configuration, benchmark corpus, README,
and Playwright suite), plus `.github/workflows/demo-ci.yaml`, compared with
`demo/demo-spec.md` FR-1–FR-8 and §5–§8. Earlier findings and their resolutions
were read first; resolved specification rulings are not reopened. This is a
review only: the findings below remain open, with suggested fixes.

#### What checks out

* Both conversion directions, all 15 `pulldown-cmark` flags, the 12 `htmd`
  options, the Pure demo preset, and Faithful's disabled line-break control
  match FR-1/FR-4. Empty parsed `skip_tags` omits the builder call.
* The preview uses the `srcdoc` DOM property, `sandbox=""`, its accessible
  title, and `referrerpolicy="no-referrer"`. `preview.rs` parses fragments,
  strips the specified elements and target attributes iteratively, and
  injects the required CSP followed by the demo's own base element. Source
  and fidelity text use ordinary Leptos text nodes.
* HTML conversion parses once, checks the parsed tree's depth before
  `tree_to_markdown`, and has native boundary/shape tests. Faithful fidelity
  uses fixed tables/math round-trip flags, canonicalizes both sides, and
  bounds the changed token middle before diffing. The storage controller
  validates the versioned schema and tests quota, external-write, and
  crash-marker transitions. The UI wiring defects below are separate from
  these passing module tests.
* The previously missing Playwright matrix and axe scans now exist:
  `demo-ci.yaml:106–154` runs Chromium, Firefox, and WebKit against the release
  artifact, and deployment depends on those jobs. Axe covers five UI states;
  the corpus timing suite reads all four committed documents and exercises
  HTML in both Pure and Faithful modes. Manifest byte sizes match the files:
  19,859; 20,019; 20,060; and 70,545 bytes. The release profile, pinned Trunk
  tools, public project path, and strict gzip-size gate match §6/§7.

**Carried forward — IR3-M1 is only partly addressed.** The new matrix, axe
scans, and corpus measurements close those portions. Deliverable 3 still
requires headless-Chromium `wasm-bindgen-test` runs in debug and release,
release-wasm nesting proofs, crash-recovery runs with `demo-panic-test` in
both profiles, and a malformed-input fuzz corpus. `demo/Cargo.toml:1–52`
contains no `wasm-bindgen-test` dependency; `demo-ci.yaml:35–169` has no such
jobs or feature-enabled test builds. `demo/README.md:103–120,203–214`
correctly describes the missing wasm suite. Finish these remaining jobs and
fixtures before treating IR3-M1 as closed; this is an existing finding, not
counted again below.

#### Blocking issues

None.

#### Medium-severity issues

* **IR4-M1 — Panic disables an empty container instead of the application
  (`demo/src/main.rs:21`; `demo/index.html:219`;
  `demo/src/panic_hook.rs:83–88`).** The static `#app-root` is empty, but
  `mount_to_body(app::App)` appends the application to `<body>`, outside that
  element. The panic hook sets `inert` only on the empty div, leaving the
  actual mode controls, editor, tabs, samples, and options outside the inert
  subtree. The overlay covers them visually, but FR-7 requires all controls
  to become permanently inoperable; moving focus and `aria-modal` do not make
  those siblings inert. The intended `#app-root` layout rules also miss the
  application. **Suggested fix:** mount `App` into the existing `#app-root`,
  keeping the crash overlay outside it, and test a deliberate panic in both
  profiles by asserting every application control is within the inert root
  and cannot receive keyboard focus.

* **IR4-M2 — Reset or an option change can bypass the input-size ceiling
  after the first oversized edit (`demo/src/app.rs:563–576,674,705,343–348`;
  `demo/src/engine.rs:342–350,365–379`).** On the first transition above
  100,000 bytes, the editor handler updates the UI buffer but calls only
  `engine.on_limit_entered()`, leaving the engine's buffer mirror at its
  previous in-range value. Reset or an option change then schedules using
  that old length; conversion reads the oversized UI buffer instead. In
  Markdown → HTML mode there is no second input-size check at dispatch, so
  100,001 bytes convert, refill the cleared output, and clear the size
  banner. An isolated native reproduction confirmed this sequence.
  **Suggested fix:** synchronize the engine buffer on every edit, including
  the first oversized one, and enforce the input ceiling against the actual
  dispatched buffer before either conversion path. Add an app-level
  regression for paste-over-limit followed by Reset, an option change, and
  switching away/back; output must remain empty until input is in range.

* **IR4-M3 — Invalid output remains copyable during the debounce window
  (`demo/src/app.rs:554–583,663–676,814–819,1050–1055`;
  `demo/src/engine.rs:342–345`).** Edits and option changes invalidate only
  the engine's internal `output_valid`; the UI's separate `ModeOutput.valid`
  stays true until a commit or clear. Both the button's disabled predicate
  and `on_use_output` consult that stale UI flag. After a successful
  conversion, change a >2,000-byte buffer and click the button before its
  250 ms conversion: the previous Source is copied into the other mode and
  the pending conversion is cancelled by the switch. This violates FR-2's
  explicit invalid-output guard. An isolated native reproduction copied the
  stale result and switched modes. **Suggested fix:** invalidate the reactive
  output on every buffer/option change and check the same authoritative
  validity in the click handler. Test that the button remains disabled
  between an edit/option change and the matching successful commit.

* **IR4-M4 — Cross-mode sample Undo snapshots the wrong buffer
  (`demo/src/app.rs:757–761`; `demo/src/samples.rs:275–292,355–358`).**
  `on_sample_click` passes the current mode's buffer to `SampleState::load`,
  whose contract requires the target mode's buffer. With distinct Markdown
  and HTML buffers, click Inline HTML while in Markdown → HTML, then Undo:
  the HTML buffer becomes the Markdown buffer's old text, losing the HTML
  text the sample replaced. If the Markdown buffer is empty, no Undo is
  offered even when non-empty HTML was overwritten. An isolated native
  reproduction restored `Saved Markdown` instead of `<p>Saved HTML</p>`.
  **Suggested fix:** resolve the sample's target mode before taking the
  snapshot and pass that mode's buffer. Add an app/Playwright case for the
  §8.5 cross-mode Inline HTML action with distinct buffers, including an
  empty current buffer and a non-empty target buffer.

* **IR4-M5 — Performance tests do not enforce the required end-to-end,
  startup, or keystroke-jank budgets
  (`demo/e2e/performance.spec.ts:14–32,35–94`;
  `demo/e2e/fixtures.ts:22–31`; `demo/playwright.config.ts:17`).** Every timing
  assertion reads only `htmd:convert`, which ends when synchronous conversion
  work returns (`demo/src/app.rs:332–336`). It excludes scheduling/debounce,
  reactive DOM updates, and preview loading. The suite therefore does not
  enforce the ≤2 kB 100 ms keystroke-to-render bound, the corpus 400 ms
  end-to-end bound, or the 1500 ms initialization bound (§6, §8.3). It also
  has no Long Tasks observer/assertion for keystroke handling, as §8.3
  requires; retaining traces only on failures does not automatically verify
  successful Firefox/WebKit runs. This is missing validation, not evidence
  that the runtime actually exceeds those budgets. **Suggested fix:** retain
  the compute measures, add timing from the input event to the matching
  preview load/paint and from navigation start to conversion-ready, and add
  Chromium Long Tasks assertions with conversion intervals distinguished
  from keystroke handling. Record the Firefox/WebKit trace/manual jank pass.

#### Low-severity / editorial

* **IR4-L1 — Skip-tags typing cancels its own 500 ms save debounce
  (`demo/src/app.rs:681–689,663–672`;
  `demo/src/storage.rs:388–390,410–419`).** The input handler first calls
  `on_skip_tags_edit`, correctly arming the text-save timer, then immediately
  calls `on_options_changed`, whose `set_options` flush cancels that timer
  and writes synchronously. Each keystroke therefore serializes and saves
  the whole workspace, contrary to FR-4:108's explicit 500 ms save rule.
  An isolated native probe of this call chain observed one write before any
  timer fired. **Suggested fix:** trigger engine invalidation/reconversion
  separately for skip-tags text edits, leaving persistence on
  `on_skip_tags_edit`; ordinary checkbox/select interactions should retain
  their immediate flush. Test the app's handler chain, since the existing
  controller-only debounce test does not exercise the extra flush.

**Total new findings: 6 (blocking 0, medium 5, low 1).** IR3-M1 remains
partly outstanding separately. All six new findings await processing.

Reviewer's judgment: the core conversion modules and new browser-test
infrastructure cover much of the specification. The main remaining defects
are in the application wiring: terminal inertness, enforcement of the input
ceiling, output validity, and cross-mode Undo. Passing module tests do not
establish these UI contracts.

Validation: root `cargo build`, `cargo clippy -- --deny warnings`, and
`cargo test` all passed. The demo's `cargo test --locked --manifest-path
demo/Cargo.toml -- --quiet` passed with **173 passed, 0 failed, 2 ignored**
(`RUST_MIN_STACK=16777216`); its locked wasm32 Clippy command with warnings
denied also passed. Four focused specification assertions in an isolated
copy under `demo/target/` failed as described in IR4-M2/M3/M4/L1; the tracked
Rust sources were not edited. The root/container mismatch was checked
against the pinned Leptos `mount_to_body` implementation. Browser tests,
Trunk, the gzip artifact check, and manual accessibility/performance passes
were not run: Node/npm and Trunk are unavailable on this shell's PATH. Only
`demo/review.md` and Nurlybek's implementation-review entries in
`demo/submission.md` were updated.

### Justin Gray

GitHub userid: JustinG1133

#### Contributions

(One-sentence contribution)

#### Specification review rounds

(Table showing final-submission spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

#### Implementation review rounds

(Table showing implementation review rounds)

#### Last implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)
