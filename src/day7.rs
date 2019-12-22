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
                    // println!("testing fr seq: {:?}", phase_seq);
                    phase_seq.reverse();
                    let mut input_mut: Vec<isize> = input.to_vec();
                    let mut stdin_buf: Vec<isize> = vec!();
                    let mut last_out = 0;
                    loop {
                        if phase_seq.len() == 0 { break };
                        stdin_buf.push(last_out); // sequence number
                        stdin_buf.push(phase_seq.pop().expect("??")); // first amps input
                        // println!("inbuf: {:?}", stdin_buf);
                        last_out = run_amp_sequence(&mut input_mut, &mut stdin_buf);
                        // println!("last_out: {}", last_out);
                    }
                    if last_out > max { max = last_out };
                    }
                }
            }
        }
    }
    println!("D71: max output= {}",  max);

    // Part 2
    max = 0;
    for a in 5..10 {
        for b in 5..10 {
            if b == a { continue; }
            for c in 5..10 {
                if c == b || c == a { continue; }
                for d in 5..10 {
                    if d == c || d == b || d == a { continue; }
                    for e in 5..10 {
                    if e == d || e == c || e == b || e == a { continue; }
                    let mut phase_seq = vec![a,b,c,d,e];
                    // println!("testing fr seq: {:?}", phase_seq);
                    phase_seq.reverse();
                    let out = amplifier_feedback_loop(input.clone(), &mut phase_seq);
                    if out > max { max = out };
                    }
                }
            }
        }
    }
    println!("D72: max output with amp feedback = {}",  max)
}

fn run_amp_sequence(input: &mut Vec<isize>, stdin_buf: &mut Vec<isize>) -> isize {
    let mut ip = 0;
    let mut rbase = 0;
    let (exitcode, out) = computer(input, stdin_buf, &mut Vec::new(), &mut ip, &mut rbase);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    out.last().expect("no output!!!?").0
}

fn amplifier_feedback_loop(input: Vec<isize>, phase_seq: &mut Vec<isize>) -> isize {
    let mut input1 = input.clone();
    let mut input2 = input.clone();
    let mut input3 = input.clone();
    let mut input4 = input.clone();
    let mut input5 = input.clone();
    let mut stdin_buf1: Vec<isize> = Vec::new();
    let mut stdin_buf2: Vec<isize> = Vec::new();
    let mut stdin_buf3: Vec<isize> = Vec::new();
    let mut stdin_buf4: Vec<isize> = Vec::new();
    let mut stdin_buf5: Vec<isize> = Vec::new();
    
    let (mut ip1, mut ip2, mut ip3, mut ip4, mut ip5) = (0, 0, 0, 0, 0);
    let (mut rbase1, mut rbase2, mut rbase3, mut rbase4, mut rbase5) = (0, 0, 0, 0, 0);
    let (mut halt1, mut halt2, mut halt3, mut halt4) = ( false, false, false, false);

    stdin_buf1.push(phase_seq.pop().expect("??"));
    stdin_buf2.push(phase_seq.pop().expect("??"));
    stdin_buf3.push(phase_seq.pop().expect("??"));
    stdin_buf4.push(phase_seq.pop().expect("??"));
    stdin_buf5.push(phase_seq.pop().expect("??"));
    
    stdin_buf1.push(0);
    
    loop {
        // println!("starting amp 1! stdin_buf1 = {:?}\n
        //                     stdin_buf2 *shud be empty* = {:?}\n
        //                             ip1 = {}",stdin_buf1, stdin_buf2, ip1 );
        if !halt1 {
            stdin_buf1.reverse();
            let output = computer(&mut input1, &mut stdin_buf1, &mut stdin_buf2, &mut ip1, &mut rbase1);
            if output.0 == 1 {
                halt1 = true;
            } 
        }
        // println!("Halted amp 1! stdin_buf1 *shud be empty* = {:?}\n
        //                     stdin_buf2 = {:?}\n
        //                             ip1 = {}\n",stdin_buf1, stdin_buf2, ip1 );
        // println!("starting amp 2! in: stdin_buf2 = {:?}\n
        //                     out: stdin_buf3 *shud be empty* = {:?}\n
        //                             ip2 = {}",stdin_buf2, stdin_buf3, ip2 );
        if !halt2 {
            stdin_buf2.reverse();
            let output = computer(&mut input2, &mut stdin_buf2, &mut stdin_buf3, &mut ip2, &mut rbase2);
            if output.0 == 1 {
                halt2 = true;
            } 
        }
        // println!("Halted amp 2! stdin_buf2 *shud be empty* = {:?}\n
        //                     stdin_buf3 = {:?}\n
        //                             ip2 = {}\n",stdin_buf2, stdin_buf3, ip2 );
        // println!("starting amp 3! stdin_buf3 = {:?}\n
        //                     stdin_buf4 *shud be empty* = {:?}\n
        //                             ip3 = {}",stdin_buf3, stdin_buf4, ip3 );
        if !halt3 {
            stdin_buf3.reverse();
            let output = computer(&mut input3, &mut stdin_buf3, &mut stdin_buf4, &mut ip3, &mut rbase3);
            if output.0 == 1 {
                halt3 = true;
            } 
        }
        // println!("Halted amp 3! stdin_buf3 *shud be empty* = {:?}\n
        //                     stdin_buf4 = {:?}\n
        //                             ip3 = {}\n",stdin_buf3, stdin_buf4, ip3 );
        // println!("starting amp 4! stdin_buf4 = {:?}\n
        //                     stdin_buf5 *shud be empty* = {:?}\n
        //                             ip4 = {}",stdin_buf4, stdin_buf5, ip4 );
        if !halt4 {
            stdin_buf4.reverse();
            let output = computer(&mut input4, &mut stdin_buf4, &mut stdin_buf5, &mut ip4, &mut rbase4);
            if output.0 == 1 {
                halt4 = true;
            } 
        }
        // println!("Halted amp 4! stdin_buf4 *shud be empty* = {:?}\n
        //                     stdin_buf5 = {:?}\n
        //                             ip4 = {}\n",stdin_buf4, stdin_buf5, ip4 );
        // println!("starting amp 5! stdin_buf5 = {:?}\n
        //                     stdin_buf1 *shud be empty* = {:?}\n
        //                             ip5 = {}",stdin_buf5, stdin_buf1, ip5 );
        stdin_buf5.reverse();
        let output = computer(&mut input5, &mut stdin_buf5, &mut stdin_buf1, &mut ip5, &mut rbase5);
        if output.0 == 1 {
            break;
        }
        // println!("Halted amp 5! stdin_buf5 *shud be empty* = {:?}\n
        //                     stdin_buf1 = {:?}\n
        //                             ip5 = {}\n",stdin_buf5, stdin_buf1, ip5 ); 
    }
    stdin_buf1.pop().expect("??")
}