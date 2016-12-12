use puzzles::read_puzzle_input;
use std::collections::VecDeque;
use regex::RegexSet;
use regex::Regex;
use regex::Captures;

enum Instruction {
    RotateRow(u8,u8),
    RotateColumn(u8,u8),
    Rect(u8,u8)
}

pub fn puzzle8() {
    let content = read_puzzle_input("day8.txt");

    let mut state  = [[0u8; 50]; 6];

    for i in 0..5 {
        println!("{}", state[0][i] + 1);
    }

    let instuctions = content.split('\n')
        .map(to_instruction)
        .collect::<Vec<Instruction>>();



}

fn to_instruction(s : &str) -> Instruction {
    lazy_static! {
       static ref RECT_REGEX: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
       static ref ROTATE_COL_REGEX: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
       static ref ROTATE_ROW_REGEX: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    }

    let i : Instruction;
    if RECT_REGEX.is_match(s) {
        i = capture_to_instruction(RECT_REGEX.captures(s))
    } else if ROTATE_COL_REGEX.is_match(s) {
        i = capture_to_instruction(ROTATE_COL_REGEX.captures(s))
    } else if ROTATE_ROW_REGEX.is_match(s) {
        i = capture_to_instruction(ROTATE_ROW_REGEX.captures(s))
    } else {
        panic!("Problem unpacking Instruction {}",s);
    }
    i
}

fn capture_to_instruction(caps : Option<Captures>) -> Instruction {
    let cap = caps.unwrap();
    Instruction::Rect(cap.at(1).unwrap().parse::<u8>().unwrap(), cap.at(2).unwrap().parse::<u8>().unwrap())
}

fn process_instruction(instruction: Instruction, display: [[u8; 50]; 6]) -> [[u8; 50]; 6] {
    match instruction {
        Instruction::Rect(x,y)          => rect(x, y, display),
        Instruction::RotateColumn(x,by) => rotate_col(x, by, display),
        Instruction::RotateRow(y,by)    => rotate_row(y, by, display)
    }
}

fn rect(x : u8, y: u8, display: [[u8; 50]; 6]) -> [[u8; 50]; 6] {
   display
}

fn rotate_col(x : u8, by: u8, display: [[u8; 50]; 6]) -> [[u8; 50]; 6] {
   display
}

fn rotate_row(y : u8, by: u8, display: [[u8; 50]; 6]) -> [[u8; 50]; 6] {
   display
}


//fn rotate(arr : &[u8]) -> [u8] {
//    let mut buf: VecDeque<u8> = arr.into_iter().cloned().collect();
//    let back = buf.pop_back();
//    buf.push_front(back.unwrap());
//    let x = buf.into_iter().collect::<Vec<u8>>();
//    x.as_slice()
//}
