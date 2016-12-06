use puzzles::read_puzzle_input;
use std::fmt::{Display, Formatter, Error};


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

    for i in 0..10 {
        let ref t = triangles[i];
        println!("{} {}", t, t.is_valid());
    }

    let count = triangles.iter().filter(|t| t.is_valid()).count();

    println!("Valid triangles = {}", count);



}

fn parse_line(line: &str) -> MaybeTriangle {
    let t : Vec<u32> = line.to_string()
        .split_whitespace()
        .map(| s : &str | s.parse::<u32>().unwrap())
        .collect();

    MaybeTriangle { x: t[0], y: t[1], z: t[2] }
}