use std::fs;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Point2D {
    x: isize,
    y: isize,
}

#[derive(PartialEq)]
enum WhoseMove {
    Santa,
    RoboSanta,
}

fn main() -> Result<(), io::Error> {
    let mut wm = WhoseMove::Santa;
    let mut santa = Point2D {
        x: 0,
        y: 0,
    };
    let mut robo_santa = Point2D {
        x: 0,
        y: 0,
    };
    let mut history = vec![santa, robo_santa];
    let input = fs::read_to_string("input.txt")?;
    for c in input.chars() {
        let mut mover;
        if WhoseMove::Santa == wm {
            mover = &mut santa;
            wm = WhoseMove::RoboSanta;
        } else {
            mover = &mut robo_santa;
            wm = WhoseMove::Santa;
        }
        if '>' == c {
            mover.x += 1;
            history.push(*mover);
        } else if '^' == c {
            mover.y += 1;
            history.push(*mover);
        } else if '<' == c {
            mover.x -=1;
            history.push(*mover);
        } else if 'v' == c {
            mover.y -= 1;
            history.push(*mover);
        }
    }
    history.sort_unstable();
    history.dedup();
    println!("Santa's location history: {:?}", history);
    println!("Houses visited: {}", history.len());
    Ok(())
}