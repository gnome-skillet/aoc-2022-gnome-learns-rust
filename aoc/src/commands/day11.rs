use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use regex::Regex;

use queues::*;

use log::info;

use env_logger;

#[derive(Parser, Debug)]
pub struct Day11 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct Operation {
    base: u32,
    multiplier: u32,
    add: u32,
}

#[derive(Debug)]
pub struct Test {
    modulo: u32,
    divisible_monkey: usize,
    indivisible_monkey: usize,
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test: Test,
    n_inspected: u32,
}

#[derive(Debug)]
pub struct MonkeyCircle {
    monkeys: Vec<Monkey>,
}

//#[derive(Debug, PartialEq, Eq)]
//pub struct OperationState<'a> {
//    power: Option<QueryParams<'a>>,
//    product: Option<&'a str>,
//}

impl MonkeyCircle {
    pub fn new() -> Self {
        let monkeys: Vec<Monkey> = vec![];
        Self { monkeys }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    pub fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            info!("Monkey {i}:");
            loop {
                let monkey_receiver = self.monkeys[i].throw_to();
                if let Some((item, catcher)) = &monkey_receiver {
                    self.monkeys[*catcher].add_item(*item);
                    info!("    Item with worry level {item} is thrown to {catcher}");
                } else {
                    break;
                }
            }
        }
    }

    pub fn play_game(&mut self, nrounds: usize) -> u32 {
        let mut highest_score: u32 = 0;
        let mut second_highest_score: u32 = 0;
        for _ in 0..nrounds {
            self.round();
        }

        for i in 0..self.monkeys.len() {
            let curr_score: u32 = self.monkeys[i].count_items_inspected();
            if curr_score > highest_score {
                second_highest_score = highest_score;
                highest_score = curr_score;
            } else if curr_score > second_highest_score {
                second_highest_score = curr_score;
            }
        }
        second_highest_score * highest_score
    }
}

impl Operation {
    pub fn new(base: u32, multiplier: u32, add: u32) -> Self {
        Self { base, multiplier, add }
    }

    fn calculate_worry(&mut self, item: u32) -> u32 {
        item.pow(self.base) * self.multiplier + self.add
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_calculate_worry() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        let mut op = Operation::new(1, 19, 0);
        let result = op.calculate_worry(79);
        assert_eq!(result, 1501);
        let result = op.calculate_worry(98);
        assert_eq!(result, 1862);

        let mut op = Operation::new(1, 1, 6);
        let result = op.calculate_worry(54);
        assert_eq!(result, 60);
        let result = op.calculate_worry(65);
        assert_eq!(result, 71);

        let mut op = Operation::new(2, 1, 0);
        let result = op.calculate_worry(79);
        assert_eq!(result, 6241);
        let result = op.calculate_worry(60);
        assert_eq!(result, 3600);
    }

    #[test]
    #[should_panic]
    fn test_parse_items() {
        let lines = vec!["line 1".to_string(), "line 2".to_string()];
        let mut mp = MonkeyParser::new(lines);
        let my_string = "Starting items: 74".to_string();
        let mut items = mp.parse_items(my_string).unwrap();
        assert_eq!(items.size(), 1);
        let worry_level = items.remove().unwrap();
        assert_eq!(worry_level, 74);
        assert_eq!(items.size(), 0);

        let my_string = "Starting items: 79, 98".to_string();
        let mut items = mp.parse_items(my_string).unwrap();
        assert_eq!(items.size(), 1);
        let worry_level = items.remove().unwrap();
        assert_eq!(worry_level, 79);
        assert_eq!(items.size(), 0);

        let my_string = "Starting items: 79, 98".to_string();
        let mut items = mp.parse_items(my_string).unwrap();
        let worry_level = items.remove().unwrap();
        assert_eq!(worry_level, 79);
        let worry_level = items.remove().unwrap();
        assert_eq!(worry_level, 98);
        let worry_level = items.remove().unwrap();
    }
}

impl Test {
    pub fn new(modulo: u32, divisible_monkey: usize, indivisible_monkey: usize) -> Self {
        Self { modulo, divisible_monkey, indivisible_monkey }
    }

    fn throw_to(&mut self, item: u32) -> usize {
        let x = if item % self.modulo == 0 {
            info!("    Current worry level is divisible by {:?}", self.modulo);
            self.divisible_monkey
        } else {
            info!("    Current worry level is not divisible by {:?}", self.modulo);
            self.indivisible_monkey
        };
        x
    }
}

#[allow(dead_code)]
enum MonkeyCircleState {
    ReadMonkey,
    ReadStartingItems,
    ReadOperation,
    ReadTestCondition,
    ReadTrueStatement,
    ReadFalseStatement,
    Stop,
}

#[allow(dead_code)]
struct MonkeyParser {
    state: MonkeyCircleState,
    lines: Vec<String>,
    line_no: usize,
    items_regex: Regex,
    op_regex: Regex,
    divisible_regex: Regex,
    true_regex: Regex,
    false_regex: Regex,
    //test_regex: Regex,
}

#[allow(dead_code)]
impl MonkeyParser {
    pub fn new(lines: Vec<String>) -> Self {
        let line_no: usize = 0;
        let state = MonkeyCircleState::ReadMonkey;
        let items_regex = Regex::new(r"^Starting items: (\d+(?:, \d+)*)$").unwrap();
        let op_regex = Regex::new(r"^Operation: new = old (\*|\+) (old|\d+)$").unwrap();

        let divisible_regex = Regex::new(r"\s*Test: divisible by (\d+)\s*").unwrap();
        let true_regex = Regex::new(r".*\s+If true: throw to monkey (\d+)\s*").unwrap();
        let false_regex = Regex::new(r".*\s+If false: throw to monkey (\d+)\s*").unwrap();
        //let test_regex = Regex::new(
        //    r"^\s*Test:\s+divisible\s+by\s+(\d+)(:?\s|\n)*f\s+true:\s+throw\s+to\s+monkey\s+(\d+)(:?\s|\n)*If\s+false:\s+divisible\s+by\s+(\d+)\s*\n$").unwrap();

        Self {
            state,
            lines,
            line_no,
            items_regex,
            op_regex,
            divisible_regex,
            true_regex,
            false_regex,
            //test_regex,
        }
    }

    pub fn parse_items(&mut self, line: String) -> Option<Queue<u32>> {
        if !self.items_regex.is_match(&line) {
            info!("parse_items: no match");
            return None;
        }
        let mut queue: Queue<u32> = queue![];
        let cap = self.items_regex.captures(&line);
        let cap = cap.unwrap();
        let item_string: String = String::from(&cap[1]);
        let v: Vec<&str> = item_string.split(|c| c == ',').collect();
        for tok in v {
            let result: u32 = tok.trim().parse::<u32>().unwrap();
            queue.add(result);
        }
        Some(queue)
    }

    pub fn parse_op(&mut self, line: String) -> Option<Operation> {
        let cap = self.op_regex.captures(&line).unwrap();
        if "*".eq(&cap[1]) && "old".eq(&cap[2]) {
            Some(Operation::new(2, 1, 0))
        } else if "*".eq(&cap[1]) {
            let prod: u32 = cap[2].parse().unwrap();
            Some(Operation::new(1, prod, 0))
        } else if "+".eq(&cap[1]) {
            let adder: u32 = cap[2].parse().unwrap();
            Some(Operation::new(1, 1, adder))
        } else {
            None
        }
    }

    pub fn parse_test(&mut self, line: String) -> Option<Test> {
        if !self.divisible_regex.is_match(&line) {
            return None;
        }
        if !self.true_regex.is_match(&line) {
            println!("true_regex: no match");
            return None;
        }
        if !self.false_regex.is_match(&line) {
            println!("false_regex: no match");
            return None;
        }
        let cap = self.divisible_regex.captures(&line).unwrap();
        let modulo: u32 = cap[1].parse().unwrap();

        let cap = self.true_regex.captures(&line).unwrap();
        let monkey_true: usize = cap[1].parse().unwrap();

        let cap = self.false_regex.captures(&line).unwrap();
        let monkey_false: usize = cap[1].parse().unwrap();
        let test: Test = Test::new(modulo, monkey_true, monkey_false);
        Some(test)
    }
}

impl Monkey {
    pub fn new(operation: Operation, test: Test) -> Self {
        let items: Vec<u32> = vec![];
        let n_inspected: u32 = 0;
        Self { items, operation, test, n_inspected }
    }

    fn throw_to(&mut self) -> Option<(u32, usize)> {
        if self.items.is_empty() {
            return None;
        }
        self.n_inspected += 1;
        let item = self.items.pop().unwrap();
        info!("  Monkey inspects an item with a worry level of {:?}.", item);
        let worry_level = self.operation.calculate_worry(item);
        info!("    Worry level is increased to {worry_level}.");
        let worry_level = get_bored(worry_level);
        info!("    Monkey gets bored with item. Worry level is divided by 3 to {worry_level}.");
        let next_monkey: usize = self.test.throw_to(worry_level);
        Some((worry_level, next_monkey))
    }

    fn add_item(&mut self, item: u32) {
        self.items.insert(0, item);
    }

    fn count_items_inspected(&mut self) -> u32 {
        self.n_inspected
    }
}

pub fn get_bored(worry_level: u32) -> u32 {
    let f_worry_level = worry_level as f32;
    (f_worry_level / 3.0) as u32
}

impl CommandImpl for Day11 {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        let lines: Vec<String> = slurp_file(&self.input)?;
        info!("print strings from {:?}", &self.input);
        //let buffered = std::fs::read_to_string(&self.input).unwrap();

        //let mut i: u32 = 1;
        //for token in buffered.split_whitespace() {
        //    info!("token {} {}", i, token);
        //    i += 1;
        //}
        info!("end print strings from {:?}", &self.input);

        let mut mp: MonkeyParser = MonkeyParser::new(lines);
        // monkey #0
        let mystring = String::from("Operation: new = old * 19");
        let op = mp.parse_op(mystring);
        //let test: Test = Test::new(23, 2, 3);
        let mystring = String::from("Starting items: 79, 98");
        let mut items = mp.parse_items(mystring).unwrap();
        let mystring = String::from(
            " Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3",
        );
        let test = mp.parse_test(mystring);
        let mut monkey: Monkey = Monkey::new(op.unwrap(), test.unwrap());
        while items.size() > 0 {
            let item = items.remove().unwrap();
            info!("Monkey 0: add {item}:");
            monkey.add_item(item);
        }

        // monkey #1
        let mystring = String::from("Operation: new = old + 6");
        let op = mp.parse_op(mystring);
        let mystring = String::from("Starting items: 54, 65, 75, 74");
        let mut items = mp.parse_items(mystring).unwrap();
        let mystring = String::from(
            " Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0",
        );
        let test = mp.parse_test(mystring);
        //let test: Test = Test::new(19, 2, 0);
        let mut monkey1: Monkey = Monkey::new(op.unwrap(), test.unwrap());
        while items.size() > 0 {
            let item = items.remove().unwrap();
            info!("Monkey 1: add {item}:");
            monkey1.add_item(item);
        }

        // monkey #2
        let mystring = String::from("Operation: new = old * old");
        let op = mp.parse_op(mystring);
        let mystring = String::from("Starting items: 79, 60, 97");
        let mut items = mp.parse_items(mystring).unwrap();
        let mystring = String::from(
            " Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3",
        );
        let test = mp.parse_test(mystring);
        //let test: Test = Test::new(13, 1, 3);
        let mut monkey2: Monkey = Monkey::new(op.unwrap(), test.unwrap());
        while items.size() > 0 {
            let item = items.remove().unwrap();
            info!("Monkey 2: add {item}:");
            monkey2.add_item(item);
        }

        // monkey #3
        let mystring = String::from("Operation: new = old + 3");
        let op = mp.parse_op(mystring);
        let mystring = String::from(
            " Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1",
        );
        let test = mp.parse_test(mystring);
        //let test: Test = Test::new(17, 0, 1);
        let mut monkey3: Monkey = Monkey::new(op.unwrap(), test.unwrap());
        let mystring = String::from("Starting items: 74");
        let result = mp.parse_items(mystring);
        let mut items = result.unwrap();

        while items.size() > 0 {
            let item = items.remove().unwrap();
            monkey3.add_item(item);
        }

        let mut monkey_circle: MonkeyCircle = MonkeyCircle::new();
        monkey_circle.add_monkey(monkey);
        monkey_circle.add_monkey(monkey1);
        monkey_circle.add_monkey(monkey2);
        monkey_circle.add_monkey(monkey3);

        let final_score: u32 = monkey_circle.play_game(20);
        println!("Final Score: {final_score}");
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
