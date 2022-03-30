#[path = "../util/mod.rs"]
mod util;

pub fn adder(a: i32, b: i32) -> i32 {
    println!("{}", util::pretty_print(a));
    println!("{}", util::pretty_print(b));
    let sum = add(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);

    sum
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
#[cfg(not(tarpaulin_include))]
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // gets compiled only when testing
mod tests {
    use super::*;

    // #[cfg_attr(tarpaulin, ignore)] // ignore test from coverage
    #[test]
    fn test_internal_add() {
        assert_eq!(3, internal_adder(1, 2)); // can test private methods
    }
}
