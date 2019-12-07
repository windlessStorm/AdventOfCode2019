use std::fs::File;
use std::io::prelude::Read;
use crate::day2::computer;

pub fn day5() {

    let fname = "D:\\aoc2019\\src\\inputs\\day5.txt";
    // let fname = "D:\\aoc2019\\src\\inputs\\test.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<isize> = inputbuf.split(",").map(|x| x.parse::<isize>().unwrap()).collect();


    // PART 1
    let mut input_mut: Vec<isize> = input.to_vec();
    let mut stdin_buf: Vec<isize> = vec!();
    
    stdin_buf.push(1);
    let (exitcode, out) = computer(&mut input_mut, &mut stdin_buf);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }

    let output = out.last().expect("no output!!!?");
    println!("D51: Program HALT with output= {:?}",  output.0);

    // Part 2
    input_mut = input.to_vec();
    stdin_buf.push(5);
    let (exitcode, out) = computer(&mut input_mut, &mut stdin_buf);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }

    let output = out.last().expect("no output!!!?");
    println!("D52: Program HALT with output= {:?}",  output.0);
}
