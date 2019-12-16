use std::{
    fs,
    io,
    path::Path,
};

#[derive(Debug)]
enum Instruction {
    Add {
        op1:     Operand,
        op2:     Operand,
        address: Operand,
    },
    Halt,
    Input(Operand),
    JumpIfFalse {
        condition: Operand,
        address:   Operand,
    },
    JumpIfTrue {
        condition: Operand,
        address:   Operand,
    },
    Multiply {
        op1:     Operand,
        op2:     Operand,
        address: Operand,
    },
    Output(Operand),
    TestEqual {
        op1: Operand,
        op2: Operand,
        address: Operand,
    },
    TestLessThan {
        op1: Operand,
        op2: Operand,
        address: Operand,
    },
}

impl Instruction {
    fn new(mem: &[isize], ip: usize, opcode: OpCode) -> Instruction {
        let OpCode {
            opcode_type,
            parameter1_mode,
            parameter2_mode,
            parameter3_mode,
        } = opcode;
        match opcode_type {
            OpCodeType::Add => Instruction::Add {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
            OpCodeType::Halt => Instruction::Halt,
            OpCodeType::JumpIfFalse => Instruction::JumpIfFalse {
                condition: Operand::new(parameter1_mode, mem[ip + 1]),
                address:   Operand::new(parameter2_mode, mem[ip + 2]),
            },
            OpCodeType::JumpIfTrue => Instruction::JumpIfTrue {
                condition: Operand::new(parameter1_mode, mem[ip + 1]),
                address:   Operand::new(parameter2_mode, mem[ip + 2]),
            },
            OpCodeType::Input => Instruction::Input(
                Operand::new(parameter1_mode, mem[ip + 1]),
            ),
            OpCodeType::Multiply => Instruction::Multiply {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
            OpCodeType::Output => Instruction::Output(
                Operand::new(parameter1_mode, mem[ip + 1]),
            ),
            OpCodeType::TestLessThan => Instruction::TestLessThan {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
            OpCodeType::TestEqual => Instruction::TestEqual {
                op1:     Operand::new(parameter1_mode, mem[ip + 1]),
                op2:     Operand::new(parameter2_mode, mem[ip + 2]),
                address: Operand::new(parameter3_mode, mem[ip + 3]),
            },
        }
    }
}

#[derive(Debug)]
enum Mode {
    AddressMode,
    ImmediateMode,
}

#[derive(Debug)]
struct OpCode {
    opcode_type: OpCodeType,
    parameter1_mode: Mode,
    parameter2_mode: Mode,
    parameter3_mode: Mode,
}

impl OpCode {
    fn new(mut opcode: isize) -> Result<OpCode, ()> {
        let opcode_type = match opcode%100 {
             1 => OpCodeType::Add,
             2 => OpCodeType::Multiply,
             3 => OpCodeType::Input,
             4 => OpCodeType::Output,
             5 => OpCodeType::JumpIfTrue,
             6 => OpCodeType::JumpIfFalse,
             7 => OpCodeType::TestLessThan,
             8 => OpCodeType::TestEqual,
            99 => OpCodeType::Halt,
            _ => return Err(()),
        };
        opcode /= 100;
        let parameter1_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            _ => return Err(()),
        };
        opcode /= 10;
        let parameter2_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            _ => return Err(()),
        };
        opcode /= 10;
        let parameter3_mode = match opcode % 10 {
            0 => Mode::AddressMode,
            1 => Mode::ImmediateMode,
            _ => return Err(()),
        };
        Ok(Self {
            opcode_type,
            parameter1_mode,
            parameter2_mode,
            parameter3_mode,
        })
    }
}

#[derive(Debug)]
enum OpCodeType {
    Add,
    Halt,
    Input,
    JumpIfFalse,
    JumpIfTrue,
    Output,
    Multiply,
    TestEqual,
    TestLessThan,
}

#[derive(Debug)]
enum Operand {
    Address(usize),
    Immediate(isize),
}

impl Operand {
    fn new(mode: Mode, argument: isize) -> Operand {
        match mode {
            Mode::AddressMode => Self::Address(argument as usize),
            Mode::ImmediateMode => Self::Immediate(argument),
        }
    }
}

fn execute_intcode_program(mem: &mut[isize]) -> isize {
    let mut ip = 0;
    loop {
        let opcode = OpCode::new(mem[ip]).unwrap();
        let instruction = Instruction::new(mem, ip, opcode);
        match instruction {
            Instruction::Add { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of an \
                        Add instruction cannot use immediate mode."),
                };
                mem[address] = op1 + op2;
                ip += 4;
            },
            Instruction::Halt => break,
            Instruction::Input(op1) => {
                let mut input = String::new();
                if let Err(e) = io::stdin().read_line(&mut input) {
                    panic!(e);
                }
                match op1 {
                    Operand::Address(p) => {
                        mem[p] = input.trim().parse().unwrap();
                    },
                    Operand::Immediate(_) => panic!("An Input instruction's \
                        operand cannot use immediate mode."),
                };
                ip += 2;
            },
            Instruction::JumpIfFalse { condition, address } => {
                let condition = match condition {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                if 0 == condition {
                    ip = address as usize;
                } else {
                    ip += 3;
                }
            },
            Instruction::JumpIfTrue { condition, address } => {
                let condition = match condition {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                if 0 != condition {
                    ip = address as usize;
                } else {
                    ip += 3;
                }
            },
            Instruction::Multiply { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of a \
                        Multiply instruction cannot use immediate mode."),
                };
                mem[address] = op1*op2;
                ip += 4;
            },
            Instruction::Output(op1) => {
                match op1 {
                    Operand::Address(p) => println!("{}", mem[p]),
                    Operand::Immediate(v) => println!("{}", v),
                };
                ip += 2;
            },
            Instruction::TestEqual { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of a \
                        TestEqual instruction cannot use immediate mode."),
                };
                if op1 == op2 {
                    mem[address] = 1;
                } else {
                    mem[address] = 0;
                }
                ip += 4;
            },
            Instruction::TestLessThan { op1, op2, address } => {
                let op1 = match op1 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let op2 = match op2 {
                    Operand::Address(p) => mem[p],
                    Operand::Immediate(v) => v,
                };
                let address = match address {
                    Operand::Address(p) => p,
                    Operand::Immediate(_) => panic!("The third operand of a \
                        TestEqual instruction cannot use immediate mode."),
                };
                if op1 < op2 {
                    mem[address] = 1;
                } else {
                    mem[address] = 0;
                }
                ip += 4;
            },
        }
    }
    //println!("{:?}", mem);
    mem[0]
}

fn find_noun_and_verb(src: &[isize]) -> Option<(isize, isize)> {
    const MAGIC_NUM: isize = 19_690_720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut src_copy = vec![0; src.len()];
            src_copy.copy_from_slice(src);
            src_copy[1] = noun;
            src_copy[2] = verb;
            if MAGIC_NUM == execute_intcode_program(&mut src_copy) {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn parse_source(path: &Path) -> io::Result<Vec<isize>> {
    let src = fs::read_to_string(path)?;
    Ok(src.trim()
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect())
}

fn main() -> io::Result<()> {
    let mut src = parse_source(Path::new("input/day5-input.txt")).unwrap();
    execute_intcode_program(&mut src);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_test0() {
        let inputs = [
            "input/day2-test0.txt",
            "input/day2-test1.txt",
            "input/day2-test2.txt",
            "input/day2-test3.txt",
            "input/day2-test4.txt",
        ];
        let outputs = [
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![2, 0, 0, 0, 99],
            vec![2, 3, 0, 6, 99],
            vec![2, 4, 4, 5, 99, 9801],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ];
        assert_eq!(inputs.len(), outputs.len());
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            let mut src = parse_source(Path::new(input)).unwrap();
            execute_intcode_program(&mut src);
            assert_eq!(*output, src);
        }
    }

    #[test]
    fn day2_part1() {
        let mut src = parse_source(Path::new("input/day2-input1.txt")).unwrap();
        assert_eq!(10566835, execute_intcode_program(&mut src));
    }

    #[test]
    fn day2_part2() {
        let src = parse_source(Path::new("input/day2-input2.txt")).unwrap();
        if let Some((noun, verb)) = find_noun_and_verb(&src) {
            assert_eq!(2347, 100*noun + verb);
        } else {
            panic!();
        }
    }
    
    #[test]
    fn day5_test0() {
        let inputs = [
            "input/day5-test0.txt",
            "input/day5-test1.txt",
        ];
        let outputs = [
            vec![1002, 4, 3, 4, 99],
            vec![1101, 100, -1, 4, 99],
        ];
        assert_eq!(inputs.len(), outputs.len());
        for (input, output) in inputs.into_iter().zip(outputs.into_iter()) {
            let mut src = parse_source(Path::new(input)).unwrap();
            execute_intcode_program(&mut src);
            assert_eq!(*output, src);
        }
    }

}
