#[allow(dead_code)]
fn another_function() {
    super::another_util::another_util_fn();
    println!("From another");
}

#[cfg(test)]
mod extra_tests;
