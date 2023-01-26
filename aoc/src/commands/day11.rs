use std::{collections::VecDeque, fmt::Display, fs::read_to_string, path::PathBuf};

use super::{CommandImpl, DynError};

use env_logger;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1,
    character::complete::multispace0, character::complete::multispace1, multi::separated_list1,
    sequence::delimited, sequence::preceded, *,
};

use std::fs::read;

use clap::Parser;

// this is not my solution
// used solely to learn nom
// reference     https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2022/rust for a
// real solution

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Old => "itself".to_string(),
                Value::Num(num) => num.to_string(),
            }
        )
    }
}

#[derive(Debug)]
enum Operation {
    Multiply((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    modulo: u64,
    divisible_monkey: usize,
    indivisible_monkey: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    n_inspected: u64,
}

fn boredom_correction(item: u64, correction_factor: u64) -> u64 {
    let item: f32 = item as f32;
    let correction_factor: f32 = correction_factor as f32;
    let newvalue: f32 = item / correction_factor;
    newvalue.floor() as u64
}

impl Monkey {
    pub fn new(items: VecDeque<u64>, operation: Operation, test: Test) -> Self {
        let mut n_inspected: u64 = 0;
        Self { items, operation, test, n_inspected }
    }

    fn count_items(&self) -> usize {
        self.items.len()
    }

    fn has_items(&self) -> bool {
        self.items.len() == 0
    }

    fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

    fn inspect_item(&mut self, relief_lowers_worry_level: bool, magic_trick: u64) -> u64 {
        self.n_inspected += 1;
        let item = self.items.pop_front().unwrap();
        //println!("\tMonkey inspects an item with a worry level of {item}");
        match &self.operation {
            Operation::Multiply((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let result = num_a * num_b;
                //println!("\t\tWorry level is multiplied by {num_b} to {result}");
                let newresult: u64 = boredom_correction(result, magic_trick);
                //println!("\t\tMonkey gets bored with item. Worry level is divided by {magic_trick} to {newresult}.");
                newresult
            }
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let result = num_a + num_b;
                //println!("\t\tWorry level increases by {num_b} to {result}");
                let newresult: u64 = boredom_correction(result, magic_trick);
                //println!("\t\tMonkey gets bored with item. Worry level is divided by {magic_trick} to {newresult}.");
                newresult
            }
        }
        //let result = if relief_lowers_worry_level { worry_level / 3 } else { worry_level };
    }

    fn throw_to(&self, item: u64) -> usize {
        if item % self.test.modulo == 0 {
            //println!("\t\tCurrent worry level is divisible by {:?}.", self.test.modulo);
            self.test.divisible_monkey
        } else {
            //println!("\t\tCurrent worry level is not divisible by {:?}.", self.test.modulo);
            self.test.indivisible_monkey
        }
    }
}

#[derive(Debug)]
struct MonkeyCircle {
    monkeys: Vec<Monkey>,
    n_inspections: u64,
    n_rounds: u64,
}

impl MonkeyCircle {
    pub fn new(input: &str) -> Self {
        let monkeys: Vec<Monkey> = parse_monkeys(input);
        let mut n_inspections: u64 = 0;
        let mut n_rounds: u64 = 0;
        Self { monkeys, n_inspections, n_rounds }
    }

    pub fn print_tally(&self) {
        //println!("Round {:?}", self.n_rounds);
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get(i).unwrap();
            println!(
                "Monkey {i} has {:?} items and {:?} inspections",
                monkey.items.len(),
                monkey.n_inspected
            );
        }
    }

    pub fn inspect_items(&mut self, monkey_index: usize) {
        for _ in 0..self.monkeys[monkey_index].items.len() {
            self.n_inspections += 1;
            let from_monkey = self.monkeys.get_mut(monkey_index).unwrap();
            let item = from_monkey.inspect_item(true, 3);
            let catcher_index = from_monkey.throw_to(item);
            //println!("\t\tItem worry level {item} is thrown to monkey {catcher_index}");
            let to_monkey = self.monkeys.get_mut(catcher_index).unwrap();
            to_monkey.add_item(item);
        }
    }

    pub fn inspection_round(&mut self) {
        self.n_rounds += 1;
        for i in 0..self.monkeys.len() {
            //println!("Monkey {i}");
            self.inspect_items(i);
        }
    }

    pub fn run_n_rounds(&mut self, n: u64) {
        for i in 0..n {
            self.inspection_round();
        }
    }

    pub fn get_n_inspections(&self) -> u64 {
        self.n_inspections
    }

    pub fn get_monkey_business_score(&mut self) -> u64 {
        let mut highest_score_index: usize = 0;
        let mut highest_score: u64 = 0;
        let mut second_highest_score_index: usize = 0;
        let mut second_highest_score: u64 = 0;
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get_mut(i).unwrap();
            if monkey.n_inspected > highest_score {
                (second_highest_score, second_highest_score_index) =
                    (highest_score, highest_score_index);
                (highest_score, highest_score_index) = (monkey.n_inspected, i);
            } else if monkey.n_inspected > second_highest_score {
                (second_highest_score, second_highest_score_index) = (monkey.n_inspected, i);
            }
        }
        highest_score * second_highest_score
    }
}

fn parse_value(input: &str) -> IResult<&str, Value> {
    let (input, _) = multispace0(input)?;
    let (input, v) = alt((tag("old"), digit1))(input)?;
    if is_string_numeric(v.to_string()) {
        let v = v.parse::<u64>().unwrap();
        Ok((input, Value::Num(v)))
    } else {
        Ok((input, Value::Old))
    }
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _x) = tag("Operation: new = ")(input)?;
    let (input, value_1) = parse_value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value_2) = parse_value(input)?;
    let result = match operator {
        "*" => Operation::Multiply((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => panic!("unknown operator"),
    };
    Ok((input, result))
}

fn parse_modulo(input: &str) -> IResult<&str, u64> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("Test: divisible by ")(input)?;
    let (input, modulo) = digit1(input)?;
    let modulo = modulo.parse::<u64>().unwrap();
    Ok((input, modulo))
}

fn parse_throw_to_monkey(input: &str) -> IResult<&str, usize> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("If ")(input)?;
    let (input, _) = alt((tag("false"), tag("true")))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(": throw to monkey ")(input)?;
    let (input, catcher) = digit1(input)?;
    let catcher = catcher.parse::<usize>().unwrap();
    Ok((input, catcher))
}

fn parse_test(input: &str) -> IResult<&str, Test> {
    let (input, modulo) = parse_modulo(input)?;
    let (input, divisible_monkey) = parse_throw_to_monkey(input)?;
    let (input, indivisible_monkey) = parse_throw_to_monkey(input)?;
    Ok((input, Test { modulo, divisible_monkey, indivisible_monkey }))
}

fn items1(input: &str) -> IResult<&str, VecDeque<u64>> {
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;

    Ok((input, VecDeque::from(items)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _) = delimited(tag("Monkey "), nom::character::complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = items1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, operation) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = parse_test(input)?;
    Ok((input, Monkey::new(items, operation, test)))
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let (_, monkeys) = separated_list1(tag("\n\n"), parse_monkey)(input).unwrap();
    monkeys
}

#[derive(Parser, Debug)]
pub struct Day11 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day11 {
    fn main(&self) -> Result<(), DynError> {
        env_logger::init();
        let file = read_to_string(&self.input).unwrap();
        //let file = read(&self.input).expect("Error in reading the file");
        let mut monkey_circle: MonkeyCircle = MonkeyCircle::new(&file);
        //println!("monkey circle {:?}", monkey_circle);
        monkey_circle.run_n_rounds(20);
        let final_score: u64 = monkey_circle.get_monkey_business_score();
        println!("Final Score: {final_score}");
        monkey_circle.print_tally();
        // 35343
        // 121103
        // 121800
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
