use std::fs::File;
use std::io::prelude::Read;

pub fn day2() {

    // create all combinations for cross product (0,1,2,3,..99) X (0,1,2,3,..99)
    // pair = [(0,0), (0,1).. (0,99), (1,0), (1,1), (1,2).. (1,99).......... (99,99)]
    let mut pairs: Vec<(u8, u8)> = Vec::new();
    for i in 0..100 {
        for j in 0..100 {
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
    let mut ip = 0;
    let mut rbase = 0;
    let mut stdinbuf: Vec<isize> = Vec::new();
    let (exitcode, _) = computer(&mut input_mut, &mut stdinbuf, &mut Vec::new(), &mut ip, &mut rbase);
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
        ip = 0;
        rbase = 0;
        let (exitcode, _) = computer(&mut input_mut, &mut stdinbuf, &mut Vec::new(), &mut ip, &mut rbase);
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

fn parse_instruction(n: usize) -> (usize, usize, usize, usize) {
    (n % 100, (n/100) % 10, (n/1000) % 10, (n/10000) % 10)
}

pub fn computer(input: &mut Vec<isize>, 
                    stdinbuf: &mut Vec<isize>, 
                        stdoutbuf: &mut Vec<isize>,
                            ip: &mut usize, 
                                rbase: &mut usize) 
                            -> (isize, Vec<(isize, Vec<String>)>) {
    let mut output_stream:Vec<(isize, Vec<String>)> = Vec::new();
    let mut exec_trace: Vec<String> = Vec::new();
    if input.len() < 1000 { input.resize(10000, 0) }
    // let mut ip = ip;
    // println!("Starting machine!\n Input={:?}", input);
    loop {
        let instruction = input[*ip];
        // exec_trace.push(format!("instruction: {}, ip: {}, rbase: {}\n", instruction, ip, rbase));
        
        let (opcode, _mod1, _mod2, _mod3) = parse_instruction(instruction as usize);
        // exec_trace.push(format!("op: {:?} mod1: {:?} , mod2: {:?}, mod3: {:?}\n", 
                                            // opcode, _mod1, _mod2, _mod3));
        let val1: isize;
        let val2: isize;

        // println!("************************************************");
        // println!("instruction: {}, ip: {}, rbase: {}", instruction, ip, rbase);
        // println!("op: {:?} mod1: {:?} , mod2: {:?}, mod3: {:?}", 
        //                                     opcode, _mod1, _mod2, _mod3);
        // println!("stdout : {:?}, stdin: {:?}", stdoutbuf, stdinbuf);
        // println!("{:?}", exec_trace.last());
        
        // println!("Next 4 input : {} {} {} {}", input[*ip], input[*ip+1], input[*ip+2], input[*ip+3]);

        match opcode {
            1 => { // ADD
                let param3 = input[*ip+3];
                let mem =   if _mod3 == 2 {
                                (*rbase as isize + param3) as usize
                            } else { // _mod3 cant be 1, just 0 and 2
                                param3 as usize
                            };

                val1 =      if _mod1 == 0 {
                                input[input[*ip+1] as usize]
                            } else if _mod1 == 1  {
                                input[*ip+1]
                            } else { // _mod1 == 2
                                input[(*rbase as isize + input[*ip+1]) as usize]
                            };
                val2 =      if _mod2 == 0 {
                                input[input[*ip+2] as usize]
                            } else if _mod2 == 1 {
                                input[*ip+2]
                            } else {// _mod2 == 2
                                input[(*rbase as isize + input[*ip+2]) as usize]
                            };
                input[mem] = val1 + val2;
                *ip += 4;
                exec_trace.push(
                            format!("*ADD*\n\
                                Store address: {}, val1: {}, val2: {}, Sum: {}\n", 
                                        mem, val1, val2, val1+val2)
                            );
            }
            2 => { // MUL
                let param3 = input[*ip+3];
                let mem =   if _mod3 == 2 {
                                (*rbase as isize + param3) as usize
                            } else { // _mod3 cant be 1, just 0 and 2
                                param3 as usize
                            };
                val1 =      if _mod1 == 0 {
                                input[input[*ip+1] as usize]
                            } else if _mod1 == 1 {
                                input[*ip+1]
                            } else {// _mod1 == 2
                                input[(*rbase as isize + input[*ip+1]) as usize]
                            };
                val2 =      if _mod2 == 0 {
                                input[input[*ip+2] as usize]
                            } else if _mod2 == 1 {
                                input[*ip+2]
                            } else {// _mod2 == 2
                                input[(*rbase as isize + input[*ip+2]) as usize]
                            };
                input[mem] = val1 * val2;
                *ip += 4;
                exec_trace.push(
                            format!("*MUL*\n\
                                Store address: {}, val1: {}, val2: {}, Mul: {}\n", 
                                        mem, val1, val2, val1*val2)
                            );
            }
            3 => { // INPUT
                let param = input[*ip+1];
                let mem =   if _mod1 == 2 {
                                (*rbase as isize + param) as usize
                            } else { // _mod3 cant be 1, just 0 and 2
                                param as usize
                            };
                if stdinbuf.is_empty() { return (2, output_stream) }
                input[mem] = stdinbuf.pop().expect("Err: STDIN empty");
                *ip += 2;
                exec_trace.push(
                            format!("*INPUT*\n\
                                Store address: {}, input : {}\n", 
                                        mem, input[mem as usize])
                            );
            }
            4 => { // OUTPUT
                let mem_addr = input[*ip+1];
                let out =   if _mod1 == 0 {
                                input[mem_addr as usize]
                            } else if _mod1 == 1 {
                                mem_addr as isize
                            } else {// _mod1 == 2
                                input[(*rbase as isize + mem_addr) as usize]
                            };
                exec_trace.push(
                            format!("*OUT*\n\
                                val: {}\n", 
                                        out)
                            );
                // if out != 0 && input[ip+2] != 99 {
                //     println!("Execution trace :\n");
                //     for trace in exec_trace.clone() {
                //         print!("{}",  trace);
                //     }
                //     panic!("panic!");
                // }
                stdoutbuf.push(out);
                output_stream.push((out, exec_trace.to_vec()));
                exec_trace.clear();
                *ip += 2;
            }
            5 => { // jump-if-true
                let val =   if _mod1 == 0 {
                                input[input[*ip + 1] as usize]
                            } else if _mod1 == 1 {
                                input[*ip+1]
                            } else {// _mod1 == 2
                                input[(*rbase as isize + input[*ip+1]) as usize]
                            };
                let jmp =   if _mod2 == 0 {
                                input[input[*ip + 2] as usize]
                            } else if _mod2 == 1 {
                                input[*ip+2]
                            } else {// _mod2 == 2
                                input[(*rbase as isize + input[*ip+2]) as usize]
                            };
                *ip =        if val != 0 {
                                jmp as usize
                            } else {
                                *ip + 3
                            };
                exec_trace.push(
                            format!("*jump-if-true*\n\
                                Value: {}, jmp_location  : {}\n", 
                                        val, jmp)
                            );
            },

            6 => { // jump-if-fase
                let val =   if _mod1 == 0 {
                                input[input[*ip + 1] as usize]
                            } else if _mod1 == 1 {
                                input[*ip+1]
                            } else {// _mod1 == 2
                                input[(*rbase as isize + input[*ip+1]) as usize]
                            };
                let jmp =   if _mod2 == 0 {
                                input[input[*ip + 2] as usize]
                            } else if _mod2 == 1 {
                                input[*ip+2]
                            } else {// _mod2 == 2
                                input[(*rbase as isize + input[*ip+2]) as usize]
                            };
                *ip =        if val == 0 {
                                jmp as usize
                            } else {
                                *ip + 3
                            };
                exec_trace.push(
                            format!("*jump-if-true*\n\
                                Value: {}, jmp_location  : {}\n", 
                                        val, jmp)
                            );
            },

            7 => { // less-than
                let param3 = input[*ip+3];
                let mem =   if _mod3 == 2 {
                                (*rbase as isize + param3) as usize
                            } else { // _mod3 cant be 1, just 0 and 2
                                param3 as usize
                            };
                val1 =      if _mod1 == 0 {
                                input[input[*ip+1] as usize]
                            } else if _mod1 == 1 {
                                input[*ip+1]
                            } else {// _mod1 == 2
                                input[(*rbase as isize + input[*ip+1]) as usize]
                            };
                val2 =      if _mod2 == 0 {
                                input[input[*ip+2] as usize]
                            } else if _mod2 == 1 {
                                input[*ip+2]
                            } else {// _mod2 == 2
                                input[(*rbase as isize + input[*ip+2]) as usize]
                            };
                input[mem] = if val1 < val2 {
                                1
                            } else {
                                0
                            };
                *ip += 4;
                exec_trace.push(
                            format!("*less-than*\n\
                                Value1: {}, Value2: {} mem  : {}\n", 
                                        val1, val2, mem)
                            );
            },
            8 => { // equals
                let param3 = input[*ip+3];
                let mem =   if _mod3 == 2 {
                                (*rbase as isize + param3) as usize
                            } else { // _mod3 cant be 1, just 0 and 2
                                param3 as usize
                            };
                val1 =      if _mod1 == 0 {
                                input[input[*ip+1] as usize]
                            } else if _mod1 == 1 {
                                input[*ip+1]
                            } else {// _mod1 == 2
                                input[(*rbase as isize + input[*ip+1]) as usize]
                            };
                val2 =      if _mod2 == 0 {
                                input[input[*ip+2] as usize]
                            } else if _mod2 == 1 {
                                input[*ip+2]
                            } else {// _mod2 == 2
                                input[(*rbase as isize + input[*ip+2]) as usize]
                            };
                input[mem] = if val1 == val2 {
                                1
                            } else {
                                0
                            };
                *ip += 4;
                exec_trace.push(
                            format!("*equals*\n\
                                Value1: {}, Value2: {} mem  : {}\n", 
                                        val1, val2, mem)
                            );
            },
            9 => { // relative-base-offset
                let val = if _mod1 == 0 {
                            input[input[*ip+1] as usize]
                        } else if _mod1 == 1 {
                            input[*ip+1]
                        } else { // 2
                            input[(*rbase as isize + input[*ip+1]) as usize]
                        };
                *rbase = (*rbase as isize + val) as usize;
                *ip += 2;
            }
            99 => { // HALT
                return (1, output_stream)
            }
            _ => { // INVALID
                println!(" PANIC!!! {} {}", input[*ip], opcode);
                return (-1, output_stream)
            }
        }
    }
}