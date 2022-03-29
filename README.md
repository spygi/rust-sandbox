Sandbox rust project to test tooling.

# Dev environment
Using dev containers in VS Code: 
- Debugging: VSLLDB VS Code extension
- Linting (clippy) & formatting (rustfmt) on save
```
"editor.formatOnSave": true, 
    "[rust]": { 
        "editor.defaultFormatter": "matklad.rust-analyzer"
    },
```        

# Testing
- By default cargo test captures output of successful tests, to show it use `cargo test --
--show-output` or `-- --nocapture`
- Tests (all types) are denoted with the `#[test]` attribute.
- Run all tests with `cargo test`

## Unit tests
- Usually in the same file as the code
- Can access private methods with `use super::*;`
- Use #[cfg(test)] to not have the compiled unless under test
- Run only library tests: `cargo test --lib`
  - Also runs adder component's tests
- Run only binary tests (main.rs): `cargo test --bin rust_sandbox` where rust_sandbox is the name of the binary

## Integration tests
- Usually organized in a tests/ folder
- By default they target library code (lib.rs), which can be access with `crate_name::method`
  - The default crate name is the name of the package.
  - If you want to test another (sub)component, it has to be referenced by the lib.rs e.g. `pub mod
    component_name;` and then it can be accessed from the integration tests with
    `crate_name::component_name`
- Each file there is a different crate which [means multiple executables](https://mozilla.github.io/application-services/book/design/test-faster.html#appendix-how-to-avoid-redundant-compiles-for-benchmarks-and-integration-tests) are created: one for each file.
  - Instead organize them in directories with a main.rs -> these directories are discovered by
    default
    ([tip](https://www.reddit.com/r/rust/comments/gq9rmq/comment/frsxbx3/?utm_source=share&utm_medium=web2x&context=3))
    If you do this however and you do a cargo test it will run the tests twice.
- In order to run only integration tests, create a test target [like
  here](https://joshleeb.com/blog/rust-integration-tests/) and use with `cargo test --test integration` 
- Some pointers in creating a test harness [with setup and shutdown
  code](https://tjtelan.com/blog/rust-custom-test-harness/)

## E2E tests
- E2E tests exercise the binary in Rust terms.
- [assert_cmd package](https://crates.io/crates/assert_cmd) can be used to discover the binary code
- See a usage in practice in [the bat
  repo](https://github.com/sharkdp/bat/blob/master/tests/integration_tests.rs)

  
