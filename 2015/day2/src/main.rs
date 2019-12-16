use std::io::{BufReader, prelude::*};
use std::fs::File;

// Macro is from "Rust by Example".
// `min!` will calculate the minimum of any number of arguments.
/*macro_rules! min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, min!($($y),+))
    )
}*/

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let reader = BufReader::new(f);
    let mut area_wrapping_paper_total: usize = 0;
    let mut length_ribbon_total: usize = 0;
    // Determine the wrapping paper required for each gift.
    for line in reader.lines() {
        let line = line?;
        // Line is in the form "<int>x<int>x<int>". Convert from &str
        // (I think?) to Vec<usize>.
        let mut hlw: Vec<usize> = line
            .split('x')
            .map(|elem| elem.parse().unwrap())
            .collect();
        hlw.sort_unstable();
        // Compute the surface area of each side of the gift.
        let area_hl = hlw[0]*hlw[1];
        let area_hw = hlw[0]*hlw[2];
        let area_lw = hlw[1]*hlw[2];
        // Allow some slack, which is the smallest of the above areas.
        //let slack = min!(area_hl, area_hw, area_lw);
        let slack = hlw[0]*hlw[1];
        // Compute the amount of wrapping paper required.
        let area_wrapping_paper = 2*area_hl + 2*area_hw + 2*area_lw
            + slack;
        // Add it to the total.
        area_wrapping_paper_total += area_wrapping_paper;
        // Determine the required length of ribbon for the gift.
        let length_ribbon = 2*hlw[0] + 2*hlw[1]
            + hlw[0]*hlw[1]*hlw[2];
        length_ribbon_total += length_ribbon;
        //println!("{} {}", area_wrapping_paper, length_ribbon);
    }
    // Print the result!
    println!("Total wrapping paper required: {} square feet.",
        area_wrapping_paper_total);
    println!("Total length of ribbon required: {} feet.",
        length_ribbon_total);
    Ok(())
}