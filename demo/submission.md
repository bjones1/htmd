Project 2 submission
====================

Team members are Muzanbekova Kamila, Danial Baitakov, Justin Gray, and Nurlybek Aimagambetov. The specification consists of
the file [demo-spec.md](demo-spec.md).

Draft submission
----------------

### Contributions

* Muzanbekova Kamila: Initiated the repository fork, created the critical demo-spec.md and review.md files, added teammates to the repository to enable collaboration, and began the initial code review process for the project specification.
Completed the specification analysis and performance audit up to and including point B6, made the corresponding edits to the `review.md` file, and moved the irrelevant points into the specification. Round 1-4 review.
* Danial Baitakov: Finished the review work from B7 through L10, applying each finding to the spec and marking it resolved in review.md. Added the artifacts the professor asked for (contribution list, review history, compile verification output) and ran the round-5 review cycle until nothing was left open. Also ran the round-6 review and added five more findings from a spec-quality checklist, then left round 6 open so a teammate could take it.
* Justin Gray: (Victoria's contributions)
* Nurlybek Aimagambetov: Clarified typing protection and added acceptance coverage case, initial workspace state, corrected the nesting-limit explanation

### Compiler test

The following statements summarize LLM output on each team member's computer,
based on the prompt "Verify that you can compile and run the project in the
current git repo."

#### Muzanbekova Kamila

### LLM Verification Output
 
**Prompt applied:** 
*"Verify that you can compile and run the project in the current git repo."*
 
**LLM Response:**
Verified. The repo builds, lints, tests, and runs cleanly.
 
| Check | Command | Result |
| :--- | :--- | :--- |
| **Build** | `cargo build` |  Finished dev profile |
| **Lint** | `cargo clippy -- -deny warnings` |  No warnings |
| **Tests** | `cargo test` |  266 passed, 1 ignored (the table-generator test), 0 failed |
| **Run (examples)** | `cargo run --example table` |  Printed converted Markdown |
| **Run (examples)** | `cargo run --example simple` |  Ran after rebuild |
 
*Environment Note:* 
The first run of `target/debug/examples/simple.exe` was temporarily blocked by Windows Smart App Control ("Application Control Policy blocked this file", os error 4551) due to an unsigned binary reputation check. A fresh `rustc hello-world` and the `table` example executed correctly. Rebuilding `simple` under a different debug profile resolved the signature prompt, allowing the execution to bypass the environment restriction and successfully pass all internal code assertions. No repository source files were modified during this execution check (`git status` remained clean)

#### Danial Baitakov

**Prompt applied:**
*"Verify that you can compile and run the project in the current git repo."* — run by GLM-5.3 (OpenCode) on Linux, repository at commit `bb9161d`.

**LLM Response:**
Verified. The repo builds, lints, tests, and runs cleanly.

| Check | Command | Result |
| :--- | :--- | :--- |
| **Build** | `cargo build` | Finished dev profile |
| **Lint** | `cargo clippy -- --deny warnings` | No warnings |
| **Tests** | `cargo test` | 267 passed, 1 ignored (the table-generator test), 0 failed |
| **Run (examples)** | `cargo run --example table` | Printed the converted Markdown table |
| **Run (examples)** | `cargo run --example simple` | Self-checking example (assertions only, no printed output); exited 0 with all assertions passing |

*Environment note:* Linux; no environment restrictions encountered; no repository source files were modified during this check.

#### Nurlybek Aimagambetov

**Prompt applied:** 
*"Verify that you can compile and run the project in the current git repo."*

**LLM Response:**
Verified: the library compiles and runs through its test suite.
- cargo build — passed
- cargo clippy -- --deny warnings — passed
- cargo test — passed, including doctests; one test intentionally ignored
The default target directory produced filesystem errors. All checks passed using CARGO_TARGET_DIR=$env:TEMP\htmd-verification-target. No source files changed.

#### Justin Gray

(Summarized LLM output)

### Spec review rounds
 
The following table summarizes the design and review iterations for the `htmd / pulldown-cmark live demo` specification. It tracks the document's evolution from initial conceptual sketches to a production-ready specification optimized for WebAssembly runtime stability and browser security.
 
| Round | Findings | Main changes made |
| :---: | :---: | :--- |
| **1** | **18** | Narrowed the scope to strict one-way conversion modes, established a compressed WASM asset budget, and defined initial verifiable acceptance criteria. |
| **2** | **26** | Resolved diff pipeline contradictions, added the missing Section 5 UI sketch, formalized the localStorage validation schema, and eliminated runtime panics on empty tag inputs. |
| **3** | **12** | Enforced total preview isolation using a sandboxed `<iframe>` with a strict Content Security Policy to eliminate XSS risks and hidden third-party network requests. |
| **4** | **38** | Verified all settings against `htmd 0.5.5` and `pulldown-cmark 0.13.4`, expanded the option mapping to 15 flags, resolved reset semantics, and added deep DOM nesting bounds. |
| **5** | **12** | Made performance and size budgets measurable (artifact, command, profile), defined the demo build contract and CI workflow, normalized `skip_tags`, hardened the iframe/panic-hook/debounce contracts, and renumbered sections; then reconciled long-task vs compute budgets, unified storage version handling, added the persisted-state schema, moved the nesting guard to a streaming pre-scan, and pinned the 2 KB threshold. |
| **6** | **11** | Open, awaiting processing: six LLM findings (missing axe-core CI step, underspecified nesting pre-scan, unadjudicable forced-tick latency rule, `&nbsp;` exemption inconsistency, missing acceptance criteria for "Use Output as Input"/"Reload Sample", preset naming variance) plus five spec-quality checklist findings (word tokenizer definition, code-point vs locale sorting, NFC normalization, first-run state, banner byte units). |
| **7** | **8** | Addressed some findings from the previous round. A follow-up review identified 8 new non-critical issues, some of which I fixed while dismissing the rest as not applicable |

---
 
#### Round Context & Analysis
 
1. **Round 1 (Conceptual Scope):** Replaced unstable two-way sync ideas with a deterministic "Use Output as Input" feature, split per-mode state variables, and introduced performance budgets evaluated via the browser's Performance API.
2. **Round 2 (Pipeline Logic):** Designed a word-level text diff built strictly on canonicalized HTML fragments using `html5ever`, dropped unreachable library error wrappers, and fixed concurrency timers to ensure typing does not wipe pending changes.
3. **Round 3 (Browser Security & Sandboxing):** Blocked raw HTML passthrough vulnerabilities, isolated event handling vectors (`onclick`, `onerror`), explicitly mapped the `pulldown-cmark` bitflags, and unified the 100 KB limit using UTF-8 byte arrays.
4. **Round 4 (Crate Alignment & Edge Cases):** Expanded the UI flags to match all 15 bitflags available in `pulldown-cmark 0.13.4`, protected the recursive WASM stack walker against deeply nested elements, and separated "Reset Options" from "Reset Sample" logic.
5. **Round 5 (Measurability & Buildout):** Tied every budget to an artifact and CI command (`gzip -6` size check, `performance.measure` latency), defined the standalone-crate build contract with a `path` dependency on `htmd`, added the persisted-state schema and a streaming nesting guard, and reconciled the 100 ms compute vs 350–400 ms end-to-end latency criteria.
6. **Round 6 (Open):** A fresh-context review produced six findings (axe-core missing from the CI step list, underspecified nesting pre-scan, unadjudicable forced-tick latency rule, `&nbsp;` exemption inconsistency, missing acceptance bullets for "Use Output as Input"/"Reload Sample", preset naming variance); a specification-quality checklist pass added five more (word tokenization for the diff, code-point vs locale-aware sorting, NFC normalization, first-run state, banner byte units). All 11 remain open for processing.
7. **Round 7 (Various issues):** Fixed some of the issues in previous round review (Preset naming variance, omitted axe-core from CI, first run state), improved typing protection wording and added coverage.


### Last LLM review

Prompting **GLM-5.3** (OpenCode `general` subagent, fresh context, standard
effort), with *"You are an LLM performing a fresh, independent design review
of a specification document. This is round 6 of an iterative review
process… Check internal consistency, verifiability, technical accuracy against
the repo source, completeness gaps, and editorial problems; do not re-raise
findings already recorded as resolved"* produced the following output. No
finding was deemed incorrect or irrelevant, so nothing is struck through; all
11 findings remain open for processing by the next team member. Five of the
eleven (R6-M4, R6-M5, R6-L4, R6-L5, R6-L6) were added separately by Danial
Baitakov from a specification-quality checklist and are marked as such below.

#### What checks out

- FR-4's `htmd` option list matches `src/options.rs` exactly: all 12 names, every variant, and every default (Atx, Asterisks, TwoSpaces, Inlined, Full, Fenced, Backticks, Asterisk, spacing 3/2, `preformatted_code: false`, `translation_mode: Pure`); the 1–4 spacing ranges match the clamp bounds documented in `src/options.rs`.
- FR-4's `br_style`-ignored-in-Faithful note is accurate (`src/element_handler/br.rs:12-17` emits literal `<br>` in Faithful mode), and disabling the control is a sound consequence.
- The `skip_tags` panic claim is correct: `skip_tags` forwards to `add_handler`, which asserts `!tags.is_empty()` (`src/lib.rs:248-250`, `src/element_handler/mod.rs:396-400`), and it takes `Vec<&str>` keyed by exact string, so FR-4's lowercasing rationale and empty-result rule are grounded.
- FR-7's "future-proof `Result` wrapper" framing is accurate: `convert` returns `io::Result<String>` (`src/lib.rs:212`) with no reachable `Err` (`html_to_tree` always `Ok` at `src/lib.rs:148-166`; `tree_to_markdown` returns `String`).
- `htmd` 0.5.5 matches `Cargo.toml:3`; root `Cargo.toml` has `[profile.release] debug = true` and no `[workspace]`, so §6's "root profile does not apply" and Deliverable 1's standalone-crate/`path` claims hold.
- 8.1's gate commands match `AGENTS.md` exactly.
- The math-sample round-trip claim is grounded in `src/element_handler/span.rs:17-23` (`math math-inline`/`math math-display` → `$`/`$$`).
- Numbers are internally consistent across the document: 1.5 MB = 1,572,864 bytes with strictly-under/fail-at-≥ in §6, Deliverable 3, and 8.1; 100 KB = 100,000 UTF-8 bytes with strict `>` and exact-boundary tests in FR-2/FR-7/8.6; 100 ms compute vs 100 ms e2e (≤2,000 bytes) vs 400 ms e2e (2,001–100,000 bytes) in FR-2/§6/8.3/8.5; 1500 ms cold start in §6/8.3; 512 nesting levels in FR-7/8.6; 500 ms storage debounce in FR-8/8.6.
- The FR-8 persisted schema's `cmark` object has exactly the 7 FR-4 flags and its `htmd` object has exactly FR-4's 12 options; unknown-field and version-wipe rules are stated once and used consistently.
- Section numbering (1–8) and all cross-references ("section 6" from §5/8.2/8.3, "section 8.3" from 8.5) resolve correctly.

#### Blocking issues

None.

#### Medium-severity issues

- **R6-M1 — CI workflow omits the axe-core step it is required to run (Deliverable 3 vs §6 Accessibility, 8.4):** the workflow step list does not include axe-core while §6 and 8.4 require it in CI. *Fix: add an explicit axe-core step (e.g., an axe scan driven by the Playwright suite).*
- **R6-M2 — Nesting pre-scan algorithm is underspecified and can miscount both ways (FR-7):** "counting unmatched open tags" does not define void elements, self-closing syntax, closing-tag decrements, `<` inside attribute values, or `<` inside raw-text/comment content; an under-count admits exactly the stack-trap input the guard rejects. *Fix: specify tokenizer-aware scan rules, or reuse an incremental html5ever tokenizer driven iteratively.*
- **R6-M3 — "No visible input lag" criterion is unadjudicable for the forced tick (8.3 vs FR-2):** the 900 ms forced tick fires during typing yet is a scheduled conversion, so a tester cannot decide whether a ~100 ms block mid-typing violates the criterion. *Fix: state whether forced-tick conversions count as keystroke handling.*
- **R6-M4 — Word tokenization for the diff is undefined (FR-5, 8.7) [checklist]:** `similar::TextDiff::from_words` splits by Unicode word boundaries only with the crate's unicode feature; the spec pins "similar 2.x" but not the feature set, and never defines "word". CJK text, combining marks, and ZWJ sequences can tokenize differently between the two sides (e.g., Kazakh ма́ма with U+0301), producing false diffs. *Fix: pin UAX #29 word boundaries, compare tokens by code point, add combining-mark and CJK golden cases.*
- **R6-M5 — "Alphabetically"/"lexicographically" sorting is collation-undefined (FR-5) [checklist]:** attribute and class-token sorting never says code point vs locale collation; code point order is not alphabetical order in non-Latin scripts (Kazakh ә), and canonical output must be identical across browser locales. *Fix: state all canonicalization sorting is by Unicode code point, never locale collation.*

#### Low-severity / editorial

- **R6-L1 — `&nbsp;` normalization exemption is inconsistent with the whitespace exemption list (FR-5):** nbsp normalization exempts only "raw-text elements", an undefined term; as written `&nbsp;` inside `<pre>`/`<code>` is normalized, hiding a real difference. *Fix: apply the same preserved-element list.*
- **R6-L2 — "Use Output as Input" and "Reload Sample" lack acceptance criteria (FR-2, FR-6 vs section 8).** *Fix: add 8.5/8.6 bullets for the copy-with-Undo round trip and Reload Sample scope.*
- **R6-L3 — Preset naming variance (FR-4/§5 vs FR-8/8.6):** "Demo Preset" vs "Demo Default Preset" read as two presets. *Fix: pick one name.*
- **R6-L4 — Unicode normalization of text nodes is unspecified (FR-5) [checklist]:** NFC/NFD equivalents produce false diffs. *Fix: apply NFC (not NFKC, which folds ligatures like ﬁ) outside preserved elements; add a golden case.*
- **R6-L5 — First-run state is undefined (FR-8 vs FR-2/FR-4) [checklist]:** initial mode, tab, and sample preloading are never stated. *Fix: state first-run defaults explicitly.*
- **R6-L6 — Banner text says "100 KB" but the limit is 100,000 bytes (FR-7) [checklist]:** a 100,400-byte input is rejected under a banner reading "100 KB". *Fix: change banner text to "100,000 bytes" or declare the KB wording deliberate.*

**Total findings: 11 (blocking 0, medium 5, low 6) — 6 from the subagent review, 5 from the specification-quality checklist pass.**

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
