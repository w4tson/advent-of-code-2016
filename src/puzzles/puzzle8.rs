use puzzles::read_puzzle_input;
use std::collections::VecDeque;
use regex::Regex;
use regex::Captures;
use std::cmp::min;

type Display = [[u8; 50]; 6];

enum Instruction {
    RotateRow(u8,u8),
    RotateColumn(u8,u8),
    Rect(u8,u8)
}

pub fn puzzle8() {
    let content = read_puzzle_input("day8.txt");
    let display : Display = [[0u8; 50]; 6];

    let result = content.split('\n')
        .map(to_instruction)
        .fold(display, process_instruction);

    print_display(result);
    println!("Total = {}", num_of_pixels_on(result));

}

fn num_of_pixels_on(display: Display) -> u16 {
    let mut count : u16 = 0;
    for i in 0..6 {
        for j in 0..50 {
            count = count + display[i as usize ][j as usize ] as u16;
        }
    }
    count
}

fn print_display(display: Display) {
    println!();
    for i in 0..6 {
        for j in 0..50 {
            let pixel = match display[i as usize ][j as usize ] {
                0 => ' ',
                _ => '#'
            };
            print!("{} ", pixel);

        }
        print!("\n");
    }
}

fn to_instruction(s : &str) -> Instruction {
    lazy_static! {
       static ref RECT_REGEX: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
       static ref ROTATE_COL_REGEX: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
       static ref ROTATE_ROW_REGEX: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
    }

    match s {
        rect if RECT_REGEX.is_match(rect) => to_rect(capture_to_instruction(RECT_REGEX.captures(s))),
        rect if ROTATE_COL_REGEX.is_match(rect) => to_rotate_col(capture_to_instruction(ROTATE_COL_REGEX.captures(s))),
        rect if ROTATE_ROW_REGEX.is_match(rect) => to_rotate_row(capture_to_instruction(ROTATE_ROW_REGEX.captures(s))),
        _ => panic!("Problem unpacking Instruction {}",s)
    }
}

fn to_rect(a : (u8 , u8)) -> Instruction {
    let (x,y) = a;
    Instruction::Rect(x,y)
}

fn to_rotate_col(a : (u8 , u8)) -> Instruction {
    let (x,y) = a;
    Instruction::RotateColumn(x,y)
}

fn to_rotate_row(a : (u8 , u8)) -> Instruction {
    let (x,y) = a;
    Instruction::RotateRow(x,y)
}

fn capture_to_instruction(caps : Option<Captures>) -> (u8, u8) {
    let cap = caps.unwrap();
    (cap.at(1).unwrap().parse::<u8>().unwrap(), cap.at(2).unwrap().parse::<u8>().unwrap())
}

fn process_instruction(display: Display, instruction: Instruction) -> Display {
    match instruction {
        Instruction::Rect(x, y)          => rect(x, y, display),
        Instruction::RotateColumn(x, by) => rotate_col(x, by, display),
        Instruction::RotateRow(y, by)    => rotate_row(y, by, display)
    }
}

fn rect(x : u8, y: u8, mut display: Display) -> Display {

    for i in 0..min(y, 6) {
        for j in 0..min(x, 50) {
            display[i as usize ][j as usize ] = 1;
        }
    }
//    println!("rect {}x{}:\n", x,y);
    print_display(display);

    display

}

fn rotate_col(x : u8, by: u8, mut display: Display) -> Display {
    let mut v : Vec<u8> = vec![];
    for i in 0..6 {
        v.push(display[i as usize ][x as usize]);
    }
    let rotated = rotate(&v[..], by);

    for i in 0..6 {
        display[i as usize ][x as usize] = rotated[i];
    }

//    println!("rotate col x={} by {}:\n", x,by);
    print_display(display);

    display
}

fn rotate_row(y : u8, by: u8, mut display: Display) -> Display {
    let mut row = display[y as usize];
    let rotated = rotate(&row, by);

    //http://stackoverflow.com/questions/29784502/convert-vectors-to-arrays-and-back
    for (place, element) in row.iter_mut().zip(rotated.iter()) {
        *place = *element;
    };
    display[y as usize] = row;

//    println!("rotate row y={} by {}:\n", y, by);
    print_display(display);

    display
}


fn rotate(arr : &[u8], times: u8) -> Vec<u8> {
    let mut buf: VecDeque<u8> = arr.iter().map(|x| *x).collect();

    for _ in 0..times {
        let back = buf.pop_back();
        buf.push_front(back.unwrap());
    }

    buf.into_iter().collect::<Vec<u8>>()
}
