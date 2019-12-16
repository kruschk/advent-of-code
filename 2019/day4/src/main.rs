fn satisfies_criteria(password: &str) -> bool {
    let mut iterable = password.chars();
    let mut last = iterable.next().unwrap();
    println!("{}", last);
    for c in iterable {
        println!("{}", c);
        if last <= c {
            last = c;
        } else {
            return false;
        }
    }
    true
}

fn satisfies_criteria_num(mut password: usize) -> bool {
    let mut last = password%10;
    let mut counts = [0u8; 10];
    counts[last] += 1;
    password /= 10;
    for _ in 0..(password as f64).log10() as usize + 1 {
        let lsd = password%10;
        counts[lsd] += 1;
        if last < lsd {
            return false;
        } else {
            last = lsd;
        }
        password /= 10;
    }
    for &count in counts.iter() {
        if 2 == count {
            return true;
        }
    }
    false
}

fn main() {
    let lbound = 372304;
    let ubound = 847060;
    let mut count = 0;
    for i in lbound..=ubound {
        if satisfies_criteria_num(i) {
            count += 1;
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1()  { assert!(!satisfies_criteria_num(111111)); }
    #[test]
    fn test2()  { assert!(!satisfies_criteria_num(223450)); }
    #[test]
    fn test3()  { assert!(!satisfies_criteria_num(123789)); }
    #[test]
    fn test4()  { assert!( satisfies_criteria_num(112233)); }
    #[test]
    fn test5()  { assert!(!satisfies_criteria_num(123444)); }
    #[test]
    fn test6()  { assert!( satisfies_criteria_num(111122)); }
    #[test]
    fn test7()  { assert!(!satisfies_criteria_num(111123)); }
    #[test]
    fn test8()  { assert!(!satisfies_criteria_num(135679)); }
    #[test]
    fn test9()  { assert!(!satisfies_criteria_num(012210)); }
    #[test]
    fn test10() { assert!(!satisfies_criteria_num(135659)); }
}
