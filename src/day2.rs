use std::fs::File;
use std::io::prelude::Read;

pub fn day2() {

    // create all combinations for cross product (0,1,2,3,..99) X (0,1,2,3,..99)
    // pair = [(0,0), (0,1).. (0,99), (1,0), (1,1), (1,2).. (1,99).......... (99,99)]
    let a: Vec<u8> = (0..100).collect();
    let b: Vec<u8> = (0..100).collect();
    let mut pairs: Vec<(u8, u8)> = Vec::new();
    for i in a {
        for j in b.clone() {
            pairs.push((i,j))
        }
    }

    let fname = "D:\\aoc2019\\src\\inputs\\day2.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<usize> = inputbuf.split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    let mut input_mut: Vec<usize> = input.to_vec();
    let exitcode = computer(&mut input_mut, 12, 2);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    let mut result = input_mut[0];
    println!("D21: Program HALT with defaukt pair {} and {} with position [0]= {}", 12, 2, result);
    for (noun, verb) in pairs.clone() {
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
        let opcode = input[i];
        match opcode {
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
        i+=4; // JUMP 4 MEMORY LOCATION TO GET TO NEXT opcode
    }
    return 1;
}