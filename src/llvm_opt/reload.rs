/* use crate::structures::llvm_struct::*;

pub trait Reload {
    type ReloadResult;
    fn reload<UpdateLabel, BlockFilter, InstrFilter>(
        self,
        update_label: &mut UpdateLabel,
        bb_filter: &BlockFilter,
        instr_filter: &InstrFilter,
    ) -> Self::ReloadResult
        where
            UpdateLabel: FnMut(&str) -> String,
            BlockFilter: Fn(&str) -> bool,
            InstrFilter: Fn(usize) -> bool;
}

impl Reload for FuncDef {
    type ReloadResult = Self;

    fn reload<UpdateLabel, BlockFilter, InstrFilter>(
        mut self,
        update_label: &mut UpdateLabel,
        bb_filter: &BlockFilter,
        instr_filter: &InstrFilter,
    ) -> Self::ReloadResult
        where
            UpdateLabel: FnMut(&str) -> String,
            BlockFilter: Fn(&str) -> bool,
            InstrFilter: Fn(usize) -> bool,
    {
        let instr_cnt = self.count_instr();
        let new_local_vars = self
            .local_vars
            .into_iter()
            .enumerate()
            .filter(|(i, a)| bb_filter(a.label.as_str()) && instr_filter(instr_cnt + i))
            .map(|(_, info)| info)
            .collect();
        self.local_vars = new_local_vars;

        let mut new_blocks: Vec<Block> = self
            .blocks
            .into_iter()
            .filter_map(|b| b.reload(update_label, bb_filter, instr_filter))
            .collect();
        new_blocks.first_mut().unwrap().ins_num = 0;
        for i in 1..new_blocks.len() {
            new_blocks[i].ins_num = new_blocks[i.clone() - 1].count_instr();
        }
        self.blocks = new_blocks;

        self
    }
}

impl Block {
    fn reload<UpdateLabel, BlockFilter, InstrFilter>(
        mut self,
        update_label: &mut UpdateLabel,
        bb_filter: &BlockFilter,
        instr_filter: &InstrFilter,
    ) -> Option<Self>
        where
            UpdateLabel: FnMut(&str) -> String,
            BlockFilter: Fn(&str) -> bool,
            InstrFilter: Fn(usize) -> bool,
    {
        if !bb_filter(self.block_label.as_str()) {
            return None;
        }
        let mut instr_cnt = self.ins_num;

        let new_phi_instr: Vec<Instruction> = self
            .phi_ins
            .into_iter()
            .filter(|_| {
                let cond = instr_filter(instr_cnt.clone());
                instr_cnt += 1;
                cond
            })
            .map(|p| p.reload(update_label))
            .collect();

        let new_instrs: Vec<Instruction> = self
            .nor_ins
            .into_iter()
            .filter(|_| {
                let cond = instr_filter(instr_cnt.clone());
                instr_cnt += 1;
                cond
            })
            .map(|i| i.reload(update_label))
            .collect();

        let ter_instr: Option<Instruction>;
        if let Some(ter) = self.ter_ins {
            ter_instr = Some(ter.reload(update_label));
        } else {
            ter_instr = None;
        }
        self.phi_ins = new_phi_instr;
        self.nor_ins = new_instrs;
        self.ter_ins = ter_instr;
        Some(self)
    }
}

impl Instruction {
    fn update_labels<UpdateLabel>(str_vec: Vec<&str>, update_label: &mut UpdateLabel) -> Vec<String>
        where
            UpdateLabel: FnMut(&str) -> String,
    {
        str_vec.into_iter().map(|s| update_label(s)).collect()
    }

    fn reload<UpdateLabel>(self, update_label: &mut UpdateLabel) -> Self
        where
            UpdateLabel: FnMut(&str) -> String,
    {
        let (ty, str_vec, ty_vec) = self.fetch_info();
        let str_vec = Self::update_labels(str_vec, update_label);
        let str_vec_ref = str_vec.iter().map(|s| s.as_str()).collect();
        Instruction::make_instruction(ty, str_vec_ref, ty_vec)
    }
}
 */