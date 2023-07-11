use std::io::*;
pub trait WriteText {
    fn writetext(&self, output: &mut impl Write);
}