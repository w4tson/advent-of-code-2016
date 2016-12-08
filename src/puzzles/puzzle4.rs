use puzzles::read_puzzle_input;
use std::fmt::{Display, Formatter, Error};
use std::collections::BTreeMap;
use std::cmp::Ordering;
use regex::Regex;

#[derive(Debug)]
struct Room {
    name: String,
    sector: u32,
    checksum: String
}

impl Room {
    pub fn is_checksum_correct(&self) -> bool {
        let mut letters = BTreeMap::new();

        for char in self.name.chars() {
            let count = match letters.get(&char) {
                Some(c) => c + 1,
                _ => 1
            };
            letters.insert(char, count);
        }

        //collect a tuple of entries to work on
        let mut entries : Vec<(&char,&u32)> = letters.iter().collect();

        entries.sort_by(sort_by_occurence_then_alpha);
        let actual_checksum  = entries.iter().take(5).map(|&(a, _)| *a).collect::<String>();

        actual_checksum.eq(&self.checksum)
    }
}

//if the number of occurrences is tied then return the comparison of the alphas
//else just compare the occurences
fn sort_by_occurence_then_alpha(a : &(&char, &u32), b : &(&char, &u32)) -> Ordering {
    match (*a, *b) {
        ((x,y), (u,v)) if y.cmp(v) == Ordering::Equal => x.cmp(u),
        ((_,y), (_,v)) => v.cmp(y)
    }
}

impl  Display for Room {
    fn fmt(&self, f:&mut Formatter) -> Result<(), Error> {
        write!(f, "{} {} {}", self.name, self.sector, self.checksum)
    }
}

pub fn puzzle4() {
    let content = read_puzzle_input("day4.txt");

    let sector_total : u32 = content.split('\n')
        .map(string_to_room)
        .filter(|r| r.is_checksum_correct())
        .map(|r| r.sector)
        .sum();

    println!("Total of valid sectors = {}", sector_total);
}

fn string_to_room(s: &str) -> Room {
    //third party macro to ensure regex compiled once
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([A-Za-z-]+)-(\d+)\[([A-Za-z]+)\]").unwrap();
    }

    let caps = RE.captures(s).unwrap();

    match (caps.at(1), caps.at(2), caps.at(3)) {
        ( Some(room), Some(sector), Some(checksum) ) => create_room(room, sector, checksum),
        _ => panic!("Problem unpacking room")
    }
}

fn create_room (name: &str, sector_str: &str, checksum: &str) -> Room {

    let sector = match sector_str.parse::<u32>() {
        Ok(s) => s,
        _ => panic!("problem parsing {} ",sector_str)
    };

    let unhypenated_name = name.to_string().replace("-", "");

    Room { name: unhypenated_name , sector: sector, checksum: checksum.to_string() }
}
