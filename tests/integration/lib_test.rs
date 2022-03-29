#[path = "../common/util.rs"]
mod util;

#[test]
fn int_lib_test() {
    util::helper_method();
    assert_eq!(rust_sandbox::main(3, 2), 5);
}
