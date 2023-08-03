use crate::structures::riscv_struct::RiscV;
use crate::riscv_gen::LinearScan;

impl RiscV {
        /// 对每个函数进行寄存器分配
        pub fn alloc_regs(&mut self) {
            for func in self.text.funcs.iter_mut() {
                let mut allocator = LinearScan::new();
                allocator.alloc_regs(func);
                //把虚拟寄存器更改为物理寄存器
                func.assign_register(allocator.get_alloc_res());
                // 展开函数调用，使用分配的寄存器
                func.handel_call( allocator.get_alloc_res());
                // 对溢出的寄存器进行重写
                func.rewrite_spilled(allocator.get_spilled());
            }
        }
}