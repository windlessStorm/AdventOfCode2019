use std::fs::File;
use std::io::prelude::Read;
use crate::day2::computer;

pub fn day9() {

    let fname = "D:\\aoc2019\\src\\inputs\\day9.txt";
    // let fname = "D:\\aoc2019\\src\\inputs\\test.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<isize> = inputbuf.split(",").map(|x| x.parse::<isize>().unwrap()).collect();

    // PART 1
    let mut input_mut: Vec<isize> = input.to_vec();
    let mut stdin_buf: Vec<isize> = vec!();
    let mut stdout_buf: Vec<isize> = vec!();
    
    stdin_buf.push(1);
    let mut ip = 0;
    let mut rbase = 0;
    let (exitcode, _) = computer(&mut input_mut, &mut stdin_buf, &mut stdout_buf, &mut ip, &mut rbase);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    // println!("D91: Program HALT with output= {:?}",  out);
    // let output = out.last().expect("no output!!!?");
    println!("D91: Program HALT with BOOST keycode = {:?}",  stdout_buf);

    // Part 2
    input_mut = input.to_vec();
    stdout_buf.clear();
    stdin_buf.push(2);
    ip = 0; 
    rbase = 0;
    let (exitcode, _) = computer(&mut input_mut, &mut stdin_buf, &mut stdout_buf, &mut ip, &mut rbase);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    // println!("D91: Program HALT with output= {:?}",  out);
    // let output = out.last().expect("no output!!!?");
    println!("D92: Program HALT with coordinates of Ceres monitoring station = {:?}",  stdout_buf);
}
