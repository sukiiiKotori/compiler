pub mod peephole;

use crate::structures::riscv_struct::*;

pub fn optimise_riscv(riscv: &mut RiscV) {
    riscv.elininate_load();
    riscv.eliminate_move();
}