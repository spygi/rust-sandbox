use rust_sandbox::adder_component::util::Printer;

#[test]
fn int_subcomponent_test() {
    let printer = Printer {};
    assert_eq!(rust_sandbox::adder_component::add(3, 2, printer), 5);
}
