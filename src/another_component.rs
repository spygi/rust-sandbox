#[allow(dead_code)]
pub fn another_function() {
    super::another_util::another_util_fn();
    println!("From another public");
}

#[allow(dead_code)]
fn another_private_function() {
    crate::another_util::another_util_fn();
    println!("From another private");
}

#[cfg(test)]
mod extra_tests;
