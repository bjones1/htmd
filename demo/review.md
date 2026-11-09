Review: htmd / pulldown-cmark live demo spec
============================================

The spec is well organized: goals and non-goals are clear, requirements are
numbered, and there are milestones and acceptance criteria. The main problems
are a few technical claims that won't hold up in the browser, unresolved
decisions that block implementation, and acceptance criteria that can't be
verified. I've ordered findings by severity.

Blocking / high severity

<strong>5\. Option lists are open-ended ("and any other options the crate
exposes").</strong><br>FR-5 can't be tested if the option set isn't enumerated.
Also, the option names appear to be written from memory. Please verify against
the versions you'll pin:

* Does `htmd` have a `translation_mode` with `Faithful` vs. pure? Confirm the
  exact name and variants.
* `pulldown-cmark` options are bitflags (`Options::ENABLE_TABLES`, etc.), and
  availability of math, heading attributes, and others depends on the crate
  version.

Pin versions in section 6 and list the exact option set for the first release,
which would also close open question 5.

<strong>6\. Several open questions block design and should be resolved before
M1.</strong><br>Repo location, path vs. crates.io dependency, and hosting affect
`Cargo.toml`, CI, and the acceptance criteria (which says `cargo build` "for the
library and the demo," implying a workspace). The two-way sync question (FR-3,
"optional") changes the state model significantly. Decide it now and move it out
of "optional." Recommendation: ship the direction toggle only, with one-way
conversion, and treat two-way sync as a stretch goal.

<strong>7\. Section 6 conflicts with itself on structure.</strong><br>It says
the demo is "not a member of the library's package unless the maintainers prefer
a workspace," yet the acceptance criteria and `htmd` path dependency assume a
shared repo. Also, `demo/demo-spec.md` lists the spec inside the crate, while
the references link to `../translation_approach.md`, which only resolves from
one layout. Pick one.

<strong>8\. Acceptance criteria are not all verifiable.</strong>

* "Works in Chrome, Firefox, and Safari" has no test procedure. Specify versions
  and a manual checklist, or add `wasm-bindgen-test` in headless browsers.
* "No visible lag while typing" and "first conversion under 100 ms" need a
  defined benchmark document and hardware.
* "WCAG 2.1 AA where practical" is too soft to accept against. List the concrete
  checks (keyboard operation, contrast, labels, focus order, live region for
  errors).
* Criteria are all checkboxes about happy paths. Add negative cases: malformed
  HTML, empty input, huge input, invalid option combinations.

<strong>9\. Deliverable 5 is a conflict of interest.</strong><br>`review.md`
"with the team's review findings and resolutions" is a deliverable of the
project being reviewed. Fine as a process artifact, but the spec should say when
review happens (which milestone) and who resolves findings.

Lower severity / gaps
---------------------

* <strong>Error handling (FR-8):</strong> "failing input highlighted where
  practical." `html5ever` is very forgiving and `htmd` rarely errors on
  malformed HTML, so most "errors" will be silent weirdness, not failures. Say
  what the UI shows in that case, and note that panics in WASM abort the module
  and require `console_error_panic_hook` plus a recovery path (a panic leaves
  the WASM instance unusable until reload).
* <strong>Debounce (FR-3):</strong> a fixed 150–300 ms debounce means every
  keystroke on a small document feels delayed. Consider converting immediately
  for small inputs and debouncing only above a size threshold.
* <strong>Size budget (section 8):</strong> "document the release `.wasm` size"
  sets no target. Give a budget (e.g., under N MB gzipped) and the build profile
  (`opt-level = "z"`, `lto`, `codegen-units = 1`, `wasm-opt`) so size
  regressions are catchable.
* <strong>Samples (FR-7):</strong> "inline HTML, math" samples need
  `pulldown-cmark` math enabled, and `htmd` doesn't convert math specially.
  Define expected output so the samples double as regression tests.
* <strong>Persistence non-goal vs. UX:</strong> no persistence means a stray
  reload loses input. Consider URL-hash or `sessionStorage` state; the non-goal
  says no sharing, but a reload guard is cheap. Decide explicitly.
* <strong>Options reset (FR-5):</strong> "reset to defaults" should specify
  whether it resets only the current direction's options or both, since options
  are "grouped by direction."
* <strong>Licensing:</strong> state the demo's license and confirm compatibility
  with `htmd`, `pulldown-cmark`, and Leptos (all permissive, but worth one
  line).
* <strong>Deployment:</strong> the deliverable says "workflow or documented
  manual deployment." Pick one; a GitHub Actions workflow is testable, a doc is
  not. Also, Trunk's `--public-url` matters for GitHub Pages subpaths, and
  should be called out.
* <strong>Section 5 UI sketch:</strong> the layout shows the round-trip panel
  always visible, but FR-6 says it's only for HTML → Markdown mode. Clarify when
  it appears.
* <strong>Nit:</strong> the spec says "(for example, a 1 MB page)" for large
  inputs but gives no performance target for it; add one or state it as best
  effort.

What's good
-----------

* Clear separation of goals and non-goals, which keeps scope honest.
* Dependency direction (demo → library, never reverse) is stated explicitly.
* Milestones are incremental and each produces something runnable.
* Static-only, no-backend constraint is consistent throughout.
* Feedback-loop guard for two-way sync is anticipated.

Suggested changes before implementation starts
----------------------------------------------

1. Add **M0**: WASM feasibility spike for `htmd` (and `pulldown-cmark`).
2. Decide Web Worker vs. main-thread conversion and revise FR-8 and section 6
   accordingly.
3. Require a sandboxed iframe + CSP for rendered output.
4. Redefine the round-trip diff (normalization, options, algorithm).
5. Pin crate versions and enumerate the exact options for release 1.
6. Resolve open questions 1, 2, 4, and 5; defer 3 (hosting) to M6.
7. Convert soft acceptance criteria into measurable ones.

If useful, I can draft a revised version of sections 5 and 8 with these changes
applied, or write the `review.md` deliverable in the format the spec calls for.
