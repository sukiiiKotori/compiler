use std::collections::VecDeque;
use crate::utils::check::*;
use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;
use crate::riscv_gen::register_type::*;

// 寄存器资源
pub struct RegisterResource {
    // 为一个双端队列组成的数组，其中数组中每一个队列对应不同的寄存器
    // free_regs[0] 是 临时整数寄存器
    // free_regs[1] 是 函数参数寄存器
    // free_regs[2] 是 保存整数寄存器
    // free_regs[3] 是 临时浮点寄存器数组
    // free_regs[4] 是 浮点函数参数寄存器数组
    // free_regs[5] 是 保存浮点寄存器数组
    // [f]t0是保留[浮点]返回值寄存器
    // int 类型寄存器队列 下标 + 3 是对应的 浮点类型寄存器
    pub free_regs: Vec<VecDeque<&'static str>>,
}

// 寄存器资源
impl RegisterResource {
    pub fn new() -> Self {
        Self {
            free_regs: Vec::new(),
        }
    }

    // 加载空闲寄存器
    pub fn load_free_regs(&mut self) {
        self.free_regs.push(TEMPORARY.iter().cloned().collect());
        self.free_regs.push(FUNC_ARG.iter().cloned().collect());
        self.free_regs.push(SAVED.iter().cloned().collect());
        self.free_regs.push(FLOAT_TEMPORARY.iter().cloned().collect());
        self.free_regs.push(FLOAT_FUNC_ARG.iter().cloned().collect());
        self.free_regs.push(FLOAT_SAVED.iter().cloned().collect());
    }

    // 根据条件函数移除寄存器
    pub fn remove_regs(&mut self, cond: impl Fn(&str) -> bool) {
        for group in self.free_regs.iter_mut() {
            group.retain(|r| !cond(r));
        }
    }

    // 根据条件筛选寄存器，如果寄存器满足条件，返回（类型队列在数组中下标，在队列中下标）
    pub fn check_reg(group_idx: usize, idx: usize, reg: &str, regtype_filter: impl Fn(&str) -> bool + Copy) -> Option<(usize, usize)> {
        if regtype_filter(reg) {
            Some((group_idx, idx))
        } else {
            None
        }
    }

    // 寄存器选择
    pub fn pick_register(&mut self, ty: &RegType, regtype_filter: impl Fn(&str) -> bool + Copy) -> Option<(usize, usize)> {
        // 根据需求类型创建一个获取类型优先级队列，然后根据优先级队列尝试选择寄存器
        let type_priority: &[usize; 3];

        match ty {
            RegType::TempInt => type_priority = &[0, 1, 2],
            RegType::SavedInt => type_priority = &[2, 0, 1],
            RegType::TempFloat => type_priority = &[3, 4, 5],
            RegType::SavedFloat => type_priority = &[5, 3, 4],
        }
        self.free_regs[type_priority[0]].iter().enumerate().find_map(|(i, r)| Self::check_reg(type_priority[0], i, r, regtype_filter))
            .or(self.free_regs[type_priority[1]].iter().enumerate().find_map(|(i, r)| Self::check_reg(type_priority[1], i, r, regtype_filter)))
            .or(self.free_regs[type_priority[2]].iter().enumerate().find_map(|(i, r)| Self::check_reg(type_priority[2], i, r, regtype_filter)))
    }

    // 使用筛选函数，根据类型，从空闲寄存器中获取一个需求类型的寄存器
    pub fn get_register(&mut self, ty: &RegType, regtype_filter: impl Fn(&str) -> bool + Copy) -> Option<&'static str> {
        if let Some((group_idx, idx)) = self.pick_register(ty, regtype_filter) {
            let reg = self.free_regs.get_mut(group_idx).unwrap().remove(idx).unwrap();
            Some(reg)
        } else {
            None
        }
    }

    // 获取寄存器类型 对应的队列 在数组中的下标
    fn get_regqueue_idx(reg: &'static str) -> usize {
        match &reg[0..1] {
            "f" => {
                Self::get_regqueue_idx(&reg[1..]) + 3
            }
            "t" => 0,
            "a" => 1,
            "s" => 2,
            _ => panic!("Register name error"),
        }
    }

    // 将寄存器重新放入空闲寄存器列表
    pub fn free_register(&mut self, reg: &'static str) {
        self.free_regs.get_mut(Self::get_regqueue_idx(reg)).unwrap().push_front(reg)
    }
}

// 根据指令类型确定输入输出
impl BinInstr {
    fn get_io(&self) -> (Option<&str>, Vec<&str>) {
        (Some(&self.dst), vec!(&self.src))
    }
}

impl CondTriInstr {
    fn get_io(&self) -> (Option<&str>, Vec<&str>) {
        self.tri.get_io()
    }
}

impl TriInstr {
    fn get_io(&self) -> (Option<&str>, Vec<&str>) {
        (Some(&self.dst), vec!(&self.op1, &self.op2))
    }
}

impl MemInstr {
    fn get_io(&self) -> (Option<&str>, Vec<&str>) {
        (Some(&self.val), vec!(&self.base, &self.offset))
    }
}

impl AsmInstr {
    // 根据筛选条件（是否需要寄存器的条件），将指令输入输出中需要寄存器的部分返回
    pub fn io_filter<'asm>(output: Option<&'asm str>, inputs: Vec<&'asm str>, filter_cond: impl Fn(&'asm str) -> bool) -> (Option<&'asm str>, Vec<&'asm str>) {
        let new_output: Option<&str>;
        let mut new_inputs: Vec<&str> = vec!();
        if let Some(output) = output {
            let output = output;
            if filter_cond(output) {
                new_output = Some(output);
            } else {
                new_output = None;
            }
        } else {
            new_output = None;
        }

        for input in inputs.into_iter() {
            if filter_cond(input) {
                new_inputs.push(input);
            }
        }

        (new_output, new_inputs)
    }

    // 获取指令对应的输入输出数据
    pub fn get_io(&self) -> (Option<&str>, Vec<&str>) {
        match self {
            AsmInstr::Li(bin) | AsmInstr::La(bin) => {
                match bin {
                    BinInstr { dst, src: _ } => {
                        (Some(dst), vec!())
                    }
                }
            }
            AsmInstr::Mv(bin) | AsmInstr::Fmv(bin, _, _) | AsmInstr::Sextw(bin) |
            AsmInstr::Fcvt(bin, _, _) | AsmInstr::Seqz(bin) | AsmInstr::Snez(bin) => {
                bin.get_io()
            }
            AsmInstr::Addi(tri) | AsmInstr::Xori(tri) | AsmInstr::Slti(tri) |
            AsmInstr::Slli(tri) | AsmInstr::Srli(tri) | AsmInstr::Srai(tri) => {
                match tri {
                    TriInstr { width: _, dst, op1, op2: _ } => {
                        (Some(dst), vec!(op1))
                    }
                }
            }
            AsmInstr::Add(tri) | AsmInstr::Sub(tri) | AsmInstr::Mul(tri) |
            AsmInstr::Div(tri) | AsmInstr::Rem(tri) | AsmInstr::Slt(tri) |
            AsmInstr::Sgt(tri) | AsmInstr::Flt(tri) | AsmInstr::Fle(tri) |
            AsmInstr::Feq(tri) | AsmInstr::Fadd(tri) | AsmInstr::Fsub(tri) |
            AsmInstr::Fmul(tri) | AsmInstr::Fdiv(tri) => {
                tri.get_io()
            }
            AsmInstr::Store(mem, _) => {
                let (output, mut inputs) = mem.get_io();
                if output.is_some() {
                    let output = output.unwrap();
                    inputs.push(output);
                }
                (None, inputs)
            }
            AsmInstr::Load(mem, _) => {
                mem.get_io()
            }
            AsmInstr::Branch(cond_tri) => {
                let (output, _) = cond_tri.get_io();
                (None, vec!(output.unwrap()))
            }
            AsmInstr::Jump(_) => (None, vec!()),
            AsmInstr::Ret() => (None, vec!()),
            AsmInstr::Call(ret, _, params, _) => (Some(ret.as_str()), params.iter().map(|s| s.as_str()).collect()),
        }
    }

    // 获取指令中需要使用的寄存器(虚拟or物理)
    pub fn get_regs(&self) -> (Option<&str>, Vec<&str>) {
        let (output, inputs) = self.get_io();
        // 获取实际需要寄存器的io
        AsmInstr::io_filter(output, inputs, |reg| {
            is_num_label(reg) || is_temp_opr(reg) || ALL_REGS.contains(reg)
        })
    }
}

