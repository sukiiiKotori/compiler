use std::collections::{HashSet, HashMap, VecDeque};
use crate::riscv_gen::build::PTR_WIDTH;
use crate::riscv_gen::select::FLOAT_PREFIX;
use crate::riscv_gen::linearscan::Interval;
use crate::structures::symbol::SymbolWidth;
use crate::utils::check::*;
use crate::structures::riscv_struct::*;
use crate::structures::riscv_regs::*;


pub trait RegisterAllocator {
    fn new() -> Self;
    fn alloc_regs(&mut self, func: &mut AsmFunc);
    fn get_spilled(&self) -> &HashSet<String>;
    fn get_alloc_res(&self) -> &HashMap<String, &'static str>;
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum RegType {
    TempInt,
    SavedInt,
    TempFloat,
    SavedFloat,
}

pub struct RegisterResource {
    // 0 => TEMPORARY
    // 1 => FUNC_ARG
    // 2 => SAVED
    // 3, 4, 5 => float
    pub free_regs: Vec<VecDeque<&'static str>>,
}

pub fn phy_is_float(phy: &str) -> bool {
    &phy[0..1] == FLOAT_PREFIX
}

pub fn get_preserved_regs() -> HashMap<RegType, Vec<&'static str>> {
    let mut res = HashMap::new();
    let preserved_int = PRESERVED.iter().rev().map(|r| *r).collect();
    let preserved_float = FLOAT_PRESERVED.iter().rev().map(|r| *r).collect();
    res.insert(RegType::TempInt, preserved_int);
    res.insert(RegType::TempFloat, preserved_float);
    res
}

impl RegType {
    pub fn classify_label(is_float: bool, is_saved: bool) -> Self {
        match (is_float, is_saved) {
            (false, false) => Self::TempInt,
            (false, true) => Self::SavedInt,
            (true, false) => Self::TempFloat,
            (true, true) => Self::SavedFloat,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Self::TempInt | Self::SavedInt => false,
            Self::TempFloat | Self::SavedFloat => true,
        }
    }

    pub fn is_saved(&self) -> bool {
        match self {
            Self::TempInt | Self::TempFloat => false,
            Self::SavedInt | Self::SavedFloat => true,
        }
    }
}

impl RegisterResource {
    pub fn new() -> Self {
        Self {
            free_regs: Vec::new(),
        }
    }

    pub fn load_free_regs(&mut self) {
        // 0 => TEMPORARY
        // 1 => FUNC_ARG
        // 2 => SAVED
        // 3, 4, 5 => float
        self.free_regs.push(TEMPORARY.iter().cloned().collect());
        self.free_regs.push(FUNC_ARG.iter().cloned().collect());
        self.free_regs.push(SAVED.iter().cloned().collect());
        self.free_regs.push(FLOAT_TEMPORARY.iter().cloned().collect());
        self.free_regs.push(FLOAT_FUNC_ARG.iter().cloned().collect());
        self.free_regs.push(FLOAT_SAVED.iter().cloned().collect());
    }

    pub fn evict_regs(&mut self, evict: impl Fn(&str) -> bool) {
        for group in self.free_regs.iter_mut() {
            group.retain(|r| !evict(r));
        }
    }

    pub fn filter_map_regs(group_idx: usize, idx: usize, reg: &str, filter_regs: impl Fn(&str) -> bool + Copy) -> Option<(usize, usize)> {
        if filter_regs(reg) {
            Some((group_idx, idx))
        } else {
            None
        }
    }

    pub fn pick_register(&mut self, ty: &RegType, filter_regs: impl Fn(&str) -> bool + Copy) -> Option<(usize, usize)> {
        let prio: &[usize; 3];

        // 0 => TEMPORARY
        // 1 => FUNC_ARG
        // 2 => SAVED
        // 3, 4, 5 => float
        match ty {
            RegType::TempInt => prio = &[0, 1, 2],
            RegType::SavedInt => prio = &[2, 0, 1],
            RegType::TempFloat => prio = &[3, 4, 5],
            RegType::SavedFloat => prio = &[5, 3, 4],
        }
        self.free_regs[prio[0]].iter().enumerate().find_map(|(i, r)| Self::filter_map_regs(prio[0], i, r, filter_regs))
            .or(self.free_regs[prio[1]].iter().enumerate().find_map(|(i, r)| Self::filter_map_regs(prio[1], i, r, filter_regs)))
            .or(self.free_regs[prio[2]].iter().enumerate().find_map(|(i, r)| Self::filter_map_regs(prio[2], i, r, filter_regs)))
    }

    pub fn get_register(&mut self, ty: &RegType, filter_regs: impl Fn(&str) -> bool + Copy) -> Option<&'static str> {
        if let Some((group_idx, idx)) = self.pick_register(ty, filter_regs) {
            let reg = *self.free_regs.get_mut(group_idx).unwrap().get(idx).unwrap();
            Some(reg)
        } else {
            None
        }
    }

    pub fn pop_register(&mut self, ty: &RegType, filter_regs: impl Fn(&str) -> bool + Copy) -> Option<&'static str> {
        if let Some((group_idx, idx)) = self.pick_register(ty, filter_regs) {
            let reg = self.free_regs.get_mut(group_idx).unwrap().remove(idx).unwrap();
            Some(reg)
        } else {
            None
        }
    }

    fn get_group_idx(reg: &'static str) -> usize {
        // 0 => TEMPORARY
        // 1 => FUNC_ARG
        // 2 => SAVED
        // 3, 4, 5 => float
        match &reg[0..1] {
            "f" => {
                Self::get_group_idx(&reg[1..]) + 3
            }
            "t" => 0,
            "a" => 1,
            "s" => 2,
            _ => panic!("Register name error"),
        }
    }

    pub fn push_register(&mut self, reg: &'static str) {
        self.free_regs.get_mut(Self::get_group_idx(reg)).unwrap().push_front(reg)
    }
}

impl RiscV {
    pub fn save_registers(&mut self) {
        for func in self.text.funcs.iter_mut() {
            func.save_registers();
        }
    }
}

impl AsmFunc {
    fn store_saved(&mut self, reg: &str) {
        self.stack.push_normal(reg, 8);
        let first_block = self.blocks.first_mut().unwrap();
        if phy_is_float(reg) {
            first_block.instrs.insert(0, AsmInstr::make_instr(AsmInstrType::Store, vec!(reg, "sp", reg, FLOAT_PREFIX), vec!(PTR_WIDTH), vec!()));
        } else {
            first_block.instrs.insert(0, AsmInstr::make_instr(AsmInstrType::Store, vec!(reg, "sp", reg), vec!(PTR_WIDTH), vec!()));
        }
    }

    fn save_registers(&mut self) {
        let regs = self.used_saved.iter().map(|r| (*r).clone()).collect::<Vec<_>>();
        for reg in regs.iter() {
            self.store_saved(reg);
        }
        for block in self.blocks.iter_mut() {
            block.save_registers(&regs);
        }
    }

    pub fn interval_cross_call(&mut self, live: &Interval, virt: &str) -> bool {
        let mut res = false;
        for (_, depth_first_pos, cross_virts) in self.call_info.iter_mut() {
            let cross = live.intervals.iter().any(|range| range.left < *depth_first_pos.as_ref().unwrap() && range.right > *depth_first_pos.as_ref().unwrap());
            if cross {
                res = true;
                cross_virts.insert(String::from(virt));
            }
        }
        res
    }

    pub fn is_float(&self, label: &str) -> bool {
        *self.label_type.get(label).expect(&format!("Type of {} havent added", label)) == SymbolWidth::Float
    }

    pub fn used_saved(&mut self, phy: &'static str) {
        self.used_saved.insert(phy);
    }

    #[allow(unused)]
    pub fn get_label_weight(&self) -> HashMap<String, usize> {
        let mut res = HashMap::new();
        for block in self.blocks.iter() {
            for instr in block.instrs.iter() {
                let (output, inputs) = instr.get_labels();
                if let Some(output) = output {
                    if res.contains_key(output) {
                        *res.get_mut(output).unwrap() += block.weight;
                    } else {
                        res.insert(String::from(output), block.weight);
                    }
                }
                for input in inputs.into_iter() {
                    if res.contains_key(input) {
                        *res.get_mut(input).unwrap() += block.weight;
                    } else {
                        res.insert(String::from(input), block.weight);
                    }
                } // for
            } // for instr
        } // for block
        res
    }
} // impl

impl AsmBlock {
    fn restore_saved(&mut self, position: usize, reg: &str) {
        if phy_is_float(reg) {
            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Load, vec!(reg, "sp", reg, FLOAT_PREFIX), vec!(PTR_WIDTH), vec!()));
        } else {
            self.instrs.insert(position, AsmInstr::make_instr(AsmInstrType::Load, vec!(reg, "sp", reg), vec!(PTR_WIDTH), vec!()));
        }
    }

    pub fn save_registers(&mut self, used_saved: &Vec<&'static str>) {
        let positions = self.instrs.iter()
            .enumerate()
            .filter_map(|(idx, instr)| {
                match instr {
                    AsmInstr::Ret(_) => Some(idx),
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        for position in positions.into_iter().rev() {
            for reg in used_saved.into_iter() {
                self.restore_saved(position, reg);
            }
        }
    }
}

impl AsmInstr {
    pub fn filter_regs<'asm>(output: Option<&'asm str>, inputs: Vec<&'asm str>, filter_reg: impl Fn(&'asm str) -> bool) -> (Option<&'asm str>, Vec<&'asm str>) {
        let new_output: Option<&str>;
        let mut new_inputs: Vec<&str> = vec!();
        if let Some(output) = output {
            let output = output;
            if filter_reg(output) {
                new_output = Some(output);
            } else {
                new_output = None;
            }
        } else {
            new_output = None;
        }

        for input in inputs.into_iter() {
            if filter_reg(input) {
                new_inputs.push(input);
            }
        }

        (new_output, new_inputs)
    }

    #[allow(unused)]
    pub fn get_virt_regs(&self) -> (Option<&str>, Vec<&str>) {
        let (output, inputs) = self.get_labels();
        AsmInstr::filter_regs(output, inputs, |reg| {
            is_num_label(reg) || is_temp_opr(reg)
        })
    }

    #[allow(unused)]
    pub fn get_phy_regs(&self) -> (Option<&str>, Vec<&str>) {
        let (output, inputs) = self.get_labels();
        AsmInstr::filter_regs(output, inputs, |reg| {
            ALL_REGS.contains(reg)
        })
    }

    pub fn get_regs(&self) -> (Option<&str>, Vec<&str>) {
        let (output, inputs) = self.get_labels();
        AsmInstr::filter_regs(output, inputs, |reg| {
            is_num_label(reg) || is_temp_opr(reg) || ALL_REGS.contains(reg)
        })
    }

    pub fn get_labels(&self) -> (Option<&str>, Vec<&str>) {
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
                bin.get_regs()
            }
            AsmInstr::Addi(tri) | AsmInstr::Xori(tri) | AsmInstr::Slti(tri) => {
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
                tri.get_regs()
            }
            AsmInstr::Store(mem, _) => {
                let (output, mut inputs) = mem.get_regs();
                if output.is_some() {
                    let output = output.unwrap();
                    inputs.push(output);
                }
                (None, inputs)
            }
            AsmInstr::Load(mem, _) => {
                mem.get_regs()
            }
            AsmInstr::Branch(cond_tri) => {
                let (output, _) = cond_tri.get_regs();
                (None, vec!(output.unwrap()))
            }
            AsmInstr::Jump(_) => (None, vec!()),
            AsmInstr::Ret(ret_val) => (None, vec!(ret_val.as_str())),
            AsmInstr::Call(ret, _, params, _) => (Some(ret.as_str()), params.iter().map(|s| s.as_str()).collect()),
        }
    }
}

impl BinInstr {
    fn get_regs(&self) -> (Option<&str>, Vec<&str>) {
        (Some(&self.dst), vec!(&self.src))
    }
}

impl CondTriInstr {
    fn get_regs(&self) -> (Option<&str>, Vec<&str>) {
        self.tri.get_regs()
    }
}

impl TriInstr {
    fn get_regs(&self) -> (Option<&str>, Vec<&str>) {
        (Some(&self.dst), vec!(&self.op1, &self.op2))
    }
}

impl MemInstr {
    fn get_regs(&self) -> (Option<&str>, Vec<&str>) {
        (Some(&self.val), vec!(&self.base, &self.offset))
    }
}

