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

Compiler verification not run; member no longer in the team.


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

* Initiated the repository fork, created the initial `demo-spec.md` and `review.md` files, and added teammates to the repository to enable collaboration.
* Conducted initial specification analysis and performance auditing (Rounds 1–4 and **bjones1** review feedback), establishing strict one-way conversion modes, compressed WASM size and latency budgets, sandboxed `<iframe>` CSP isolation, `localStorage` schema validation, and verified `htmd 0.5.5` / `pulldown-cmark 0.13.4` options and recursion bounds.
* Verified compiler, linter (`cargo clippy`), test suite (`cargo test`), and example binaries (`table` and `simple`), diagnosing and resolving a Windows Smart App Control (`os error 4551`) execution block.
* Resolved all 9 findings from the Round 10 specification review (`R10-M1`–`R10-M4`, `R10-L1`–`R10-L5`) and 8 of 10 findings from the Round 11 specification review (`R11-M1`, `R11-M2`, `R11-M4`–`R11-M6`, `R11-L2`–`R11-L4`) in `demo-spec.md`, updating Sections 4, 8.5, 8.6, and 8.7 accordingly.

#### Specification review rounds

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| **1–4 & bjones1 review** | **94+** | Addressed all feedback from **bjones1** alongside Rounds 1–4: established strict one-way conversion scope, compressed WASM size and latency budgets, sandboxed `<iframe>` CSP isolation, localStorage schema validation, and verified `htmd 0.5.5` / `pulldown-cmark 0.13.4` options and recursion bounds. |
| **10** | **9** (4 medium, 5 low) | Resolved all 9 findings: fixed iframe `load` pipeline stalls (R10-M1), trimmed trailing `<pre>` newlines in diff canonicalization (R10-M2), stripped `<noscript>` and `<template>` in preview sanitization (R10-M3), added crash-loop recovery markers (R10-M4), and addressed all 5 low-severity items (R10-L1–R10-L5). |
| **11** | **10** (6 medium, 4 low) | Resolved 8 of 10 findings (`R11-M1`, `R11-M2`, `R11-M4`, `R11-M5`, `R11-M6`, `R11-L2`, `R11-L3`, `R11-L4`): aligned FR-1 with all 15 `pulldown-cmark` sidebar flags, added `<noscript>` and `<template>` removal to the FR-1 preview walk, pinned `htmd` options in the FR-5 fidelity check, retained the crash marker until user recovery action, clarified the test suspension point in FR-2, specified Source output copying for "Use Output as Input", defined output/diff clearing on `Err`, and fixed editorial typos. (`R11-M3` and `R11-L1` remain open.) |


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

(Table showing implementation review rounds)

#### Last implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

### Danial Baitakov

GitHub userid: exekerey

#### Contributions

(Danial's contributions)

#### Build and run test

(Summarized LLM output)

#### Specification review rounds

(Table showing spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

### Nurlybek Aimagambetov

GitHub userid: nurlybek-aimagambetov

#### Contributions

(Nurlybek's contributions)

#### Build and run test

(Summarized LLM output)

#### Specification review rounds

(Table showing spec review rounds)

#### Last specification review

(LLM and prompt used)

(LLM output, possibly with strikethrough)
