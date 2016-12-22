use puzzles::read_puzzle_input;
use regex::Regex;
use std::fmt::{Display, Formatter, Error};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Debug)]
enum BotType {
    Bot(u32),
    Output(u32)
}

type Bots = Vec<Bot>;

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
    inputs: RefCell<(Option<u32>, Option<u32>)>,
    high: BotType,
    low: BotType,
    delivered: RefCell<bool>   //tracks whether this bot has delivered its payload to its low and high locations
}

impl Bot {
    fn is_bot_definition(s : &str) -> bool {
        lazy_static! {
           static ref BOT_INSTRUCTION: Regex = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
        }
        BOT_INSTRUCTION.is_match(&s)
    }

    pub fn add_value(&self, val : u32) {
        let mut inputs = self.inputs.borrow_mut();
        let new_inputs = match *inputs {
            (Some(a), None) | (None, Some(a)) => (Some(a), Some(val)),
            (None, None)  => (Some(val), None),
            _ => *inputs
        };
        *inputs = new_inputs;
    }

    pub fn has_2_inputs(&self) -> bool {
        let delivered = self.delivered.borrow();
        let inputs = self.inputs.borrow();

        match (*inputs, *delivered) {
            ((Some(_), Some(_)), false) => true,
            _ => false
        }
    }

    pub fn get_low_value(&self) -> u32 {
        match *self.inputs.borrow() {
            (Some(i), Some(j)) if i < j => i,
            (Some(_), Some(k)) => k,
            _ => panic!("problem getting low value for bot {}", self)
        }
    }

    pub fn get_high_value(&self) -> u32 {
        match *self.inputs.borrow() {
            (Some(i), Some(j)) if i > j => i,
            (Some(_), Some(k)) => k,
            _ => panic!("problem getting high value for bot {}", self)
        }
    }
}

impl Display for Bot {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let inputs = match *self.inputs.borrow() {
            (Some(i), Some(j)) => format!("({}, {})",i, j),
            (Some(i), None) | (None, Some(i)) => format!("({}, _)", i),
            _ => "(_, _)".to_string()
        };
        write!(f, "Bot [ name: {}, high: {}, low: {}, inputs: {} ] ", self.name, self.high, self.low, inputs)
    }
}

pub fn puzzle10() {
    let content = read_puzzle_input("day10.txt");

    let bots = content
        .split('\n')
        .filter(|b| Bot::is_bot_definition(b))
        .map(to_bot)
        .collect::<Bots>();

    let mut outputs : BTreeMap<u32, u32> = BTreeMap::new();

    assert_eq!(210, bots.len());

    let initializations : Vec<(u32,u32)> = content.split('\n')
        .filter(|x| x.starts_with("value"))
        .map(|init_instruction| init_tuple(init_instruction))
        .collect();

    assert_eq!(21, initializations.len());

    //intialize the bots
    for &(init_value, bot_num) in &initializations {
        let bot : &Bot = get_bot_by_id(&bots, bot_num);

        bot.add_value(init_value);
        println!("instruction {}   {}", bot, bot.has_2_inputs());
    }

    let b = get_bot_by_id(&bots, 12);


    assert_eq!(true, b.has_2_inputs());


    while has_bots_with_2_inputs(&bots) {

        let bot = bots.iter().find(|bot| bot.has_2_inputs()).unwrap();

        let (is_low_bot, id_low) = is_bot(&(bot.low));
        let (is_high_bot, id_high) = is_bot(&(bot.high));

        print!("Bot {}", bot.name);

        if is_low_bot {
            let low_bot = get_bot_by_id(&bots, id_low);
            low_bot.add_value(bot.get_low_value());
            print!(" delivery to {} ", bot.low);
        } else {
            outputs.insert(id_low, bot.get_low_value());
            print!(" delivery to output {} ", bot.low);
        }

        if is_high_bot {
            let high_bot = get_bot_by_id(&bots, id_high);
            high_bot.add_value(bot.get_high_value());
            println!("and {}", id_high);
        } else {
            outputs.insert(id_low, bot.get_low_value());
            println!("and output {}", id_high);
        }

        *bot.delivered.borrow_mut() = true;

    }

    println!("\n\n\n");
    bots.iter().inspect(|bot| println!("{}", bot)).collect::<Vec<&Bot>>();

    let answer_bot = bots.iter().find(|bot| {
        match *bot.inputs.borrow() {
            (Some(61), Some(17)) | (Some(17), Some(61)) => true,
            _ => false
        }
    }).expect("Not Found");

    println!("Result BOT is {} ", answer_bot);

    outputs.keys().take(3).inspect(|i| println!("{}", outputs.get(i).unwrap())).collect::<Vec<&u32>>();

    let sum_of_first_3_outputs = outputs.keys().take(3).fold(1, |acc, val| acc * outputs.get(val).unwrap());

    println!("Result of sum of first 3 outputs : {}", sum_of_first_3_outputs);
}


fn is_bot(maybe_bot: &BotType) -> (bool, u32) {
    match *maybe_bot {
        BotType::Bot(id) => (true, id),
        BotType::Output(id) => (false, id)
    }
}

fn has_bots_with_2_inputs(bots : &Bots) -> bool {
    bots.iter()
        .any(|bot| bot.has_2_inputs())
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
        (Some(x), Some(y)) => (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()),
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
        (Some(x), Some(y), Some(z)) => (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap(), z.parse::<u32>().unwrap()),
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

    Bot { name: BotType::Bot(bot_number), inputs: RefCell::new((None, None)), low: low, high: high, delivered: RefCell::new(false) }
}