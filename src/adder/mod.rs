#[path = "../util/mod.rs"]
mod util;

/// ```
/// let result = rust_sandbox::adder::add(-2, 3);
/// assert_eq!(result, 1);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    println!("{}", util::pretty_print(a));
    println!("{}", util::pretty_print(b));

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
mod tests {
    use super::*;

    // #[cfg_attr(tarpaulin, ignore)] // ignore test from coverage
    #[test]
    fn test_private_add() {
        assert_eq!(3, private_add(1, 2)); // can test private methods
    }
}
