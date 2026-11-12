# Team LLM review and project verification

This file records the team's specification-review process and the project verification evidence. Technical requirements remain in [`spec.md`](spec.md). The review history below describes the reviews evidenced in our working discussion; it does not claim that all four contributors ran separate review rounds or that earlier findings were fixed before the final revision.

## 1. Team contributions

The following one-sentence descriptions reflect the agreed work split; the team should confirm that each description matches the actual final commits before submitting the pull request.

- **Yuriy:** Defined the project scope and shared test-case format, and integrated the sections into a common specification.
- **Sam:** Inventoried existing tests and developed the migration approach for standardized Faithful-mode round-trip cases.
- **Alibek:** Defined strict HTML-comparison rules, documented differences, and stability criteria.
- **Kamila:** Defined the round-trip execution and failure-reporting strategy, reviewed cross-section consistency, and coordinated compilation and test verification.

## 2. LLM review history

The recorded reviews focused on Section 4 and its consistency with Sections 1–3. Round 3 included a cross-document consistency check; this is **not** a claim that every other author's section underwent its own repeated review. Finding counts refer to the respective review outputs, including repeated observations across rounds.

| **Round** | **Findings** | **Main changes made** |
| --------- | ------------ | --------------------- |
| 1 | 2 | Identified that `round_trip` alone does not expose intermediate Markdown or per-case option handling; noted missing explicit `DocumentedInstability` and Markdown-only case handling; no edits applied in that round. |
| 2 | 2 | Rechecked Section 4 against Sections 1–3; confirmed ambiguity in separating htmd options from pulldown-cmark flags and in the stability assertion; no edits applied in that round. |
| 3 | 3 | Checked whole-spec consistency with emphasis on Section 4; flagged the `Stable` versus `DocumentedInstability` exception, Markdown-only failure reporting and reliance on the default `round_trip` helper; no edits applied in that round. |
| 4 | 0 | Applied concise revisions to Sections 3–4: explicit stable/unstable expectations; independent Markdown assertions; per-stage options; expectation-specific failure reports; final document-level consistency review found no additional relevant issues in the supplied specification. |

**Provenance note:** Rounds 1–3 summarize reviews discussed in the team's working exchange, including a GPT-6 review. Round 4 is a final textual consistency review performed with OpenAI GPT-6 on the updated attached `spec.md`. Exact prompts and outputs from other team members' review sessions were not supplied; attach them if the team has them.

## 3. Final LLM review — Section 4

**Model:** OpenAI GPT-6.

**Exact review prompt used for the final written consistency check:**

> Review the updated `spec.md`, concentrating on Section 4 and its consistency with Sections 1–3. Check the handling of default versus generalized round-trip helpers, htmd and pulldown-cmark options, `Stable` versus `DocumentedInstability` expectations, Markdown-only cases, failure reporting, and the verification command list. Report only concrete outstanding problems in the specification; do not infer that the proposed runner has been implemented.

**Final review output:**

> No additional relevant findings in the supplied specification. Section 4 now distinguishes the stage-specific converter options; preserves the intermediate Markdown; specifies `Stable` and approved `DocumentedInstability` comparisons; allows exact-Markdown cases without an HTML expectation; and reports failures according to which assertion failed. Section 3 now states the documented-instability exception explicitly. This is a consistency review of the written strategy, not a source-code execution or validation of the unimplemented generalized runner.

## 4. Project compilation and run verification

**Exact instructor-requested prompt:**

> Verify that you can compile and run the project in the current git repo.

**Verification assistant:** GitHub Copilot in VS Code (the exact underlying model was not supplied and must be filled in from the original chat session, if available).

**LLM output (successful verification response, preserved from the provided conversation):**

> I’m re-running the repo verification in the actual project root with the Rust toolchain available so I can give you a final yes/no status based on fresh build and run output.
>
> Ran terminal command: `$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; Set-Location "c:\Users\Documents\KBTU\Data mining\advanced\New folder\htmd"; cargo build; cargo clippy -- --deny warnings; cargo test; cargo run --example simple`
>
> Read [](file:///c%3A/Users/AppData/Roaming/Code/User/workspaceStorage/85bbe8ed1e7a15c7c5aad0216f8c0faa/GitHub.copilot-chat/chat-session-resources/6379a0aa-2159-4942-9666-4dc444547949/call_En95w4LXJJBKApVGZA6tasmd__vscode-1794233934546/content.txt#1-1), lines 1 to 260
>
> Ran terminal command: `$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; Set-Location "c:\Users\bagdatkamilla\Documents\KBTU\advanced\New folder\htmd"; cargo run --example simple; $code = $LASTEXITCODE; Write-Host "RUN_EXIT:$code"; exit $code`
>
> ## ✅ Verification passed
>
> I verified the project in the current repo with the requested checks:
>
> - `cargo clippy -- --deny warnings` → exit code 0
> - `cargo test` → all reported test groups passed with 0 failed
> - `cargo run --example simple` → exit code 0
>
> ### Evidence
> The final sample run completed successfully:
>
> ```text
>     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
>      Running `target\debug\examples\simple.exe`
> RUN_EXIT:0
> ```
>
> This confirms the project currently compiles and runs successfully in this repo.

**Evidence interpretation:** The assistant reported success for Clippy, tests, and the example. The response did not separately report `cargo build`'s exit code, although the final sample executable ran successfully. For a fully auditable record of every command, attach the original full terminal log or run the commands separately and record each exit code.
