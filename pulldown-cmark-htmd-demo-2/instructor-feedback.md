Spec review: htmddemo2-spec.md

Reviewed against htmd 0.5.5 at 83b7903. I checked the claims in §2.1 against
src/lib.rs, src/options.rs and src/element_handler/mod.rs. Where this
review describes htmd's behavior, I confirmed it by running a probe crate
against the working tree. All the API facts in §2.1 hold: field names, defaults,
derives, the skip_tags(vec![]) assert, and the 1–4 clamp. The findings below
are listed most severe first.

High

1. Deep nesting overflows the stack, and the crash banner never appears (§2.1, §4.3)

§2.1 says the realistic failure is "a panic inside the library", and §4.3
builds the crash handling on the panic hook. However, htmd walks the DOM
recursively. Input nested deeply enough overflows the stack, and a stack
overflow is not a panic:

"<blockquote>".repeat(n) + "x"	Input size	Native release build, 1 MiB stack
n = 700	8.4 KB	ok
n = 1000	12 KB	thread 'main' has overflowed its stack
The wasm32 stack is also 1 MiB by default. On wasm, a stack overflow raises a
JS RuntimeError and never calls the panic hook. The page therefore freezes
silently, which is exactly what the forced-panic criterion in §9 is meant to
rule out. A 12 KB paste is far below the 1 MiB guard, and a permalink of such
input reproduces the crash on every load.

Suggested fixes, in order of preference:

Run state::convert in a Web Worker. If the worker dies, the main thread
shows the error in the status line and starts a new worker, and the page
never becomes unusable. This also keeps typing responsive on large input.
Alternatively, also show the banner from a window error listener that
catches the wasm RuntimeError. Raising the stack size (for example with
-C link-arg=-zstack-size=...) only moves the threshold.
Either way, add a deep-nesting case to the Edge cases preset or to the tests,
and correct the "realistic failure" sentence in §2.1. Separately, the
deep-nesting overflow is a library robustness bug worth filing on its own.

2. When the app first writes the hash is unspecified, which conflicts with §4.6 (§4.1, §4.6)

§4.6 says a bad hash from a hashchange stays "until the next state change".
The spec never says whether the initial state counts as a state change. With the
obvious implementation, a hash-sync Effect that runs on mount:

a load whose hash fails to decode replaces the user's bad hash with the
Basic example's state within 500 ms, so the user can't fix it, and the
decode-failure notice may clear at once;
a first visit with no hash gains #v=1&h=… straight away.
Specify that the app writes the hash only after a user-initiated change of
input or settings, and define "state change" (§4.1, §4.6) the same way.

3. A stale hash is left in the URL when the state is over the limit (§4.3, §4.6)

When the input is over 1 MiB or the encoded hash is over 32,768 characters,
"the hash is not updated". The URL keeps an older state, so a reload or a
bookmark silently restores old input. The notice is gone by then, and the
crash banner is the only place that warns about this. Decide whether to clear
the hash (replaceState to the bare path) or keep it. If you keep it, say so
deliberately, and make the notice text say that the URL holds an older version.

4. A pending hash write races with loading a hash (§4.6)

Two things are unspecified:

Which signal the hash encodes. It could be the raw text or
debounced_input (§4.3). Share's "flush" only captures the latest
keystrokes if it is the raw text.
What happens to a pending 500 ms write when a hashchange loads new
state. If the pending write captured the old state, it overwrites the hash
the user just navigated to.
Require loading from a hash to cancel any pending write.

Medium

5. A hashchange to an empty hash is unspecified (§4.6)

On load, an empty hash shows the default example. After a manual edit, Back
returns to the original entry, which usually has no hash. The decoder treats
that as a missing v, which is an Err and shows the notice. Say whether
this case should instead load the default example.

6. Two incorrect statements in §4.4 (Preview)

"htmd … doesn't escape ~, so <p>~~x~~</p> would preview as
struck-through". htmd escapes a ~ at the start of a line, because it could
open a fence: that input converts to \~~x~~. The claim holds mid-line, so
use <p>a ~~x~~</p> as the example. The decision to leave strikethrough off
is still right.
"The repo's own round-trip tests use the same options (tests/common/mod.rs)."
They don't: round_trip uses CommonMarkOptions::empty(). Only the table
tests (basic_tests.rs:725, :961) pass ENABLE_TABLES.
7. The default Pure mode writes script and style content as text (§4.5; affects provisional decision C1/C2)

With the demo defaults (library defaults, no skip_tags),
<p>a</p><script>alert(1)</script><style>p{}</style><p>b</p> converts to
a\n\nalert(1)\n\np{}\n\nb. <style> content in <head> also leaks
(x{}\n\na). Pasting real-world HTML into the playground therefore shows CSS
and JS in the Markdown. This is useful input for the team's pending decision
on the defaults. At a minimum, the "Mixed / real-world article snippet" preset
should contain no <script> or <style>, unless that output is intended.

8. Gaps in the frame CSP (§4.4, §5)

default-src does not cover base-uri or form-action, and DNS-prefetch and
preconnect hints from <link> are not reliably subject to CSP in every
browser:

base-uri: user HTML can supply its own <base href=…>. Add
base-uri 'none'; the spec's own <base target="_blank"> has no href, so
it isn't affected.
form-action: the sandbox already blocks form submission, so this gap
is harmless.
<link rel="dns-prefetch"> and <link rel="preconnect">: these may
cause DNS or TCP traffic, which conflicts with the absolute "no network
calls" rule in §5. Either rewrite rel= the way http-equiv is rewritten,
or list this as a second known exception.
9. CI pins Trunk's checksum but nothing else (§7)

Trunk downloads wasm-bindgen-cli and binaryen (wasm-opt) at build time
without checksum verification, so the sha256sum -c step protects only one
of three binaries. Either accept this and say so, or install and verify all
three, and turn off Trunk's auto-download.
No step uses --locked. A library PR that raises a dependency requirement
makes Cargo rewrite demo/Cargo.lock silently in CI, so the build no longer
matches the committed lock. Use --locked, and note that such PRs must also
update demo/Cargo.lock. That file is under demo/, so the
"only file outside demo/" rule still holds.
Low

The provisional-decisions header gives the agent no instruction (header).
It doesn't say whether to implement the provisional choices as written or
to stop and ask. Add "implement as written".
Clear and Examples behave inconsistently (§4.2). Examples asks before
discarding custom input, but Clear doesn't. A set_text-based Clear also
loses the browser's undo history. Also specify that the Examples
<select> reverts when the user cancels the confirm.
Share has no fallback (§4.6). Unlike Copy, Share has no fallback when
the Clipboard API is unavailable. One option is to show the URL in a
selected read-only field.
The iframes have no title (§4.7). Both iframes need title
attributes for WCAG 4.1.2.
Timer precision varies by browser (§4.1, §5). Firefox and Safari
coarsen performance.now() to about 1 ms on pages that aren't
cross-origin isolated, so "one decimal" always shows .0 there. This is
fine because §5 measures in Chrome, but say so.
The reason given for the empty [workspace] is wrong (§3). The root
Cargo.toml has no [workspace], so the empty table isn't what keeps the
root build unaffected. Its real job is to guard against a future root
workspace adopting demo/.
The debounce timing needs three clarifications (§4.3).
Initialize debounced_input to the initial text, not to an empty
string.
Have example and permalink loads set it directly, so the output isn't
stale for 100 ms.
Say whether the 1 MiB guard checks the raw text or the debounced text.
Two permalink details are unspecified (§4.6).
Say what the skip_tags text field shows after a permalink loads. It
can only show the normalized list, because dropped entries aren't
encoded.
Say whether the decoder percent-decodes the hash first. Some chat apps
and mail clients rewrite , and : as %2C and %3A. As written,
strict decoding rejects such links.