use std::collections::{HashSet, HashMap, BTreeSet};
use crate::utils::check::*;
use crate::utils::float::*;
use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;
use crate::structures::symbol::*;
use crate::riscv_gen::stack_slot::StackSlot;
use crate::riscv_gen::asm_select::FLOAT_PREFIX;
use crate::riscv_gen::reg::RegisterAllocator;


pub const NORMAL_WIDTH: isize = 4;
pub const PTR_WIDTH: isize = 8;

impl RiscV {
    /// 向text段的函数列表中添加一个新的函数,使用给定的函数标签和函数类型
    pub fn push_func(&mut self, func_label: &str, func_type: SymbolWidth) {
        self.text.funcs.push(AsmFunc::new(func_label, func_type));
    }
    /// 向代码块列表中添加一个新的代码块，使用给定的代码块标签和深度
    pub fn push_block(&mut self, block_label: &str, depth: usize) {
        let curr_func = self.text.curr_func();
        curr_func.push_block(block_label, depth);
    }
    /// 获取当前正在处理的函数和代码块，并向当前代码块的指令列表中添加一个新的指令
    pub fn push_instr(&mut self, instr: AsmInstr) {
        let curr_func = self.text.curr_func();
        let curr_block = curr_func.curr_block();
        curr_block.push_instr(instr);
    }
    /// 根据给定的指令类型、字符串向量、宽度数值和类型向量生成一个新的指令<br>
    /// 并将其添加到当前代码块的指令列表中
    pub fn gen_instr(&mut self, ty: AsmInstrType, str_vec: Vec<&str>, width_num: Option<isize>, ty_vec: Vec<SymbolWidth>) {
        let instr = AsmInstr::make_instr(ty, str_vec, width_num, ty_vec);
        self.push_instr(instr);
    }
    /// 获取当前正在处理的函数和代码块，并向当前代码块的后继列表中添加一个新的后继标签
    pub fn push_successor(&mut self, succ: &str) {
        let curr_func = self.text.curr_func();
        let curr_block = curr_func.curr_block();
        curr_block.push_successor(succ);
    }
    /// 标记当前正在处理的函数为调用函数
    pub fn mark_call(&mut self) {
        self.text.funcs.last_mut().unwrap().mark_call();
    }
    /// 向当前正在处理的函数的标签类型映射中插入一个新的标签和类型的对应关系
    pub fn insert_label_type(&mut self, label: &str, width: &SymbolWidth) {
        self.text.funcs.last_mut().unwrap().label_type.insert(label.to_string(), width.clone());
    }
    /// 对每个函数进行寄存器分配
    pub fn alloc_regs<Alloc: RegisterAllocator>(&mut self) {
        for func in self.text.funcs.iter_mut() {
            let mut allocator = Alloc::new();
            allocator.alloc_regs(func);
            //把虚拟寄存器更改为物理寄存器
            func.assign_register(allocator.get_alloc_res());
            // 展开函数调用，使用分配的寄存器
            func.unfold_call( allocator.get_alloc_res());
            // 对溢出的寄存器进行重写
            func.rewrite_spilled(allocator.get_spilled());
        }
    }
} // imp

impl TextSection {
    /// 获取当前函数
    pub fn curr_func(&mut self) -> &mut AsmFunc {
        self.funcs.last_mut().unwrap()
    }
}

impl AsmFunc {
    /// 添加代码块
    pub fn push_block(&mut self, block_label: &str, depth: usize) {
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
    /// 返回当前正在处理的函数的最后一个代码块的可变引用
    pub fn curr_block(&mut self) -> &mut AsmBlock{
        self.blocks.last_mut().unwrap()
    }
    /// 标记当前正在处理的函数为调用函数
    fn mark_call(&mut self) {
        // 标记寄存器"ra"为已使用
        self.used_saved("ra");
        // 获取当前函数的最后一个代码块
        let last_block = self.blocks.last().unwrap();
        // 计算当前函数的指令总数（前驱指令数 + 当前代码块的指令数）
        let instr_cnt = last_block.pre_instr_cnt + last_block.instrs.len();
        // 将调用函数的信息（指令总数、返回值位置为None、调用的寄存器集合为空）添加到函数的调用信息列表中
        self.call_info.push((instr_cnt, None, HashSet::new()));
    }
    /// 展开调用信息
    pub fn unfold_call(&mut self, alloc_res: &HashMap<String, &'static str>) {
        let mut call_info_ref = self.call_info.iter().collect::<Vec<_>>();
        // 对当前函数中的代码块从后往前遍历
        for block in self.blocks.iter_mut().rev() {
            loop {
                // 如果还存在待展开的函数调用信息
                if let Some(this_call_info) = call_info_ref.last() {
                    let this_idx = this_call_info.0;
                    // 如果待展开的函数调用信息的索引大于等于当前代码块的前驱指令数
                    if this_idx >= block.pre_instr_cnt {
                        let position = this_idx - block.pre_instr_cnt;
                        // 在当前代码块中展开函数调用
                        block.unfold_call(&mut self.stack, this_call_info, alloc_res,  position);
                        call_info_ref.pop();
                    } else {
                        break;
                    }
                } else {
                    // 不存在待展开的调用信息，返回
                    return;
                }
            }
        }
    }
}
/// 用于展开函数调用的上下文结构体
pub struct UnfoldCallContext<'a> {
    /// 整型计数器
    pub int_cnt: usize,
    /// 浮点型计数器
    pub float_cnt: usize,
    /// 栈长度
    pub stack_len: isize,
    /// 栈槽的可变引用
    pub stack: &'a mut StackSlot,
    /// 无效寄存器的可变引用
    pub invalid_regs: &'a mut HashSet<&'static str>,
    /// 被存储的寄存器的可变引用
    pub stored_regs: &'a mut BTreeSet<&'a str>,
}

impl<'a> UnfoldCallContext<'a> {
    pub fn new(
        int_cnt: usize,
        float_cnt: usize,
        stack_len: isize,
        stack: &'a mut StackSlot,
        invalid_regs: &'a mut HashSet<&'static str>,
        stored_regs: &'a mut BTreeSet<&'a str>,
    ) -> Self {
        UnfoldCallContext{int_cnt, float_cnt, stack_len, stack, invalid_regs, stored_regs}
    }
}

impl AsmBlock {
    /// 将指令添加到代码块的指令列表中
    pub fn push_instr(&mut self, instr: AsmInstr) {
        self.instrs.push(instr)
    }
    /// 将后继代码块的标签添加到代码块的后继列表中
    fn push_successor(&mut self, succ: &str) {
        self.successor.push(String::from(succ));
    }

    pub fn load_float_param<'a>(
        &mut self, 
        param: &'a String, 
        position: usize, 
        context: &mut UnfoldCallContext<'a>,
    ) {
        // 检查浮点数计数器是否超过了浮点型函数参数寄存器的数量
        if context.float_cnt >= FLOAT_FUNC_ARG.len() {
            // 如果超过了数量，则需要将参数存储在栈上
            context.stack_len += 4;
            let stack_pos = format!("-{}", context.stack_len);
            // 如果参数是一个立即数
            if is_immediate(param.as_str()) {
                let imm = double_to_float(param.as_str());
                // 在指定位置插入存储立即数的指令
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Store,
                    vec!(PRESERVED[1], "sp",stack_pos.as_str(), ),
                    Some(NORMAL_WIDTH),
                    vec!()
                ));
                // 在指定位置插入加载立即数的指令
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Li,
                    vec!(PRESERVED[1], imm.as_str()),
                    None,
                    vec!()
                ));
            } else if context.invalid_regs.contains(param.as_str()) {
                // 如果参数是一个无效的寄存器，则抛出错误
                panic!("riscv_gen/build.rs:load_float_param,invalid regs");
            } else {
                // 正常参数，在指定位置插入存储参数的指令
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Store,
                    vec!(param.as_str(), "sp", stack_pos.as_str(), FLOAT_PREFIX),
                    Some(NORMAL_WIDTH),
                    vec!()
                ));
            }
        } else {
            // 如果浮点型计数器未超过浮点型函数参数的数量
            context.invalid_regs.remove(FLOAT_FUNC_ARG[context.float_cnt]);
            if is_immediate(param.as_str()) {
                let imm = double_to_float(param.as_str());
                self.instrs.insert(
                    position, 
                    AsmInstr::make_instr(
                        AsmInstrType::Fmv, 
                        vec!(FLOAT_FUNC_ARG[context.float_cnt], PRESERVED[1]), 
                        None, 
                        vec!(SymbolWidth::Float, SymbolWidth::I32)
                    )
                );
                self.instrs.insert(
                    position, 
                    AsmInstr::make_instr(
                        AsmInstrType::Li, 
                        vec!(PRESERVED[1], &imm), 
                        None, 
                        vec!()
                    )
                );
            } else if context.invalid_regs.contains(param.as_str()) {
                let stored_pos = format!("stored.{}", param);
                context.stored_regs.insert(param.as_str());
                context.stack.push_normal(stored_pos.as_str(), 8);
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Load,
                    vec!(FLOAT_FUNC_ARG[context.float_cnt], "sp", stored_pos.as_str(), FLOAT_PREFIX),
                    Some(NORMAL_WIDTH),
                    vec!()
                ));
            } else {
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Fmv,
                    vec!(FLOAT_FUNC_ARG[context.float_cnt], param.as_str()),
                    Some(NORMAL_WIDTH),
                    vec!(SymbolWidth::Float, SymbolWidth::Float)
                ));
            }
        }
        context.float_cnt += 1;
    }

    pub fn load_int_param<'a>(
        &mut self, 
        param: &'a String, 
        position: usize, 
        param_size: isize, 
        context: &mut UnfoldCallContext<'a>,
    ) {
        if context.int_cnt >= FUNC_ARG.len() {
            context.stack_len += param_size;
            let stack_pos = format!("-{}", context.stack_len);
            if is_immediate(param.as_str()) {
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Store,
                    vec!(PRESERVED[1], "sp", stack_pos.as_str()),
                    Some(param_size),
                    vec!()
                ));
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Li,
                    vec!(PRESERVED[1], param.as_str()),
                    None,
                    vec!()
                ));
            } else if context.invalid_regs.contains(param.as_str()) {
                panic!("Should not appear");
            } else {
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Store,
                    vec!(param.as_str(), "sp", stack_pos.as_str()),
                    Some(param_size),
                    vec!()
                ));
            }
        } else {
            context.invalid_regs.remove(FUNC_ARG[context.int_cnt]);
            if is_immediate(param.as_str()) {
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Li,
                    vec!(FUNC_ARG[context.int_cnt], param.as_str()),
                    Some(param_size),
                    vec!()
                ));
            } else if context.invalid_regs.contains(param.as_str()) {
                let stored_pos = format!("stored.{}", param);
                context.stored_regs.insert(param.as_str());
                context.stack.push_normal(stored_pos.as_str(), 8);
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Load,
                    vec!(FUNC_ARG[context.int_cnt], stored_pos.as_str()),
                    Some(PTR_WIDTH),
                    vec!()
                ));
            } else {
                self.instrs.insert(position, AsmInstr::make_instr(
                    AsmInstrType::Mv,
                    vec!(FUNC_ARG[context.int_cnt], param.as_str()),
                    None,
                    vec!()
                ));
            }
        }
        context.int_cnt += 1;
    }
    /// 对调用指令进行展开<br>
    /// 处理函数调用的参数传递和返回值处理。
    fn unfold_call(
        &mut self,
        stack: &mut StackSlot,
        this_call_info: &(usize, Option<usize>, HashSet<String>),
        alloc_res: &HashMap<String, &'static str>,
        position: usize
    ) {
        let (ret_val, params, types) = match self.instrs.get(position).unwrap() {
            AsmInstr::Call(r, name, p, t) => {
                if name == "memset" {
                    return;
                }
                (
                String::from(r),
                p.iter().map(|p| String::from(p)).collect::<Vec<_>>(),
                t.iter().cloned().collect::<Vec<_>>()
                )
            },
            _ => panic!("Position error"),
        };
    
        // 获取穿过当前Call指令的Temporary寄存器
        let mut stored_regs = this_call_info.2.iter()
            .filter_map(|r| alloc_res.get(r)
                .filter(|phy| TEMP_SET.contains(*phy) || FLOAT_TEMP_SET.contains(*phy))
            )
            .cloned()
            .collect::<BTreeSet<_>>();
    
        // 恢复穿过当前Call指令的Temporary寄存器
        for temp in stored_regs.iter() {
            let stored_pos = format!("stored.{}", temp);
            stack.push_normal(stored_pos.as_str(), 8);
            let mut prefix = "";
            if FLOAT_TEMP_SET.contains(temp) {
                prefix = FLOAT_PREFIX;
            }
            self.instrs.insert(position+1, AsmInstr::make_instr(
                AsmInstrType::Load,
                vec!(temp, "sp", stored_pos.as_str(), prefix),
                Some(PTR_WIDTH),
                vec!()
            ));
        }
    
        // 存储调用返回值
        if !ret_val.is_empty() {
            if types[0] == SymbolWidth::Float {
                self.instrs.insert(position+1, AsmInstr::make_instr(
                    AsmInstrType::Fmv,
                    vec!(ret_val.as_str(), "fa0"),
                    None,
                    vec!(SymbolWidth::Float, SymbolWidth::Float)
                ));
            } else {
                self.instrs.insert(position+1, AsmInstr::make_instr(
                    AsmInstrType::Mv,
                    vec!(ret_val.as_str(), "a0"),
                    None,
                    vec!()
                ));
            }
        }
    
        // 获取存储参数的寄存器，并把它加入invalid_regs的集合中
        let mut int_pos = 0;
        let mut float_pos = 0;
        let mut invalid_regs = HashSet::new();
        for ty in types.iter().skip(1) {
            if *ty == SymbolWidth::Float {
                if float_pos < FLOAT_FUNC_ARG.len() {
                    invalid_regs.insert(FLOAT_FUNC_ARG[float_pos]);
                }
                float_pos += 1;
            } else {
                if int_pos < FUNC_ARG.len() {
                    invalid_regs.insert(FUNC_ARG[int_pos]);
                }
                int_pos += 1;
            }
            if int_pos >= FUNC_ARG.len() && float_pos >= FLOAT_FUNC_ARG.len() {
                break;
            }
        }

        // 将参数值装载到指定寄存器或者栈槽位置
        let mut context = UnfoldCallContext::new(
            0,
            0,
            0,
            stack,
            &mut invalid_regs,
            &mut stored_regs
        );
        for (_, (param, ty)) in params.iter().zip(types.iter().skip(1)).enumerate() {
            if *ty == SymbolWidth::Float {
                self.load_float_param(param, position, &mut context);
            } else if let SymbolWidth::Arr{tar: _, dims: _} = ty {
                self.load_int_param(param, position, 8, &mut context);
            } else {
                self.load_int_param(param, position, 4, &mut context);
            }
        }

        // 保存参数冲突的寄存器和穿越生命周期的寄存器
        for reg in context.stored_regs.iter() {
            let stored_reg = format!("stored.{}", reg);
            self.instrs.insert(position, AsmInstr::make_instr(
                AsmInstrType::Store,
                vec![reg, "sp", stored_reg.as_str()],
                Some(PTR_WIDTH),
                vec![]
            ));
        }
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
                    str_vec.into_iter().skip(2).map(|s| String::from(s)).collect(),
                    ty_vec
                )
            },
        }
    }
}