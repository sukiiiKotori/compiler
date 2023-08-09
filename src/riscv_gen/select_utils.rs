use std::collections::HashSet;
use crate::utils::float::*;
use crate::utils::check::*;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;

/// 向代码块列表中添加一个新的代码块，使用给定的代码块标签和深度
pub fn push_block(block_label: &str, depth: usize, func: &mut AsmFunc) {
    func.push_block(block_label, depth);
}
/// 根据给定的指令类型、字符串向量、宽度数值和类型向量生成一个新的指令<br>
/// 并将其添加到当前代码块的指令列表中
pub fn gen_instr(ty: AsmInstructionType, str_vec: Vec<&str>, width_num: Option<isize>, ty_vec: Vec<SymbolWidth>, func: &mut AsmFunc) {
    let instr = AsmInstruction::make_instr(ty, str_vec, width_num, ty_vec);
    let curr_block = func.blocks.last_mut().unwrap();
    curr_block.instrs.push(instr);
}
/// 获取当前正在处理的函数和代码块，并向当前代码块的后继列表中添加一个新的后继标签
pub fn push_successor(succ: &str, func: &mut AsmFunc) {
    let curr_block = func.blocks.last_mut().unwrap();
    curr_block.successor.push(succ.to_string());
}
/// 标记当前正在处理的函数为调用函数
pub fn mark_call(func: &mut AsmFunc) {
    func.mark_call();
}
/// 向当前正在处理的函数的标签类型映射中插入一个新的标签和类型的对应关系
pub fn insert_label_type(label: &str, width: &SymbolWidth, func: &mut AsmFunc) {
    func.label_type.insert(label.to_string(), width.clone());
}
//将一个浮点立即数load至寄存器中。
//select_cnt：虚拟寄存器编号
//op：浮点立即数(以IEEE754 Double的64bit形式保存)
//返回虚拟寄存器编号，如%2
pub fn load_float_imm(select_cnt: &mut usize, op: &str, func: &mut AsmFunc) -> String {
    let imm = double_to_float(&op);
    let imm_reg = pop_temp_label(select_cnt, &SymbolWidth::I32, func);
    let dst_reg = pop_temp_label(select_cnt, &SymbolWidth::Float, func);
    gen_instr(AsmInstructionType::Li, vec!(&imm_reg, &imm), None, vec![], func);
    gen_instr(AsmInstructionType::Fmv, vec!(&dst_reg, &imm_reg), None, vec!(SymbolWidth::Float, SymbolWidth::I32), func);
    dst_reg
}
//检查操作数是否为浮点立即数
pub fn check_float_op(select_cnt: &mut usize, op: &str, func: &mut AsmFunc) -> String {
    if is_immediate(op) {
        load_float_imm(select_cnt, op, func)
    } else {
        op.to_string()
    }
}

pub fn pop_temp_label(cnt: &mut usize, ty: &SymbolWidth, func: &mut AsmFunc) -> String {
    let res = format!("%temp.{}", cnt);
    insert_label_type(&res, ty, func);
    *cnt += 1;
    res
}

impl AsmFunc {
    /// 添加代码块
    fn push_block(&mut self, block_label: &str, depth: usize) {
        // 如果代码块列表为空，则直接添加一个新的代码块到列表中
        if self.blocks.is_empty() {
            self.blocks.push(AsmBlock::new(block_label, 0, depth));
        } else {
            // 否则，获取最后一个代码块的前驱指令数和自身指令数
            let last_block = self.blocks.last().unwrap();
            let new_pre_instr_cnt = last_block.pre_instr_cnt + last_block.instrs.len();
            // 得到当前块的前驱指令数，push
            self.blocks.push(AsmBlock::new(block_label, new_pre_instr_cnt, depth));
        }
    }
    /// 标记当前正在处理的函数为调用函数
    fn mark_call(&mut self) {
        // 标记寄存器"ra"为已使用
        self.used_saved.insert("ra");
        // 获取当前函数的最后一个代码块
        let last_block = self.blocks.last().unwrap();
        // 计算当前函数的指令总数（前驱指令数 + 当前代码块的指令数）
        let instr_cnt = last_block.pre_instr_cnt + last_block.instrs.len();
        // 将调用函数的信息（指令总数、返回值位置为None、调用的寄存器集合为空）添加到函数的调用信息列表中
        self.call_info.push((instr_cnt, None, HashSet::new()));
    }
}

impl AsmInstruction {
    pub fn make_instr(ty: AsmInstructionType, str_vec: Vec<&str>, width_num: Option<isize>, ty_vec: Vec<SymbolWidth>) -> Self {
        match ty {
            AsmInstructionType::Li => AsmInstruction::Li(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstructionType::La => AsmInstruction::La(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstructionType::Mv => AsmInstruction::Mv(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstructionType::Fmv => AsmInstruction::Fmv(BinInstr::new(str_vec[0], str_vec[1]), ty_vec[0].clone(), ty_vec[1].clone()),
            AsmInstructionType::Addi => AsmInstruction::Addi(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Add => AsmInstruction::Add(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Sub => AsmInstruction::Sub(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Mul => AsmInstruction::Mul(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Div => AsmInstruction::Div(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Rem => AsmInstruction::Rem(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Xori => AsmInstruction::Xori(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Slli => AsmInstruction::Slli(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Srli => AsmInstruction::Srli(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Srai => AsmInstruction::Srai(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Fadd => AsmInstruction::Fadd(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Fsub => AsmInstruction::Fsub(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Fmul => AsmInstruction::Fmul(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Fdiv => AsmInstruction::Fdiv(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Fcvt => AsmInstruction::Fcvt(BinInstr::new(str_vec[0], str_vec[1]), ty_vec[0].clone(), ty_vec[1].clone()),
            AsmInstructionType::Slt => AsmInstruction::Slt(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Slti => AsmInstruction::Slti(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Sgt => AsmInstruction::Sgt(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Seqz => AsmInstruction::Seqz(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstructionType::Snez => AsmInstruction::Snez(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstructionType::Flt => AsmInstruction::Flt(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Fle => AsmInstruction::Fle(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Feq => AsmInstruction::Feq(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstructionType::Store => AsmInstruction::Store(MemInstr::new(width_num.unwrap(), str_vec[0], str_vec[1], str_vec[2]), str_vec.get(3).map_or(String::from(""), |p| String::from(*p))),
            AsmInstructionType::Load => AsmInstruction::Load(MemInstr::new(width_num.unwrap(), str_vec[0], str_vec[1], str_vec[2]), str_vec.get(3).map_or(String::from(""), |p| String::from(*p))),
            AsmInstructionType::Branch => AsmInstruction::Branch(CondTriInstr::new(str_vec[0], None, str_vec[1], str_vec[2], str_vec[3])),
            AsmInstructionType::Jump => AsmInstruction::Jump(String::from(str_vec[0])),
            AsmInstructionType::Ret => AsmInstruction::Ret(),
            AsmInstructionType::Call => {
                AsmInstruction::Call(
                    String::from(str_vec[0]),
                    String::from(str_vec[1]),
                    str_vec.iter().skip(2).map(|s| s.to_string()).collect(),
                    ty_vec
                )
            },
        }
    }
}