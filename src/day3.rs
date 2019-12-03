use std::fs::File;
use std::io::prelude::Read;
use std::collections::HashMap;

pub fn day3() {
  let fname = "D:\\aoc2019\\src\\inputs\\day3.txt";
  let mut f = File::open(fname).expect("Unable to open file!");

  let mut inputbuf = String::new();
  f.read_to_string(&mut inputbuf).expect("unable to read to string!!");

  let path: Vec<&str> = inputbuf.split("\r\n").collect();

  // trace the path of the wires to get coordinates and step counts as hashmaps path_red and path_blue
  let path_red = trace_path(path[0].split(',').collect()).expect("Failed!");
  let path_blue = trace_path(path[1].split(',').collect()).expect("Failed!");

  let mut closest_intersection_distance = std::i32::MAX;
  let mut smallest_total_step_count = std::usize::MAX;

  // So we got two hashmaps, each containing information about each wires path (x,y) and 
  // steps taken to get to each points on their path (step_count).
  // We compare them and find the values for closest point and fastest point of intersection
  for (point_red, step_count_red) in &path_red {
    match path_blue.get(point_red) {
      Some(step_count_blue) => {
        let (a, b) = point_red;
        let sum = a.abs() + b.abs();
        if closest_intersection_distance > sum {
          closest_intersection_distance = sum;
        }
        if smallest_total_step_count > step_count_blue + step_count_red {
          smallest_total_step_count = step_count_blue + step_count_red 
        }
      }
      None => {}
    }
  }
  println!("D31: Closes intersection distance = {}", closest_intersection_distance);
  println!("D32: Smallest total step count = {}", smallest_total_step_count);
}

fn trace_path(path: Vec<&str>) -> Result<HashMap<(i32, i32), usize>, &str> {
  let mut cur_x: i32 = 0;
  let mut cur_y: i32 = 0;
  // we use route as a map to store the (x,y) coordinates thhrough wich we pass, and the step_count
  let mut route : HashMap<(i32,i32), usize> = HashMap::new();
  let mut step_count: usize = 0;

  for step in path {
    // step ~ "R23" means go right 23 steps. We reverse it to "32R", to easily pop 'R' out,
    // and reverse it back. So we get direction as 'R' and steps as '23'.
    let mut step_mut = step.chars().rev().collect::<String>();
    let direction = step_mut.pop().unwrap();
    step_mut = step_mut.chars().rev().collect::<String>();

    // Walk the virtual 2D map with direction and steps, and store 
    // coordinates and step count with each step
    match direction {
      'R' => {
        let new_x = cur_x + step_mut.parse::<i32>().unwrap();
        for _ in cur_x..new_x {
          cur_x += 1;
          step_count += 1;
          route.insert((cur_x, cur_y), step_count);
        }
      }
      'L' => {
        let new_x = cur_x - step_mut.parse::<i32>().unwrap();
        for _ in new_x..cur_x {
          cur_x -= 1;
          step_count += 1;
          route.insert((cur_x, cur_y), step_count);
        }
      }
      'U' => {
        let new_y = cur_y + step_mut.parse::<i32>().unwrap();
        for _ in cur_y..new_y {
          cur_y += 1;
          step_count += 1;
          route.insert((cur_x, cur_y), step_count);
        }
      }
      'D' => {
        let new_y = cur_y - step_mut.parse::<i32>().unwrap();
        for _ in new_y..cur_y {
          cur_y -= 1;
          step_count += 1;
          route.insert((cur_x, cur_y), step_count);
        }
      }
      _ => {
        return Err("INVALID DIRECTION!");
      }
    }
  }
  Ok(route)
}