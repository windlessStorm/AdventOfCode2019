use std::fs::File;
use std::io::prelude::Read;
use crate::day2::computer;
use std::collections::HashMap;

#[derive(Clone)]
#[derive(Debug)]
enum Color {
    Black = 0,
    White = 1
}

pub fn day11() {

    let fname = "D:\\aoc2019\\src\\inputs\\day11.txt";
    // let fname = "D:\\aoc2019\\src\\inputs\\test.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<isize> = inputbuf.split(",").map(|x| x.parse::<isize>().unwrap()).collect();

    // PART 1
    let mut input_mut: Vec<isize> = input.to_vec();
    let mut stdin_buf: Vec<isize> = vec!();
    let mut stdout_buf: Vec<isize> = vec!();
    let mut painted_panels : HashMap<(isize,isize), Color> = HashMap::new();
    let mut current_direction = 0; // 0-up, 1-right, 2-down, 3-left
    let mut cur_pos: (isize, isize) = (0, 0);

    let mut ip = 0;
    let mut rbase = 0;
    loop {
        match painted_panels.get(&cur_pos) {
            Some(c) => stdin_buf.push(c.clone() as isize),
            None => stdin_buf.push(0)
        }
        // println!("ip: {}, stdin: {:?}", ip, stdin_buf);
        let (exitcode, _) = computer(&mut input_mut, &mut stdin_buf, &mut stdout_buf, &mut ip, &mut rbase);
        if exitcode == -1 {
            panic!("Exit code: {} !! Something went wrong!!", exitcode);
        }
        if exitcode == 1 {
            break;
        }
        assert_eq!(stdin_buf.len(), 0);
        assert_eq!(stdout_buf.len(), 2);
        let c = stdout_buf[0];
        let turn = stdout_buf[1] as usize;
        if c == 0 { 
            painted_panels.insert(cur_pos, Color::Black); 
        } else { 
            painted_panels.insert(cur_pos, Color::White); 
        }
        robot_move(turn, &mut cur_pos, &mut current_direction);
        stdout_buf.clear();
    }
    println!("D111: Program HALT with Painted panel count: {}", painted_panels.keys().len());

    // PART 2
    input_mut = input.to_vec();
    painted_panels.clear();
    stdout_buf.clear();
    stdin_buf.clear();
    current_direction = 0; // 0-up, 1-right, 2-down, 3-left
    cur_pos = (0, 0);
    painted_panels.insert(cur_pos, Color::White);

     // all black panels at init
    ip = 0;
    rbase = 0;
    let mut i = 0;
    loop {
        // println!("Iteration: {}", i);
        match painted_panels.get(&cur_pos) {
            Some(c) => stdin_buf.push(c.clone() as isize),
            None => stdin_buf.push(0)
        }
        // println!("ip: {}, stdin: {:?}", ip, stdin_buf);
        let (exitcode, _) = computer(&mut input_mut, &mut stdin_buf, &mut stdout_buf, &mut ip, &mut rbase);
        if exitcode == -1 {
            panic!("Exit code: {} !! Something went wrong!!", exitcode);
        }
        if exitcode == 1 {
            break;
        }
        // println!("ip: {}, stdout: {:?}", ip, stdout_buf);
        assert_eq!(stdin_buf.len(), 0);
        assert_eq!(stdout_buf.len(), 2);
        let c = stdout_buf[0];
        let turn = stdout_buf[1] as usize;
        if c == 0 { 
            painted_panels.insert(cur_pos, Color::Black); 
        } else { 
            painted_panels.insert(cur_pos, Color::White); 
        }
        // println!("Robot will move now!!");
        robot_move(turn, &mut cur_pos, &mut current_direction);
        stdout_buf.clear();
        i+=1;
    }

    println!("D112: Licence plate has been painted!:");
    paint_hull(painted_panels);
}

fn robot_move(turn: usize, cur_pos: &mut (isize, isize), current_direction: &mut isize) {
    // println!("Trying to move robot.. cur_pos: {:?}, cur_dir: {:?}, turn: {}", cur_pos, current_direction, turn);
    let ds: Vec<(isize, isize)> = vec!((0,1),(1,0),(0,-1),(-1,0));  // direction unit vectors up, right, down, left
    if turn == 0 {
        if *current_direction == 0 {
            *current_direction = 3;
        } else {
            *current_direction = ((*current_direction) - 1) % 4;
        }
    } else  {
        *current_direction = ((*current_direction) + 1) % 4;
    }
    // println!("Current direction: {}", *current_direction);
    cur_pos.0 += ds[(*current_direction) as usize].0; // x
    cur_pos.1 += ds[(*current_direction) as usize].1; // y
}

fn paint_hull(painted_panels : HashMap<(isize,isize), Color>) {
    for i in -1..7 {
        for j in -2..42 {
            match painted_panels.get(&(j, -i)) {
            Some(c) =>  {
                if c.clone() as usize == 1 {
                    print!("██");
                } else {
                    print!("  ");
                }
            }
            None => print!("  ")
            }
        }
        print!("\n");
    }
}