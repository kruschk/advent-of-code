use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

// Valid string: "R", "U", "L", "D".
impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if "R" == s {
            Ok(Direction::Right)
        } else if "U" == s {
            Ok(Direction::Up)
        } else if "L" == s {
            Ok(Direction::Left)
        } else if "D" == s {
            Ok(Direction::Down)
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct DirLenInstruction {
    dir: Direction,
    len: usize,
}

// Valid string: "Rx", "Ux", "Lx", or "Dx", where x is an integer.
impl FromStr for DirLenInstruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_char_boundary(1) {
            let (dir, len) = s.split_at(1);
            let dir = dir.parse().unwrap();
            let len = len.parse().unwrap();
            Ok(Self { dir, len })
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
struct WireRouteDirLen(Vec<DirLenInstruction>);

// Valid string (e.g.): "R75,D30,R83,U83,L12,D49,R71,U7,L72".
impl FromStr for WireRouteDirLen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut route = Vec::new();
        for substring in s.split(',') {
            let dli = substring.parse().unwrap();
            route.push(dli);
        }
        Ok(Self(route))
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point2D {
    x: i32,
    y: i32,
}

// Was used for part 1.
/*impl Point2D {
    fn manhattan_distance_from_origin(self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}*/

#[derive(Debug, Eq, PartialEq)]
struct WireRoutePoint2D(HashMap<Point2D, usize>);

impl From<WireRouteDirLen> for WireRoutePoint2D {
    fn from(wrdl: WireRouteDirLen) -> Self {
        let mut current_pos = Point2D { x: 0, y: 0 };
        let mut route = HashMap::new();
        let mut path_length: usize = 0;
        for dli in wrdl.0.iter() {
            if Direction::Right == dli.dir {
                for _ in 1..=dli.len {
                    current_pos.x += 1;
                    path_length += 1;
                    route.insert(current_pos, path_length);
                }
            } else if Direction::Up == dli.dir {
                for _ in 1..=dli.len {
                    current_pos.y += 1;
                    path_length += 1;
                    route.insert(current_pos, path_length);
                }
            } else if Direction::Left == dli.dir {
                for _ in 1..=dli.len {
                    current_pos.x -= 1;
                    path_length += 1;
                    route.insert(current_pos, path_length);
                }
            } else if Direction::Down == dli.dir {
                for _ in 1..=dli.len {
                    current_pos.y -= 1;
                    path_length += 1;
                    route.insert(current_pos, path_length);
                }
            }
        }
        Self(route)
    }
}

// TODO: implement an error type for WireRoutePoint2D.
// TODO: clean up unnecessary types (e.g. Direction, DirLenInstruction, maybe
// WireRouteDirLen).

fn main() -> io::Result<()> {
    // Iterate over each line of the input file. */
    let f = File::open("test0.txt")?;
    let f = BufReader::new(f);
    let mut lines = f.lines();
    // Get the first set of points from the first line.
    let wrp2d1: WireRoutePoint2D = lines
        .next()
        .unwrap()?
        .parse::<WireRouteDirLen>()
        .unwrap()
        .into();
    // Get the second set of points from the second line.
    let wrp2d2: WireRoutePoint2D = lines
        .next()
        .unwrap()?
        .parse::<WireRouteDirLen>()
        .unwrap()
        .into();
    let mut common_points = HashMap::new();
    // Find which points are common to both sets.
    for (key, value) in wrp2d1.0 {
        if wrp2d2.0.contains_key(&key) {
            let other_value = wrp2d2.0.get(&key).unwrap();
            common_points.insert(key, value + other_value);
        }
    }
    // Compute the common point whose distance from the origin is minimum.
    let min = common_points
        .iter()
        .min_by(|(_, value1), (_, value2)| value1.cmp(value2))
        .unwrap()
        .1;
    // Print result.
    println!("{}", min);
    Ok(())
}
