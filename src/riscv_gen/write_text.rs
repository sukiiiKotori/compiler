use std::io;
use crate::structures::symbol::*;
use crate::structures::riscv_struct::*;
use crate::structures::writetext_trait::*;

fn width_name(width: isize) -> &'static str {
    match width {
       1 => "b",
       2 => "h",
       4 => "w",
       8 => "d",
       _ => todo!(),
    }
}

impl WriteText for RiscV {
    fn writetext(&self, output: &mut impl io::Write) {
        //声明改汇编为位置无关代码，便于链接器做静态链接
        write!(output, "\t.option nopic\n").unwrap();
        self.data.writetext(output);
        self.text.writetext(output);
    }
}

impl WriteText for DataSection {
    fn writetext(&self, output: &mut impl io::Write){
        if !self.datas.is_empty() {
            write!(output, "\t.section\t.data\n").unwrap();
            self.datas.iter().for_each(|data| data.writetext(output)); 
        }
    }
}

impl WriteText for TextSection {
    fn writetext(&self, output: &mut impl io::Write){
        self.funcs.iter().for_each(|fun| fun.writetext(output));
    }
}

impl WriteText for AsmFunc {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.text\n").unwrap();
        write!(output, "\t.align\t1\n").unwrap();
        write!(output, "\t.global\t{}\n", self.label).unwrap();
        write!(output, "\t.type\t{}, @function\n", self.label).unwrap();
        write!(output, "{}:\n", self.label).unwrap();
        self.blocks.iter().for_each(|block| block.writetext(output));
    }
}

impl WriteText for DataSectionItem {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.globl\t{}\n", self.label).unwrap();
        //因为类型只有i32和float或其数组，所以对齐值都是4B
        write!(output, "\t.align\t2\n").unwrap();
        write!(output, "\t.type\t{}, @object\n", self.label).unwrap();
        if let SymbolType{width: SymbolWidth::Arr{tar: _, dims}, is_const: _} = &self.ty {
            let array_size = dims.iter().fold(4, |acc, x| acc * x);
            write!(output, "\t.size\t{}, {}\n", self.label, array_size).unwrap();
            write!(output, "{}:\n", self.label).unwrap();
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t{}\n", array_size).unwrap();
            } else {
                self.init_vals.iter().for_each(|value| {
                    write!(output, "\t.word\t{}\n", value).unwrap();
                })
            }
        } else {
            write!(output, "\t.size\t{}, 4\n", self.label).unwrap();
            write!(output, "{}:\n", self.label).unwrap();
            match self.init_vals.first() {
                Some(value) => write!(output, "\t.word\t{}\n", value).unwrap(),
                None => write!(output, "\t.zero\t4\n").unwrap(),
            };
        }
    }
}

impl AsmBlock {
    fn writetext(&self, output: &mut impl io::Write) {
        write!(output, "{}:\n", self.label).unwrap();
        self.instrs.iter().for_each(|instr| instr.writetext(output));
    }
}

impl AsmInstruction {
    fn writetext(&self, output: &mut impl io::Write) {
        match self {
            AsmInstruction::Li(bin) => {
                write!(output, "\tli\t").unwrap();
                bin.writetext(output);
            },
            AsmInstruction::La(bin) => {
                write!(output, "\tla\t").unwrap();
                bin.writetext(output);
            },
            AsmInstruction::Mv(bin) => {
                write!(output, "\tmv\t").unwrap();
                bin.writetext(output);
            },
            AsmInstruction::Fmv(bin, dst, src) => {
                if dst == src {
                    write!(output, "\tfmv.d\t").unwrap();
                        
                } else {
                    match dst {
                        SymbolWidth::Float => {
                            write!(output, "\tfmv.w.x\t").unwrap();
                        },
                        SymbolWidth::I32 => {
                            write!(output, "\tfmv.x.w\t").unwrap();
                        },
                        _ => todo!(),
                    }
                }
                bin.writetext(output);
            },
            AsmInstruction::Addi(tri) => {
                write!(output, "\taddi\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Add(tri) => {
                write!(output, "\tadd\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Sub(tri) => {
                write!(output, "\tsub\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Mul(tri) => {
                if let TriInstr{width: Some(8), dst, op1, op2} = tri {
                    write!(output, "\tmul\t{}, {}, {}\n", dst, op1, op2).unwrap();
                    return;
                }
                write!(output, "\tmul").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Div(tri) => {
                write!(output, "\tdiv").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Rem(tri) => {
                write!(output, "\trem").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Fadd(tri) => {
                write!(output, "\tfadd.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Fsub(tri) => {
                write!(output, "\tfsub.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Fmul(tri) => {
                write!(output, "\tfmul.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Fdiv(tri) => {
                write!(output, "\tfdiv.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Xori(tri) => {
                write!(output, "\txori\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Slli(tri) => {
                write!(output, "\tslli\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Srli(tri) => {
                write!(output, "\tsrli\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Srai(tri) => {
                write!(output, "\tsrai\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Fcvt(bin, dst, _) => {
                if dst == &SymbolWidth::Float {
                    write!(output, "\tfcvt.s.w\t{}, {}, rtz\n", bin.dst, bin.src).unwrap();
                } else {
                    write!(output, "\tfcvt.w.s\t{}, {}, rtz\n", bin.dst, bin.src).unwrap();
                }
            },
            AsmInstruction::Slt(tri) => {
                write!(output, "\tslt\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Slti(tri) => {
                write!(output, "\tslti\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Sgt(tri) => {
                write!(output, "\tsgt\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Seqz(bin) => {
                write!(output, "\tseqz\t").unwrap();
                bin.writetext(output);
            },
            AsmInstruction::Snez(bin) => {
                write!(output, "\tsnez\t").unwrap();
                bin.writetext(output);
            },
            AsmInstruction::Flt(tri) => {
                write!(output, "\tflt.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Fle(tri) => {
                write!(output, "\tfle.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Feq(tri) => {
                write!(output, "\tfeq.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstruction::Store(mem, prefix) => {
                write!(output, "\t{}s", prefix).unwrap();
                mem.writetext(output);
            },
            AsmInstruction::Load(mem, prefix) => {
                write!(output, "\t{}l", prefix).unwrap();
                mem.writetext(output);
            },
            AsmInstruction::Branch(cond_tri) => {
                write!(output, "\tb").unwrap();
                cond_tri.writetext(output);
            },
            AsmInstruction::Jump(dst) => {
                write!(output, "\tj\t{}\n", dst).unwrap();
            },
            AsmInstruction::Ret() => {
                write!(output, "\tret\n").unwrap();
            },
            AsmInstruction::Call(_, func_name, _, _) => {
                if func_name != "memset" {
                    write!(output, "\tcall\t{}\n",func_name).unwrap();
                }
            },
        }
    }
}

impl WriteText for BinInstr {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "{}, {}\n", self.dst, self.src).unwrap();
    }
}

impl WriteText for CondTriInstr {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "{}\t", self.cond).unwrap();
        self.tri.writetext(output);
    }
}

impl WriteText for TriInstr {
    fn writetext(&self, output: &mut impl io::Write){
        if let Some(width) = self.width {
            write!(output, "{}\t{}, {}, {}\n", width_name(width), self.dst, self.op1, self.op2).unwrap();
        } else {
            write!(output, "{}, {}, {}\n", self.dst, self.op1, self.op2).unwrap();
        }
    }
}

impl WriteText for MemInstr {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "{}\t{}, {}({})\n", width_name(self.width), self.val, self.offset, self.base).unwrap();
    }
}

