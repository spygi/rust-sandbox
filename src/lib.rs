pub mod adder;

pub fn main(a: i32, b: i32) -> i32 {
    println!("From lib");
    adder::add(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib() {
        assert_eq!(crate::main(3, 2), 5);
    }
}
