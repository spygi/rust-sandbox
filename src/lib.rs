pub mod adder_component;

pub fn main(a: i32, b: i32) -> i32 {
    println!("From lib");
    adder_component::add(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib() {
        assert_eq!(crate::main(3, 2), 5);
    }
}
