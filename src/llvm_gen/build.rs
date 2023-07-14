use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;
use crate::structures::scopes::*;

impl LLVMProgram {
    fn curr_func(&mut self) -> Option<&mut FuncDef> {
        self.func_def.last_mut()
    }

    pub fn push_global_var(&mut self, id: &String, ty: &SymbolType, init_types: Vec<&SymbolType>, init_vals: Vec<&String>) {
        let mut types: Vec<SymbolType> = vec!();
        let mut vals: Vec<String> = vec!();
        let mut init_numbers: Vec<InitNumber> = Vec::new();

        for init_type in init_types.iter() {
            types.push((*init_type).clone());
        }
        for init_val in init_vals.iter() {
            vals.push(String::from(*init_val));
        }
        
        for i in 0..types.len() {
            let init_number = InitNumber {
                init_type: types[i].clone(),
                init_val: vals[i].clone(),
            };
            init_numbers.push(init_number);
        }
        let global_var = GlobalVar {
            var_name: String::from(id),
            var_type: ty.clone(),
            init_num: init_numbers,
        };
        self.global_var.push(global_var);
        
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

impl Instruction {
    pub fn make_instruction(instr_type: InstructionType, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) -> Self {
        let bin_op: BinaryOp;
        let conver_op: CastOp;

        if str_vec.len() >= 3 && ty_vec.len() >= 1{
            bin_op = BinaryOp {
                res: String::from(str_vec[0]),
                op_type: ty_vec[0].clone(),
                op1: String::from(str_vec[1]),
                op2: String::from(str_vec[2]),
            };
        } else {
            bin_op = BinaryOp::new();
        }

        if str_vec.len() >= 2 && ty_vec.len() >= 2{
            conver_op = CastOp {
                res: String::from(str_vec[0]),
                type_1: ty_vec[0].clone(),
                type_2: ty_vec[1].clone(),
                val: String::from(str_vec[1]),
            };
        } else {
            conver_op = CastOp::new();
        }

        match instr_type {
            InstructionType::Add => Instruction::Add(bin_op),
            InstructionType::Sub => Instruction::Sub(bin_op),
            InstructionType::Mul => Instruction::Mul(bin_op),
            InstructionType::Sdiv => Instruction::Sdiv(bin_op),
            InstructionType::Srem => Instruction::Srem(bin_op),
            InstructionType::Fadd => Instruction::Fadd(bin_op),
            InstructionType::Fsub => Instruction::Fsub(bin_op),
            InstructionType::Fmul => Instruction::Fmul(bin_op),
            InstructionType::Fdiv => Instruction::Fdiv(bin_op),
            InstructionType::ZeroExt => Instruction::ZeroExt(conver_op),
            InstructionType::I32ToFloat => Instruction::I32ToFloat(conver_op),
            InstructionType::FloatToI32 => Instruction::FloatToI32(conver_op),
            InstructionType::Cmp => Instruction::Cmp(
                String::from(str_vec[0]),
                BinaryOp{
                    res: String::from(str_vec[1]),
                    op_type: ty_vec[0].clone(),
                    op1: String::from(str_vec[2]),
                    op2: String::from(str_vec[3]),
                },
            ),
            InstructionType::Fcmp => {
                let mut cond = String::from(str_vec[0]);
                if cond.contains("s") {
                    cond = cond.replace("s", "o");
                } else if !cond.contains("o") {
                    cond.insert(0, 'o');
                }
                Instruction::Fcmp(
                    cond,
                    BinaryOp{
                        res: String::from(str_vec[1]),
                        op_type: ty_vec[0].clone(),
                        op1: String::from(str_vec[2]),
                        op2: String::from(str_vec[3]),
                    },
                )
            },
            InstructionType::Phi => {
                assert!(str_vec.len() % 2 != 0, "Phi candidates number error.");
                let mut new_vec = str_vec;
                new_vec.reverse();
                let res = String::from(new_vec.pop().unwrap());
                let mut candidates: Vec<(String, String)> = vec!();
                while !new_vec.is_empty() {
                    candidates.push(
                        (String::from(new_vec.pop().unwrap()), 
                         String::from(new_vec.pop().unwrap())),
                    );
                }
                Instruction::Phi(res, ty_vec[0].clone(), candidates)
            },
            InstructionType::Alloca => Instruction::Alloca {
                res: String::from(str_vec[0]),
                ty: ty_vec[0].clone(), 
                len: String::from(str_vec[1]),
            },
            InstructionType::Store => Instruction::Store {
                ty: ty_vec[0].clone(),
                value: String::from(str_vec[0]),
                ptr: String::from(str_vec[1]),
                len: String::from(str_vec[2]),
            },
            InstructionType::Load => Instruction::Load {
                res: String::from(str_vec[0]),
                ty: ty_vec[0].clone(), 
                ptr: String::from(str_vec[1]),
                len: String::from(str_vec[2]),
            },
            InstructionType::Call => {
                let mut params: Vec<(String, SymbolType)> = vec!();
                for cnt in 2..str_vec.len() {
                    params.push((String::from(str_vec[cnt]), ty_vec[cnt-1].clone()));
                }
                Instruction::Call (
                    String::from(str_vec[0]),
                    String::from(str_vec[1]),
                    ty_vec[0].clone(),
                    params,
                )
            },
            InstructionType::GetElemPtr => {
                let mut idx: Vec<String> = vec!();
                for cnt in 2..str_vec.len() {
                    idx.push(String::from(str_vec[cnt]));
                }
                Instruction::GetElemPtr (
                    String::from(str_vec[0]),
                    ty_vec[0].clone(),
                    String::from(str_vec[1]),
                    idx,
                )
            },
            InstructionType::BitCast => Instruction::BitCast (
                String::from(str_vec[0]),
                ty_vec[0].clone(),
                String::from(str_vec[1]),
                ty_vec[1].clone(),
            ),
            InstructionType::Comment => Instruction::Comment (
                String::from(str_vec[0]),
            ),
            InstructionType::Ret => {
                if str_vec.is_empty() {
                    Instruction::Ret(ty_vec[0].clone(), None)
                } else {
                    Instruction::Ret(ty_vec[0].clone(), Some(String::from(str_vec[0])))
                }
            },
            InstructionType::Br => {
                if str_vec[0] == "" {
                    Instruction::Br(None, String::from(str_vec[1]), None)
                } else {
                    Instruction::Br(Some(String::from(str_vec[0])), String::from(str_vec[1]), Some(String::from(str_vec[2])))
                }
            }
        } // match
    } // fn
}

