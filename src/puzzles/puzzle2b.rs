
use puzzles::read_puzzle_input;

#[derive(Debug, Clone, Copy)]
struct Keypad {
    key: char
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { key: '5' }
    }

    pub fn up(&mut self) -> Keypad {
        let hex_val = self.key.to_digit(16).unwrap_or_default();
        let new_val = match hex_val {
            5 | 2 | 1 | 4 | 9 => hex_val,
            3  => 0x01,
            6 | 7 | 8 | 0x0A | 0x0B | 0x0C  => hex_val - 4 ,
            0x0D => 0x0B,
            _ => panic!("problem here")
        };
        let new_char = format!("{0:x}", new_val).chars().nth(0).unwrap_or_default();
        Keypad { key: new_char }
    }

    pub fn down(&mut self) -> Keypad {
        let hex_val = self.key.to_digit(16).unwrap_or_default();
        let new_val = match hex_val {
            5 | 0x0A | 0x0D | 0x0C | 9 => hex_val,
            0x0B => 0x0D,
            6 | 7 | 8 | 2 | 3 | 4  => hex_val + 4,
            1 => 3,
            _ => panic!("problem here")
        };
        let new_char = format!("{0:x}", new_val).chars().nth(0).unwrap_or_default();
        Keypad { key: new_char }
    }

    pub fn left(&mut self) -> Keypad {
        let hex_val = self.key.to_digit(16).unwrap_or_default();
        let new_val = match hex_val {
            1 | 2 | 5 | 0x0A | 0x0D => hex_val,
            _ => hex_val -1
        };
        let new_char = format!("{0:x}", new_val).chars().nth(0).unwrap_or_default();
        Keypad { key: new_char }
    }

    pub fn right(&mut self) -> Keypad {
        let hex_val = self.key.to_digit(16).unwrap_or_default();
        let new_val = match hex_val {
            1 | 4 | 9 | 0x0C | 0x0D => hex_val,
            _ => hex_val +1
        };
        let new_char = format!("{0:x}", new_val).chars().nth(0).unwrap_or_default();
        Keypad { key: new_char }
    }


}

pub fn puzzle2b() {
    let content = read_puzzle_input("day2.txt");
    let lines = content.split("\n");
    let mut starting_point = Keypad::new();

    for line in lines {
        let result = line.chars()
            .fold(starting_point, calc_next_move);

        starting_point = result;

        println!("{}", result.key);
    }
}

fn calc_next_move(mut acc: Keypad, c: char ) -> Keypad {
    match c {
        'U' => acc.up(),
        'D' => acc.down(),
        'L' => acc.left(),
        'R' => acc.right(),
        _   => panic!("argh!")
    }
}

