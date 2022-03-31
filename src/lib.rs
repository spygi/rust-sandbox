pub mod adder_component;
pub mod another_component;

// by importing another_util here we can access it via super::another_util or crate::another_util;
// otherwise we can do so by the path
pub mod another_util;

pub fn main(a: i32, b: i32) -> i32 {
    println!("From lib");
    adder_component::add(a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn unit_lib_test() {
        assert_eq!(crate::main(3, 2), 5);
    }
}
