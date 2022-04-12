// if we want to avoid paths here
// we could import all modules on the lib with `pub mod util;`
// and here `use super::util;`
#[path = "../util/mod.rs"]
pub mod util;

use crate::adder_component::util::*;

/// This is a doc test.
///
/// # Examples
/// ```
/// let result = rust_sandbox::adder_component::add(-2, 3);
/// assert_eq!(result, 1);
/// ```
pub fn add<T: Print>(a: i32, b: i32, printer: T) -> i32 {
    println!("{}", printer.pretty_print(a));
    println!("{}", printer.pretty_print(b));

    let sum = private_add(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);

    sum
}

#[allow(dead_code)]
#[cfg(not(tarpaulin_include))]
pub fn add_two(a: i32) -> i32 {
    private_add(a, 2)
}

fn private_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // gets compiled only when testing
#[path = "./tests/more_unit_tests.rs"]
mod more_unit_tests;

mod tests {
    // #[cfg_attr(tarpaulin, ignore)] // ignore test from coverage
    #[test]
    fn unit_private_add_test() {
        // can test private methods this way or by `use super::*;`
        assert_eq!(3, crate::adder_component::private_add(1, 2));
    }

    #[test]
    fn unit_test_mocking() {
        let mut mock = super::MockPrint::new();
        mock.expect_pretty_print()
            .return_const_st(String::from("mocked return"))
            .times(2);

        assert_eq!(5, crate::adder_component::add(2, 3, mock));
    }
}
