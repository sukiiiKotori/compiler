pub mod remove_ld;
pub mod remove_mv;
use crate::structures::riscv_struct::*;

impl RiscV {
    pub fn optimise_riscv(&mut self) {
        self.remove_ld();
        self.remove_mv();
    }
}