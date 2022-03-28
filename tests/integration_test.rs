// use std::default;

// #[test]
// fn test_adds_two() {
//     assert_eq!(3, add_two(1, 2));
// }
// extern crate lib;
// mod adder;
#![no_main]

// use adder or rust_sandbox doesn't help

#[test]
fn test_add() {
    // when using the package (and therefore also default crate name) rust_sandbox::add(3, 2)
    // VSCode is happy but not cargo test or rustc

    // compiling adder or lib.rs and adding it as an extern lib to rustc also doesn't do the trick
    assert_eq!(adder::add(3, 2), 5);
}
