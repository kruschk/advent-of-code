use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("input.txt")?;
    let mut floor = 0;
    let mut pos = None;
    for (index, c) in input.chars().enumerate() {
        if '(' == c {
            floor += 1;
        } else if ')' == c {
            floor -= 1;
            if floor < 0 && None == pos {
                pos = Some(index + 1);
            }
        }
    }
    println!("Final floor: {}", floor);
    if let Some(position) = pos {
        println!("First position going below ground level (1-indexed): {}", position);
    } else {
        println!("Santa never went below ground level.")
    }
    Ok(())
}