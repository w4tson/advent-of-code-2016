use puzzles::read_puzzle_input;
use regex::Regex;

pub fn puzzle7() {
    let content = read_puzzle_input("day7.txt");

    let tls_count = content.split_whitespace()
        .filter(|s| middle_section_does_not_contain_abba(s))
        .map(strip_middle)
        .filter(|converted| ip_supports_tls(converted))
        .count();

    println!("{}", tls_count);
}

//replaces the hypernet parts with white space
fn strip_middle(s : &str) -> String {
    lazy_static! {
       static ref RE: Regex = Regex::new(r"\[\w+\]").unwrap();
    }
    RE.replace_all(s, " ")
}

//true if the stripped seq contains anny abba sequences
fn ip_supports_tls(input: &str) -> bool {
    input.split_whitespace()
    .any(contains_abba)
}

fn middle_section_does_not_contain_abba(s: &str) -> bool {
    lazy_static! {
       static ref RE: Regex = Regex::new(r"\[(\w+)\]").unwrap();
    }

    !RE.captures_iter(s)
        .map(|caps| caps.at(1).unwrap())
        .any(contains_abba)
}

//gah, no backreferences in rust regex as far as i can tell so this will have to do
fn contains_abba(s : &str) -> bool {
    let window_size = 4;
    let mut chars = s.chars().collect::<Vec<char>>();
    for _ in 0..window_size { chars.push('0'); } //pad the end to make the sliding window easier
    let slice = chars.as_slice();

    (0..slice.len()-window_size).any(|i| {
        match chars[i..i+window_size] {
            [a, b, c, d] if a == d && b == c && a != b=> true,
            _ => false
        }
    })
}



