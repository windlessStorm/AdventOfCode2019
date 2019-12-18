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
    let vmap = visibility_map(coordinates.clone());

    // PART 1
    let (max, mut vset) =  best_view(vmap.clone());
    println!("D101: Best visibility: {} astroids", max);

    // PART 2
    vset = sort_by_ang_des(vset);
    let a200 = vset[199];
    println!("D102: 200th Astroid to be destroyed ( x, y, atan(xy) ): {:?} and ans = {}",
                                                                         a200, a200.0*100+a200.1);
}

fn sort_by_ang_des(input: Vec<(isize, isize, f32)>) -> Vec<(isize, isize, f32)> {
    let mut vset = input.clone();
    vset.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    vset.reverse();
    vset
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

fn best_view(vmap: HashMap<(isize, isize), Vec<(isize, isize, f32)>>) -> ( usize, Vec<(isize, isize, f32)>) {
    let mut vmax = 0;
    let mut vset:Vec<(isize, isize, f32)> = Vec::new();
    for (_, v) in &vmap {
        if v.len() > vmax { vmax = v.len(); vset = v.clone() }
        // println!("{:?} => {:?}\n\n", p, v);
    }
    (vmax, vset)
}

fn visibility_map(coordinates: Vec<(isize, isize)>) -> HashMap<(isize, isize), Vec<(isize, isize, f32)>>{
    let mut vmap: HashMap<(isize, isize), Vec<(isize, isize, f32)>> = HashMap::new();
    for c1 in coordinates.clone() {
        let mut vlist: Vec<(isize, isize, f32)> = Vec::new();
        let mut mset: HashSet<(isize, isize)> = HashSet::new();
        for c2 in coordinates.clone() {
            if c2==c1 { continue }
            let m = simple_fraction(c2.1 - c1.1, c2.0 - c1.0);
            if !mset.contains(&m) {
                mset.insert(m);
                let ang = ((c2.0 - c1.0) as f32).atan2((c2.1-c1.1) as f32);
                vlist.push((c2.0, c2.1, ang));
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
    loop {
        if b == 0 { break}
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

#[test]
fn p1() {
   let mut input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
    assert_eq!(41, best_view(visibility_map(parse_input(input.to_string()))).0);

    input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
    assert_eq!(35, best_view(visibility_map(parse_input(input.to_string()))).0);
    
    input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
    assert_eq!(33, best_view(visibility_map(parse_input(input.to_string()))).0);

    input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    assert_eq!(210, best_view(visibility_map(parse_input(input.to_string()))).0);
}

#[test]
fn p2() {
   let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    let a200 = sort_by_ang_des(best_view(visibility_map(parse_input(input.to_string()))).1)[199];
    assert_eq!(802, a200.0*100+a200.1);
    // assert_eq!(802, part2(visibility_map(parse_input(input.to_string()))).0);
}