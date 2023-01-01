use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use std::fs;

use std::collections::HashMap;

use petgraph::graph::Graph;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

#[derive(Parser, Debug)]
pub struct Day21 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
enum Operation {
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Assign(i32),
}

impl Operation {
    fn is_assignment(&self) -> bool {
        match self {
            Operation::Assign(x) => true,
            _ => false,
        }
    }
    fn value(&self) -> &i32 {
        match self {
            Operation::Assign(x) => x,
            _ => panic!("cannot run value on expression"),
        }
    }
    fn calculate(&self, a: &Self, b: &Self) -> Operation {
        match self {
            Operation::Add(x, y) => Operation::Assign(a.value() + b.value()),
            Operation::Subtract(x, y) => Operation::Assign(a.value() - b.value()),
            Operation::Multiply(x, y) => Operation::Assign(a.value() * b.value()),
            Operation::Divide(x, y) => Operation::Assign(a.value() / b.value()),
            _ => panic!("cannot run value on expression"),
        }
    }

    fn lhs(&self) -> String {
        match self {
            Operation::Add(x, _) => x.to_string(),
            Operation::Subtract(x, _) => x.to_string(),
            Operation::Multiply(x, _) => x.to_string(),
            Operation::Divide(x, _) => x.to_string(),
            _ => panic!("cannot run value on expression"),
        }
    }
}

impl Clone for Operation {
    fn clone(&self) -> Operation {
        match self {
            Operation::Add(a, b) => Operation::Add(a.to_string(), b.to_string()),
            Operation::Subtract(a, b) => Operation::Subtract(a.to_string(), b.to_string()),
            Operation::Multiply(a, b) => Operation::Multiply(a.to_string(), b.to_string()),
            Operation::Divide(a, b) => Operation::Divide(a.to_string(), b.to_string()),
            Operation::Assign(a) => Operation::Assign(*a),
        }
    }
}

fn assignment(input: &str) -> IResult<&str, Operation> {
    let mut parse = map_res(digit1, |s: &str| s.parse::<i32>());
    let (input, value) = parse(input)?;
    Ok((input, Operation::Assign(value)))
}

fn summand(input: &str) -> IResult<&str, Operation> {
    let (input, (summand1, summand2)) = separated_pair(alpha1, tag(" + "), alpha1)(input)?;
    Ok((input, Operation::Add(summand1.to_string(), summand2.to_string())))
}

fn difference(input: &str) -> IResult<&str, Operation> {
    let (input, (minuend, subtrahend)) = separated_pair(alpha1, tag(" - "), alpha1)(input)?;
    Ok((input, Operation::Subtract(minuend.to_string(), subtrahend.to_string())))
}

fn product(input: &str) -> IResult<&str, Operation> {
    let (input, (multiplier, multiplicand)) = separated_pair(alpha1, tag(" * "), alpha1)(input)?;
    Ok((input, Operation::Multiply(multiplier.to_string(), multiplicand.to_string())))
}

fn quotient(input: &str) -> IResult<&str, Operation> {
    let (input, (dividend, divisor)) = separated_pair(alpha1, tag(" / "), alpha1)(input)?;
    Ok((input, Operation::Divide(dividend.to_string(), divisor.to_string())))
}

fn expression(input: &str) -> IResult<&str, Operation> {
    let (input, op) = alt((product, quotient, summand, difference, assignment))(input)?;
    Ok((input, op))
}

fn parse_monkey_jobs(input: &str) -> IResult<&str, Vec<(&str, Operation)>> {
    let (input, vecs) =
        separated_list1(newline, separated_pair(alpha1, tag(": "), expression))(input)?;
    Ok((input, vecs))
}

impl CommandImpl for Day21 {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        let file = fs::read_to_string(&self.input).unwrap();
        let x = parse_monkey_jobs(&file).unwrap();
        let map = x.1.into_iter().map(|chunk| (chunk.0, chunk.1)).collect::<HashMap<_, _>>();
        let mut g: Graph<&str, &str> = Graph::new();
        for (k, _) in map.iter() {
            g.add_node(k);
        }
        for (k, v) in map.iter() {
            println!("{:?} {:?}", k, v);
        }
        println!("{:?}", g);
        println!("");

        println!("{:?}", map);
        Ok(())
    }
}
