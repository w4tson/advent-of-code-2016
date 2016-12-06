use puzzles::read_puzzle_input;

#[derive(Debug, Clone, Copy)]
struct Keypad {
    key: u32
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { key: 5 }
    }

    pub fn up(&mut self) -> Keypad {
        let new_val = match self.key {
            1 | 2 | 3 => self.key,
            _ => self.key-3
        };
        Keypad { key: new_val }
    }

    pub fn down(&mut self) -> Keypad {
        let new_val = match self.key {
            7 | 8 | 9 => self.key,
            _ => self.key+3
        };
        Keypad { key: new_val }
    }

    pub fn right(&mut self) -> Keypad {
        let new_val = match self.key {
            3 | 6 | 9 => self.key,
            _ => self.key+1
        };
        Keypad { key: new_val }
    }

    pub fn left(&mut self) -> Keypad {
        let new_val = match self.key {
            1 | 4 | 7 => self.key,
            _ => self.key-1
        };
        Keypad { key: new_val }
    }
}

pub fn puzzle2a() {
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
