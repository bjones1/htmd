Project 2 submission
====================

Team members are Armanzhan Onggarbay, Nurlybek Orynbassar, Miras Shaltayev, and Aidyn Tazhibay. The specification consists of the file [testing_approach.md](testing_approach.md).

Draft submission
----------------

### Contributions

* Armanzhan Onggarbay: (Armanzhan's contributions)
* Nurlybek Orynbassar: (Nurlybek's contributions)
* Miras Shaltayev: Wrote the initial draft of `testing_approach.md`.
* Aidyn Tazhibay: (Aidyn's contributions)

### Compiler test

The following statements summarize LLM output on each team member's computer, based on the prompt "Verify that you can compile and run the project in the current git repo."

#### Armanzhan Onggarbay

(Summarized LLM output)

#### Nurlybek Orynbassar

Claude Opus 5.5 (Claude Code 2.1.319) found that the project builds and runs on this machine with no changes: `cargo build` finished with no errors or warnings, and `cargo clippy -- --deny warnings` exited with status 0 (it touched `src/lib.rs` first to force a fresh check instead of a cached one). Since `htmd` is a library, it ran the bundled examples: `cargo run --example simple` exited with code 0 and prints nothing, because it only runs `assert_eq!` checks which held, and `cargo run --example table` printed the expected Markdown table. `cargo test` passed 267 tests across all suites with 0 failures and one test ignored in the source.

#### Miras Shaltayev

Claude Sonnet 5.5 found that `cargo` was installed in `~/.cargo/bin` but not on the `PATH`, and fixed this with `export PATH="$HOME/.cargo/bin:$PATH"`. After that, `cargo build` finished with no errors, `cargo run --example simple` exited with code 0 (it prints nothing, since the example only runs `assert_eq!` checks that passed), and `cargo test` passed in every test group, with one test ignored in the source. It did not run `cargo clippy -- --deny warnings`, which the CI also requires.

#### Aidyn Tazhibay

Claude Opus 5.5 (Claude Code, medium effort) found that the project compiles and runs on this machine with Rust 1.99.1, which I had installed with `rustup-init`: `cargo build` finished with no errors, and `cargo clippy -- --deny warnings` reported no warnings. Since `htmd` is a library, it ran the bundled examples: `cargo run --example simple` printed nothing, because it only runs `assert_eq!` checks which held, and `cargo run --example table` printed the expected Markdown table. `cargo test` passed 267 tests across all suites with 0 failures and one test ignored in the source.

### Spec review rounds

(Table showing spec review rounds)

### Last LLM review

(LLM and prompt used)

(LLM output, possibly with strikethrough)
