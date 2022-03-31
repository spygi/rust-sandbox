#[cfg(test)]
mod tests {
    #[test]
    fn unit_extra_add_test() {
        assert_eq!(3, crate::adder_component::private_add(1, 2));
    }
}
