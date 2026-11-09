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

Prompting Claude Opus 5.5 High effort with "review @demo-spec.md"
Output: 
I reviewed the spec as a fresh round 10. I checked it against src/lib.rs, src/dom_walker.rs, src/element_handler/pre.rs, tests/common/mod.rs and the round-9 notes in review.md. Cargo isn't installed here, so I couldn't open the html5ever/markup5ever_rcdom sources, and I've marked the claims that depend on them. I haven't edited any files.

What still holds: the 12 htmd options and their defaults match src/options.rs, and the 15-flag count for pulldown-cmark is right. html_to_tree → tree_to_markdown is exactly what convert does (src/lib.rs:148–214), so the parse-once nesting check works as written.

Medium
M1. A conversion that never reloads the iframe stalls the pipeline for good (FR-2, line 55). A run counts as "in flight" until the iframe fires load for its commit. Several kinds of run never set srcdoc, so that load never comes:

a stale run, which "writes nothing";
a run whose output is cleared (empty input, the 2 MB limit, an Err banner, a size or nesting limit);
a run still waiting for load when the mode is switched.
With one run allowed at a time, the queued run then never starts, and conversion stops for the rest of the session. The 8.6 concurrency test, with its 300 ms delay, hits the stale case directly.

Fix: a run that writes nothing, or doesn't navigate the iframe, ends at commit. Say what "clear the preview" does to srcdoc. Ignore a load that doesn't match the current commit.

M2. Every ordinary code block shows a false diff (FR-5, line 112). CommonMark code blocks always end with a line ending, so pulldown-cmark renders <pre><code>x\n</code></pre>. The usual input <pre><code>x</code></pre> is converted to ```\nx\n``` (see tests/code_tests.rs), which comes back with the extra \n. Because <pre> is a preserved subtree, nothing trims it, so the diff flags it. The real-world page in the corpus will show this on every code block. The basics sample passes 8.7's zero-diff test only if its HTML happens to include the \n.

Fix: on both sides, remove one trailing U+000A from the last text inside each <pre>. A single trailing newline doesn't render, so this is safe. Add a golden test for <pre><code>x</code></pre>.

M3. <noscript> and <template> get past preview stripping (FR-1, lines 45–46; 8.6 "Preview navigation" and "Preview stripping"). The sandboxed preview has scripting disabled, so the browser reads what's inside <noscript> as markup. html5ever parses with scripting enabled by default, so the walk sees that content as plain text and strips nothing in it. If the serializer then writes it out raw, as I believe it does by default, the link reaches the preview with its target intact. This last step is unverified because I couldn't open the crate sources. For example:


<noscript><a href="https://x.example" target="_self">click</a></noscript>
With target="_self" the click loads x.example into the preview, which breaks the 8.6 navigation test. <link rel=preconnect> inside <noscript> gets through the same way, and CSP doesn't cover preconnect.

<template> has a similar gap. RcDom stores template contents apart from the element's children, so a walk over children never sees them. Browsers turn <template shadowrootmode="open"> into a live shadow root, so a target attribute inside it survives.

Fix: remove <noscript> and <template> elements whole in the preview walk (FR-5 already drops <template>), or parse and serialize with scripting off. Add both cases to the 8.6 stripping test.

M4. An input that causes a panic can lock the user in a crash loop (FR-7, lines 133–134; FR-8, lines 159–160). Three places can save the crashing input before or after the panic:

the flush on UI actions, which runs before "Use Output as Input" or a sample load converts;
the 500 ms text-save timer;
the pagehide flush when the user clicks reload, as the overlay tells them to.
On the next load, startup converts the saved buffer once, it panics again, and the only way out is clearing site data by hand.

Fix: have the panic hook and the error/unhandledrejection listeners stop all further storage writes and set a crash-marker key. If the marker is present at startup, restore the workspace but skip the first conversion, and show a banner offering to convert or clear.

Low
L1. Typing in skip_tags counts as an option change (FR-4, FR-8). On a 100 KB input, every keystroke runs a full conversion and a synchronous save of the whole workspace. Debounce this field the same way as text input. FR-8 also doesn't say whether a stored skip_tags longer than 200 characters is invalid.
L2. No one has checked that 512 levels fit on the wasm stack (FR-7, line 131). The stack is 1 MiB by default. Each level runs about six to eight frames (walk_node → walk_element → handler → walk_children), so 512 levels could come close. Run the "512 levels converts" test with the element that uses the most stack (nested lists, tables or blockquotes), in both translation modes and in the release build. Or set the stack size explicitly.
L3. The fidelity panel and the converter parse the input differently (FR-5, line 108). The panel parses the original as a fragment, but htmd parses it as a whole document (parse_document, src/lib.rs:148). For full-page input like the corpus page, the doctype, <html>/<body> attributes and <head> contents are handled differently on the two sides. Either canonicalize from the same tree htmd converted, or state this as a limitation.
L4. FR-2 says each mode has its own fidelity panel (line 58), but FR-5 limits the panel to HTML → Markdown mode. This is an editorial fix.
L5. Undo after a sample load restores the buffer but not the selected sample (FR-6). A later Reload Sample then loads the sample the user just undid. Say whether that's intended.
Still open from round 9
R9-L3: the strip = true trade-off, and how to trigger a deliberate panic in a release build.
R9-L7: no time limit for "neither hangs".
R9-L8: mixed byte-unit conventions.
R9-L9: the Pure preset hides the fidelity check by default.
Round 8's browser-floor and editorial findings.
Total: 9 new findings (0 blocking, 4 medium, 5 low). 


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
| **5** | **12**                | Made performance and size budgets measurable (artifact, command, profile), defined the demo build contract and CI workflow, normalized `skip_tags`, hardened the iframe/panic-hook/debounce contracts, and renumbered sections; then reconciled long-task vs compute budgets, unified storage version handling, added the persisted-state schema, moved the nesting guard to a streaming pre-scan, and pinned the 2 KB threshold.            |
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
   100 KB limit using UTF-8 byte arrays.
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
  FR-8):</strong> on a 100 KB input every keystroke runs a full conversion and a
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

### Contributions

* Muzanbekova Kamila: (Andy's contributions)
* Danial Baitakov: (Bob's contributions)
* Justin Gray: (Victoria's contributions)
* Nurlybek Aimagambetov: (Zara's contributions)

### Spec review rounds

(Table showing spec review rounds)

### Implementation review rounds

(Table showing implementation review rounds)

### Last LLM spec review

(LLM output, possibly with strikethrough)

### Last LLM implementation review

(LLM and prompt used)

(LLM output, possibly with strikethrough)
