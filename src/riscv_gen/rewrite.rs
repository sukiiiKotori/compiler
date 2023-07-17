use std::collections::{HashSet, HashMap};
use crate::riscv_gen::select::FLOAT_PREFIX;
use crate::riscv_gen::build::PTR_WIDTH;
use crate::riscv_gen::reg::{RegType, get_preserved_regs, phy_is_float};
use crate::structures::riscv_struct::*;
use crate::structures::symbol::SymbolWidth;

impl AsmFunc {
    //将被临时分配到内存中的寄存器替换为对应的内存位置，并插入相关的指令以加载和存储这些寄存器的值。
    //处理寄存器分配过程中的临时变量溢出问题.
    pub fn rewrite_spilled(&mut self, spilled: &HashSet<String>){
        let preserved_regs = get_preserved_regs();

        for block in self.blocks.iter_mut() {
            let len = block.instrs.len();
            for cnt in (0..len).rev() {
                let mut output_map = None;
                let mut inputs_map = Vec::new();
                let mut new_preserved_regs = preserved_regs.clone();
                block.instrs.get_mut(cnt).unwrap().rewrite(
                    |ty| {
                        // 重写操作应该发生在寄存器保存之后，此时函数调用的参数已经进行了赋值
                        ty != AsmInstrType::Jump && ty != AsmInstrType::Call
                    },
                    |output, inputs| {
                    if let Some(output) = output {
                        if spilled.contains(output.as_str()) {
                            let reg_ty = RegType::classify_label(self.label_type.get(output.as_str()).unwrap() == &SymbolWidth::Float, false);
                            let phy_reg = new_preserved_regs.get_mut(&reg_ty).unwrap().last().unwrap();
                            output_map = Some((String::from(output.as_str()), *phy_reg));
                            *output = String::from(*phy_reg);
                        }
                    }
                    for input in inputs.into_iter() {
                        if spilled.contains(input.as_str()) {
                            let reg_ty = RegType::classify_label(self.label_type.get(input.as_str()).unwrap() == &SymbolWidth::Float, false);
                            let phy_reg = new_preserved_regs.get_mut(&reg_ty).unwrap().pop().unwrap();
                            inputs_map.push((String::from(input.as_str()), phy_reg));
                            *input = String::from(phy_reg);
                        }
                    } 
                });
                //若是输入，则溢出到栈里后调用store指令，
                if let Some((virt, phy)) = output_map {
                    let mut prefix = "";
                    if phy_is_float(phy) {
                        prefix = FLOAT_PREFIX;
                    }
                    let spilled_mark = format!("spilled.{}", virt);
                    self.stack.push_normal(spilled_mark.as_str(), 8);
                    block.instrs.insert(cnt+1, AsmInstr::make_instr(AsmInstrType::Store, vec!(phy, "sp", spilled_mark.as_str(), prefix), Some(PTR_WIDTH), vec!()));
                }
                //若是输出，则溢出到栈里调用load
                for (virt, phy) in inputs_map.into_iter() {
                    let mut prefix = "";
                    if phy_is_float(phy) {
                        prefix = FLOAT_PREFIX;
                    }
                    let spilled_mark = format!("spilled.{}", virt);
                    self.stack.push_normal(spilled_mark.as_str(), 8);
                    block.instrs.insert(cnt, AsmInstr::make_instr(AsmInstrType::Load, vec!(phy, "sp", spilled_mark.as_str(), prefix), Some(PTR_WIDTH), vec!()));
                }
            }
        }
    }

    pub fn assign_register(&mut self, virt_to_phy: &HashMap<String, &'static str>) {
        for block in self.blocks.iter_mut() {
            for instr in block.instrs.iter_mut() {
                instr.rewrite(
                    |_| {
                        true
                    },
                    |output, inputs|{
                        if let Some(output) = output {
                            *output = virt_to_phy.get(output).map_or(String::from(output.as_str()), |p| String::from(*p));
                        }
                        for input in inputs.into_iter() {
                            *input = virt_to_phy.get(input).map_or(String::from(input.as_str()), |p| String::from(*p));
                        }
                    });
            }
        } // for
    } // fn
}

impl AsmInstr {
    pub fn rewrite(&mut self, filter_type: impl Fn(AsmInstrType) -> bool, mut map_labels: impl FnMut(Option<&mut String>, Vec<&mut String>)) {
        let ty = self.fetch_type();
        match self {
            AsmInstr::Fmv(bin, _, _) | AsmInstr::Fcvt(bin, _, _) | AsmInstr::Sextw(bin) |
            AsmInstr::Li(bin) | AsmInstr::La(bin) | AsmInstr::Mv(bin) | AsmInstr::Seqz(bin) | AsmInstr::Snez(bin) => {
                match bin {
                    BinInstr{dst, src} => {
                        if filter_type(ty) {
                            map_labels(Some(dst), vec!(src));
                        }
                    }
                }
            },
            AsmInstr::Addi(tri) | AsmInstr::Add(tri) | AsmInstr::Sub(tri) | 
            AsmInstr::Mul(tri) | AsmInstr::Div(tri) | AsmInstr::Rem(tri) |
            AsmInstr::Slli(tri) | AsmInstr::Srli(tri) | AsmInstr::Srai(tri) |
            AsmInstr::Xori(tri) | AsmInstr::Slt(tri) | AsmInstr::Slti(tri) |
            AsmInstr::Flt(tri) | AsmInstr::Fle(tri) | AsmInstr::Feq(tri) |
            AsmInstr::Fadd(tri) | AsmInstr::Fsub(tri) | AsmInstr::Fmul(tri) | AsmInstr::Fdiv(tri) |
            AsmInstr::Sgt(tri) | AsmInstr::Branch(CondTriInstr{cond: _, tri}) => {
                match tri {
                    TriInstr{width: _, dst, op1, op2} => {
                        if filter_type(ty) {
                            map_labels(Some(dst), vec!(op1, op2));
                        }
                    }
                }
            },
            AsmInstr::Store(mem, _) => {
                match mem {
                    MemInstr{width: _, val, base, offset} => {
                        if filter_type(AsmInstrType::Store) {
                            map_labels(None, vec!(val, base, offset));
                        }
                    }
                }
            },
            AsmInstr::Load(mem, _) => {
                match mem {
                    MemInstr{width: _, val, base, offset} => {
                        if filter_type(AsmInstrType::Load) {
                            map_labels(Some(val), vec!(base, offset));
                        }
                    }
                }
            },
            AsmInstr::Ret(ret_val) => {
                if filter_type(AsmInstrType::Ret) {
                    map_labels(None, vec!(ret_val));
                }
            },
            AsmInstr::Jump(dst) => {
                if filter_type(AsmInstrType::Jump) {
                    map_labels(None, vec!(dst));
                }
            },
            AsmInstr::Call(ret_val, _, params, _) => {
                if filter_type(AsmInstrType::Call) {
                    map_labels(Some(ret_val), params.iter_mut().collect());
                }
            },
        }
    }
}

