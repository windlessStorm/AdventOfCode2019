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

    let input: Vec<isize> = inputbuf.split(",").map(|x| x.parse::<isize>().unwrap()).collect();


    // PART 1
    let mut input_mut: Vec<isize> = input.to_vec();
    input_mut[1] = 12;
    input_mut[2] = 2;
    let (exitcode, _) = computer(&mut input_mut);
    if exitcode != 1{
        panic!("Exit code: {} !! Something went wrong!!", exitcode);
    }
    // PART 2
    let mut result = input_mut[0];
    println!("D21: Program HALT with default pair {} and {} with position [0]= {}", 12, 2, result);
    for (noun, verb) in pairs.clone() {
        input_mut = input.to_vec();
        input_mut[1] = noun as isize;
        input_mut[2] = verb as isize;
        let (exitcode, _) = computer(&mut input_mut);
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

// pub struct 

fn parse_instruction(n: usize) -> (usize, usize, usize, usize) {
    let mut mod321op: String = n.to_string().chars().collect();
    // println!("{:?}", mod321op);
    let op;
    // let mut mod1 = "".to_string();
    // let mut mod2 = "".to_string();
    // let mut mod3 = "".to_string();
    let mut mod1 = 0;
    let mut mod2 = 0;
    let mut mod3 = 0;
    match mod321op.len() {
    
        1 | 2 => {
            match mod321op.parse::<usize>() {
                Ok(v) => op = v,
                _ => {
                    println!("Failed for: {:?}\n Full instruction: {}", mod321op, n);
                    panic!("parse int faied!");
                }
            }
        }
        3 => {
            let tmp: String = mod321op.drain(..(mod321op.len()-2)).collect();
            match tmp.parse::<usize>() {
                Ok(v) => mod1 = v,
                _ => {
                    // println!("{:?}", tmp);
                    println!("Failed for: {:?}\n Full instruction: {}", tmp, n);
                    panic!("parse int faied!");
                }
            }
            // mod1 = tmp.parse::<usize>().unwrap();
            // op = mod321op.parse::<usize>().unwrap();
            match mod321op.parse::<usize>() {
                Ok(v) => op = v,
                _ => {
                    // println!("{:?}", mod321op);
                    println!("Failed for: {:?}\n Full instruction: {}", mod321op, n);
                    panic!("parse int faied!");
                }
            }
        }
        4 | 5 => {
            let mut mod321: String = mod321op.drain(..(mod321op.len()-2)).collect();
            // op = mod321op.parse::<usize>().unwrap();
            match mod321op.parse::<usize>() {
                Ok(v) => op = v,
                _ => {
                    // println!("{:?}", mod321op);
                    println!("Failed for: {:?}\n Full instruction: {}", mod321op, n);
                    panic!("parse int faied!");
                }
            }
            let mut mod32: String = mod321.drain(..(mod321.len()-1)).collect();
            // mod1 = mod321.parse::<usize>().unwrap();
            match mod321.parse::<usize>() {
                Ok(v) => mod1 = v,
                _ => {
                    // println!("{:?}", mod321);
                    println!("Failed for: {:?}\n Full instruction: {}", mod321, n);
                    panic!("parse int faied!");
                }
            }
            let tmp: String = mod32.drain(..(mod32.len()-1)).collect();
            // mod3 = tmp.parse::<usize>().unwrap();
            if tmp.len() != 0 {
                match tmp.parse::<usize>() {
                    Ok(v) => mod3 = v,
                    _ => {
                        println!("{:?}", tmp);
                        println!("Failed for: {:?}\n Full instruction: {}", tmp, n);
                        panic!("parse int faied!");
                    }
                }
            }
            // mod2 = mod32.parse::<usize>().unwrap();
            match mod32.parse::<usize>() {
                Ok(v) => mod2 = v,
                _ => { 
                        // println!("{:?}", mod32);
                        println!("Failed for: {:?}\n Full instruction: {}", mod32, mod321op);
                        panic!("parse int faied!");
                }
            }
        }
        _ => {
            panic!("Invalid Instruction format!");
        }
            
    }
    // println!("op: {:?} mod1: {:?} , mod2: {:?}, mod3: {:?}", op, mod1, mod2, mod3);
    (op, mod1, mod2, mod3)
}

pub fn computer(input: &mut Vec<isize>) -> (i8, Vec<String>) {
    // println!("Input tape: {:?}", input);
    let mut i = 0;
    let mut jmp_val;
    let mut output_stream:Vec<String> = Vec::new();
    let mut exec_trace: Vec<String> = Vec::new();
    // println!("input size {}", input.len());
    while i < input.len() {
        let instruction = input[i];
        // println!("inst: {}, index: {}", instruction, i);
        
        exec_trace.push(format!("inst: {}, index: {}\n", instruction, i));
        let (opcode, _mod1, _mod2, _mod3) = parse_instruction(instruction as usize);
        exec_trace.push(format!("op: {:?} mod1: {:?} , mod2: {:?}, mod3: {:?}\n", opcode, _mod1, _mod2, _mod3));
        let val1: isize;
        let val2: isize;
        let store_location: usize;
        match opcode {
            1 => { // ADD
                let store_location = input[i+3];
                val1 = 
                        if _mod1 == 0 {
                            input[input[i+1] as usize]
                            } 
                        else  {
                            input[i+1]
                        };
                val2 = 
                        if _mod2 == 0 {
                            input[input[i+2] as usize]
                            } 
                        else  {
                            input[i+2]
                        };
                input[store_location as usize] = val1 + val2;
                jmp_val = 4;
                exec_trace.push(
                            format!("*ADD*\n\
                                Store address: {}, val1: {}, val2: {}, Sum: {}\n", 
                                        store_location,val1, val2, val1+val2)
                            );
            }
            2 => { // MUL
                store_location = input[i+3] as usize;
                val1 = 
                        if _mod1 == 0 {
                            input[input[i+1] as usize]
                            } 
                        else  {
                            input[i+1]
                        };
                val2 = 
                        if _mod2 == 0 {
                            input[input[i+2] as usize]
                            } 
                        else  {
                            input[i+2]
                        };
                input[store_location as usize] = val1 * val2;
                jmp_val = 4;
                exec_trace.push(
                            format!("*MUL*\n\
                                Store address: {}, val1: {}, val2: {}, Mul: {}\n", 
                                        store_location,val1, val2, val1*val2)
                            );
            }
            3 => { // INPUT
                store_location = input[i+1] as usize;
                let simulated_input = 1; // Maybe can take real input from stdin? will see later :)
                input[store_location as usize] = simulated_input;
                jmp_val = 2;
            }
            4 => { // OUTPUT
                let mem_addr = input[i+1] as usize;
                let out = if _mod1 == 0 {
                                    input[mem_addr].to_string()
                            } else {
                                    mem_addr.to_string()
                            };
                exec_trace.push(
                            format!("*OUT*\n\
                                val: {}\n", 
                                        out)
                            );
                if out != "0" && input[i+1] != 99 {
                    println!("Execution trace :\n");
                    for trace in exec_trace.clone() {
                        print!("{}",  trace);
                    }
                    panic!("panic!");
                }
                exec_trace.clear();
                output_stream.push(out);
                jmp_val = 2;
            }
            99 => { // HALT
                return (1, output_stream)
            }
            _ => { // INVALID
                println!(" PANIC!!! {}", input[i]);
                return (-1, output_stream)
            }
        }
        i+=jmp_val; // JUMP 4 MEMORY LOCATION TO GET TO NEXT opcode
        // println!("jmp: {}", jmp_val);
    }
    (1, output_stream)
}