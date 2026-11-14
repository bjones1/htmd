Spec review: htmd and pulldown-cmark playground
===============================================

This is a review of [htmddemo2-spec.md](htmddemo2-spec.md), dated 2026-11-14.
It lists 7 findings and 5 smaller points, and it replaces the earlier review of
the same date.

Unlike the earlier review, this one ran the converters. Every row under
Expected results was run through htmd 0.5.5 and pulldown-cmark 0.13.4, built
from this repository.

The most consequential finding is that sharing input of up to 1 MiB is probably
not achievable (1).

Line numbers refer to the spec as it stood on the review date.

What was verified
-----------------

* **Expected results.** All 27 HTML → Markdown rows, all 9 Markdown → HTML
  rows, and the Round trip row give the stated output.
* **Basic round trip.** A paragraph with bold and italic text survives two
  direction switches, apart from a trailing newline.
* **Defaults table.** It matches `Options::default()` in `src/options.rs`, and
  `<noscript>` "on" matches htmd's `scripting_enabled: true`.
* **Extension list.** The 15 checkboxes are exactly the 15 option flags of
  pulldown-cmark 0.13.4.
* **Footnotes coupling.** `ENABLE_OLD_FOOTNOTES` already includes the footnotes
  bit in the library, so the two-way rule on lines 153–154 matches it.

Findings
--------

1. **Share for "any input up to 1 MiB" is probably not achievable** (lines
   246–248 and 304–305). Encoding 1 MiB into an address gives roughly 1.4 MB
   before any compression. That is near or past the URL limits of the browsers
   the spec requires, and phones are the tightest. Either set a much lower
   share limit, separate from the 1 MiB conversion limit, or say the criterion
   is tested only up to a stated size.

2. **The address does not say it uses the fragment** (lines 246–250). If the
   state went in the query string, GitHub Pages would reject long addresses and
   the input would be sent to the server. One sentence saying the state lives
   after `#` settles this.

3. **"Always up to date" conflicts with the large-input pause** (lines 44–45 vs
   lines 265–266). Above 1 MiB the output is deliberately stale until "Convert
   anyway". The spec does not say what the direction switch and Copy do then,
   or after a crash. It also does not say what happens when a direction switch
   produces an input over 1 MiB.

4. **Back and Forward have nothing defined to go back to** (lines 249–250 and
   257–258). The spec says keystrokes add no history entry, but never says
   which actions do. If none do, Back leaves the site.

5. **A hang is not covered by "Crashes"** (lines 270–273). A converter that
   loops forever freezes the page silently, which is what the section forbids.
   Either rule hangs out of scope or require that the page stays responsive.

6. **"No other option changes" contradicts the footnotes rule** (lines 153–154
   vs lines 179–180). Picking the Old-style footnotes preset checks old-style
   footnotes, which also checks footnotes.

7. **Three options have no Expected results row:** Spaces after list number,
   the `<noscript>` option, and Line break in Faithful mode. Display math is
   also missing in the HTML → Markdown direction. These rows were produced in
   the same run and can be added as they are:

   | Input | Settings | Output |
   | --- | --- | --- |
   | `<ol start="3"><li>a</li></ol>` | 1 space after list number | `3.␣a` |
   | `<p>a</p><noscript><b>x</b></noscript>` | | `a⏎⏎\<b>x\</b>` |
   | `<p>a</p><noscript><b>x</b></noscript>` | noscript as raw text off | `a⏎⏎**x**` |
   | `<p>a<br>b</p>` | Faithful | `a<br>b` |
   | `<p><span class="math math-display">x^2</span></p>` | | `$$x^2$$` |

Smaller points
--------------

* **Skip tags case** (line 121). htmd's `skip_tags` is case-sensitive: passing
  `SCRIPT` skipped nothing in the run. The "case-insensitive" rule holds only
  if the playground lowercases the names, and a row with `SCRIPT` would pin
  that down.
* **1 MiB unit** (line 265). The spec does not say bytes or characters, while
  the status line counts characters.
* **Header version** (line 28 vs lines 248–249). The header shows only the htmd
  version, though the output can change with either library.
* **Formatting** (line 248). The line is not wrapped like the rest of the file.
* **Still open from the earlier review:**

  * out-of-range or empty values in the number fields;
  * partly valid addresses, and a format version newer than the page;
  * how long "Convert anyway" applies;
  * which tab is selected when Original disappears on a direction switch;
  * what "edited" means after a direction switch or a permalink load.
