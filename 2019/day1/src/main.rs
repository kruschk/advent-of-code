use std::io::{BufReader, self, prelude::*};
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    let mut fuel_total = 0;
    for line in f.lines() {
        // Determine the fuel required for the mass of the module.
        let mass_module: usize = line?.parse().unwrap();
        let mut fuel_module = mass_module/3 - 2;
        println!("Fuel required for the mass of the module: {}", fuel_module);
        // Determine the fuel required for the mass of the fuel.
        let mut fuel_mass = fuel_module;
        loop {
            fuel_mass = fuel_mass/3;
            if 2 < fuel_mass {
                fuel_mass -= 2;
                println!("{}", fuel_mass);
                fuel_module += fuel_mass;
            } else {
                break;
            }
        }
        println!("Total fuel required for the module and its fuel: {}", fuel_module);
        // Add the fuel requirement for this module to the total.
        fuel_total += fuel_module;
    }
    println!("Total fuel required: {}", fuel_total);
    Ok(())
}