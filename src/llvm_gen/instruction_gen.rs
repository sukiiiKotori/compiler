use crate::structures::llvm_struct::*;
use crate::structures::symbol::*;

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
    pub fn update_instruction(&mut self, str_vec: Vec<&str>, ty_vec: Vec<&SymbolType>) {
        let bin_op: BinaryOp;
        let cast_op: CastOp;

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
            cast_op = CastOp {
                res: String::from(str_vec[0]),
                type_1: ty_vec[0].clone(),
                type_2: ty_vec[1].clone(),
                val: String::from(str_vec[1]),
            };
        } else {
            cast_op = CastOp::new();
        }

        match self {
            Instruction::Add(bin) => *bin = bin_op,
            Instruction::Sub(bin) => *bin = bin_op,
            Instruction::Mul(bin) => *bin = bin_op,
            Instruction::Sdiv(bin) => *bin = bin_op,
            Instruction::Srem(bin) => *bin = bin_op,
            Instruction::Fadd(bin) => *bin = bin_op,
            Instruction::Fsub(bin) => *bin = bin_op,
            Instruction::Fmul(bin) => *bin = bin_op,
            Instruction::Fdiv(bin) => *bin = bin_op,
            Instruction::ZeroExt(cast) => *cast = cast_op,
            Instruction::I32ToFloat(cast) => *cast = cast_op,
            Instruction::FloatToI32(cast) => *cast = cast_op,
            Instruction::Cmp(cond, bin) => {
                *cond = str_vec[0].to_string();
                *bin = BinaryOp {
                    res: str_vec[1].to_string(),
                    op_type: ty_vec[0].clone(),
                    op1: str_vec[2].to_string(),
                    op2: str_vec[3].to_string(),
                };
            },
            Instruction::Fcmp(cond, bin) => {
                *cond = str_vec[0].to_string();
                *bin = BinaryOp {
                    res: str_vec[1].to_string(),
                    op_type: ty_vec[0].clone(),
                    op1: str_vec[2].to_string(),
                    op2: str_vec[3].to_string(),
                };
            },
            Instruction::Phi(res_old, ty, branches) => {
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
                *res_old = res;
                *ty = ty_vec[0].clone();
                *branches = candidates;
            },
            Instruction::Alloca{res, ty, len} => {
                *res = String::from(str_vec[0]);
                *ty = ty_vec[0].clone();
                *len = String::from(str_vec[1]);
            },
            Instruction::Store{ty, value, ptr, len} => {
                *ty = ty_vec[0].clone();
                *value = String::from(str_vec[0]);
                *ptr = String::from(str_vec[1]);
                *len = String::from(str_vec[2]);
            },
            Instruction::Load{res, ty, ptr, len} => {
                *res = String::from(str_vec[0]);
                *ty = ty_vec[0].clone();
                *ptr = String::from(str_vec[1]);
                *len = String::from(str_vec[2]);
            },
            Instruction::Call(res, label, ty, params_old) => {
                let mut params: Vec<(String, SymbolType)> = vec!();
                for cnt in 2..str_vec.len() {
                    params.push((String::from(str_vec[cnt]), ty_vec[cnt-1].clone()));
                }
                *res = str_vec[0].to_string();
                *label = str_vec[1].to_string();
                *ty = ty_vec[0].clone();
                *params_old = params;
            },
            Instruction::GetElemPtr(dst, ty, ptr, idx_old) => {
                let mut idx: Vec<String> = vec!();
                for cnt in 2..str_vec.len() {
                    idx.push(String::from(str_vec[cnt]));
                }
                *dst = str_vec[0].to_string();
                *ty = ty_vec[0].clone();
                *ptr = str_vec[1].to_string();
                *idx_old = idx;
            },
            Instruction::BitCast(res, ty_src, val, ty_dst) => {
                *res = str_vec[0].to_string();
                *ty_src = ty_vec[0].clone();
                *val = str_vec[1].to_string();
                *ty_dst = ty_vec[1].clone();
            },
            Instruction::Comment(message) => *message = str_vec[0].to_string(),
            Instruction::Ret(ty, value) => {
                //如果没有返回值
                *ty = ty_vec[0].clone();
                if str_vec.is_empty() {
                    *value = None;
                } else {
                    *value = Some(str_vec[0].to_string());
                }
            },
            Instruction::Br(cond, and_true, and_false) => {
                //如果没有条件
                *and_true = str_vec[1].to_string();
                if !str_vec[0].is_empty() {
                    *cond = Some(str_vec[0].to_string());
                    *and_false = Some(str_vec[2].to_string());
                }
            }
        } // match
    }
}

