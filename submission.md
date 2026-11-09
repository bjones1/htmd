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

(Summarized LLM output)

#### Miras Shaltayev

Claude Sonnet 5.5 found that `cargo` was installed in `~/.cargo/bin` but not on the `PATH`, and fixed this with `export PATH="$HOME/.cargo/bin:$PATH"`. After that, `cargo build` finished with no errors, `cargo run --example simple` exited with code 0 (it prints nothing, since the example only runs `assert_eq!` checks that passed), and `cargo test` passed in every test group, with one test ignored in the source. It did not run `cargo clippy -- --deny warnings`, which the CI also requires.

#### Aidyn Tazhibay

Claude Opus 5.5 found that Rust was not installed on Windows 10, so it installed the stable toolchain (Rust 1.99.1) with `rustup-init` and the MSVC C++ build tools. Cargo's PATH change applied only to new terminals, so it added `~/.cargo/bin` to the `PATH` of the open terminal. After that, `cargo build` finished with no errors, `cargo test` passed in every test group (267 passed, 0 failed, 1 ignored in the source), `cargo run --example simple` printed nothing because the example only runs `assert_eq!` checks that passed, and `cargo clippy -- --deny warnings` reported no warnings.

### Spec review rounds

(Table showing spec review rounds)

### Last LLM review

(LLM and prompt used)

(LLM output, possibly with strikethrough)
