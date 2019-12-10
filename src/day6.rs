use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn day6() {
    let fname = "D:\\aoc2019\\src\\inputs\\day6.txt";

    // PART 1
    let f = File::open(fname).expect("Unable to open file!");
    let buf = BufReader::new(f);
    let orbit_list: Vec<String> = buf.lines().map(|x| x.expect("??")).collect();
    let m = create_orbitmap(orbit_list);
    let current_node= "COM".to_string();
    let orbit_map = tag_tree_dfs(m,current_node, 0);
    let c = get_count(orbit_map);
    println!("D61: Total orbits in orbit map = {}", c);

    // PART 2
    
}

fn tag_tree_dfs(orbit_map: HashMap<String, (Vec<String>, usize)>, current_node: String, tag: usize) 
                                            -> HashMap<String, (Vec<String>, usize)> {
    let mut newmap = orbit_map.clone();
    
    match orbit_map.get(&current_node) {
        Some(v) => {
                newmap.insert(current_node, (v.0.clone(), tag));
                for orbit in v.0.clone() {
                    newmap = tag_tree_dfs(newmap, orbit, tag+1);
                }
        }
        None => {}
    }

    return newmap
}

fn get_count(orbitmap: HashMap<String, (Vec<String>, usize)>) -> usize {
    let mut count = 0;
    for (_,c) in orbitmap {
        count += c.1
    }
    count
}

fn create_orbitmap (orbit_list: Vec<String>) -> HashMap<String, (Vec<String>, usize)> {
    let mut orbitmap : HashMap<String, (Vec<String>, usize)> = HashMap::new();
    for i in orbit_list {
        let objects: Vec<String> = i.split(")").map(|x| x.to_string()).collect();
        let ob1 = &objects[0];
        let ob2 = &objects[1];
        match orbitmap.get(ob1) {
            Some(v) => {
                let mut n = v.0.to_vec();
                n.push(ob2.clone());
                orbitmap.insert(ob1.clone(), (n.clone(), 0));
            }
            None => {
                let mut n: Vec<String> = Vec::new();
                n.push(ob2.clone());
                orbitmap.insert(ob1.clone(), (n.clone(), 0));
            }
        }
        match orbitmap.get(ob2) {
            Some(_) => {}
            None => {
                let n: Vec<String> = Vec::new();
                orbitmap.insert(ob2.clone(), (n.clone(), 0));
            }
        }
    }

    orbitmap
}