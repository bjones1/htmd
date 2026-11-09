Project 2 submission
====================

Team members are Muzanbekova Kamila, Danial Baitakov, Justin Gray, and Nurlybek Aimagambetov. The specification consists of
the file [demo-spec.md](demo-spec.md).

Draft submission
----------------

### Contributions

* Muzanbekova Kamila: Initiated the repository fork, created the critical demo-spec.md and review.md files, added teammates to the repository to enable collaboration, and began the initial code review process for the project specification.
Completed the specification analysis and performance audit up to and including point B6, made the corresponding edits to the `review.md` file, and moved the irrelevant points into the specification. Round 1-4 review.
* Danial Baitakov: (Bob's contributions)
* Justin Gray: (Victoria's contributions)
* Nurlybek Aimagambetov: (Zara's contributions)

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

(Summarized LLM output)

#### Nurlybek Aimagambetov

(Summarized LLM output)

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

---
 
#### Round Context & Analysis
 
1. **Round 1 (Conceptual Scope):** Replaced unstable two-way sync ideas with a deterministic "Use Output as Input" feature, split per-mode state variables, and introduced performance budgets evaluated via the browser's Performance API.
2. **Round 2 (Pipeline Logic):** Designed a word-level text diff built strictly on canonicalized HTML fragments using `html5ever`, dropped unreachable library error wrappers, and fixed concurrency timers to ensure typing does not wipe pending changes.
3. **Round 3 (Browser Security & Sandboxing):** Blocked raw HTML passthrough vulnerabilities, isolated event handling vectors (`onclick`, `onerror`), explicitly mapped the `pulldown-cmark` bitflags, and unified the 100 KB limit using UTF-8 byte arrays.
4. **Round 4 (Crate Alignment & Edge Cases):** Expanded the UI flags to match all 15 bitflags available in `pulldown-cmark 0.13.4`, protected the recursive WASM stack walker against deeply nested elements, and separated "Reset Options" from "Reset Sample" logic.

### Last LLM review

(LLM and prompt used)

(LLM output, possibly with strikethrough)

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
