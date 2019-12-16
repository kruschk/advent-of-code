use std::io;
use std::str::FromStr;
use std::num::ParseIntError;

enum Operation {
    Toggle,
    TurnOff,
    TurnOn,
}

struct SantaInstruction {
    op: Operation,
    region: Rectangle,
}

impl FromStr for SantaInstruction {
    type Err = ParseIntError;
    fn 
}

struct Point2D {
    x: usize,
    y: usize,
}

struct Point2DError(ParseIntError)

impl FromStr for Point2D {
    type Err = ParseIntError; // Really, this should be a ParsePoint2DError.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.trim_matches(|p| '(' == p || ')' == p )
            .split(',');
        let x = coords.next().unwrap().parse::<usize>()?;
        let y = coords.next().unwrap().parse::<usize>()?;
        Ok(Self { x, y })
    }
}

struct Rectangle {
    p1: Point2D,
    p2: Point2D,
}

impl FromStr for Rectangle {
    type Err = RectangleError;
}

const GRID_LENGTH: usize = 10;

struct LightGrid {
    array: [[bool; GRID_LENGTH]; GRID_LENGTH],
}

impl LightGrid {
    fn print(&self) {
        for arr in self.array.iter().rev() {
            println!("{}", arr.iter()
                .map(|&elem| if elem { "1" } else { "0" })
                .collect::<Vec<_>>()
                .as_slice()
                .join(" ")
            );
        }
    }

    fn toggle(&mut self, r: Rectangle) {
        unimplemented!();
    }

    fn turn_on(&mut self, r: Rectangle) {
        unimplemented!();
    }

    fn turn_off(&mut self, r: Rectangle) {
        unimplemented!();
    }
}

fn main() {
    let mut lg = LightGrid {
        array: [[false; GRID_LENGTH]; GRID_LENGTH]
    };
    lg.array[3][1] = true;
    lg.print();
}