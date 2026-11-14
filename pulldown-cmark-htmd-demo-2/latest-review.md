Spec review: htmd and pulldown-cmark playground
===============================================

This is a review of [htmddemo2-spec.md](htmddemo2-spec.md), dated 2026-11-14.
It lists 30 findings: inconsistencies, incomplete scenarios, and vague
objectives. It does not cover technical implementation.

The findings come from reading the spec against itself. The converters were not
run, so the rows under Expected results are unchecked.

The most consequential findings are the large-input permalink contradiction
(1), the stale-output scenarios (13), and the undefined "round-trips" criterion
(7).

Line numbers refer to the spec as it stood on the review date.

Inconsistencies
---------------

1. **Large input contradicts the permalink guarantee** (line 257 vs lines 238
   and 290). Input over 1 MiB is not stored in the address, yet the address
   "restores them exactly" and a Share link "reproduces the page". The spec
   does not say what the address holds in that state (the old input, or none)
   or what Share does.

2. **"Undo is not required" vs Back and Forward** (line 42 vs line 249). Back
   and Forward must load the state in the address, but the spec never says
   which actions create a history entry. If none do, Back has nothing to go to;
   if a direction switch does, Back is an undo.

3. **"First visit" has no meaning** (line 154 vs line 299). Nothing is saved
   outside the address, so the page cannot tell a first visit from a later one.
   It presumably means "an address with no state".

4. **The Markdown Basic preset is never a default.** The page opens in HTML →
   Markdown, and switching direction carries the converted output over, so the
   Markdown Basic preset appears only if picked from the menu.

5. **Hidden vs disabled** (lines 100–107). Code fence and Reference style are
   hidden when they do not apply, while Line break is disabled. The column is
   headed "Shown" but holds both behaviours, with no reason given for the
   difference.

6. **Confirmation is applied unevenly** (line 50). Loading an example over
   edited input asks first. Clear, the direction switch, Back and Forward, and
   Reset to defaults discard the same work without asking.

7. **"Round-trips" is used in two senses** (line 233 vs line 289). Line 233
   means the text comes back identical. Line 289 cannot mean that for the HTML
   Basic example unless the converters happen to be exact inverses, so the
   criterion has no pass condition.

8. **Only one version is shown** (line 28 vs line 240). The header shows only
   the htmd version, but the Permalinks section says output can change with
   either library.

9. **Defaults have two sources** (line 93). "Defaults are htmd's own defaults"
   sits beside a table of fixed values. The spec does not say which wins if
   htmd changes a default, and that also changes what "differs from its
   default" means for old permalinks.

10. **Expected results mix exact and descriptive rows** (lines 225 and 228).
    Most rows give exact text, but these two give descriptions. No row says
    whether a trailing newline counts, which also affects the character count
    in the status line.

11. **Share is missing from the page layout** (lines 26–36). The layout list
    does not place the Share button. Copy has a "browser does not allow
    copying" message, but Share, which also copies, does not.

12. **"A group of controls for each converter"** (line 85) is true in only one
    direction. It is also unclear whether Reset to defaults resets the htmd
    group while it is hidden.

Incomplete scenarios
--------------------

13. **Stale output.** Output can lag the input during the typing pause, while a
    large input is paused, and after a failure. The spec does not say what the
    direction switch, Copy, and the Rendered tab do then, or whether the output
    is marked as out of date.

14. **Tab selection on a direction switch.** If Original is selected and the
    user switches to Markdown → HTML, where that tab does not exist, the
    selected tab is undefined. The spec also does not say whether the selected
    tab or the panel's collapsed state are part of the permalink.

15. **Footnotes coupling is one-way** (line 148). Unchecking Footnotes while
    Old-style is checked is undefined. So is unchecking Old-style, and so is a
    hand-edited address with old-style on and footnotes off.

16. **Number fields.** Empty, 0, 9, 2.5, or a half-typed value in a 1–4 field
    has no defined result (clamp, reject, or keep the last valid value). The
    same applies to out-of-range values in an address.

17. **Partly valid addresses** (line 246). The spec covers only "cannot be
    decoded". Undefined cases are an unknown option, one bad value among good
    ones, and a format version newer than the page. It also omits the direction
    switch from the actions that replace an invalid address.

18. **"Convert anyway" scope** (line 257). The spec does not say whether it
    applies once, until the input drops below 1 MiB, or for the session. It
    also does not say whether the limit is checked when a direction switch or
    an example produces the large input, or whether 1 MiB means bytes or
    characters (the status line counts characters).

19. **Fails vs crashes** (line 260) get the same treatment. A recoverable error
    should arguably let the user keep editing. Other gaps:

    * a conversion that hangs instead of crashing;
    * a failure in the Rendered pass only;
    * no way to recover the text, since Start over discards it and Reload loses
      any input too large for the address.

20. **Links in the Rendered and Original tabs** (line 269). "Navigates the
    page" does not say what a click does: nothing, or open a new tab. Forms,
    iframes, and input `<style>` that could restyle the playground are also
    uncovered.

21. **Relative image addresses** (line 204). Only remote images are said to
    show as broken. `i.png`, used in the Links and images preset and in the
    Expected results, would resolve against the site itself; `data:` images are
    not mentioned.

22. **"Edited"** (line 50) is undefined after a permalink load, a direction
    switch, Clear, or an edit that is then reverted.

23. **When conversion runs** (line 79). The spec covers typing and option
    changes only. Page load, example load, Clear, paste, and the direction
    switch are not covered. Skip tags is a typed option, so "immediately" means
    on every keystroke.

24. **Markdown → HTML presets and their extensions** (line 172). The spec does
    not say whether picking the Strikethrough preset turns strikethrough on. If
    not, the preset shows nothing by default; if so, examples change options,
    which the spec does not mention.

25. **Options panel.** Behaviour on resize or rotation across 800 px is
    undefined. A permalink with non-default options opened on a phone has the
    panel collapsed, with no sign that hidden settings are active.

26. **Skip tags.** "Valid tag name" is undefined (hyphenated custom elements,
    for example). The spec does not say whether ignored entries stay in the
    address, or how skipping `noscript` interacts with the noscript checkbox.

27. **Empty input, and feedback after a successful Copy or Share,** are
    unspecified. So is how several notices share the status line and when each
    one clears.

Vague objectives
----------------

28. **Unmeasurable targets:**

    * "under 10 ms in desktop Chrome" gives no hardware, and does not say
      whether the Rendered pass is included;
    * "a few kilobytes" for the Article;
    * "under 1 MB compressed" does not say which compression or whether it
      covers all assets;
    * "current" browsers;
    * "about a tenth of a second".

29. **"Status changes are announced"** (line 275). The spec does not say which
    changes; announcing time and length after every conversion would flood a
    screen reader. The Esc-then-Tab exit is also not required to be
    discoverable.

30. **Acceptance criteria are thin.**

    * "Every option changes the output of at least one example" does not say
      which output: pulldown-cmark options in HTML → Markdown only affect the
      Rendered tab.
    * No listed preset clearly exercises Faithful mode.
    * Nothing tests invalid links, large input, crashes, Back and Forward,
      accessibility, themes, size, or the phone layout.
    * Most pulldown-cmark extensions, the noscript option, and "Spaces after
      list number" have no Expected results row.
    * "Links to the source repository" does not say whether that is htmd
      upstream or this fork.
