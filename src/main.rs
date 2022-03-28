use rand::prelude::*;
// mod lib;

#[cfg(not(tarpaulin_include))]
fn main() {
    let mut rng = rand::thread_rng();
    let _a = rng.gen_range(0..10);
    let _b = rng.gen_range(0..10);

    // lib::adder(a, b);
}
