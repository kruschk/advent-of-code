use std::fs;
use std::io;

fn evaluate_intcode_program(mem: &mut [isize]) -> isize {
    let mut ip = 0;
    loop {
        let instruction = mem[ip];
        if 99 == instruction {
            break;
        } else if 1 == instruction || 2 == instruction {
            let addr_op1 = mem[ip + 1] as usize;
            let addr_op2 = mem[ip + 2] as usize;
            let addr_out = mem[ip + 3] as usize;
            if mem.len() < addr_out || mem.len() < addr_op2
                    || mem.len() < addr_op1 {
                println!("Invalid address.");
                break;
            } else {
                if 1 == instruction {
                    mem[addr_out] = mem[addr_op1] + mem[addr_op2];
                } else if 2 == instruction {
                    mem[addr_out] = mem[addr_op1]*mem[addr_op2];
                }
            }
            ip += 4;
        } else {
            println!("Invalid instruction.");
            break;
        }
    }
    mem[0]
}

fn find_noun_and_verb(src: &[isize]) -> Option<(isize, isize)> {
    const MAGIC_NUM: isize = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut src_copy = vec![0; src.len()];
            src_copy.copy_from_slice(src);
            src_copy[1] = noun;
            src_copy[2] = verb;
            if MAGIC_NUM == evaluate_intcode_program(&mut src_copy) {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn main() -> io::Result<()> {
    let src = fs::read_to_string("input.txt")?;
    let src: Vec<isize> = src
        .split(',')
        .map(|elem| elem.parse().unwrap())
        .collect();
    if let Some((noun, verb)) = find_noun_and_verb(&src) {
        println!("Found the correct noun ({}) and verb ({}).", noun, verb);
        println!("The resultant nounverb is: {}", 100*noun + verb);
    } else {
        println!("Couldn't find the correct noun and verb.");
    }
    Ok(())
}