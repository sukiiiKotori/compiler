use std::io;

use crate::structures::symbol::*;
use crate::structures::riscv_struct::*;
use crate::structures::writetext_trait::*;
use crate::utils::float::double_to_float;
use crate::utils::check::*;

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
        write!(output, "\t.option nopic\n").unwrap();
        self.rodata.writetext(output);
        self.data.writetext(output);
        self.text.writetext(output);
    }
}

impl WriteText for RoDataSection {
    fn writetext(&self, output: &mut impl io::Write){
        if self.datas.is_empty() {
            return ;
        }
        write!(output, "\t.text\n").unwrap();
        write!(output, "\t.section\t.rodata\n").unwrap();
        for data in self.datas.iter() {
            data.writetext(output);
        }
    }
}

impl WriteText for DataSection {
    fn writetext(&self, output: &mut impl io::Write){
        if self.datas.is_empty() {
            return ;
        }
        write!(output, "\t.text\n").unwrap();
        write!(output, "\t.section\t.data\n").unwrap();
        for data in self.datas.iter() {
            data.writetext(output);
        }
    }
}

fn writetext_init(output: &mut impl io::Write, init_vals: &Vec<String>) {
    // 状态码
    // 0 => 初始状态
    // 1 => 读取记录零的长度的状态
    let mut state = 0;
    let mut cnt = 0;
    for val in init_vals.iter() {
        match val.as_str() {
            "0" | "0.0" => {
                if state == 0 {
                    state = 1;
                }
                cnt += 4;
            },
            _ => {
                if state == 1 {
                    write!(output, "\t.zero\t{}\n", cnt).unwrap();
                    state = 0;
                    cnt = 0;
                }
                if is_hex(val.as_str()) && val.len() == 18 {
                    write!(output, "\t.word\t{}\n", double_to_float(val.as_str())).unwrap();
                } else {
                    write!(output, "\t.word\t{}\n", val).unwrap();
                }
            },
        }
    }
    if state == 1 {
        write!(output, "\t.zero\t{}\n", cnt).unwrap();
    }
}

impl WriteText for DataSectionItem {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.globl\t{}\n", self.label).unwrap();
        write!(output, "\t.align\t2\n").unwrap();
        write!(output, "\t.type\t{}, @object\n", self.label).unwrap();
        if let SymbolType{width: SymbolWidth::Arr{tar: _, dims}, is_const: _} = &self.ty {
            let array_size = dims.iter().map(|d| *d as usize).product::<usize>()*4;
            write!(output, "\t.size\t{}, {}\n", self.label, array_size).unwrap();
            write!(output, "{}:\n", self.label).unwrap();
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t{}\n", array_size).unwrap();
            } else {
                writetext_init(output, &self.init_vals);
            }
        } else {
            write!(output, "\t.size\t{}, 4\n", self.label).unwrap();
            write!(output, "{}:\n", self.label).unwrap();
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t4\n").unwrap();
            } else {
                let val = self.init_vals.get(0).unwrap();
                if is_hex(val.as_str()) && val.len() == 18 {
                    write!(output, "\t.word\t{}\n", double_to_float(val.as_str())).unwrap();
                } else {
                    write!(output, "\t.word\t{}\n", val).unwrap();
                }
            }
        }
    }
}

impl WriteText for TextSection {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.text\n").unwrap();
        write!(output, "\t.align\t1\n").unwrap();
        for func in self.funcs.iter() {
            func.writetext(output);
        }
    }
}

impl WriteText for AsmFunc {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.global\t{}\n", self.label).unwrap();
        write!(output, "\t.type\t{}, @function\n", self.label).unwrap();
        write!(output, "{}:\n", self.label).unwrap();
        for block in self.blocks.iter() {
            block.writetext(output);
        }
    }
}

impl AsmBlock {
    fn writetext(&self, output: &mut impl io::Write) {
        if !self.label.contains("._entry") {
            write!(output, "{}:\n", self.label).unwrap();
        }
        for instr in self.instrs.iter() {
            instr.writetext(output);
        }
    }
}

impl AsmInstr {
    fn writetext(&self, output: &mut impl io::Write) {
        match self {
            AsmInstr::Li(bin) => {
                write!(output, "\tli\t").unwrap();
                bin.writetext(output);
            },
            AsmInstr::La(bin) => {
                write!(output, "\tla\t").unwrap();
                bin.writetext(output);
            },
            AsmInstr::Mv(bin) => {
                write!(output, "\tmv\t").unwrap();
                bin.writetext(output);
            },
            AsmInstr::Fmv(bin, dst, src) => {
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
            AsmInstr::Sextw(bin) => {
                write!(output, "\tsext.w\t").unwrap();
                bin.writetext(output);
            },
            AsmInstr::Addi(tri) => {
                write!(output, "\taddi\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Add(tri) => {
                write!(output, "\tadd\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Sub(tri) => {
                write!(output, "\tsub\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Mul(tri) => {
                if let TriInstr{width: Some(8), dst, op1, op2} = tri {
                    write!(output, "\tmul\t{}, {}, {}\n", dst, op1, op2).unwrap();
                    return;
                }
                write!(output, "\tmul").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Div(tri) => {
                write!(output, "\tdiv").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Rem(tri) => {
                write!(output, "\trem").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Fadd(tri) => {
                write!(output, "\tfadd.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Fsub(tri) => {
                write!(output, "\tfsub.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Fmul(tri) => {
                write!(output, "\tfmul.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Fdiv(tri) => {
                write!(output, "\tfdiv.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Xori(tri) => {
                write!(output, "\txori\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Slli(tri) => {
                write!(output, "\tslli\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Srli(tri) => {
                write!(output, "\tsrli\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Srai(tri) => {
                write!(output, "\tsrai\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Fcvt(bin, dst, src) => {
                if dst == src {
                    panic!("Two types should be different");
                } else {
                    if *dst == SymbolWidth::Float {
                        write!(output, "\tfcvt.s.w\t{}, {}, rtz\n", bin.dst, bin.src).unwrap();
                    } else {
                        write!(output, "\tfcvt.w.s\t{}, {}, rtz\n", bin.dst, bin.src).unwrap();
                    }
                }
            },
            AsmInstr::Slt(tri) => {
                write!(output, "\tslt\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Slti(tri) => {
                write!(output, "\tslti\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Sgt(tri) => {
                write!(output, "\tsgt\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Seqz(bin) => {
                write!(output, "\tseqz\t").unwrap();
                bin.writetext(output);
            },
            AsmInstr::Snez(bin) => {
                write!(output, "\tsnez\t").unwrap();
                bin.writetext(output);
            },
            AsmInstr::Flt(tri) => {
                write!(output, "\tflt.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Fle(tri) => {
                write!(output, "\tfle.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Feq(tri) => {
                write!(output, "\tfeq.s\t").unwrap();
                tri.writetext(output);
            },
            AsmInstr::Store(mem, prefix) => {
                write!(output, "\t{}s", prefix).unwrap();
                mem.writetext(output);
            },
            AsmInstr::Load(mem, prefix) => {
                write!(output, "\t{}l", prefix).unwrap();
                mem.writetext(output);
            },
            AsmInstr::Branch(cond_tri) => {
                write!(output, "\tb").unwrap();
                cond_tri.writetext(output);
            },
            AsmInstr::Jump(dst) => {
                write!(output, "\tj\t{}\n", dst).unwrap();
            },
            AsmInstr::Ret() => {
                write!(output, "\tret\n").unwrap();
            },
            AsmInstr::Call(_, func_name, _, _) => {
                write!(output, "\tcall\t{}\n",func_name).unwrap();
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

