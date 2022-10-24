pub mod data;
pub mod datetime;
pub mod output;
pub mod parser;
pub mod pattern;
pub mod resolver;

pub use pattern::{Pattern, PatternElement};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
