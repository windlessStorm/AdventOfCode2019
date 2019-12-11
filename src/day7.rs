use std::fs::File;
use std::io::prelude::Read;
use crate::day2::computer;

pub fn day7() {

    let fname = "D:\\aoc2019\\src\\inputs\\day7.txt";
    // let fname = "D:\\aoc2019\\src\\inputs\\test.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<isize> = inputbuf.split(",").map(|x| x.parse::<isize>().unwrap()).collect();

    // PART 1
    let mut max = 0;
    for a in 0..5 {
        for b in 0..5 {
            if b == a { continue; }
            for c in 0..5 {
                if c == b || c == a { continue; }
                for d in 0..5 {
                    if d == c || d == b || d == a { continue; }
                    for e in 0..5 {
                    if e == d || e == c || e == b || e == a { continue; }
                    let mut phase_seq = vec![a,b,c,d,e];
                    println!("testing fr seq: {:?}", phase_seq);
                    phase_seq.reverse();
                    let mut input_mut: Vec<isize> = input.to_vec();
                    let mut stdin_buf: Vec<isize> = vec!();
                    let mut last_out = 0;
                    loop {
                        if phase_seq.len() == 0 { break };
                        stdin_buf.push(last_out); // sequence number
                        stdin_buf.push(phase_seq.pop().expect("??")); // first amps input
                        println!("inbuf: {:?}", stdin_buf);
                        last_out = run_amp_sequence(&mut input_mut, &mut stdin_buf);
                        println!("last_out: {}", last_out);
                    }
                    if last_out > max { max = last_out };
                    }
                }
            }
        }
    }
    // loop {
    //     if amps.len() == 0 { break };
    //     stdin_buf.push(last_out); // sequence number
    //     stdin_buf.push(amps.pop().expect("??")); // first amps input
    //     last_out = run_amp_sequence(&mut input_mut, &mut stdin_buf);
    //     net_out += last_out;
    // }
    println!("D71: max output= {:?}",  max);
    // Part 2
}

// fn permute(arr: Vec<usize>) -> Vec<Vec<usize>>{


//     return vec![vec![1],vec![2],vec![2]];
// }

fn run_amp_sequence(input: &mut Vec<isize>, stdin_buf: &mut Vec<isize>) -> isize {
    let (exitcode, out) = computer(input, stdin_buf);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    out.last().expect("no output!!!?").0
}