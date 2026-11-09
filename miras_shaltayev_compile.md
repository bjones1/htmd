### - Verify that you can compile and run the project in the current git repo.                                                                         

### Claude Sonnet 5.5 answer:
``` 
⏺ Bash(cargo build 2>&1 | tail -5; ls; ls examples src/bin 2>/dev/null; grep -n -A5 '^\[\[bin\]\]' Cargo.toml)
  ⎿  (eval):1: command not found: cargo
     AGENTS.md
     Cargo.lock
     … +12 lines (ctrl+o to expand)
  ⎿  Allowed by auto mode

  Listed 1 directory (ctrl+o to expand)

⏺ Cargo exists in ~/.cargo/bin but is not on PATH.

⏺ Bash(cargo build 2>&1 | tail -5; ls; ls examples src/bin 2>/dev/null; grep -n -A5 '^\[\[bin\]\]' Cargo.toml)
  ⎿  (eval):1: command not found: cargo
     AGENTS.md
     Cargo.lock
     … +12 lines (ctrl+o to expand)
  ⎿  Allowed by auto mode

  Listed 1 directory (ctrl+o to expand)

⏺ Cargo exists in ~/.cargo/bin but is not on PATH.

⏺ Bash(export PATH="$HOME/.cargo/bin:$PATH"; cargo build 2>&1 | tail -5; ls examples/simple; cargo run --example simple 2>&1 | tail -15)
  ⎿     Compiling xml5ever v0.39.0
        Compiling html5ever v0.39.0
        Compiling markup5ever_rcdom v0.39.0+unofficial
     … +18 lines (ctrl+o to expand)
  ⎿  Allowed by auto mode
     
⏺ The example printed nothing. I will check what it does and run the tests.

⏺ Bash(export PATH="$HOME/.cargo/bin:$PATH"; cat examples/simple/main.rs; cargo test 2>&1 | grep -E "^test result|FAILED|failed" | sort | uniq -c)
  ⎿  use htmd::{Element, HtmlToMarkdown, element_handler::Handlers, options::Options};

     fn main() {
     … +43 lines (ctrl+o to expand)
  ⎿  Allowed by auto mode
  
⏺ The project compiles, runs, and passes its tests.

  - PATH: cargo is in ~/.cargo/bin, but that folder is not on your PATH. Run export PATH="$HOME/.cargo/bin:$PATH" first. Add the line to your ~/.zshrc
    to make it permanent.
  - Build: cargo build finished with no errors.
  - Run: cargo run --example simple exited with code 0. It prints nothing because the example only runs assert_eq! checks, and all of them passed.
  - Tests: cargo test passed in every test group. The one ignored test is marked as ignored in the source.

  I did not run cargo clippy -- --deny warnings, which the CI also requires. Tell me if you want me to run it.

✻ Baked for 58s · done 5:20 PM
```
