Spec review: htmd and pulldown-cmark playground
===============================================

This is review round 9 of [htmddemo2-spec.md](htmddemo2-spec.md), dated
2026-11-14. It replaces the round 8 review of the same date.

Model: Claude Fable 5.2. Prompt:

> Review the spec at @pulldown-cmark-htmd-demo-2/htmddemo2-spec.md . Evaluate
> against @pulldown-cmark-htmd-demo-2/acceptance-criteria.md and submission
> template.

Findings refer to spec sections, not line numbers, since the fixes below moved
the lines.

Summary
-------

The spec is close to the point where a review gives only trivial findings:
one contradiction, three small gaps, and four trivial points. Nothing is
overspecified; the only implementation choices are Leptos 0.8 and GitHub
Pages, both under Constraints.

Converters run
--------------

Every row under Expected results, and the Round trip, was run against htmd
0.5.5 and pulldown-cmark 0.13.4 in a scratch crate. All rows give the stated
output. The htmd defaults in the options table match `Options::default()`, the
`<noscript>` default matches the builder's default, and the 15 extensions match
pulldown-cmark's option flags, including old-style footnotes implying
footnotes.

Findings
--------

1. **Crash and stale output contradict each other** (Output pane; Limits and
   failures). The Output pane section said every tab keeps its last content
   while the output is stale. Stale output includes "after a crash", but a
   crash replaces the output pane with a message. *Fixed:* the rule now
   applies only while conversion is paused.

2. **The skip tags `SCRIPT` row is not htmd's own output** (Expected results).
   The section says each row was produced with htmd 0.5.5, but htmd matches
   tag names by case: given `SCRIPT`, it returns `a⏎⏎alert(1)`. The row holds
   only because the playground's Skip tags rule ignores case. *Fixed:* the
   section's introduction now says so.

3. **The double-switch criterion fails on a trailing newline** (Acceptance
   criteria). pulldown-cmark ends its output with a newline, which is not
   "whitespace between tags". *Fixed:* the criterion also ignores a trailing
   newline.

4. **Two address rules were missing** (Permalinks). The spec did not say what
   Back or Forward to an address with no saved state shows, or whether a
   loaded address or a pending typing update wins. *Fixed:* such an address
   loads the default example, and the loaded state replaces keystrokes still
   inside the typing pause.

Trivial points, left as they are
--------------------------------

5. "Valid tag name" under Skip tags is not defined (for example, whether
   `my-element` counts).
6. The 10 ms speed target names the browser but no hardware.
7. The spec does not say whether the address still updates when the user
   edits the input after a crash.
8. The Pages address belongs to the team's fork, while the pull request
   targets the instructor's fork, where a push to `main` would not publish to
   that address.

Submission file
---------------

The review also compared [submission.md](submission.md) with the assignment
and the template. These points were fixed in the same round:

* The team name and GitHub userids were missing.
* Rounds 5–8 came after the instructor's review of the draft (round 4), so
  they belong to the final submission, not the draft.
* The final submission's review table and last review were empty.
* The round 8 row said the acceptance criteria were cut "from 7 to 4" (the
  spec has 2) and that the address format was specified (the spec only says
  when an address is invalid).
* Ademi's compiler test was a table pasted as loose lines.

These points remain for the team:

* The draft's "Last LLM review" shows the round 1 review, not the last draft
  review (round 3).
* Final contributions for Ademi and Alisher are not filled in, and neither has
  a commit after the draft; the assignment requires one from each member.
* The implementation and its review are excluded, as agreed with the
  instructor.
* The template groups sections by team member; the file groups them by topic.
