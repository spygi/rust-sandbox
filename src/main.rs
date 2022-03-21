mod util; // bring module util in file util.rs in scope

use rand::prelude::*;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0..10);
    let b = rng.gen_range(0..10);

    println!("{}", util::pretty_print(a));
    println!("{}", util::pretty_print(b));
    println!("Sum of {} and {} is {}", a, b, add(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[cfg_attr(tarpaulin, ignore)]
    fn test_add() {
        assert_eq!(3, add(1, 2));
    }
}
