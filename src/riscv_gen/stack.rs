use std::collections::{HashMap, HashSet};
use crate::utils::check::*;
use crate::structures::riscv_struct::*;
use crate::riscv_gen::reg::*;

#[derive(Debug)]
/// 在栈上为变量分配的内存空间<br>
/// 栈是一种特定的数据结构，用于存储函数参数和局部变量
pub struct StackSlot {
    /// 最终栈槽的大小<br>
    /// 所有变量和参数在栈上所占用的总空间大小
    pub frame_size: isize,
    /// 最终的映射关系<br>
    /// 将变量名映射到栈槽的位置
    pub map: HashMap<String, isize>,
    /// 已加入到栈槽的变量的集合<br>
    /// 用于确保每个变量只被加入一次
    pub pushed: HashSet<String>,
    /// 存储参数的槽<br>
    /// 每个元组包含参数的变量名和对应的长度
    pub param_slot: Vec<(String, isize)>,
    /// 存储变量和虚拟寄存器的槽<br>
    /// 每个元组包含变量名或者虚拟寄存器的名字以及对应的长度
    pub normal_slot: Vec<(String, isize)>,
}

impl StackSlot {
    /// 创建一个新的StackSlot实例
    pub fn new() -> Self {
        StackSlot {
            frame_size: 0,
            map: HashMap::new(),
            pushed: HashSet::new(),
            param_slot: Vec::new(),
            normal_slot: Vec::new(),
        }
    }
    /// 将参数变量加入到栈槽中
    pub fn push_param(&mut self, label: &str, len: isize) {
        // 首先判断是否已被加入
        if !self.pushed.contains(label) {
            // 将变量名加入到已加入栈槽的变量名的HashSet中
            self.pushed.insert(String::from(label));
            // 将变量名和长度的元组加入到参数的槽中
            self.param_slot.push((String::from(label), len));
        }
    }
    /// 将普通变量加入到栈槽中
    pub fn push_normal(&mut self, label: &str, len: isize) {
        if !self.pushed.contains(label) {
            self.pushed.insert(String::from(label));
            self.normal_slot.push((String::from(label), len));
        }
    }
    /// 确定化栈槽的最终大小和每个变量在栈槽中的位置
    pub fn deterministic(&mut self) {
        // 计算栈槽的最终大小
        self.frame_size = self.normal_slot.iter()
            // 将param_slot字段中的元组追加到遍历中
            .chain(self.param_slot.iter()) 
            // 初始值为0，累加每个元组的第二个元素(len)到累加器(acc)，得到最终的栈槽大小
            .fold(0, |acc, (_, len)| acc + len); 
        // 初始位置为零，先插入param_slot
        let this_pos = self.insert_map(self.param_slot.clone(), 0);
        // 再插入normal_slot
        self.insert_map(self.normal_slot.clone(), this_pos);
    }
    /// 栈插槽插入到map最终映射关系中
    pub fn insert_map(&mut self, slot: Vec<(String, isize)>, mut this_pos: isize) -> isize {
        for (label, len) in slot.iter() {
            this_pos += len;  // 更新当前变量的栈槽位置
            self.map.insert(String::from(label), self.frame_size-this_pos);
        }
        this_pos
    }
    /// 获取变量在栈槽中的位置
    pub fn get_pos(&self, label: &str) -> isize {
        // 判断变量名是否为十进制数值
        if is_decimal(label) {
            // 如果是十进制数值，则将其解析为isize类型的值并返回
            label.parse().unwrap()
        } else {
            // 如果不是十进制数值，则通过变量名在映射关系中查找对应的栈槽位置，并返回该位置的值
            *self.map.get(label).expect(&format!("{} is not inside stack", label))
        }
    }
}

impl AsmFunc {
    pub fn deterministic_stack(&mut self) {
        self.stack.deterministic();
    }

    pub fn stack_alloc_free(&mut self) {
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
    
    pub fn map_addr(&mut self) {
        

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

