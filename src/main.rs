use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

fn main() {
    day1();
    day2();
}

fn day1() {
    let fname = "D:\\aoc2019\\src\\inputs\\day11.txt";
    // let fname = "D:\\aoc2019\\src\\inputs\\test.txt"

    let f = File::open(fname).expect("Unable to open file!");
    let buf = BufReader::new(f);
    let module_weight_list = buf.lines();
    let mut total_fuel_weight: u32 = 0;
    // Using by_ref will use referece of the object, as we dont want to exhaust the 
    // module_wieght_list by reading it till end, so we can use it later also
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

fn day2() {
    let a: Vec<u8> = (0..100).collect();
    let b: Vec<u8> = (0..100).collect();
    let mut pair: Vec<(u8, u8)> = Vec::new();
    for i in a {
        for j in b.clone() {
            pair.push((i,j))
        }
    }
    let fname = "D:\\aoc2019\\src\\inputs\\day2.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<usize> = inputbuf.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    let mut input_mut: Vec<usize> = input.to_vec();
    let exitcode = computer(&mut input.to_vec(), 12, 2);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    let mut result = input_mut[0];
    println!("D21: Program HALT with defaukt pair {} and {} with position [0]= {}", 12, 2, result);
    for (noun, verb) in pair.clone() {
        input_mut = input.to_vec(); 
        let exitcode = computer(&mut input_mut, noun, verb);
        if exitcode != 1{
            panic!("Exit code: {} !! Something went wrong!!", exitcode);
        }
        result = input_mut[0];
        if result == 19690720 {
            println!("D22: Program HALT with position[0]=19690720 for ({}, {}) pair where 100*{0}+{1}=  {}", noun, verb, 100*input_mut[1]+input_mut[2]);
            break;
        }
    }
}

fn computer(input: &mut Vec<usize>, noun: u8, verb: u8) -> i8 {
    input[1] = noun as usize;
    input[2] = verb as usize;
    let mut i = 0;
    while i < input.len() {
        let instruction = input[i];
        match instruction {
            1 => { // ADD
                let store_location = input[i+3] as usize;
                input[store_location] = input[input[i+1] as usize] + input[input[i+2] as usize];
            }
            2 => { // MUL
                let store_location = input[i+3] as usize;
                input[store_location] = input[input[i+1] as usize] * input[input[i+2] as usize];
            }
            99 => { // HALT
                return 1;
            }
            _ => { // INVALID
                return -1;
            }
        }
        i+=4; // JUMP 4 MEMORY LOCATION TO GET TO NEXT INSTRUCTION
    }
    return 1;
}