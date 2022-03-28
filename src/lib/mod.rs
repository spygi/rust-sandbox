mod util;

pub fn adder(a: u32, b: u32) -> u32 {
    println!("{}", util::pretty_print(a));
    println!("{}", util::pretty_print(b));
    let sum = add(a, b);
    println!("Sum of {} and {} is {}", a, b, sum);
    sum
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)] // gets compiled only when testing
mod tests {
    use super::*;

    #[test]
    // #[cfg_attr(tarpaulin, ignore)]
    fn test_add() {
        assert_eq!(3, add(1, 2)); // can test private methods
    }
}
