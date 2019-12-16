use md5;

fn main() {
    //let input = "bgvyzdsv";
    let input = "bgvyzdsv";
    for i in 1.. {
        let test = format!("{}{}", input, i);
        let digest = format!("{:x}", md5::compute(&test));
        let check = digest
            .chars()
            .take(6)
            .all(|elem| '0' == elem);
        if check {
            println!("Test string: {}", test);
            println!("MD5: {}", digest);
            println!("Answer: {}", i);
            break;
        }
    }
}
