use puzzles::read_puzzle_input;
use std::fmt::{Display, Formatter, Error};

#[allow(dead_code)]
enum Instruction {
    Left(i32),
    Right(i32)
}

#[allow(dead_code)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Display for Instruction {

    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        match *self {
            Instruction::Left(x) => write!(f, "L {}", x),
            Instruction::Right(x) => write!(f, "R {}", x),
        }

    }
}

pub fn puzzle1a() {
    let content = read_puzzle_input("day1.txt");
    let instructions = content.split(", ");

    let (x, y, _) = instructions
        .map(|i| to_instruction(&i))
        .fold( (0, 0, Direction::North), reduce_instruction);

    println!("( {}, {} )", x, y);
    println!("Total displacement = {}", x.abs() + y.abs());


}

fn reduce_instruction(acc: (i32, i32, Direction), i : Instruction) -> (i32, i32, Direction) {
    let (x,y,direction) = acc;

    match (i, direction) {
        (Instruction::Left(distance), Direction::North) => (x-distance, y, Direction::West),
        (Instruction::Left(distance), Direction::East)  => (x, y+distance, Direction::North),
        (Instruction::Left(distance), Direction::South)  => (x+distance, y, Direction::East),
        (Instruction::Left(distance), Direction::West)  => (x, y-distance, Direction::South),

        (Instruction::Right(distance), Direction::North)  => (x+distance, y, Direction::East),
        (Instruction::Right(distance), Direction::East)  => (x, y-distance, Direction::South),
        (Instruction::Right(distance), Direction::South)  => (x-distance, y, Direction::West),
        (Instruction::Right(distance), Direction::West)  => (x, y+distance, Direction::North),
    }
}

fn to_instruction(instruction: &str) -> Instruction {
    let (direction, distance_str) = instruction.split_at(1);

    let distance = distance_str.parse::<i32>().unwrap();

    match (direction, distance) {
        ("L", d) => Instruction::Left(d),
        (_, d) =>  Instruction::Right(d)
    }
}

