use std::io;
use std::error::Error;

use crate::sym_type::*;
use crate::riscv::*;
use crate::midend::dump::Dump;
use crate::float::double_to_float;

fn width_name(width: isize) -> &'static str {
    match width {
       1 => "b",
       2 => "h",
       4 => "w",
       8 => "d",
       _ => todo!(),
    }
}

impl Dump for RiscV {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "\t.option nopic\n")?;
        self.rodata.dump(output)?;
        self.data.dump(output)?;
        self.text.dump(output)?;
        Ok(())
    }
}

impl Dump for RoDataSection {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        if self.datas.is_empty() {
            return Ok(());
        }
        write!(output, "\t.text\n")?;
        write!(output, "\t.section\t.rodata\n")?;
        for data in self.datas.iter() {
            data.dump(output)?;
        }
        Ok(())
    }
}

impl Dump for DataSection {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        if self.datas.is_empty() {
            return Ok(());
        }
        write!(output, "\t.text\n")?;
        write!(output, "\t.section\t.data\n")?;
        for data in self.datas.iter() {
            data.dump(output)?;
        }
        Ok(())
    }
}

fn dump_init(output: &mut impl io::Write, init_vals: &Vec<String>) -> Result<(), Box<dyn Error>>{
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
                if crate::util::is_hex(val.as_str()) && val.len() == 18 {
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

impl Dump for DataSectionItem {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "\t.globl\t{}\n", self.label)?;
        write!(output, "\t.align\t2\n")?;
        write!(output, "\t.type\t{}, @object\n", self.label)?;
        if let SymType{width: Width::Arr{tar: _, dims}, is_const: _} = &self.ty {
            let array_size = dims.iter().map(|d| *d as usize).product::<usize>()*4;
            write!(output, "\t.size\t{}, {}\n", self.label, array_size)?;
            write!(output, "{}:\n", self.label)?;
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t{}\n", array_size)?;
            } else {
                dump_init(output, &self.init_vals)?;
            }
        } else {
            write!(output, "\t.size\t{}, 4\n", self.label)?;
            write!(output, "{}:\n", self.label)?;
            if self.init_vals.is_empty() {
                write!(output, "\t.zero\t4\n")?;
            } else {
                let val = self.init_vals.get(0).unwrap();
                if crate::util::is_hex(val.as_str()) && val.len() == 18 {
                    write!(output, "\t.word\t{}\n", double_to_float(val.as_str()))?;
                } else {
                    write!(output, "\t.word\t{}\n", val)?;
                }
            }
        }
        Ok(())
    }
}

impl Dump for TextSection {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "\t.text\n")?;
        write!(output, "\t.align\t1\n")?;
        for func in self.funcs.iter() {
            func.dump(output)?;
        }
        Ok(())
    }
}

impl Dump for AsmFunc {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "\t.global\t{}\n", self.label)?;
        write!(output, "\t.type\t{}, @function\n", self.label)?;
        write!(output, "{}:\n", self.label)?;
        for block in self.blocks.iter() {
            block.dump(output)?;
        }
        Ok(())
    }
}

impl AsmBlock {
    fn dump(&self, output: &mut impl io::Write) -> Result<(), Box<dyn Error>> {
        if !self.label.contains("._entry") {
            write!(output, "{}:\n", self.label)?;
        }
        for instr in self.instrs.iter() {
            instr.dump(output)?;
        }
        Ok(())
    }
}

impl AsmInstr {
    fn dump(&self, output: &mut impl io::Write) -> Result<(), Box<dyn Error>> {
        match self {
            AsmInstr::Li(bin) => {
                write!(output, "\tli\t")?;
                bin.dump(output)?;
            },
            AsmInstr::La(bin) => {
                write!(output, "\tla\t")?;
                bin.dump(output)?;
            },
            AsmInstr::Mv(bin) => {
                write!(output, "\tmv\t")?;
                bin.dump(output)?;
            },
            AsmInstr::Fmv(bin, dst, src) => {
                if dst == src {
                    match dst {
                        Width::Float => {
                            write!(output, "\tfmv.d\t")?;
                        },
                        _ => todo!(),
                    }
                } else {
                    match dst {
                        Width::Float => {
                            write!(output, "\tfmv.w.x\t")?;
                        },
                        Width::I32 => {
                            write!(output, "\tfmv.x.w\t")?;
                        },
                        _ => todo!(),
                    }
                }
                bin.dump(output)?;
            },
            AsmInstr::Sextw(bin) => {
                write!(output, "\tsext.w\t")?;
                bin.dump(output)?;
            },
            AsmInstr::Addi(tri) => {
                write!(output, "\taddi\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Add(tri) => {
                write!(output, "\tadd\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Sub(tri) => {
                write!(output, "\tsub\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Mul(tri) => {
                if let TriInstr{width: Some(8), dst, op1, op2} = tri {
                    write!(output, "\tmul\t{}, {}, {}\n", dst, op1, op2)?;
                    return Ok(());
                }
                write!(output, "\tmul")?;
                tri.dump(output)?;
            },
            AsmInstr::Div(tri) => {
                write!(output, "\tdiv")?;
                tri.dump(output)?;
            },
            AsmInstr::Rem(tri) => {
                write!(output, "\trem")?;
                tri.dump(output)?;
            },
            AsmInstr::Fadd(tri) => {
                write!(output, "\tfadd.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Fsub(tri) => {
                write!(output, "\tfsub.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Fmul(tri) => {
                write!(output, "\tfmul.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Fdiv(tri) => {
                write!(output, "\tfdiv.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Xori(tri) => {
                write!(output, "\txori\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Fcvt(bin, dst, src) => {
                if dst == src {
                    panic!("Two types should be different");
                } else {
                    if *dst == Width::Float {
                        write!(output, "\tfcvt.s.w\t{}, {}, rtz\n", bin.dst, bin.src)?;
                    } else {
                        write!(output, "\tfcvt.w.s\t{}, {}, rtz\n", bin.dst, bin.src)?;
                    }
                }
            },
            AsmInstr::Slt(tri) => {
                write!(output, "\tslt\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Slti(tri) => {
                write!(output, "\tslti\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Sgt(tri) => {
                write!(output, "\tsgt\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Seqz(bin) => {
                write!(output, "\tseqz\t")?;
                bin.dump(output)?;
            },
            AsmInstr::Snez(bin) => {
                write!(output, "\tsnez\t")?;
                bin.dump(output)?;
            },
            AsmInstr::Flt(tri) => {
                write!(output, "\tflt.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Fle(tri) => {
                write!(output, "\tfle.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Feq(tri) => {
                write!(output, "\tfeq.s\t")?;
                tri.dump(output)?;
            },
            AsmInstr::Store(mem, prefix) => {
                write!(output, "\t{}s", prefix)?;
                mem.dump(output)?;
            },
            AsmInstr::Load(mem, prefix) => {
                write!(output, "\t{}l", prefix)?;
                mem.dump(output)?;
            },
            AsmInstr::Branch(cond_tri) => {
                write!(output, "\tb")?;
                cond_tri.dump(output)?;
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

impl Dump for BinInstr {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "{}, {}\n", self.dst, self.src)?;
        Ok(())
    }
}

impl Dump for CondTriInstr {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "{}\t", self.cond)?;
        self.tri.dump(output)?;
        Ok(())
    }
}

impl Dump for TriInstr {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        if let Some(width) = self.width {
            write!(output, "{}\t{}, {}, {}\n", width_name(width), self.dst, self.op1, self.op2)?;
        } else {
            write!(output, "{}, {}, {}\n", self.dst, self.op1, self.op2)?;
        }
        Ok(())
    }
}

impl Dump for MemInstr {
    type Out = ();
    fn dump(&self, output: &mut impl io::Write) -> Result<Self::Out, Box<dyn Error>> {
        write!(output, "{}\t{}, {}({})\n", width_name(self.width), self.val, self.offset, self.base)?;
        Ok(())
    }
}

