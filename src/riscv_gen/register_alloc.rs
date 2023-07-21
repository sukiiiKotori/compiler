use std::collections::{HashSet, HashMap};
use crate::structures::riscv_struct::AsmFunc;

pub trait RegisterAllocator {
    fn new() -> Self;
    fn alloc_regs(&mut self, func: &mut AsmFunc);
    fn get_spilled(&self) -> &HashSet<String>;
    fn get_alloc_res(&self) -> &HashMap<String, &'static str>;
}