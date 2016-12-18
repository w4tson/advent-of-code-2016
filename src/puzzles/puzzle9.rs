use puzzles::read_puzzle_input;
use regex::Regex;
use std::fmt::{Display,Formatter, Result};

#[derive(Debug, Clone, Copy)]
enum TokenType {
    Content,
    Marker,
    CompressedContent
}

impl Display for TokenType {

    fn fmt(&self, f: &mut Formatter) -> Result {
        let output = match *self {
            TokenType::Content => "Content",
            TokenType::Marker => "Marker",
            TokenType::CompressedContent => "CompressedContent",
        };
        write!(f, "[{}]", output)
    }
}

#[derive(Clone, Debug)]
struct State {
    token: TokenType,
    compression_length: u64,
    compression_factor: u64,
    total: u64,
    buf: Vec<char>
}

impl State {
    pub fn new() -> State {
        State { token: TokenType::Content, compression_length: 0, compression_factor: 0, buf: vec![], total: 0 }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.total)
    }
}

pub fn puzzle9() {
    let content = read_puzzle_input("day9.txt");

    //test the calc_marker
    let a = "(3x3)".chars().collect::<Vec<char>>();
    let (x, y) = calc_marker(&a);
    assert_eq!(x * y, 9);

    let test0 = "B";
    let test1 = "ADVENT";
    let test2 = "A(1x5)BC";
    let test3 = "(3x3)XYZ";
    let test4 = "A(2x2)BCD(2x2)EFG";
    let test5 = "(6x1)(1x3)A";
    let test6 = "X(8x2)(3x3)ABCY";
    let test8 = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
    let test9 = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";

    assert_eq!(decompressed_length_str(test0), 1);
    assert_eq!(decompressed_length_str(test1), 6);
    assert_eq!(decompressed_length_str(test2), 7);
    assert_eq!(decompressed_length_str(test3), 9);
    assert_eq!(decompressed_length_str(test4), 11);
    assert_eq!(decompressed_length_str(test5), 3);
    assert_eq!(decompressed_length_str(test6), 20);
    assert_eq!(decompressed_length_str(test8), 241920);
    assert_eq!(decompressed_length_str(test9), 445);
    println!("Content: {}", content);
    println!("{} {}", "Decompressed length = ", decompressed_length_str(&content)); //11558231665
}

fn decompressed_length_str(str : &str) -> u64 {
    decompressed_length(&(str.chars().collect::<Vec<char>>()))
}

fn decompressed_length(s : &Vec<char>) -> u64 {
    let final_state : State = s.iter()
        .map(|c| *c)
        .fold(State::new(), reduce);
    final_state.total
}

fn reduce(state: State, c : char) -> State {
    let s = &state;
    match (s.token.clone(), s.compression_length){
        (TokenType::Content, _) if c == '('                 => handle_marker_start(s),
        (TokenType::Content , _)                            => handle_content(s),
        (TokenType::Marker , _) if c == ')'                 => handle_marker_end(s),
        (TokenType::Marker  , _)                            => handle_marker(s, c),
        (TokenType::CompressedContent , comp) if comp > 1   => handle_compression(s, c),
        (TokenType::CompressedContent , comp) if comp == 1  => handle_compression_end(&state, c),
        _ => panic!("Unexpected state {}", s.token.clone())
    }
}

fn handle_content(state: &State) -> State {
    let s  = state.clone();
    State { total: state.total + 1,  .. s }
}

fn handle_marker_start(state: &State) -> State {
    let s  = state.clone();
    State { token: TokenType::Marker, .. s }
}

fn handle_marker(state: &State, c : char) -> State {
    let mut new_state  = state.clone();
    new_state.buf.push(c);
    new_state
}

fn handle_marker_end(state: &State) -> State {
    let (x,y) = calc_marker(&state.buf);
    State { token: TokenType::CompressedContent, compression_length: x, compression_factor: y ,buf: vec![], total: state.total }
}

fn handle_compression(state: &State, c: char) -> State {
    let mut new_state = state.clone();
    new_state.buf.push(c);
    new_state.compression_length -= 1;
    new_state
}

fn handle_compression_end(state: &State, c: char) -> State {
    let mut new_buffer = state.buf.clone();
    new_buffer.push(c);
    let decompressed_length = decompressed_length(&new_buffer);
    State { token: TokenType::Content, compression_length: 0, compression_factor: 0, buf: vec![], total: state.total + (decompressed_length*state.compression_factor )}
}

fn calc_marker(vec_of_chars : &Vec<char>) -> (u64, u64) {
    let marker = vec_of_chars.iter().map(|x| *x).collect::<String>();

    lazy_static! {
       static ref RE: Regex = Regex::new(r"(\d+)+x(\d+)").unwrap();
    }

    let caps = RE.captures(&marker);
    let cap = match caps {
        Some(capture) => capture,
        _ => panic!("problem unpacking marker ({})", marker)
    };
    match (cap.at(1), cap.at(2)) {
        (Some(x),Some(y)) => (x.parse::<u64>().unwrap() , y.parse::<u64>().unwrap()),
        _ => panic!("Problem capturing groups for marker ({})", marker)

    }
}