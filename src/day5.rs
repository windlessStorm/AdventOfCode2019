use std::fs::File;
use std::io::prelude::Read;
use crate::day2::computer;

pub fn day5() {

    let fname = "D:\\aoc2019\\src\\inputs\\day5.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<isize> = inputbuf.split(",").map(|x| x.parse::<isize>().unwrap()).collect();


    // PART 1
    let mut input_mut: Vec<isize> = input.to_vec();
    // input_mut[1] = 12;
    // input_mut[2] = 2;
    let (exitcode, out) = computer(&mut input_mut);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    println!("D51: Program HALT with output pattern= {:?}",  out);
    // // PART 2
    // let mut result = input_mut[0];
    // println!("D21: Program HALT with default pair {} and {} with position [0]= {}", 12, 2, result);
    // for (noun, verb) in pairs.clone() {
    //     input_mut = input.to_vec();
    //     input_mut[1] = noun as usize;
    //     input_mut[2] = verb as usize;
    //     let (exitcode, _) = computer(&mut input_mut);
    //     if exitcode != 1{
    //         panic!("Exit code: {} !! Something went wrong!!", exitcode);
    //     }
    //     result = input_mut[0];
    //     if result == 19690720 {
    //         println!("D22: Program HALT with position[0]=19690720 for ({}, {}) pair where 100*{0}+{1}=  {}", noun, verb, 100*input_mut[1]+input_mut[2]);
    //         break;
    //     }
    // }
}
