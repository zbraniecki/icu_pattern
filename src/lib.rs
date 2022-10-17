pub mod pattern;
pub mod replacement;

pub use pattern::{Pattern, PatternElement};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
