use std::fs::File;
use std::io::prelude::Read;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day10() {
    let fname = "D:\\aoc2019\\src\\inputs\\day10.txt";
    let mut f = File::open(fname).expect("Unable to open file!");

    let mut inputbuf = String::new();
    f.read_to_string(&mut inputbuf).expect("unable to read to string!!");
    let coordinates = parse_input(inputbuf.clone());
    // println!("{:?}", coordinates);
    let vmap = visibility_map(coordinates.clone());
    //   println!("vmap: {:?}", vmap);
    let mut vmax = 0;
    for (_, v) in &vmap {
        if v.len() > vmax { vmax = v.len() }
        // println!("{:?} => {:?}\n\n", p, v);
    }
    println!("D101: Best visibility: {} astroids", vmax-1);
}

fn parse_input(inputbuf: String) -> Vec<(isize, isize)> {
    let input: Vec<char> = inputbuf.chars().collect();
    let mut i: isize= 0; 
    let mut j: isize = 0;
    let mut map: Vec<(isize, isize)> = Vec::new();

    for v in input {
        match v {
            '\n'    => { j+=1; i=0; continue }
            '\r'    => { continue }
            '.'     => {i+=1; continue } 
            '#'     => { map.push((i,j)); i+=1; continue }
            _       => panic!("Invaid input!")
        }
    }
    map
}

fn visibility_map(coordinates: Vec<(isize, isize)>) -> HashMap<(isize, isize), Vec<(isize, isize)>>{
    let mut vmap: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();
    for c1 in coordinates.clone() {
        let mut vlist: Vec<(isize, isize)> = Vec::new();
        let mut mset: HashSet<(isize, isize)> = HashSet::new();
        for c2 in coordinates.clone() {
            let m = simple_fraction(c2.1 - c1.1, c2.0 - c1.0);
            if !mset.contains(&m) {
                mset.insert(m);
                vlist.push(c2);
            }
        }
        vmap.insert(c1, vlist);
    }
    vmap
}

fn simple_fraction(x: isize, y: isize) -> (isize, isize) {
    if x == 0 {
        (0, y.signum())
    } else if y == 0 {
        (x.signum(), 0)
    } else {
        let d = gcd(x.abs(), y.abs());
        (x / d, y / d)
    }
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

#[test]
fn parse() {
    assert_eq!((1,0), parse_input(".##\n#..###".to_string())[0]);
    assert_eq!((2,0), parse_input(".##\n#..###".to_string())[1]);
    assert_eq!((0,1), parse_input(".##\n#..###".to_string())[2]);
    assert_eq!(6, parse_input(".##\n#..###".to_string()).len());
}