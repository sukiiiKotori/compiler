use std::collections::HashSet;
use crate::structures::riscv_struct::*;
use crate::structures::symbol::*;

impl RiscV {
    /// 向text段的函数列表中添加一个新的函数,使用给定的函数标签和函数类型
    pub fn push_func(&mut self, func_label: &str, func_type: SymbolWidth) {
        self.text.funcs.push(AsmFunc::new(func_label, func_type));
    }
    /// 向代码块列表中添加一个新的代码块，使用给定的代码块标签和深度
    pub fn push_block(&mut self, block_label: &str, depth: usize) {
        let curr_func = self.text.funcs.last_mut().unwrap();
        curr_func.push_block(block_label, depth);
    }
    /// 获取当前正在处理的函数和代码块，并向当前代码块的指令列表中添加一个新的指令
    pub fn push_instr(&mut self, instr: AsmInstr) {
        let curr_func = self.text.funcs.last_mut().unwrap();
        let curr_block = curr_func.blocks.last_mut().unwrap();
        curr_block.instrs.push(instr);
    }
    /// 根据给定的指令类型、字符串向量、宽度数值和类型向量生成一个新的指令<br>
    /// 并将其添加到当前代码块的指令列表中
    pub fn gen_instr(&mut self, ty: AsmInstrType, str_vec: Vec<&str>, width_num: Option<isize>, ty_vec: Vec<SymbolWidth>) {
        let instr = AsmInstr::make_instr(ty, str_vec, width_num, ty_vec);
        self.push_instr(instr);
    }
    /// 获取当前正在处理的函数和代码块，并向当前代码块的后继列表中添加一个新的后继标签
    pub fn push_successor(&mut self, succ: &str) {
        let curr_func = self.text.funcs.last_mut().unwrap();
        let curr_block = curr_func.blocks.last_mut().unwrap();
        curr_block.successor.push(succ.to_string());
    }
    /// 标记当前正在处理的函数为调用函数
    pub fn mark_call(&mut self) {
        self.text.funcs.last_mut().unwrap().mark_call();
    }
    /// 向当前正在处理的函数的标签类型映射中插入一个新的标签和类型的对应关系
    pub fn insert_label_type(&mut self, label: &str, width: &SymbolWidth) {
        self.text.funcs.last_mut().unwrap().label_type.insert(label.to_string(), width.clone());
    }
} // imp

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

impl AsmInstr {
    pub fn make_instr(ty: AsmInstrType, str_vec: Vec<&str>, width_num: Option<isize>, ty_vec: Vec<SymbolWidth>) -> Self {
        match ty {
            AsmInstrType::Li => AsmInstr::Li(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstrType::La => AsmInstr::La(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstrType::Mv => AsmInstr::Mv(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstrType::Fmv => AsmInstr::Fmv(BinInstr::new(str_vec[0], str_vec[1]), ty_vec[0].clone(), ty_vec[1].clone()),
            AsmInstrType::Sextw => AsmInstr::Sextw(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstrType::Addi => AsmInstr::Addi(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Add => AsmInstr::Add(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Sub => AsmInstr::Sub(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Mul => AsmInstr::Mul(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Div => AsmInstr::Div(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Rem => AsmInstr::Rem(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Xori => AsmInstr::Xori(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Slli => AsmInstr::Slli(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Srli => AsmInstr::Srli(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Srai => AsmInstr::Srai(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Fadd => AsmInstr::Fadd(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Fsub => AsmInstr::Fsub(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Fmul => AsmInstr::Fmul(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Fdiv => AsmInstr::Fdiv(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Fcvt => AsmInstr::Fcvt(BinInstr::new(str_vec[0], str_vec[1]), ty_vec[0].clone(), ty_vec[1].clone()),
            AsmInstrType::Slt => AsmInstr::Slt(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Slti => AsmInstr::Slti(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Sgt => AsmInstr::Sgt(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Seqz => AsmInstr::Seqz(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstrType::Snez => AsmInstr::Snez(BinInstr::new(str_vec[0], str_vec[1])),
            AsmInstrType::Flt => AsmInstr::Flt(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Fle => AsmInstr::Fle(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Feq => AsmInstr::Feq(TriInstr::new(width_num, str_vec[0], str_vec[1], str_vec[2])),
            AsmInstrType::Store => AsmInstr::Store(MemInstr::new(width_num.unwrap(), str_vec[0], str_vec[1], str_vec[2]), str_vec.get(3).map_or(String::from(""), |p| String::from(*p))),
            AsmInstrType::Load => AsmInstr::Load(MemInstr::new(width_num.unwrap(), str_vec[0], str_vec[1], str_vec[2]), str_vec.get(3).map_or(String::from(""), |p| String::from(*p))),
            AsmInstrType::Branch => AsmInstr::Branch(CondTriInstr::new(str_vec[0], None, str_vec[1], str_vec[2], str_vec[3])),
            AsmInstrType::Jump => AsmInstr::Jump(String::from(str_vec[0])),
            AsmInstrType::Ret => AsmInstr::Ret(),
            AsmInstrType::Call => {
                AsmInstr::Call(
                    String::from(str_vec[0]),
                    String::from(str_vec[1]),
                    str_vec.iter().skip(2).map(|s| s.to_string()).collect(),
                    ty_vec
                )
            },
        }
    }
}