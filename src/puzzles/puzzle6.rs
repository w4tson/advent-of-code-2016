use puzzles::read_puzzle_input;
use std::collections::HashMap;
use std::collections::BTreeMap;

pub fn puzzle6(){
    let content = read_puzzle_input("day6.txt");
    let mut counts : HashMap<u32, HashMap<char, u32>> = HashMap::new();   //(colId -> (char -> count))

    //intialise this first because i'm terrible at rust
    for i in 0..8 {
        counts.insert(i as u32, HashMap::new());
    }

    for s in content.split("\n") {
        update(s.to_string(), &mut counts);
    }

    let mut r = BTreeMap::new();
    for (i, v) in counts.iter() {
        r.insert(i, most_popular_char(v));
    }

    //TODO: work out how to collect &chars to String
    for a in r.values() {
        print!("{}", a);
    }

    //Part 2 of the puzzle
    r.clear();
    println!("\n");

    for (i, v) in counts.iter() {
        r.insert(i, least_popular_char(v));
    }

    for a in r.values() {
        print!("{}", a);
    }
}

fn update(s : String, counts : &mut HashMap<u32, HashMap<char, u32>>) {
    for i in 0..8 {
        let u = i as u32;
        let c : char = s.chars().nth(i).unwrap();

        let map_for_col = counts.get_mut(&u).unwrap();
        let new_val = match map_for_col.get(&c) {
            Some(val) => val + 1,
            _ => 1

        };
        map_for_col.insert(c, new_val);
    }
}

fn most_popular_char(map: &HashMap<char, u32>) -> char {
    let mut result = map.values().collect::<Vec<&u32>>();
    result.sort();
    let max_val = **(result.last().unwrap());
    let result : Vec<(&char, &u32)> = map.iter().filter(|&(_,v)| *v == max_val).collect();
    let &(&key , _) = result.first().unwrap();
    key
}

//Obviously this is naff
fn least_popular_char(map: &HashMap<char, u32>) -> char {
    let mut result = map.values().collect::<Vec<&u32>>();
    result.sort();
    let max_val = **(result.first().unwrap());
    let result : Vec<(&char, &u32)> = map.iter().filter(|&(_,v)| *v == max_val).collect();
    let &(&key , _) = result.first().unwrap();
    key
}