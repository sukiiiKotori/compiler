pub mod peephole;
use crate::structures::riscv_struct::*;

impl RiscV {
    pub fn optimise_riscv(&mut self) {
        self.elininate_load();
        self.eliminate_move();
    }
}