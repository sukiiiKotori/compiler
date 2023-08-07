use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;
use crate::structures::scopes::*;

impl LLVMProgram {
    fn curr_func(&mut self) -> Option<&mut FuncDef> {
        self.func_def.last_mut()
    }

    pub fn push_global_var(&mut self, id: &str, ty: &SymbolType, init_vals: Vec<&String>) {
        self.global_var.push(GlobalVar {
            var_name: id.to_string(),
            var_type: ty.clone(),
            init_values: init_vals.iter().map(|value| value.to_string()).collect(),
        });
    }

    pub fn push_func_decl(&mut self, func_type: &SymbolType, func_name: &str, types: Vec<&SymbolType>) {
        let mut ftypes: Vec<SymbolType> = vec!();
        for ty in types {
            ftypes.push(ty.clone());
        }
        self.func_decl.push(
            FuncDecl {
                func_name: func_name.to_string(),
                func_type: func_type.clone(),
                param_types: ftypes,
            }
        );
    }

    pub fn push_func(&mut self, func_type: &SymbolType, func_name: &str, params: Vec<(String, SymbolType)>) {
        self.func_def.push(
            FuncDef {
                func_type: func_type.clone(),
                func_name: func_name.to_string(),
                params: vec![],
                blocks: vec![],
                local_vars: vec![]
            }
        );

        let curr_func = self.curr_func().unwrap();
        
        for (param_name, param_type) in params.into_iter() {
            curr_func.params.push(
                Param {
                    param_name,
                    param_type
                }
            );
        }
        
        curr_func.blocks.push(
            Block {
                block_label: "_entry".to_string(),
                phi_ins: Vec::new(),
                nor_ins: Vec::new(),
                ter_ins: None,
                ins_num: 0,
                depth: 0,
            }
        );
    }

    pub fn push_bb(&mut self, block_label: &str, scopes: &Scopes) {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        assert!(curr_bb_mut.ter_ins.is_some(), "It's not allowed to push a new basic block before pushing terminator instr.\n");
        
        let mut depth = scopes.get_depth();
        if block_label.contains("while_entry") {
            depth += 1;
        }

        curr_func.blocks.push(
            Block {
                block_label: block_label.to_string(),
                phi_ins: Vec::new(),
                nor_ins: Vec::new(),
                ter_ins: None,
                ins_num: curr_func.count_instr(),
                depth,
            }
        );
    }

    pub fn get_block_label(&mut self) -> String {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        String::from(&curr_bb_mut.block_label)
    }

    pub fn push_phi(&mut self, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        let phi_instr = Instruction::make_instruction(InstructionType::Phi, str_vec, ty_vec);
        curr_bb_mut.phi_ins.push(phi_instr);
    }

    pub fn push_comment(&mut self, cotent: &str) {
        let str_vec = vec!(cotent);
        self.push_instr(InstructionType::Comment, str_vec, vec!());
    }

    pub fn insert_instr(&mut self, instr: Instruction) {
        let curr_func = self.curr_func().expect(&format!("Push instr {:?} in void func", instr));
        let curr_bb_mut = curr_func.curr_bb_mut().expect(&format!("Push instr {:?} in void func", instr));
        curr_bb_mut.nor_ins.push(instr);
    }

    pub fn insert_alloc(&mut self, alloc: Instruction, bb_label: &str) {
        let curr_func = self.curr_func().expect(&format!("Push instr {:?} in void func", alloc));
        curr_func.local_vars.push(
            LocalVar { 
                ins: alloc, 
                label: bb_label.to_string() 
            }
        );
    }
    
    pub fn push_instr(&mut self, instr_type: InstructionType, str_vec: Vec<&str>, types: Vec<&SymbolType>) {
        let instr = Instruction::make_instruction(instr_type, str_vec, types);
        self.insert_instr(instr);
    }

    pub fn push_ter_instr(&mut self, ter_type: InstructionType, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        assert!(curr_bb_mut.ter_ins.is_none(), "Already pushed one terminator instr in this basic block before.");

        let ter_ins = Instruction::make_instruction(ter_type, str_vec, ty_vec);
        curr_bb_mut.ter_ins = Some(ter_ins);
    }
}

impl FuncDef {
    pub fn curr_bb(&self) -> Option<&Block> {
        self.blocks.last()
    }

    pub fn curr_bb_mut(&mut self) -> Option<&mut Block> {
        self.blocks.last_mut()
    }
    
    pub fn count_instr(&self) -> usize {
        let curr_bb = self.curr_bb().unwrap();
        curr_bb.count_instr()
    }
}

impl Block {
    fn count_self(&self) -> usize {
        self.phi_ins.len() 
            + self.nor_ins.len() 
            + self.ter_ins.is_some() as usize
    }

    pub fn count_instr(&self) -> usize {
        self.ins_num + self.count_self()
    }
}