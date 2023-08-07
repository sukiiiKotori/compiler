use crate::structures::llvm_struct::*;

impl FuncDef {
    pub fn rewrite<UpdateLabel, BlockFilter, InstrFilter>(
        &mut self,
        update_label: &mut UpdateLabel,
        bb_filter: &BlockFilter,
        instr_filter: &InstrFilter,
    )
        where
            UpdateLabel: FnMut(&str) -> String,
            BlockFilter: Fn(&str) -> bool,
            InstrFilter: Fn(i32) -> bool,
    {
        let mut instr_cnt = self.count_instr() as i32 - 1;
        //过滤掉不需要的局部变量
        self.local_vars.retain(|var| {
            instr_cnt += 1;
            bb_filter(&var.label) && instr_filter(instr_cnt)
        });
        //过滤掉不需要的基本块
        self.blocks.retain(|block| bb_filter(&block.block_label));
        //将留下的基本块做修改
        self.blocks.iter_mut().for_each(|block| block.rewrite(update_label, instr_filter));

        self.blocks.first_mut().unwrap().ins_num = 0;
        for i in 1..self.blocks.len() {
            self.blocks[i].ins_num = self.blocks[i - 1].count_instr();
        }
    }
}

impl Block {
    fn rewrite<UpdateLabel, InstrFilter>(
        &mut self,
        update_label: &mut UpdateLabel,
        instr_filter: &InstrFilter,
    )
        where
            UpdateLabel: FnMut(&str) -> String,
            InstrFilter: Fn(i32) -> bool,
    {
        let mut instr_cnt = self.ins_num as i32 - 1;

        self.phi_ins.retain_mut(|phi_instr| {
            instr_cnt += 1;
            if instr_filter(instr_cnt) {
                phi_instr.rewrite(update_label);
                true
            } else {
                false
            }
        });

        self.nor_ins.retain_mut(|nor_instr| {
            instr_cnt += 1;
            if instr_filter(instr_cnt) {
                nor_instr.rewrite(update_label);
                true
            } else {
                false
            }
        });

        if let Some(ter) = &mut self.ter_ins {
            ter.rewrite(update_label);
        }
    }
}

impl Instruction {
    fn update_labels<UpdateLabel>(str_vec: Vec<&str>, update_label: &mut UpdateLabel) -> Vec<String>
        where
            UpdateLabel: FnMut(&str) -> String,
    {
        str_vec.into_iter().map(|s| update_label(s)).collect()
    }

    fn rewrite<UpdateLabel>(&mut self, update_label: &mut UpdateLabel)
        where
            UpdateLabel: FnMut(&str) -> String,
    {
        let another = self.clone();
        let (_, str_vec, ty_vec) = another.fetch_info();
        let str_vec = Self::update_labels(str_vec, update_label);
        let str_vec_ref = str_vec.iter().map(|s| s.as_str()).collect();
        self.update_instruction(str_vec_ref, ty_vec);
    }
}
