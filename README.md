Sandbox rust project to test Rust tooling:

- [Rust](#rust)
- [Dev environment](#dev-environment)
- [Testing](#testing)
- [CI/CD](#cicd)
- [Test Coverage](#test-coverage)
- [Documentation](#documentation)

The main entry points are [main.rs](./src/main.rs) that is a thin wrapper around
the [lib.rs](./src/lib.rs) that contains the actual functionality. The rest of
the code is organised in 2 ways: 

1. Components are split in different folders e.g.
  [adder_component/](./src/adder_component/mod.rs) that is using
  [util/](./src/util/mod.rs)
1. Components reside at the top-level e.g.
   [another_component.rs](./src/another_component.rs) that is using [another_util](./src/another_util.rs)

Importing those components can be done centrally in the lib.rs (e.g. `pub mod
another_util`) or via paths e.g. util component in
[adder_component](./src/adder_component/mod.rs).

# Rust

- Rustup: is the installer of Rust (and various components) and Cargo.
  Supports different channels (stable, nightly, beta). Basic usage: `rustup default stable` or
  `nightly` to switch from one toolchain to the other.
- Cargo: package manager.
  - If you want to use nightly one: `cargo +nightly ...` 
- Rustc: the Rust compiler
  - [Editions](https://doc.rust-lang.org/edition-guide/introduction.html)

# Dev environment

The default [Rust Dev container in VS
Code](https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust) offers:

- Debugging: using vscode-lldb VS Code extension
- Code analysis: using [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension
- Linting (clippy) & formatting (rustfmt) on save
```json
"editor.formatOnSave": true, 
"[rust]": { 
    "editor.defaultFormatter": "matklad.rust-analyzer"
},
```
  - If not already installed (`rustup component list | grep installed`): `rustup component add
    clippy rustfmt`
  - Invoke directly with `cargo clippy` or `cargo fmt` respectively.

# Testing

- Tests (all types besides documentation tests) are denoted with the `#[test]` attribute.
- `cargo test` runs unit, tests under tests/ folder and documentation tests
- By default cargo test "captures" output of successful tests and does not display it, to show it
use `cargo test -- --show-output` or `-- --nocapture`

## Unit tests

- Conventionally, unit tests are in the same file as the code, see e.g. [lib.rs](./src/lib.rs) or [adder_component/mod.rs](./src/adder_component/mod.rs)
  - Tests can also be split in other files (if the files grow too much for example) e.g. see
    [adder_component/tests/](./src/adder_component/tests/more_unit_tests.rs) or
    [another_component/extra_tests.rs](./src/another_component/extra_tests.rs)
- Can access private methods with `use super::*;` or fully qualifying names `crate_name::module_name`
- Use `#[cfg(test)]` to have them compiled only under test
- Run only library tests (recursively): `cargo test --lib`
- Run only binary tests (main.rs): `cargo test --bin rust_sandbox` where "rust_sandbox" is the name of
  the binary
- Run only a specific test method: `cargo test part_of_the_test_method_name`

### Mocking libraries

Here is a (very thorough) [list comparing different mocking
libraries](https://asomers.github.io/mock_shootout/) in Rust. According to this
list (which was compiled from the author of Mockall),
[Mockall](https://crates.io/crates/mockall) is the clear winner in terms of
features and ease-of-use. It runs on stable Rust and looking at the downloads,
it looks also as the most popular with over 2M (the next one being
[Mocktopus](https://crates.io/crates/mocktopus) with 75K). It is also the
mocking library used in the [Tokio
project](https://github.com/tokio-rs/tokio/blob/master/tokio-util/tests/framed_write.rs#L111) 

For an example usage of the Mockall library, see the
[unit_test_mocking](./src/adder_component/mod.rs#L49) test method.
- Run with `cargo test unit_test_mocking -- --show-output` to see mocked results
  in the output.

## Integration tests

- Usually organized in a [tests/](./tests) folder
- By default they target library code (lib.rs), which can be accessed in the test with `crate_name::method`
  - The default crate name is the name of the package.
  - If you want to test another (sub)component, it has to be referenced by the lib.rs e.g. `pub mod
    module_name;` and then it can be accessed from the integration tests with
    `crate_name::module_name`
- Each file in the tests/ folder is a different crate which means [multiple executables are created](https://mozilla.github.io/application-services/book/design/test-faster.html#appendix-how-to-avoid-redundant-compiles-for-benchmarks-and-integration-tests): one for each file.
  - Instead organize them in an integration directory with a main.rs -> these directories are
    discovered by default.
    - The downside is that we need to remember to add the test file in the tests/integration/main.rs.
  - You can still run tests selectively by the file name or test name.
    - The file name is the name of the crate.
- In order to run *only* integration tests, create a test target [like
  shown here](https://joshleeb.com/blog/rust-integration-tests/) and use with `cargo test --test integration` 
- Some pointers in creating a test harness [with setup and shutdown
  code](https://tjtelan.com/blog/rust-custom-test-harness/)

### E2E tests

- E2E tests exercise the binary from the "outside" (similar to integration tests).
- [assert_cmd package](https://crates.io/crates/assert_cmd) can be used to access the binary
- See a usage in practice in [the bat
  repo](https://github.com/sharkdp/bat/blob/master/tests/integration_tests.rs)

## Doc tests

- They showcase how to use the code like regular docs but since they are runnable, they don't get out
  of sync from the code. 
- A different crate/main() is used, so you test the public API (similar to integration tests)
  - You would need to prefix your methods with `crate_name::`
- Run only doc tests with `cargo test --doc`
- Unfortunately, doc tests do not contribute to the test coverage in neither Tarpaulin nor Grcov
  (tools explained in [Test coverage section](#test-coverage))

# CI/CD

Github Actions
  - [Rust tools
    installed by default](https://github.com/actions/virtual-environments/blob/ubuntu20/20220227.1/images/linux/Ubuntu2004-Readme.md#rust-tools)
    on Ubuntu latest GitHub workers
  - [Action-rs repo](https://github.com/actions-rs) with various (unofficial) Rust actions
  - [Rust itself uses Github
    Actions](https://blog.rust-lang.org/inside-rust/2020/07/23/rust-ci-is-moving-to-github-actions.html)
  
[Other options](https://doc.rust-lang.org/cargo/guide/continuous-integration.html) include Azure
DevOps, Travis etc.  

# Test coverage

Some basic features that we would need are

- Ignore test code itself from coverage so that the numbers are correct
- Be able to ignore non-test code from coverage
- Run locally and inspect coverage
- Run in CI/CD and upload coverage report in a tool like Codecov

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

## Alternatives 

### Grcov

[mozilla/grcov](https://github.com/mozilla/grcov#example-how-to-generate-gcda-files-for-a-rust-project):
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
- Offers branch coverage - is it a must-have?

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

### Others
  
- [cargo-cov](https://github.com/kennytm/cov): abandoned project, no push since 2018, no release
  ever
- [kcov](https://github.com/kennytm/cargo-kcov): abandoned, last commit since 2019, last release in 2016

# Documentation 

- `cargo doc` generates a navigatable documentation
  - Copy locally "./targets/doc/" folder and open "./targets/doc/rust_sandbox/index.html"
- The generated docs could be hosted in Github pages etc.

# References

- [Rust book](https://doc.rust-lang.org/book/)
- [Rust by example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo book](https://doc.rust-lang.org/cargo/)
- [Rustup book](https://rust-lang.github.io/rustup/index.html)