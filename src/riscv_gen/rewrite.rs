use std::collections::{HashSet, HashMap};
use crate::riscv_gen::register_type::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::SymbolWidth;
use crate::structures::riscv_struct::PTR_WIDTH;

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
                        ty != AsmInstructionType::Jump && ty != AsmInstructionType::Call
                    },
                    |output, inputs| {
                    if let Some(output) = output {
                        if spilled.contains(output.as_str()) {
                            let reg_ty = RegType::get_regtype(self.label_type.get(output.as_str()).unwrap() == &SymbolWidth::Float, false);
                            let phy_reg = new_preserved_regs.get_mut(&reg_ty).unwrap().last().unwrap();
                            output_map = Some((String::from(output.as_str()), *phy_reg));
                            *output = String::from(*phy_reg);
                        }
                    }
                    for input in inputs.into_iter() {
                        if spilled.contains(input.as_str()) {
                            let reg_ty = RegType::get_regtype(self.label_type.get(input.as_str()).unwrap() == &SymbolWidth::Float, false);
                            let phy_reg = new_preserved_regs.get_mut(&reg_ty).unwrap().pop().unwrap();
                            inputs_map.push((String::from(input.as_str()), phy_reg));
                            *input = String::from(phy_reg);
                        }
                    } 
                });
                //若是输入，则溢出到栈里后调用store指令，
                if let Some((virt, phy)) = output_map {
                    let prefix = if phy.contains("f") {
                        "f"
                    } else {
                        ""
                    };
                    let spilled_mark = format!("spilled.{}", virt);
                    self.stack.push_normal(spilled_mark.as_str(), 8);
                    block.instrs.insert(cnt+1, AsmInstruction::make_instr(AsmInstructionType::Store, vec!(phy, "sp", spilled_mark.as_str(), prefix), Some(PTR_WIDTH), vec!()));
                }
                //若是输出，则溢出到栈里调用load
                for (virt, phy) in inputs_map.into_iter() {
                    let prefix = if phy.contains("f") {
                        "f"
                    } else {
                        ""
                    };
                    let spilled_mark = format!("spilled.{}", virt);
                    self.stack.push_normal(spilled_mark.as_str(), 8);
                    block.instrs.insert(cnt, AsmInstruction::make_instr(AsmInstructionType::Load, vec!(phy, "sp", spilled_mark.as_str(), prefix), Some(PTR_WIDTH), vec!()));
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

impl AsmInstruction {
    pub fn rewrite(&mut self, filter_type: impl Fn(AsmInstructionType) -> bool, mut map_labels: impl FnMut(Option<&mut String>, Vec<&mut String>)) {
        let ty = self.fetch_type();
        match self {
            AsmInstruction::Fmv(bin, _, _) | AsmInstruction::Fcvt(bin, _, _) | 
            AsmInstruction::Li(bin) | AsmInstruction::La(bin) | AsmInstruction::Mv(bin) | AsmInstruction::Seqz(bin) | AsmInstruction::Snez(bin) => {
                match bin {
                    BinInstr{dst, src} => {
                        if filter_type(ty) {
                            map_labels(Some(dst), vec!(src));
                        }
                    }
                }
            },
            AsmInstruction::Addi(tri) | AsmInstruction::Add(tri) | AsmInstruction::Sub(tri) | 
            AsmInstruction::Mul(tri) | AsmInstruction::Div(tri) | AsmInstruction::Rem(tri) |
            AsmInstruction::Slli(tri) | AsmInstruction::Srli(tri) | AsmInstruction::Srai(tri) |
            AsmInstruction::Xori(tri) | AsmInstruction::Slt(tri) | AsmInstruction::Slti(tri) |
            AsmInstruction::Flt(tri) | AsmInstruction::Fle(tri) | AsmInstruction::Feq(tri) |
            AsmInstruction::Fadd(tri) | AsmInstruction::Fsub(tri) | AsmInstruction::Fmul(tri) | AsmInstruction::Fdiv(tri) |
            AsmInstruction::Sgt(tri) | AsmInstruction::Branch(CondTriInstr{cond: _, tri}) => {
                match tri {
                    TriInstr{width: _, dst, op1, op2} => {
                        if filter_type(ty) {
                            map_labels(Some(dst), vec!(op1, op2));
                        }
                    }
                }
            },
            AsmInstruction::Store(mem, _) => {
                match mem {
                    MemInstr{width: _, val, base, offset} => {
                        if filter_type(AsmInstructionType::Store) {
                            map_labels(None, vec!(val, base, offset));
                        }
                    }
                }
            },
            AsmInstruction::Load(mem, _) => {
                match mem {
                    MemInstr{width: _, val, base, offset} => {
                        if filter_type(AsmInstructionType::Load) {
                            map_labels(Some(val), vec!(base, offset));
                        }
                    }
                }
            },
            AsmInstruction::Jump(dst) => {
                if filter_type(AsmInstructionType::Jump) {
                    map_labels(None, vec!(dst));
                }
            },
            AsmInstruction::Call(ret_val, _, params, _) => {
                if filter_type(AsmInstructionType::Call) {
                    map_labels(Some(ret_val), params.iter_mut().collect());
                }
            },
            _ => {}
        }
    }
}

