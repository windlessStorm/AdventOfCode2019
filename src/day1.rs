use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1() {
    let fname = "D:\\aoc2019\\src\\inputs\\day1.txt";

    let f = File::open(fname).expect("Unable to open file!");
    let buf = BufReader::new(f);
    let module_weight_list = buf.lines();
    let mut total_fuel_weight: u32 = 0;
    for module_weight in module_weight_list {
        let module_weight = module_weight.expect("Unable to read line").parse::<u32>().unwrap();
        total_fuel_weight += module_weight/3 - 2;
    }
    println!("D11: Fuel required: {}", total_fuel_weight);

    let f = File::open(fname).expect("Unable to open file!");
    let buf = BufReader::new(f);
    let module_weight_list = buf.lines();
    total_fuel_weight = 0;
    for module_weight in module_weight_list {
        let module_weight = module_weight.expect("Unable to read line").parse::<u32>().unwrap();
        let mut fuel_weight: i32 = ((module_weight/3 - 2)) as i32;
        while fuel_weight > 0 {
            total_fuel_weight += fuel_weight as u32;
            fuel_weight = fuel_weight/3 - 2;
        }
    }
    println!("D12: Fuel required: {}", total_fuel_weight);
}