use std::collections::{HashMap, HashSet};
use crate::utils::check::is_decimal;
use crate::structures::riscv_struct::*;
use crate::riscv_gen::reg::{
    PRESERVED,
    TEMPORARY,
    FUNC_ARG,
};

#[derive(Debug)]
pub struct StackSlot {
    pub frame_size: isize,                // 最终栈槽的大小
    pub map: HashMap<String, isize>,      // 最终的映射关系
    pub pushed: HashSet<String>,          // 已加入到栈槽的变量
    pub param_slot: Vec<(String, isize)>, // 存储参数的槽
    pub normal_slot: Vec<(String, isize)>,// 存储变量和虚拟寄存器的槽
}

impl StackSlot {
    pub fn new() -> Self {
        StackSlot {
            frame_size: 0,
            map: HashMap::new(),
            pushed: HashSet::new(),
            param_slot: Vec::new(),
            normal_slot: Vec::new(),
        }
    }

    pub fn push_param(&mut self, label: &str, len: isize) {
        if !self.pushed.contains(label) {
            self.pushed.insert(String::from(label));
            self.param_slot.push((String::from(label), len));
        }
    }

    pub fn push_normal(&mut self, label: &str, len: isize) {
        if !self.pushed.contains(label) {
            self.pushed.insert(String::from(label));
            self.normal_slot.push((String::from(label), len));
        }
    }

    pub fn deterministic(&mut self) {
        self.frame_size = self.normal_slot.iter().chain(self.param_slot.iter()).fold(0, |acc, (_, len)| acc + len);
        let mut this_pos = 0;
        for (label, len) in self.param_slot.iter() {
            this_pos += len;
            self.map.insert(String::from(label), self.frame_size-this_pos);
        }
        for (label, len) in self.normal_slot.iter() {
            this_pos += len;
            self.map.insert(String::from(label), self.frame_size-this_pos);
        }
    }

    pub fn get_pos(&self, label: &str) -> isize {
        if is_decimal(label) {
            label.parse().unwrap()
        } else {
            *self.map.get(label).expect(&format!("{} is not inside stack", label))
        }
    }
}

impl RiscV {
    pub fn deterministic_stack(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.deterministic_stack();
        }
    }

    pub fn stack_alloc_free(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.stack_alloc_free();
        }
    }

    pub fn map_addr(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.map_addr();
        }
    }
}

impl AsmFunc {
    fn deterministic_stack(&mut self) {
        self.stack.deterministic();
    }

    fn stack_alloc_free(&mut self) {
        let free_size = self.stack.frame_size.to_string();
        let alloc_size = format!("-{}", free_size);

        for (idx, block) in self.blocks.iter_mut().enumerate() {
            if idx == 0 && self.stack.frame_size > 0 {
                if self.stack.frame_size < 2040 {
                    block.instrs.insert(0, AsmInstr::make_instr(AsmInstrType::Addi, vec!("sp", "sp", alloc_size.as_str()), vec!(), vec!()));
	            } else {
	                block.instrs.insert(0, AsmInstr::make_instr(AsmInstrType::Add, vec!("sp", "sp", TEMPORARY[0]), vec!(), vec!(),));
	                block.instrs.insert(0, AsmInstr::make_instr(AsmInstrType::Li, vec!(TEMPORARY[0], alloc_size.as_str()), vec!(), vec!(),));
	            }
            }
            let last_instr = block.instrs.last();
            if let Some(AsmInstr::Ret(ret_val)) = last_instr {
                if self.stack.frame_size > 0 {
                    let before_last = block.instrs.len()-1;
                    if self.stack.frame_size < 2040 {
                        block.instrs.insert(before_last, AsmInstr::make_instr(AsmInstrType::Addi, vec!("sp", "sp", free_size.as_str()), vec!(), vec!()));
	                } else {
                        if ret_val == FUNC_ARG[0] {
	                        block.instrs.insert(before_last, AsmInstr::make_instr(AsmInstrType::Add, vec!("sp", "sp", TEMPORARY[0]), vec!(), vec!()));
	                        block.instrs.insert(before_last, AsmInstr::make_instr(AsmInstrType::Li, vec!(TEMPORARY[0], free_size.as_str()), vec!(), vec!()));
                        } else {
	                        block.instrs.insert(before_last, AsmInstr::make_instr(AsmInstrType::Add, vec!("sp", "sp", FUNC_ARG[0]), vec!(), vec!()));
	                        block.instrs.insert(before_last, AsmInstr::make_instr(AsmInstrType::Li, vec!(FUNC_ARG[0], free_size.as_str()), vec!(), vec!()));
                        }
	                }
                }
            } // if let AsmInstr::Ret
        } // for block in
    } // fn
    
    fn map_addr(&mut self) {
        use crate::utils::check::inside_imm_range;

        for block in self.blocks.iter_mut() {
            let len = block.instrs.len();
            for cnt in (0..len).rev() {
                let mut li_flag = false;
                let mut conver_flag = false;
                let mut stack_pos = String::from("");
                let mut instr_dst = None;
                let mut instr_base = None;
                let mut instr_width = None;

                let mut preserved_reg = "";
                match block.instrs.get_mut(cnt).unwrap() {
                    AsmInstr::Store(MemInstr{width, val, base, offset}, _) | AsmInstr::Load(MemInstr{width, val, base, offset}, _) => {
                        preserved_reg = PRESERVED.iter().map(|r| *r).find(|r| *r != val.as_str()).unwrap();
                        if base == "sp" {
                            stack_pos = self.stack.get_pos(offset.as_str()).to_string();
                            if inside_imm_range(stack_pos.as_str()) {
                                *offset = String::from(&stack_pos);
                            } else {
                                instr_base = Some(String::from(base.as_str()));
                                instr_width = Some(width.to_owned());
                                *base = String::from(preserved_reg);
                                *offset = String::from("0");
                                li_flag = true;
                            }
                        }
                    },
                    AsmInstr::Addi(TriInstr{width: _, dst, op1, op2}) => {
                        preserved_reg = PRESERVED.iter().map(|r| *r).find(|r| *r != dst.as_str()).unwrap();
                        if op1 == "sp" && &op2[0..1] == "#" {
                            stack_pos = self.stack.get_pos(&op2[1..]).to_string();
                            if inside_imm_range(stack_pos.as_str()) {
                                *op2 = String::from(&stack_pos);
                            } else {
                                instr_dst = Some(String::from(dst.as_str()));
                                conver_flag = true;
                                li_flag = true;
                            }
                        }
                    }
                    _ => {},
                }
                if conver_flag {
                    block.instrs.insert(cnt, AsmInstr::make_instr(AsmInstrType::Add, vec!(instr_dst.unwrap().as_str(), "sp", preserved_reg), vec!(), vec!()));
                    block.instrs.insert(cnt, AsmInstr::make_instr(AsmInstrType::Li, vec!(preserved_reg, stack_pos.as_str()), vec!(), vec!()));
                    block.instrs.remove(cnt+2);
                } else {
                    if li_flag {
                        block.instrs.insert(cnt, AsmInstr::make_instr(AsmInstrType::Add, vec!(preserved_reg, preserved_reg, instr_base.unwrap().as_str()), vec!(instr_width.unwrap()), vec!()));
                        block.instrs.insert(cnt, AsmInstr::make_instr(AsmInstrType::Li, vec!(preserved_reg, stack_pos.as_str()), vec!(), vec!()));
                    }
                } // if
            } // for cnt
        } // for block
    } // fn
} // impl

