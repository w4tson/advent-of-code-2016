use puzzles::read_puzzle_input;
use std::fmt::{Display, Formatter, Error};
use std::ops::Range;

#[derive(Debug)]
struct MaybeTriangle {
    x: u32,
    y: u32,
    z: u32
}

impl MaybeTriangle {
    pub fn is_valid(&self) -> bool {
        match (self.x, self.y, self.z) {
            (x,y,z) if x + y > z && x + z > y && y + z > x => true,
            _ => false
        }
    }
}

impl Display for MaybeTriangle {

    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        write!(f, "{}\t{}\t{}", self.x, self.y, self.z)
    }
}

pub fn puzzle3() {
    let content = read_puzzle_input("day3.txt");
    let triangles : Vec<MaybeTriangle> = content.split("\n")
        .map(parse_line)
        .collect();

    let count = how_many_valid(&triangles);

    println!("Valid triangles = {}", count);
}

fn how_many_valid(triangles : &Vec<MaybeTriangle>) -> usize {
    triangles.iter().filter(|&t| t.is_valid()).count()
}

pub fn puzzle3b() {
    let content = read_puzzle_input("day3.txt");
    let all_sides : Vec<&str> = content.split_whitespace().collect();

    println!("total = {}\n", all_sides.len());

    //compile 3 vectors by skipping 3 with an offset
    let mut filtered1:  Vec<u32> = get_every_3rd_skip_n(&all_sides, 0);
    let mut filtered2:  Vec<u32> = get_every_3rd_skip_n(&all_sides, 2);
    let mut filtered3: Vec<u32> = get_every_3rd_skip_n(&all_sides, 1);

    //concatenate for ease of calc
    filtered1.append(&mut filtered2);
    filtered1.append(&mut filtered3);

    //real shame can't use 'step_by' (#27741) in rust nightly build
    //but it was 1am when I wrote this and I was not about to
    //start shaving that particular yak
    let t : Vec<MaybeTriangle> = (0..all_sides.len()-1)
        .filter(|i| i % 3 == 0)
        .map(|a| take_3_and_make_triangle(&filtered1, a))
        .collect();

    for tri in &t {
        println!("{}", tri);
    }

    println!("{} total Triangles", t.len());
    let count = how_many_valid(&t);

    println!("Valid triangles = {}", count);
}

fn take_3_and_make_triangle(all : &Vec<u32>, i: usize ) -> MaybeTriangle {
    MaybeTriangle { x: all[i], y: all[i+1], z: all[i+2] }
}

fn get_every_3rd_skip_n(all : &Vec<&str>, n : usize) -> Vec<u32> {
    Range { start: 0, end: all.len() }
        .filter(|i| i % 3 == n)
        .map(|a| all[a])
        .map(|b| b.parse::<u32>().unwrap())
        .collect()
}

fn parse_line(line: &str) -> MaybeTriangle {
    let t : Vec<u32> = line.to_string()
        .split_whitespace()
        .map(| s : &str | s.parse::<u32>().unwrap())
        .collect();

    MaybeTriangle { x: t[0], y: t[1], z: t[2] }
}