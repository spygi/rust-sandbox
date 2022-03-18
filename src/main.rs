use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0..10);
    let b = rng.gen_range(0..10);

    println!("Sum of {} and {} is {}", a, b, add(a, b));
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn test_add() {
    assert_eq!(3, add(1, 0));
}
