use std::iter::zip;
use std::collections::{HashSet, HashMap, BTreeSet};
use crate::utils::check::*;
use crate::utils::float::*;
use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;
use crate::structures::symbol::*;
use crate::riscv_gen::stack_slot::StackSlot;
use crate::riscv_gen::select::FLOAT_PREFIX;
use crate::riscv_gen::reg::RegisterAllocator;


pub const NORMAL_WIDTH: isize = 4;
pub const PTR_WIDTH: isize = 8;

impl RiscV {
    /// 创建一个新的RiscV结构体实例，并初始化各个部分的数据结构
    pub fn new() -> Self {
        RiscV {
            text: TextSection::new(),
            data: DataSection::new(),
            rodata: RoDataSection::new(),
        }
    }
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
    /// 向data段的全局变量列表中添加一个新的全局变量，使用给定的标签、类型和初始值向量
    pub fn push_global_var(&mut self, label: &str, ty: &SymbolType, init_vals: Vec<&str>) {
        self.data.labels.insert(String::from(label));
        let val_vec = init_vals.iter().map(|v| String::from(*v)).collect::<Vec<_>>();
        self.data.datas.push(DataSectionItem{
            label: String::from(label),
            ty: ty.clone(),
            init_vals: val_vec,
        });
    }
    /// 向rodata段的全局常量列表中添加一个新的全局常量，使用给定的标签、类型和初始值向量
    pub fn push_global_const(&mut self, label: &str, ty: &SymbolType, init_vals: Vec<&str>) {
        self.data.labels.insert(String::from(label));
        let val_vec = init_vals.iter().map(|v| String::from(*v)).collect::<Vec<_>>();
        self.rodata.datas.push(DataSectionItem{
            label: String::from(label),
            ty: ty.clone(),
            init_vals: val_vec,
        });
    }
    /// 标记当前正在处理的函数为调用函数
    pub fn mark_call(&mut self) {
        self.text.funcs.last_mut().unwrap().mark_call();
    }
    /// 向当前正在处理的函数的标签类型映射中插入一个新的标签和类型的对应关系
    pub fn insert_label_type(&mut self, label: &str, width: SymbolWidth) {
        self.text.funcs.last_mut().unwrap().label_type.insert(String::from(label), width);
    }
    /// 对每个函数进行寄存器分配
    pub fn alloc_regs<Alloc: RegisterAllocator>(&mut self) {
        for func in self.text.funcs.iter_mut() {
            let mut allocator = Alloc::new();
            allocator.alloc_regs(func);
            //把虚拟寄存器更改为物理寄存器
            func.assign_register(allocator.get_alloc_res());
            // 展开函数调用，使用分配的寄存器
            func.unfold_call(&mut self.rodata, allocator.get_alloc_res());
            // 对溢出的寄存器进行重写
            func.rewrite_spilled(allocator.get_spilled());
        }
    }
} // imp

impl RoDataSection {
    /// 创建一个新的RoDataSection结构体实例，并初始化各个字段
    pub fn new() -> Self {
        RoDataSection{
            datas: Vec::new(), 
            labels: HashSet::new(),
            float_imm: HashMap::new(),
            float_imm_cnt: 0,
        }
    }
    /// 格式化浮点数立即数的标签
    pub fn format_float_imm(id: usize) -> String {
        format!("float_imm.{}", id)
    }
    /// 将浮点数立即数添加到数据段中，并返回其对应的标签id
    pub fn push_float_imm(&mut self, imm: &str) -> usize {
        if let Some(id) = self.float_imm.get(imm) {
            *id
        } else {
            // 如果浮点数立即数不在映射中，则创建一个新的标签id
            let id = self.float_imm_cnt;
            self.float_imm_cnt += 1;
            // 格式化浮点数立即数的标签
            let imm_label = Self::format_float_imm(id);
            // 将浮点数立即数添加到数据段中，使用初始值为imm的DataSectionItem
            self.datas.push(DataSectionItem{
                label: imm_label,
                ty: SymbolType::new(SymbolWidth::Float, false),
                init_vals: vec!(String::from(imm)),
            });
            // 将浮点数立即数和标签id添加到浮点数立即数映射中
            self.float_imm.insert(String::from(imm), id);
            id
        }
    }
}

impl DataSection {
    pub fn new() -> Self {
        DataSection{datas: Vec::new(), labels: HashSet::new()}
    }
}

impl TextSection {
    pub fn new() -> Self {
        TextSection{funcs: Vec::new()}
    }
    
    pub fn curr_func(&mut self) -> &mut AsmFunc {
        self.funcs.last_mut().unwrap()
    }
}

impl AsmFunc {
    pub fn new(label: &str, ret_type: SymbolWidth) -> Self {
        AsmFunc{
            label: String::from(label), 
            ret_type,
            stack: StackSlot::new(), 
            blocks: Vec::new(),
            params: HashMap::new(),
            label_type: HashMap::new(),
            call_info: Vec::new(),
            used_saved: HashSet::new(),
        }
    }

    pub fn push_block(&mut self, block_label: &str, depth: usize) {
        if self.blocks.is_empty() {
            self.blocks.push(AsmBlock::new(block_label, 0, depth));
        } else {
            let last_block = self.blocks.last().unwrap();
            let new_pre_instr_cnt = last_block.pre_instr_cnt + last_block.instrs.len();
            self.blocks.push(AsmBlock::new(block_label, new_pre_instr_cnt, depth));
        }
    }

    pub fn curr_block(&mut self) -> &mut AsmBlock{
        self.blocks.last_mut().unwrap()
    }

    fn mark_call(&mut self) {
        self.used_saved("ra");
        let last_block = self.blocks.last().unwrap();
        self.call_info.push((last_block.pre_instr_cnt + last_block.instrs.len(), None, HashSet::new()));
    }

    pub fn unfold_call(&mut self, rodata: &mut RoDataSection, alloc_res: &HashMap<String, &'static str>) {
        let mut call_info_ref = self.call_info.iter().collect::<Vec<_>>();
        for block in self.blocks.iter_mut().rev() {
            loop {
                if let Some(this_call_info) = call_info_ref.last() {
                    let this_idx = this_call_info.0;
                    if this_idx >= block.pre_instr_cnt {
                        let position = this_idx - block.pre_instr_cnt;
                        block.unfold_call(&mut self.stack, this_call_info, alloc_res, rodata, position);
                        call_info_ref.pop();
                    } else {
                        break;
                    }
                } else {
                    return;
                }
            }
        }
    }
}

pub struct UnfoldCallContext<'a> {
    pub int_cnt: usize,
    pub float_cnt: usize,
    pub stack_len: isize,
    pub stack: &'a mut StackSlot,
    pub rodata: &'a mut RoDataSection,
    pub invalid_regs: &'a mut HashSet<&'static str>,
    pub stored_regs: &'a mut BTreeSet<&'a str>,
}

impl<'a> UnfoldCallContext<'a> {
    pub fn new(
    	int_cnt: usize,
    	float_cnt: usize,
    	stack_len: isize,
    	stack: &'a mut StackSlot,
        rodata: &'a mut RoDataSection,
    	invalid_regs: &'a mut HashSet<&'static str>,
        stored_regs: &'a mut BTreeSet<&'a str>,
    ) -> Self {
        UnfoldCallContext{int_cnt, float_cnt, stack_len, stack, rodata, invalid_regs, stored_regs}
    }
}

impl AsmBlock {
    pub fn new(label: &str, pre_instr_cnt: usize, depth: usize) -> Self {
        AsmBlock {
            label: String::from(label),
            instrs: Vec::new(),
            successor: Vec::new(),
            pre_instr_cnt: pre_instr_cnt,
            weight: 10_usize.pow(depth as u32),
            depth: depth,
        }
    }

    pub fn push_instr(&mut self, instr: AsmInstr) {
        self.instrs.push(instr)
    }

    fn push_successor(&mut self, succ: &str) {
        self.successor.push(String::from(succ));
    }

    pub fn load_float_param<'a>(
        &mut self, 
        param: &'a String, 
        position: usize, 
        context: &mut UnfoldCallContext<'a>,
    ) {
	    if context.float_cnt >= FLOAT_FUNC_ARG.len() {
	        context.stack_len += 4;
            let stack_pos = format!("-{}", context.stack_len);
	        if is_immediate(param.as_str()) {
	            let imm = double_to_float(param.as_str());
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Store, vec!(PRESERVED[1], "sp", stack_pos.as_str(), ), Some(NORMAL_WIDTH), vec!()));
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Li, vec!(PRESERVED[1], imm.as_str()), None, vec!()));
            } else if context.invalid_regs.contains(param.as_str()) {
                panic!("Should not appear");
            } else {
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Store, vec!(param.as_str(), "sp", stack_pos.as_str(), FLOAT_PREFIX), Some(NORMAL_WIDTH), vec!()));
	        }
        } else {
            context.invalid_regs.remove(FLOAT_FUNC_ARG[context.float_cnt]);
	        if is_immediate(param.as_str()) {
	            let imm = double_to_float(param.as_str());
                let imm_id = context.rodata.push_float_imm(imm.as_str());
	            let imm_label = RoDataSection::format_float_imm(imm_id);
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Load, vec!(FLOAT_FUNC_ARG[context.float_cnt], PRESERVED[1], "0", FLOAT_PREFIX), Some(NORMAL_WIDTH), vec!()));
                self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::La, vec!(PRESERVED[1], imm_label.as_str()), None, vec!()));
            } else if context.invalid_regs.contains(param.as_str()) {
	            let stored_pos = format!("stored.{}", param);
	            context.stored_regs.insert(param.as_str());
	            context.stack.push_normal(stored_pos.as_str(), 8);
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Load, vec!(FLOAT_FUNC_ARG[context.float_cnt], "sp", stored_pos.as_str(), FLOAT_PREFIX), Some(NORMAL_WIDTH), vec!()));
	        } else {
                self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Fmv, vec!(FLOAT_FUNC_ARG[context.float_cnt], param.as_str()), Some(NORMAL_WIDTH), vec!(SymbolWidth::Float, SymbolWidth::Float)));
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
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Store, vec!(PRESERVED[1], "sp", stack_pos.as_str()), Some(param_size), vec!()));
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Li, vec!(PRESERVED[1], param.as_str()), None, vec!()));
	        } else if context.invalid_regs.contains(param.as_str()) {
                panic!("Should not appear");
	        } else {
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Store, vec!(param.as_str(), "sp", stack_pos.as_str()), Some(param_size), vec!()));
            }
	    } else {
            context.invalid_regs.remove(FUNC_ARG[context.int_cnt]);
	        if is_immediate(param.as_str()) {
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Li, vec!(FUNC_ARG[context.int_cnt], param.as_str()), Some(param_size), vec!()));
	        } else if context.invalid_regs.contains(param.as_str()) {
	            let stored_pos = format!("stored.{}", param);
	            context.stored_regs.insert(param.as_str());
	            context.stack.push_normal(stored_pos.as_str(), 8);
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Load, vec!(FUNC_ARG[context.int_cnt], stored_pos.as_str()), Some(PTR_WIDTH), vec!()));
            } else {
	            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Mv, vec!(FUNC_ARG[context.int_cnt], param.as_str()), None, vec!()));
	        }
	    }
        context.int_cnt += 1;
    }

    fn unfold_call(
        &mut self,
        stack: &mut StackSlot,
        this_call_info: &(usize, Option<usize>, HashSet<String>),
        alloc_res: &HashMap<String, &'static str>,
        rodata: &mut RoDataSection, position: usize
    ) {
        let ret_val: String;
        let params: Vec<String>;
	    let types: Vec<SymbolWidth>;
        match self.instrs.get(position).unwrap() {
	        AsmInstr::Call(r, _, p, t) => {
	            ret_val = String::from(r);
		        params = p.iter()
		            .map(|p| String::from(p))
                    .collect();
		        types = t.iter()
		            .map(|t| t.clone())
		            .collect();
            },
	        _ => panic!("Position error"),
        }

        // 获取穿过当前Call指令的Temporary寄存器
        let mut stored_regs = this_call_info.2.iter()
            .filter_map(|r| alloc_res.get(r).filter(|phy| TEMP_SET.contains(*phy) || FLOAT_TEMP_SET.contains(*phy)))
            .map(|r| *r)
            .collect::<BTreeSet<_>>();

        // 恢复穿过当前Call指令的Temporary寄存器
        for temp in stored_regs.iter() {
            let stored_pos = format!("stored.{}", temp);
	        stack.push_normal(stored_pos.as_str(), 8);
            let mut prefix = "";
            if FLOAT_TEMP_SET.contains(*temp) {
                prefix = FLOAT_PREFIX;
            }
            self.instrs.insert(position+1, AsmInstr::make_instr(AsmInstrType::Load, vec!(temp, "sp", stored_pos.as_str(), prefix), Some(PTR_WIDTH), vec!()));
        }

        // 存储调用返回值
	    if ret_val != "" {
	        if types[0] == SymbolWidth::Float {
	            self.instrs.insert(position+1, AsmInstr::make_instr(AsmInstrType::Fmv, vec!(ret_val.as_str(), "fa0"), None, vec!(SymbolWidth::Float, SymbolWidth::Float)));
	        } else {
	            self.instrs.insert(position+1, AsmInstr::make_instr(AsmInstrType::Mv, vec!(ret_val.as_str(), "a0"), None, vec!()));
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
                    invalid_regs.insert(FUNC_ARG[float_pos]);
                }
                int_pos += 1;
            }
            if int_pos >= FUNC_ARG.len() && float_pos >= FLOAT_FUNC_ARG.len() {
                break;
            }
        }
	
        // 将参数值装载到指定寄存器或者栈槽位置
        let mut context = UnfoldCallContext::new(0, 0, 0, stack, rodata, &mut invalid_regs, &mut stored_regs);
        for (_, (param, ty)) in zip(params.iter(), types.iter().skip(1)).enumerate() {
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
	        self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Store, vec!(reg, "sp", stored_reg.as_str()), Some(PTR_WIDTH), vec!()));
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
                AsmInstr::Call(String::from(str_vec[0]), String::from(str_vec[1]), str_vec.into_iter().skip(2).map(|s| String::from(s)).collect(), ty_vec)
            },
        }
    }
}

impl BinInstr {
    pub fn new(dst: &str, src: &str) -> Self {
        BinInstr {
            dst: String::from(dst),
            src: String::from(src),
        }
    }
}

impl CondTriInstr {
    pub fn new(cond: &str, width: Option<isize>, dst: &str, op1: &str, op2: &str) -> Self {
        CondTriInstr {
            cond: String::from(cond),
            tri: TriInstr::new(width, dst, op1, op2),
        }
    }
}

impl TriInstr {
    pub fn new(width: Option<isize>, dst: &str, op1: &str, op2: &str) -> Self {
        TriInstr {
            width,
            dst: String::from(dst),
            op1: String::from(op1),
            op2: String::from(op2),
        }
    }
}

impl MemInstr {
    pub fn new(width: isize, val: &str, base: &str, offset: &str) -> Self {
        MemInstr {
            width,
            val: String::from(val),
            base: String::from(base),
            offset: String::from(offset),
        }
    }
}

