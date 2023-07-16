use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;

impl RiscV {
    pub fn eliminate_move(&mut self) {
        self.text.funcs.iter_mut().for_each(|func| func.eliminate_move());
    }

    pub fn elininate_load(&mut self) {
        self.text.funcs.iter_mut().for_each(|func| func.eliminate_load());
    }
}

impl AsmFunc {
    pub fn eliminate_move(&mut self) {
        self.blocks.iter_mut().for_each(|block| {//遍历每个函数内部的基本块
            block.instrs.retain(|instr| {//传入一个闭包，当mv或fmv的源和目的寄存器相同时消除
                match instr {
                    AsmInstr::Fmv(bin, _ , _) | AsmInstr::Mv(bin) => bin.dst != bin.src,
                    _ => true
                }
            });
        });
    }

    pub fn eliminate_load(&mut self) {
        self.blocks.iter_mut().for_each(|block| {
            let mut replace_idx = Vec::new();
            block.instrs.iter().enumerate().for_each(|(idx, instr)| {
                if idx < block.instrs.len() - 1 { //如果不是最后一个元素
                    if let AsmInstr::Store(mem_s, perfix_s) = instr {//如果当前指令是store指令，判断其下一条指令
                        if let AsmInstr::Load(mem_l, perfix_l) = &block.instrs[idx + 1] {//如果下一条指令是load
                            if mem_s.base == mem_l.base && mem_s.offset == mem_l.offset && perfix_s == perfix_l { //如果偏移量和基地址相同，可以替换为mv
                                if perfix_s.is_empty() {//都是i32的store, load
                                    replace_idx.push((true, idx + 1, mem_l.val.clone(), mem_s.val.clone()));//is_i32,id,dst,src
                                } else {
                                    replace_idx.push((false, idx + 1, mem_l.val.clone(), mem_s.val.clone()));
                                }
                            }
                        }
                    }
                }
            });
            replace_idx.into_iter().rev().for_each(|(is_i32, idx, dst, src)| {
                block.instrs.remove(idx);
                if is_i32 {
                    block.instrs.insert(idx, AsmInstr::Mv(BinInstr::new(&dst, &src)))
                } else {
                    block.instrs.insert(idx, AsmInstr::Fmv(BinInstr::new(&dst, &src), SymbolWidth::Float, SymbolWidth::Float))
                }
            })
        });
    }
}

