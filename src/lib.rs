pub mod datetime;
pub mod output;
pub mod parser;
pub mod pattern;
pub mod ranges;

pub use pattern::{Pattern, PatternElement};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
