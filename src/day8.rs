use std::fs::File;
use std::io::prelude::Read;

pub fn day8() {

    let fname = "D:\\aoc2019\\src\\inputs\\day8.txt";
    // let fname = "D:\\aoc2019\\src\\inputs\\test.txt";
    let mut f = File::open(fname).expect("Unable to open file!");
    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

    let input: Vec<usize> = inputbuf.chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect();

    // println!("Day81: {:?}", input);

    // println!("frames: {}", input.len()/150);

    let (frames, least_zero_frame) = get_frames(input.clone());

    // println!("frames: {:?}", frames);
    // println!("yes frames: {:?}", frames[least_zero_frame]);
    println!("D81: one_count * two_count = {}", frames[least_zero_frame].1 * frames[least_zero_frame].2 );
    let final_frame =  calculate_frame(&mut input.clone());
    println!("D82: final frame rendered = ");
    paint_frame(final_frame);
}

fn get_frames(input: Vec<usize>) -> (Vec<(usize, usize, usize)>, usize) {
    
    let (mut zero_count, mut one_count, mut two_count) = (0 , 0, 0);
    let mut frames: Vec<(usize, usize, usize)> = Vec::new();
    let mut least_zero = std::usize::MAX;
    let mut least_zero_frame = 0;
    let mut i = 0;
    let layer_hieght = 6;
    let layer_width = 25;
    let layer_pixels = layer_hieght * layer_width;
    let mut current_layer = 0;
    loop  {
        if i >= input.len() {
            break;
        }

        if input[i] == 0 { zero_count += 1}
        if input[i] == 1 { one_count += 1}
        if input[i] == 2 { two_count += 1}

        if (i+1) % layer_pixels == 0 {
            frames.push((zero_count, one_count, two_count));
            if zero_count < least_zero { 
                least_zero = zero_count;
                least_zero_frame = current_layer;
            }
            zero_count = 0;
            one_count = 0;
            two_count = 0;
            current_layer += 1;
        }
        i += 1;
    }
    (frames, least_zero_frame)
}

fn calculate_frame(input: &mut Vec<usize>) -> Vec<usize>{
    let mut final_frame: Vec<usize> = Vec::new();
    for i in 0..150 {

        // println!("Calculating position: {}",i );
        let mut j = 0;
        loop {  
                let probe_index = i + (j*150);
                if probe_index > input.len() {
                    final_frame.push(2);
                    // input[probe_index] = 2; 
                    break;
                }
                match input[probe_index] {
                    2 => {}
                    0 => {
                        final_frame.push(0);
                        break;
                    }
                    1=> {
                        final_frame.push(1);
                        break;
                    }
                    _ => panic!("unexpected value! {}", input[probe_index])
                }
                j += 1;
        }
    }
    final_frame
}

fn paint_frame(frame: Vec<usize>) {
    let mut i = 0;
    // println!("\n");
    loop {
        if i >= frame.len() { break }
        if i % 25 == 0 { print!("\n")}
        let pixel = frame[i];
        if pixel == 1 {
            print!("██");
            // print!("█");
        } else {
            print!("  ");
        }
        i += 1;
    }
}