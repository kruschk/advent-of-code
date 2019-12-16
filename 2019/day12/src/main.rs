use std::{
    convert::TryInto,
    error::Error,
    io::BufReader,
    fmt::{Display, Formatter, self},
    fs::File,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
struct System {
    moons: Vec<Moon>,
    step: usize,
    te: u32,
}

impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for m in self.moons.iter() {
            if let Err(err) = write!(f, "{}", m) {
                return Err(err);
            }
        }
        Ok(())
    }
}

impl System {
    fn new() -> Self {
        Self { moons: Vec::new(), step: 0, te: 0 }
    }

    fn new_from_bufreader<T: std::io::BufRead>(buf: T)
            -> Result<Self, Box<dyn Error>> {
        let mut system = System::new();
        for line in buf.lines() {
            let line = line?;
            let position = line.parse::<Point3D>()?;
            let moon = Moon::new(position);
            system.push(moon);
        }
        Ok(system)
    }

    fn push(&mut self, m: Moon) {
        self.moons.push(m);
    }

    fn simulate(&mut self, steps: usize) -> u32 {
        println!("Step {}:", self.step);
        print!("{}", self);
        for _ in 1..=steps {
            self.update();
            self.step += 1;
            println!("Step {}:", self.step);
            print!("{}", self);
        }
        self.te
    }

    fn simulate_quiet(&mut self, steps: usize) -> u32 {
        for _ in 1..=steps {
            self.update();
            self.step += 1;
        }
        self.te
    }


    fn update(&mut self) {
        for i in 0..self.moons.len() - 1 {
            for j in i + 1..self.moons.len() {
                self.update_velocities(i, j);
            }
        }
        for moon in self.moons.iter_mut() {
            moon.update_position();
            moon.update_energies();
        }
        self.update_total_energy();
    }

    fn update_total_energy(&mut self) {
        self.te = self.moons
            .iter()
            .fold(0, |acc, x| acc + x.te);
    }

    fn update_velocities(&mut self, i: usize, j: usize) {
        // Update x-velocities.
        if self.moons[i].pos.x < self.moons[j].pos.x {
            self.moons[i].vel.x += 1;
            self.moons[j].vel.x -= 1;
        } else if self.moons[j].pos.x < self.moons[i].pos.x {
            self.moons[j].vel.x += 1;
            self.moons[i].vel.x -= 1;
        }
        // Update y-velocities.
        if self.moons[i].pos.y < self.moons[j].pos.y {
            self.moons[i].vel.y += 1;
            self.moons[j].vel.y -= 1;
        } else if self.moons[j].pos.y < self.moons[i].pos.y {
            self.moons[j].vel.y += 1;
            self.moons[i].vel.y -= 1;
        }
        // Update z-velocities.
        if self.moons[i].pos.z < self.moons[j].pos.z {
            self.moons[i].vel.z += 1;
            self.moons[j].vel.z -= 1;
        } else if self.moons[j].pos.z < self.moons[i].pos.z {
            self.moons[j].vel.z += 1;
            self.moons[i].vel.z -= 1;
        }
    }
}

#[derive(Debug)]
struct Moon {
    ke: u32,      // kinetic energy
    pe: u32,      // potential energy
    pos: Point3D, // position
    te: u32,      // total energy
    vel: Point3D, // velocity
}

impl Display for Moon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "pos={}, vel={}, pot={}, kin={}",
            self.pos, self.vel, self.pe, self.ke)
    }
}

impl Moon {
    fn new(pos: Point3D) -> Self {
        let mut moon = Self {
            pos,
            vel: Point3D::new(0, 0, 0),
            ke: 0,
            pe: 0,
            te: 0,
        };
        moon.update_energies();
        moon.update_energies();
        moon
    }

    fn update_position(&mut self) {
        // Position is just the old position plus the velocity.
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn update_energies(&mut self) {
        self.ke = (self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs())
            .try_into()
            .unwrap();
        self.pe = (self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs())
            .try_into()
            .unwrap();
        self.te = self.ke*self.pe;
    }
}


#[derive(Debug, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Point3D {
    type Err = ParseIntError;

    // Valid `s` will be in the form <x=(int), y=(int), z=(int)>.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim_matches(|p| '<' ==  p || '>' == p)
            .split(", ");
        let x = split.next()
            .expect("ParseError: missing x coordinate.")
            .trim_start_matches(|p| 'x' == p || '=' == p)
            .parse()?;
        let y = split.next()
            .expect("ParseError: missing y coordinate.")
            .trim_start_matches(|p| 'y' == p || '=' == p)
            .parse()?;
        let z = split.next()
            .expect("ParseError: missing z coordinate.")
            .trim_start_matches(|p| 'z' == p || '=' == p)
            .parse()?;
        Ok(Self { x, y, z })
    }
}

impl Display for Point3D {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

impl Point3D {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Debug)]
struct CycleCounter {
    done: bool,
    cycle: Vec<i32>,
}

impl CycleCounter {
    fn new() -> Self {
        Self { done: false, cycle: Vec::new() }
    }
}

fn is_repeating(slice: &[i32]) -> bool {
    if 0 != slice.len()%2 { return false; }
    for i in 0..slice.len()/2 {
        if slice[i] != slice[i + slice.len()/2] {
            return false;
        }
    }
    true
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while 0 != b {
        let temp = b;
        b = a%b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a*b/gcd(a, b)
}

fn lcm_iter(slice: &[usize]) -> usize {
    assert!(2 < slice.len());
    let mut result = slice[0];
    for &x in slice {
        result = lcm(x, result);
    }
    result
}

fn print_vec_rec(slice: &[usize], index: usize) {
    if slice.len() <= index { return }
    println!("{}", slice[index]);
    print_vec_rec(slice, index + 1);
}

fn lcm_rec(acc: usize, slice: &[usize], index: usize) -> usize {
    if slice.len() <= index { return acc }
    let acc = lcm(acc, slice[index]);
    lcm_rec(acc, slice, index + 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input/day12-part2-test0.txt")?;
    let f = BufReader::new(f);
    let mut system = System::new_from_bufreader(f)?;
    let mut cyclecounters: Vec<Vec<CycleCounter>>
        = vec![vec![CycleCounter::new(); 3]; system.moons.len()];
    let mut count_done = 0;
    while count_done < cyclecounters.len()*cyclecounters[0].len() {
        for i in 0..cyclecounters.len() {
            for j in 0..cyclecounters[i].len() {
                if !cyclecounters[i][j].done {
                    if 0 == j {
                        cyclecounters[i][j].cycle.push(system.moons[i].pos.x);
                    } else if 1 == j {
                        cyclecounters[i][j].cycle.push(system.moons[i].pos.y);
                    } else if 2 == j {
                        cyclecounters[i][j].cycle.push(system.moons[i].pos.z);
                    }
                    if is_repeating(&cyclecounters[i][j].cycle) {
                        cyclecounters[i][j].done = true;
                        let cycle_len = cyclecounters[i][j].cycle.len()/2;
                        cyclecounters[i][j].cycle.resize(cycle_len, 0);
                        count_done += 1;
                    }
                }
            }
        }
        system.simulate_quiet(1);
    }
    let mut v = Vec::new();
    for i in 0..cyclecounters.len() {
        for j in 0..cyclecounters[i].len() {
            v.push(cyclecounters[i][j].cycle.len());
        }
    }
    v.sort_unstable();
    v.dedup();
    print_vec_rec(&v[..], 0);
    let lcm = lcm_iter(&v[..]);
    let lcm2 = lcm_rec(1, &v[..], 0);
    println!("History will repeat itself every {} time steps.", lcm);
    println!("History will repeat itself every {} time steps.", lcm2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test0() {
        let f = File::open("input/day12-part1-test0.txt")
            .expect("Error reading input file.");
        let f = BufReader::new(f);
        let mut system = System::new_from_bufreader(f)
            .expect("Error generating system.");
        assert_eq!(179, system.simulate_quiet(10));
    }

    #[test]
    fn day12_part1_test1() {
        let f = File::open("input/day12-part1-test1.txt")
            .expect("Error reading input file.");
        let f = BufReader::new(f);
        let mut system = System::new_from_bufreader(f)
            .expect("Error generating system.");
        assert_eq!(1940, system.simulate_quiet(100));
    }

    #[test]
    fn day12_part1_test() {
        let f = File::open("input/day12-part1-input.txt")
            .expect("Error reading input file.");
        let f = BufReader::new(f);
        let mut system = System::new_from_bufreader(f)
            .expect("Error generating system.");
        assert_eq!(7758, system.simulate_quiet(1000));
    }
}
