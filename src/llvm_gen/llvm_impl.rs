use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;
use crate::llvm_gen::scopes::Scopes;

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
        self.func_decl.push(FuncDecl{
            func_id: String::from(func_name),
            func_type: func_type.clone(),
            ftypes: ftypes,
        });
    }

    pub fn push_func(&mut self, func_type: &SymbolType, func_name: &str, params: Vec<(String, SymbolType)>) {
        self.func_def.push(FuncDef{
            func_type: func_type.clone(),
            func_name: String::from(func_name),
            fparams: Vec::new(),
            blocks: Vec::new(),
            allocs: Vec::new(),
        });

        let curr_func = self.curr_func().unwrap();
        
        for (param_name, param_type) in params.into_iter() {
            curr_func.fparams.push(FParam{
                param_name: param_name,
                param_type: param_type,
            });
        }
        
        curr_func.blocks.push(BasicBlock{
            block_label: String::from("_entry"),
            phi_instr: Vec::new(),
            instrs: Vec::new(),
            ter_instr: None,
            instr_cnt: 0,
            depth: 0,
        });
    }

    pub fn push_bb(&mut self, block_label: &str, scopes: &Scopes) {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        assert!(curr_bb_mut.ter_instr.is_some(), "It's not allowed to push a new basic block before pushing terminator instr.\n");
        
        let mut depth = scopes.get_depth();
        if block_label.contains("while_entry") {
            depth += 1;
        }

        curr_func.blocks.push(BasicBlock{
            block_label: String::from(block_label),
            phi_instr: Vec::new(),
            instrs: Vec::new(),
            ter_instr: None,
            instr_cnt: curr_func.count_instr(),
            depth: depth,
        });
    }

    pub fn get_bb_label(&mut self) -> String {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        String::from(&curr_bb_mut.block_label)
    }

    pub fn push_phi(&mut self, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) {
        let curr_func = self.curr_func().unwrap();
        let curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        let phi_instr = Instr::make_instr(InstrType::Phi, str_vec, ty_vec);
        curr_bb_mut.phi_instr.push(phi_instr);
    }

    pub fn push_comment(&mut self, cotent: &str) {
        let str_vec = vec!(cotent);
        self.push_instr(InstrType::Comment, str_vec, vec!());
    }

    pub fn insert_instr(&mut self, instr: Instr) {
        let curr_func = self.curr_func().expect(&format!("Push instr {:?} in void func", instr));
        let curr_bb_mut = curr_func.curr_bb_mut().expect(&format!("Push instr {:?} in void func", instr));
        curr_bb_mut.instrs.push(instr);
    }

    pub fn insert_alloc(&mut self, alloc: Instr, bb_label: &str) {
        let curr_func = self.curr_func().expect(&format!("Push instr {:?} in void func", alloc));
        curr_func.allocs.push((alloc, String::from(bb_label)));
    }
    
    pub fn push_instr(&mut self, instr_type: InstrType, str_vec: Vec<&str>, types: Vec<&SymbolType>) {
        let instr = Instr::make_instr(instr_type, str_vec, types);
        self.insert_instr(instr);
    }

    pub fn push_ter_instr(&mut self, ter_type: InstrType, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) {
        let curr_func = self.curr_func().unwrap();
        let mut curr_bb_mut = curr_func.curr_bb_mut().unwrap();
        assert!(curr_bb_mut.ter_instr.is_none(), "Already pushed one terminator instr in this basic block before.");

        let ter_instr = Instr::make_instr(ter_type, str_vec, ty_vec);
        curr_bb_mut.ter_instr = Some(ter_instr);
    }
}

impl FuncDef {
    pub fn curr_bb(&self) -> Option<&BasicBlock> {
        self.blocks.last()
    }

    pub fn curr_bb_mut(&mut self) -> Option<&mut BasicBlock> {
        self.blocks.last_mut()
    }
    
    pub fn count_instr(&self) -> usize {
        let curr_bb = self.curr_bb().unwrap();
        curr_bb.count_instr()
    }
}

impl BasicBlock {
    pub fn count_self(&self) -> usize {
        self.phi_instr.len() 
            + self.instrs.len() 
            + self.ter_instr.as_ref().map_or(0, |_| 1)
    }

    pub fn count_instr(&self) -> usize {
        self.instr_cnt + self.count_self()
    }
}

impl Instr {
    pub fn make_instr(instr_type: InstrType, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) -> Self {
        let bin_op: BinaryOp;
        let conver_op: ConverOp;

        if str_vec.len() >= 3 && ty_vec.len() >= 1{
            bin_op = BinaryOp {
                result: String::from(str_vec[0]),
                ty: ty_vec[0].clone(),
                op1: String::from(str_vec[1]),
                op2: String::from(str_vec[2]),
            };
        } else {
            bin_op = BinaryOp::new();
        }

        if str_vec.len() >= 2 && ty_vec.len() >= 2{
            conver_op = ConverOp {
                result: String::from(str_vec[0]),
                ty: ty_vec[0].clone(),
                value: String::from(str_vec[1]),
                ty2: ty_vec[1].clone(),
            };
        } else {
            conver_op = ConverOp::new();
        }

        match instr_type {
            InstrType::Add => Instr::Add(bin_op),
            InstrType::Sub => Instr::Sub(bin_op),
            InstrType::Mul => Instr::Mul(bin_op),
            InstrType::Sdiv => Instr::Sdiv(bin_op),
            InstrType::Srem => Instr::Srem(bin_op),
            InstrType::Fadd => Instr::Fadd(bin_op),
            InstrType::Fsub => Instr::Fsub(bin_op),
            InstrType::Fmul => Instr::Fmul(bin_op),
            InstrType::Fdiv => Instr::Fdiv(bin_op),
            InstrType::Zext => Instr::Zext(conver_op),
            InstrType::Sitofp => Instr::Sitofp(conver_op),
            InstrType::Fptosi => Instr::Fptosi(conver_op),
            InstrType::Icmp => Instr::Icmp(
                String::from(str_vec[0]),
                BinaryOp{
                    result: String::from(str_vec[1]),
                    ty: ty_vec[0].clone(),
                    op1: String::from(str_vec[2]),
                    op2: String::from(str_vec[3]),
                },
            ),
            InstrType::Fcmp => {
                let mut cond = String::from(str_vec[0]);
                if cond.contains("s") {
                    cond = cond.replace("s", "o");
                } else if !cond.contains("o") {
                    cond.insert(0, 'o');
                }
                Instr::Fcmp(
                    cond,
                    BinaryOp{
                    result: String::from(str_vec[1]),
                        ty: ty_vec[0].clone(),
                        op1: String::from(str_vec[2]),
                        op2: String::from(str_vec[3]),
                    },
                )
            },
            InstrType::Phi => {
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
                Instr::Phi(res, ty_vec[0].clone(), candidates)
            },
            InstrType::Alloca => Instr::Alloca {
                result: String::from(str_vec[0]),
                ty: ty_vec[0].clone(), 
                len: String::from(str_vec[1]),
            },
            InstrType::Store => Instr::Store {
                ty: ty_vec[0].clone(),
                value: String::from(str_vec[0]),
                ptr: String::from(str_vec[1]),
                len: String::from(str_vec[2]),
            },
            InstrType::Load => Instr::Load {
                result: String::from(str_vec[0]),
                ty: ty_vec[0].clone(), 
                ptr: String::from(str_vec[1]),
                len: String::from(str_vec[2]),
            },
            InstrType::Call => {
                let mut params: Vec<(String, SymbolType)> = vec!();
                for cnt in 2..str_vec.len() {
                    params.push((String::from(str_vec[cnt]), ty_vec[cnt-1].clone()));
                }
                Instr::Call (
                    String::from(str_vec[0]),
                    String::from(str_vec[1]),
                    ty_vec[0].clone(),
                    params,
                )
            },
            InstrType::GetElemPtr => {
                let mut idx: Vec<String> = vec!();
                for cnt in 2..str_vec.len() {
                    idx.push(String::from(str_vec[cnt]));
                }
                Instr::GetElemPtr (
                    String::from(str_vec[0]),
                    ty_vec[0].clone(),
                    String::from(str_vec[1]),
                    idx,
                )
            },
            InstrType::BitCast => Instr::BitCast (
                String::from(str_vec[0]),
                ty_vec[0].clone(),
                String::from(str_vec[1]),
                ty_vec[1].clone(),
            ),// InstrType
            InstrType::Comment => Instr::Comment (
                String::from(str_vec[0]),
            ),
            InstrType::Return => {
                if str_vec.is_empty() {
                    Instr::Return(ty_vec[0].clone(), None)
                } else {
                    Instr::Return(ty_vec[0].clone(), Some(String::from(str_vec[0])))
                }
            },
            InstrType::Branch => {
                if str_vec[0] == "" {
                    Instr::Branch(None, String::from(str_vec[1]), None)
                } else {
                    Instr::Branch(Some(String::from(str_vec[0])), String::from(str_vec[1]), Some(String::from(str_vec[2])))
                }
            }
        } // match
    } // fn
}

