use std::{
    fs,
    cell::RefCell,
    cmp::Ordering,
};

#[derive(Clone, Debug, PartialEq)]
struct Asteroid {
    coordinates:  Point2D,
    count_los:    usize,  // `los` -> line-of-sight
    polar_coords: PolarCoords,
    slopes:       Vec<Fraction>,
}

impl Asteroid {
    fn new(coordinates: Point2D) -> Self {
        let polar_coords = coordinates.to_polar();
        Self {
            coordinates,
            count_los: 0,
            polar_coords,
            slopes: Vec::new(),
         }
    }

    fn get_slopes(&mut self, field: &[RefCell<Asteroid>]) {
        for asteroid in field {
            match asteroid.try_borrow() {
                Err(_) => continue,
                Ok(asteroid) => {
                    let delta_x = asteroid.coordinates.x - self.coordinates.x;
                    let delta_y = asteroid.coordinates.y - self.coordinates.y;
                    let mut slope = Fraction::new(delta_y, delta_x);
                    slope.reduce();
                    self.slopes.push(slope);
                },
            }
        }
        self.slopes.sort_unstable();
        self.slopes.dedup();
        self.count_los = self.slopes.len();
    }

    fn set_origin(&mut self, origin: &Point2D) {
        self.coordinates.x -= origin.x;
        self.coordinates.y -= origin.y;
        self.polar_coords = self.coordinates.to_polar();
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn to_polar(&self) -> PolarCoords {
        let Point2D { x, y } = *self;
        let r = f64::from(x*x + y*y).sqrt();
        let temp = x;
        let x = -y;
        let y = temp;
        let mut slope = Fraction::new(y, x);
        slope.reduce();
        let mut theta = f64::from(slope.num).atan2(f64::from(slope.den));
        if theta < 0.0 {
            theta += 2.0*std::f64::consts::PI;
        }
        PolarCoords { r, theta }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct PolarCoords {
    r:     f64,
    theta: f64,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Fraction {
    num: i32,
    den: i32,
}

impl Fraction {
    fn new(num: i32, den: i32) -> Self {
        Self { num, den }
    }
    
    fn reduce(&mut self) {
        let gcd = gcd(self.num, self.den).abs();
        if 0 != gcd {
            self.num /= gcd;
            self.den /= gcd;
        }
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while 0 != b {
        let temp = b;
        b = modulo(a, b);
        a = temp;
    }
    a
}

fn modulo(a: i32, n: i32) -> i32 {
    // Euclidean modulus:
    // https://en.wikipedia.org/wiki/Modulo_operation#In_programming_languages
    let subtraction = if a < 0 { 1 } else { 0 };
    a - n.abs()*(a/n.abs() - subtraction)
}

fn get_best_asteroid(field: &[RefCell<Asteroid>]) -> Asteroid {
    let best_asteroid = field.iter().max_by(|x, y| {
        x.borrow().count_los.cmp(&y.borrow().count_los)
    }).unwrap();
    best_asteroid.borrow().clone()
}

fn populate_field(input: &str) -> Vec<RefCell<Asteroid>> {
    let mut field = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if '#' == c {
                let coords = Point2D::new(x as i32, y as i32);
                let asteroid = Asteroid::new(coords);
                field.push(RefCell::new(asteroid));
            }
        }
    }
    for asteroid in field.iter() {
        asteroid.borrow_mut().get_slopes(&field);
    }
    field
}

fn main() {
    let input = fs::read_to_string("input/day10_part2_input.txt").expect("Error reading input file.");
    let mut field = populate_field(&input);
    let asteroid = get_best_asteroid(&field);
    let origin = asteroid.coordinates;
    let Point2D { x, y } = origin;
    println!("Best asteroid's coordinates: {},{}.", x, y);
    println!("Detectable asteroids from the best asteroid: {}", asteroid.count_los);
    for asteroid in field.iter() {
        asteroid.borrow_mut().set_origin(&origin);
    }
    for asteroid in field.iter() {
        asteroid.borrow_mut().get_slopes(&field);
    }
    field.sort_unstable_by(|a, b| {
        let PolarCoords { r: r_a, theta: theta_a } = a.borrow().polar_coords;
        let PolarCoords { r: r_b, theta: theta_b } = b.borrow().polar_coords;
        match theta_a.partial_cmp(&theta_b) {
            Some(Ordering::Equal) => (),
            Some(ord) => return ord,
            None => panic!(),
        }
        if let Some(ordering) = r_a.partial_cmp(&r_b) {
            ordering
        } else {
            panic!()
        }
    });
    let mut i = 0;
    while i < field.len() {
        let Point2D { x, y } = field[i].borrow().coordinates;
        if 0 == x && 0 == y {
            field.remove(i);
            break;
        }
        i += 1;
    }
    let mut count_vaporized = 0;
    let mut last_vaporized_index = None;
    while None == last_vaporized_index {
        let mut i = 0;
        while i < field.len() {
            let last_theta = field[i].borrow().polar_coords.theta;
            count_vaporized += 1;
            println!("Vaporized {}: {:?}, {:?}",
                count_vaporized,
                field[i].borrow().coordinates,
                field[i].borrow().polar_coords);
            if 200 == count_vaporized { 
                last_vaporized_index = Some(i);
                println!("{:?}", last_vaporized_index);
                break;
            }
            field.remove(i);
            while i < field.len()
                    && last_theta == field[i].borrow().polar_coords.theta {
                println!("Not vaporized {}: {:?}, {:?}",
                count_vaporized,
                field[i].borrow().coordinates,
                field[i].borrow().polar_coords);
                i = (i + 1)%field.len();
            }
        }
    }
    let mut result = field[last_vaporized_index.unwrap()].borrow().coordinates.clone();
    println!("{:?}", origin);
    println!("{:?}", result);
    result.x += origin.x;
    result.y += origin.y;
    println!("{:?}", result);
    println!("Final answer: {}", 100*result.x + result.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_tests() {
        let filenames = [
            "input/day10_part1_test0.txt",
            "input/day10_part1_test1.txt",
            "input/day10_part1_test2.txt",
            "input/day10_part1_test3.txt",
            "input/day10_part1_test4.txt",
        ].into_iter();
        let expected_outputs = [
            (Point2D { x:  3, y:  4 },   8),
            (Point2D { x:  5, y:  8 },  33),
            (Point2D { x:  1, y:  2 },  35),
            (Point2D { x:  6, y:  3 },  41),
            (Point2D { x: 11, y: 13 }, 210),
        ].into_iter();
        for (filename, expected_output) in filenames.zip(expected_outputs) {
            let input = fs::read_to_string(filename)
                .expect("Error reading input file.");
            let field = populate_field(&input);
            let asteroid = get_best_asteroid(&field);
            let actual_output = (asteroid.coordinates, asteroid.count_los);
            assert_eq!(*expected_output, actual_output);
        }
    }
}
