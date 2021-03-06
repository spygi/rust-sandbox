use rand::prelude::*;

fn main() {
    println!("From main");
    let mut rng = rand::thread_rng();
    let a = rng.gen_range(0..10);
    let b = rng.gen_range(0..10);

    println!("{}", rust_sandbox::main(a, b)); // call the library, `use rust_sandbox` is not needed;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_main_test() {
        main();
    }
}
