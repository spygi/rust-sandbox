Sandbox rust project to test Rust tooling:

- [Dev environment](#dev-environment)
- [Testing](#testing)
- [CI/CD](#cicd)
- [Test Coverage](#test-coverage)
- [Documentation]

# Dev environment
Using the default [Rust Dev container in VS
Code](https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust)  

- Debugging: uses vscode-lldb VS Code extension
- Code analysis: uses [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension
- Linting (clippy) & formatting (rustfmt) on save
```json
"editor.formatOnSave": true, 
"[rust]": { 
    "editor.defaultFormatter": "matklad.rust-analyzer"
},
```
Both should already be installed locally (`rustup component list`) and could be invoked manually
with `cargo clippy` or `cargo fmt` respectively.

# Testing
- Tests (all types besides documentation tests) are denoted with the `#[test]` attribute.
- `cargo test` runs unit, tests under folder tests/ and documentation tests
- By default cargo test "captures" output of successful tests and does not display it, to show it
use `cargo test -- --show-output` or `-- --nocapture`

## Unit tests
- Usually in the same file as the code
- Can access private methods with `use super::*;`
- Use #[cfg(test)] to not have the compiled unless under test
- Run only library tests: `cargo test --lib`
  - Also runs sub-component's tests
- Run only binary tests (main.rs): `cargo test --bin rust_sandbox` where rust_sandbox is the name of
  the binary
- Run only a specific test_file: `cargo test test_file`

## Integration tests
- Usually organized in a tests/ folder
- By default they target library code (lib.rs), which can be accessed in the test with `crate_name::method`
  - The default crate name is the name of the package.
  - If you want to test another (sub)component, it has to be referenced by the lib.rs e.g. `pub mod
    component_name;` and then it can be accessed from the integration tests with
    `crate_name::component_name`
- Each file in the tests/ folder is a different crate which means [multiple executables are created](https://mozilla.github.io/application-services/book/design/test-faster.html#appendix-how-to-avoid-redundant-compiles-for-benchmarks-and-integration-tests): one for each file.
  - Instead organize them in an integration directory with a main.rs -> these directories are
    discovered by default.
  - You can still run tests selectively by the file name or test name.
- In order to run *only* integration tests, create a test target [like
  here](https://joshleeb.com/blog/rust-integration-tests/) and use with `cargo test --test integration` 
- Some pointers in creating a test harness [with setup and shutdown
  code](https://tjtelan.com/blog/rust-custom-test-harness/)

### E2E tests
- E2E tests exercise the binary in Rust terms from the "outside".
- [assert_cmd package](https://crates.io/crates/assert_cmd) can be used to discover the binary
- See a usage in practice in [the bat
  repo](https://github.com/sharkdp/bat/blob/master/tests/integration_tests.rs)

## Doc tests
- A different crate/main() is used so you need to prefix your methods with `crate_name::`
  - Similar to how integration tests work
- They are not part of test coverage in neither Tarpaulin nor Grcov (listed in [Test coverage section](#test-coverage))

# CI/CD
- Github Actions
  - [Rust tools
    installed by default](https://github.com/actions/virtual-environments/blob/ubuntu20/20220227.1/images/linux/Ubuntu2004-Readme.md#rust-tools)
    on Ubuntu latest GitHub workers
  - [Action-rs repo](https://github.com/actions-rs) with various (unofficial) Rust actions
  - [Rust itself uses Github
    Actions](https://blog.rust-lang.org/inside-rust/2020/07/23/rust-ci-is-moving-to-github-actions.html)
  
[Other options](https://doc.rust-lang.org/cargo/guide/continuous-integration.html) include Azure
DevOps, Travis etc.  

# Test coverage
Some basic features that we would need here are
- Ignore test code itself from coverage
- Be able to ignore non-test code from coverage
- Run locally and on CI/CD
- Upload coverage report in a tool like Codecov

## Tarpaulin
A [Rust package](https://crates.io/crates/cargo-tarpaulin) to run tests and generate test coverage
reports. Some statistic: version 0.20, actively developed (single maintainer mostly), 70
contributors, monthly releases. Probably the most used coverage tool in the Rust community: total downloads: 378K, recent: 63K

- Run locally with `cargo tarpaulin` 
- CI/CD integration: [Github action](https://github.com/marketplace/actions/rust-tarpaulin) or
[official Docker
image](https://github.com/xd009642/tarpaulin#github-actions).
- Supported by Codecov

![Tarpaulin](/docs/tarpaulin.jpg)

[List of features](https://github.com/xd009642/tarpaulin#features)

- Ignore specific methods from coverage with `#[cfg(not(tarpaulin_include))]`
- Ignore test code (unit and integration) with a single flag (`--ignore-tests`)
- Export to HTML with `cargo tarpaulin -o html`

Limitations, see [roadmap](https://github.com/xd009642/tarpaulin#roadmap):  
- Only supports x86_64 processors running Linux
- No branch and condition coverage

## Alternatives:
- [mozilla/grcov](https://github.com/mozilla/grcov#example-how-to-generate-gcda-files-for-a-rust-project):
  Rust tool to collect and aggregate code coverage data for different languages (not just Rust).
  Total downloads: 330K, recent: 58K.
  - Can produce lcov which can be fed in Codecov [Codecov
    formats](https://docs.codecov.com/docs/supported-report-formats))
  - Available [Github action](https://github.com/actions-rs/grcov) with some [limited configuration](https://github.com/actions-rs/grcov#config) available.
  - [Using .procfraw
    file](https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project)
    does not work for me locally with a linker error `error: linking with `cc` failed:`

![Grcov](/docs/grcov.jpg)

Advantages:

- Supported by Mozilla
- Can be used in other languages too
- Offers branch coverage - is it a nust-have?

Disadvantages:

- Ease of use
  - Requires nightly toolchain which can be unstable as well as incovenient (components like clippy
  installed in stable would need to be installed again, rust-analyzer might need [additional
  config](https://rust-analyzer.github.io/manual.html#toolchain) to use the stable toolchain)
  - Awkward syntax to exclude unit tests code with regex e.g. `--excl-start '#\[cfg\(test'`)
    - Is not yet supported from the GH action (but there is a [WIP issue](https://github.com/actions-rs/grcov/issues/80))
    - Integration tests code can be ignored with `--ignore "tests/*"
- Slower compared to Tarpaulin (6 mins vs 3 mins e.g. on [this Github
  run](https://github.com/spygi/rust-sandbox/actions/runs/2065046903))
  - Requires 2 steps: cargo test (generates gcno and gcda files) and then grcov to interpret the
  coverage.
  
- [cargo-cov](https://github.com/kennytm/cov): abandoned project, no push since 2018, no release
  ever

- [kcov](https://github.com/kennytm/cargo-kcov): abandoned, last commit since 2019, last release in 2016
