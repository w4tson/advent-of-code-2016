use puzzles::read_puzzle_input;
use regex::Regex;
use regex::Captures;
use std::fmt::{Display, Formatter, Error};

#[derive(Debug)]
enum BotType {
    Bot(u32),
    Output(u32)
}

impl Display for BotType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            BotType::Bot(x) => write!(f, "Bot {}", x),
            BotType::Output(x) => write!(f, "Output {}", x),
        }
    }
}


struct Bot {
    name : BotType,
    inputs: (Option<u32>, Option<u32>),
    high: BotType,
    low: BotType
}

impl Bot {
    fn is_bot_definition(s : &str) -> bool {
        lazy_static! {
           static ref BOT_INSTRUCTION: Regex = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
        }
        BOT_INSTRUCTION.is_match(&s)
    }

//    pub fn add_value(&self, val : u32) -> Bot {
//        let new_inputs = match self.inputs {
//            (Some(a), None) | (None, Some(a)) => (Some(a), Some(val)),
//            (None, None)  => (Some(val), None),
//            _ => panic!("fully initialized")
//        };
//        Bot { name: self.name, inputs: new_inputs, high: self.high, low: self.low }
//    }
//
    pub fn add_value(&mut self, val : u32) {
        let new_inputs = match self.inputs {
            (Some(a), None) | (None, Some(a)) => (Some(a), Some(val)),
            (None, None)  => (Some(val), None),
            _ => panic!("fully initialized")
        };
        //Bot { name: self.name, inputs: new_inputs, high: self.high, low: self.low }
        self.inputs = new_inputs;
    }
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let inputs = match self.inputs {
            (Some(i), Some(j)) => format!("({}, {})",i, j),
            (Some(i), None) | (None, Some(i)) => format!("({}, _)", i),
            _ => "(_, _)".to_string()
        };
        write!(f, "Bot [ name: {}, high: {}, low: {}, inputs: {} ] ", self.name, self.high, self.low, inputs)
    }
}

pub fn puzzle10() {
    let content = read_puzzle_input("day10.txt");

    let mut bots = content
        .split('\n')
        .filter(|b| Bot::is_bot_definition(b))
        .map(to_bot)
        .collect::<Vec<Bot>>();

    for bot in &bots {
        println!("{} ", bot);
    }

    assert_eq!(210, bots.len());

    let initializations : Vec<(u32,u32)> = content.split('\n')
        .filter(|x| x.starts_with("value"))
        .map(|init_instruction| init_tuple(init_instruction))
        .collect();

    assert_eq!(21, initializations.len());




    for &(init_value, bot_num) in &initializations {
        let bot = get_bot_by_id(&bots, bot_num);
        //bot.add_value(init_value);
        println!("instruction {}", bot);
    }

}

fn init_tuple(s : &str) -> (u32, u32) {
    lazy_static! {
       static ref BOT_INSTRUCTION: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    }

    let captures = BOT_INSTRUCTION.captures(&s);

    let cap = match captures {
        Some(capture) => capture,
        _ => panic!("problem unpacking marker ({})", s)
    };

    match (cap.at(1), cap.at(2)) {
        (Some(x),Some(y)) => (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()),
        _ => panic!("Problem capturing groups for marker ({})", s)
    }
}

fn get_bot_by_id(bots: &Vec<Bot>, id: u32) -> &Bot {
    match bots.iter().find(|b| {
        match b.name {
            BotType::Output(i) | BotType::Bot(i) if id == i => true,
            _ => false
        }

    }) {
        Some(bot) => bot,
        _ => panic!("nope {}", id)
    }

}

fn to_bot(s: &str) -> Bot {
    lazy_static! {
       static ref BOT_INSTRUCTION: Regex = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
    }

    let captures = BOT_INSTRUCTION.captures(&s);

    let cap = match captures {
        Some(capture) => capture,
        _ => panic!("problem unpacking marker ({})", s)
    };

    let (bot_number, low_num, high_num) = match (cap.at(1), cap.at(3), cap.at(5)) {
        (Some(x),Some(y),Some(z)) => (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap(), z.parse::<u32>().unwrap()),
        _ => panic!("Problem capturing groups for marker ({})", s)
    };
    let low = match cap.at(2).unwrap() {
        "bot" => BotType::Bot(low_num),
        _ => BotType::Output(low_num)
    };

    let high = match cap.at(4).unwrap() {
        "bot" => BotType::Bot(high_num),
        _ => BotType::Output(low_num)
    };

    Bot { name: BotType::Bot(bot_number), inputs: (None, None), low: low, high: high}
}