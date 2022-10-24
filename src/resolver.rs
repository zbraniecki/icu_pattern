use crate::output::Output;
use crate::pattern::PatternElement;

pub trait Resolver<'output> {
    type OutputElement: 'output;
    type Output: Output<Self::OutputElement>;

    fn get(&self, key: usize, output: &mut Self::Output);
}
