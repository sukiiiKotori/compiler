use std::io;
use std::error::Error;

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
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.option nopic\n");
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
        write!(output, "\t.text\n");
        write!(output, "\t.section\t.rodata\n");
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
        write!(output, "\t.text\n");
        write!(output, "\t.section\t.data\n");
        for data in self.datas.iter() {
            data.writetext(output);
        }
    }
}

fn writetext_init(output: &mut impl io::Write, init_vals: &Vec<String>) -> Result<(), Box<dyn Error>>{
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
                    write!(output, "\t.zero\t{}\n", cnt)?;
                    state = 0;
                    cnt = 0;
                }
                if is_hex(val.as_str()) && val.len() == 18 {
                    write!(output, "\t.word\t{}\n", double_to_float(val.as_str()))?;
                } else {
                    write!(output, "\t.word\t{}\n", val)?;
                }
            },
        }
    }
    if state == 1 {
        write!(output, "\t.zero\t{}\n", cnt)?;
    }
    Ok(())
}

impl WriteText for DataSectionItem {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.globl\t{}\n", self.label);
        write!(output, "\t.align\t2\n");
        write!(output, "\t.type\t{}, @object\n", self.label);
        if let SymbolType{width: SymbolWidth::Arr{tar: _, dims}, is_const: _} = &self.ty {
            let array_size = dims.iter().map(|d| *d as usize).product::<usize>()*4;
            write!(output, "\t.size\t{}, {}\n", self.label, array_size);
            write!(output, "{}:\n", self.label);
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t{}\n", array_size);
            } else {
                writetext_init(output, &self.init_vals);
            }
        } else {
            write!(output, "\t.size\t{}, 4\n", self.label);
            write!(output, "{}:\n", self.label);
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t4\n");
            } else {
                let val = self.init_vals.get(0).unwrap();
                if is_hex(val.as_str()) && val.len() == 18 {
                    write!(output, "\t.word\t{}\n", double_to_float(val.as_str()));
                } else {
                    write!(output, "\t.word\t{}\n", val);
                }
            }
        }
    }
}

impl WriteText for TextSection {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.text\n");
        write!(output, "\t.align\t1\n");
        for func in self.funcs.iter() {
            func.writetext(output);
        }
    }
}

impl WriteText for AsmFunc {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "\t.global\t{}\n", self.label);
        write!(output, "\t.type\t{}, @function\n", self.label);
        write!(output, "{}:\n", self.label);
        for block in self.blocks.iter() {
            block.writetext(output);
        }
    }
}

impl AsmBlock {
    fn writetext(&self, output: &mut impl io::Write) -> Result<(), Box<dyn Error>> {
        if !self.label.contains("._entry") {
            write!(output, "{}:\n", self.label)?;
        }
        for instr in self.instrs.iter() {
            instr.writetext(output)?;
        }
        Ok(())
    }
}

impl AsmInstr {
    fn writetext(&self, output: &mut impl io::Write) -> Result<(), Box<dyn Error>> {
        match self {
            AsmInstr::Li(bin) => {
                write!(output, "\tli\t")?;
                bin.writetext(output);
            },
            AsmInstr::La(bin) => {
                write!(output, "\tla\t")?;
                bin.writetext(output);
            },
            AsmInstr::Mv(bin) => {
                write!(output, "\tmv\t")?;
                bin.writetext(output);
            },
            AsmInstr::Fmv(bin, dst, src) => {
                if dst == src {
                    match dst {
                        SymbolWidth::Float => {
                            write!(output, "\tfmv.d\t")?;
                        },
                        _ => todo!(),
                    }
                } else {
                    match dst {
                        SymbolWidth::Float => {
                            write!(output, "\tfmv.w.x\t")?;
                        },
                        SymbolWidth::I32 => {
                            write!(output, "\tfmv.x.w\t")?;
                        },
                        _ => todo!(),
                    }
                }
                bin.writetext(output);
            },
            AsmInstr::Sextw(bin) => {
                write!(output, "\tsext.w\t")?;
                bin.writetext(output);
            },
            AsmInstr::Addi(tri) => {
                write!(output, "\taddi\t")?;
                tri.writetext(output);
            },
            AsmInstr::Add(tri) => {
                write!(output, "\tadd\t")?;
                tri.writetext(output);
            },
            AsmInstr::Sub(tri) => {
                write!(output, "\tsub\t")?;
                tri.writetext(output);
            },
            AsmInstr::Mul(tri) => {
                if let TriInstr{width: Some(8), dst, op1, op2} = tri {
                    write!(output, "\tmul\t{}, {}, {}\n", dst, op1, op2)?;
                    return Ok(());
                }
                write!(output, "\tmul")?;
                tri.writetext(output);
            },
            AsmInstr::Div(tri) => {
                write!(output, "\tdiv")?;
                tri.writetext(output);
            },
            AsmInstr::Rem(tri) => {
                write!(output, "\trem")?;
                tri.writetext(output);
            },
            AsmInstr::Fadd(tri) => {
                write!(output, "\tfadd.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Fsub(tri) => {
                write!(output, "\tfsub.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Fmul(tri) => {
                write!(output, "\tfmul.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Fdiv(tri) => {
                write!(output, "\tfdiv.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Xori(tri) => {
                write!(output, "\txori\t")?;
                tri.writetext(output);
            },
            AsmInstr::Fcvt(bin, dst, src) => {
                if dst == src {
                    panic!("Two types should be different");
                } else {
                    if *dst == SymbolWidth::Float {
                        write!(output, "\tfcvt.s.w\t{}, {}, rtz\n", bin.dst, bin.src)?;
                    } else {
                        write!(output, "\tfcvt.w.s\t{}, {}, rtz\n", bin.dst, bin.src)?;
                    }
                }
            },
            AsmInstr::Slt(tri) => {
                write!(output, "\tslt\t")?;
                tri.writetext(output);
            },
            AsmInstr::Slti(tri) => {
                write!(output, "\tslti\t")?;
                tri.writetext(output);
            },
            AsmInstr::Sgt(tri) => {
                write!(output, "\tsgt\t")?;
                tri.writetext(output);
            },
            AsmInstr::Seqz(bin) => {
                write!(output, "\tseqz\t")?;
                bin.writetext(output);
            },
            AsmInstr::Snez(bin) => {
                write!(output, "\tsnez\t")?;
                bin.writetext(output);
            },
            AsmInstr::Flt(tri) => {
                write!(output, "\tflt.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Fle(tri) => {
                write!(output, "\tfle.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Feq(tri) => {
                write!(output, "\tfeq.s\t")?;
                tri.writetext(output);
            },
            AsmInstr::Store(mem, prefix) => {
                write!(output, "\t{}s", prefix)?;
                mem.writetext(output);
            },
            AsmInstr::Load(mem, prefix) => {
                write!(output, "\t{}l", prefix)?;
                mem.writetext(output);
            },
            AsmInstr::Branch(cond_tri) => {
                write!(output, "\tb")?;
                cond_tri.writetext(output);
            },
            AsmInstr::Jump(dst) => {
                write!(output, "\tj\t{}\n", dst)?;
            },
            AsmInstr::Ret(_) => {
                write!(output, "\tret\n")?;
            },
            AsmInstr::Call(_, func_name, _, _) => {
                write!(output, "\tcall\t{}\n",func_name)?;
            },
        }
        Ok(())
    }
}

impl WriteText for BinInstr {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "{}, {}\n", self.dst, self.src);
    }
}

impl WriteText for CondTriInstr {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "{}\t", self.cond);
        self.tri.writetext(output);
    }
}

impl WriteText for TriInstr {
    fn writetext(&self, output: &mut impl io::Write){
        if let Some(width) = self.width {
            write!(output, "{}\t{}, {}, {}\n", width_name(width), self.dst, self.op1, self.op2);
        } else {
            write!(output, "{}, {}, {}\n", self.dst, self.op1, self.op2);
        }
    }
}

impl WriteText for MemInstr {
    fn writetext(&self, output: &mut impl io::Write){
        write!(output, "{}\t{}, {}({})\n", width_name(self.width), self.val, self.offset, self.base);
    }
}

