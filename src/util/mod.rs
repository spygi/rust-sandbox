#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait Print {
    fn pretty_print(&self, a: i32) -> String;
}

pub struct Printer {}

impl Print for Printer {
    fn pretty_print(&self, a: i32) -> String {
        format!("Given number is {}", a)
    }
}
