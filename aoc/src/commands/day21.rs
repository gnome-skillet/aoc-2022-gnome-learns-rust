use std::path::PathBuf;

use clap::Parser;

use std::{fmt::Display, fmt::Formatter, fs::File};

//use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use std::fs;

use std::io::Write;

use std::collections::HashMap;

use petgraph::{
    dot::{Config, Dot},
    graph::Graph,
    prelude::DiGraphMap,
    visit::{DfsPostOrder, Topo, Visitable, Walker},
};

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

#[derive(Debug, Eq, PartialEq)]
enum Equation {
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Assign(i64),
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Equation::Add(x, y) => {
                write!(f, "{x} + {y}")
            }
            Equation::Subtract(x, y) => {
                write!(f, "{x} - {y}")
            }
            Equation::Multiply(x, y) => {
                write!(f, "{x} * {y}")
            }
            Equation::Divide(x, y) => {
                write!(f, "{x} / {y}")
            }
            Equation::Assign(x) => {
                write!(f, "{x}")
            }
        }
    }
}

impl Equation {
    fn value(&self) -> &i64 {
        match self {
            Equation::Assign(x) => x,
            _ => panic!("cannot run value on expression"),
        }
    }
    fn calculate(&self, a: &Self, b: &Self) -> Equation {
        match self {
            Equation::Add(_, _) => Equation::Assign(a.value() + b.value()),
            Equation::Subtract(_, _) => Equation::Assign(a.value() - b.value()),
            Equation::Multiply(_, _) => Equation::Assign(a.value() * b.value()),
            Equation::Divide(_, _) => Equation::Assign(a.value() / b.value()),
            _ => panic!("cannot run value on expression"),
        }
    }
}

impl Clone for Equation {
    fn clone(&self) -> Equation {
        match self {
            Equation::Add(a, b) => Equation::Add(a.to_string(), b.to_string()),
            Equation::Subtract(a, b) => Equation::Subtract(a.to_string(), b.to_string()),
            Equation::Multiply(a, b) => Equation::Multiply(a.to_string(), b.to_string()),
            Equation::Divide(a, b) => Equation::Divide(a.to_string(), b.to_string()),
            Equation::Assign(a) => Equation::Assign(*a),
        }
    }
}

fn assignment(input: &str) -> IResult<&str, Equation> {
    let mut parse = map_res(digit1, |s: &str| s.parse::<i64>());
    let (input, value) = parse(input)?;
    Ok((input, Equation::Assign(value)))
}

fn summand(input: &str) -> IResult<&str, Equation> {
    let (input, (summand1, summand2)) = separated_pair(alpha1, tag(" + "), alpha1)(input)?;
    Ok((input, Equation::Add(summand1.to_string(), summand2.to_string())))
}

fn difference(input: &str) -> IResult<&str, Equation> {
    let (input, (minuend, subtrahend)) = separated_pair(alpha1, tag(" - "), alpha1)(input)?;
    Ok((input, Equation::Subtract(minuend.to_string(), subtrahend.to_string())))
}

fn product(input: &str) -> IResult<&str, Equation> {
    let (input, (multiplier, multiplicand)) = separated_pair(alpha1, tag(" * "), alpha1)(input)?;
    Ok((input, Equation::Multiply(multiplier.to_string(), multiplicand.to_string())))
}

fn quotient(input: &str) -> IResult<&str, Equation> {
    let (input, (dividend, divisor)) = separated_pair(alpha1, tag(" / "), alpha1)(input)?;
    Ok((input, Equation::Divide(dividend.to_string(), divisor.to_string())))
}

fn expression(input: &str) -> IResult<&str, Equation> {
    let (input, op) = alt((product, quotient, summand, difference, assignment))(input)?;
    Ok((input, op))
}

fn parse_monkey_jobs(input: &str) -> IResult<&str, Vec<(&str, Equation)>> {
    let (input, vecs) =
        separated_list1(newline, separated_pair(alpha1, tag(": "), expression))(input)?;
    Ok((input, vecs))
}

impl CommandImpl for Day21 {
    fn main(&self) -> Result<(), DynError> {
        //////println!("EX: {:?}", self.input);
        let file = fs::read_to_string(&self.input).unwrap();
        let x = parse_monkey_jobs(&file).unwrap();
        let map = x.1.into_iter().map(|chunk| (chunk.0, chunk.1)).collect::<HashMap<_, _>>();
        let mut g: Graph<&str, String> = Graph::new();
        let mut stack: Vec<String> = vec![String::from("root")];
        //////println!("before: {:?}", map);
        for (k, v) in map.iter() {
            //println!("{v}");
            g.add_node(k);
        }
        for (k, v) in map.iter() {
            match v {
                Equation::Add(x, y) => {
                    let parent = g.node_indices().find(|i| g[*i] == *k).unwrap();
                    let lhs = g.node_indices().find(|i| g[*i] == x).unwrap();
                    let rhs = g.node_indices().find(|i| g[*i] == y).unwrap();
                    g.add_edge(parent, lhs, String::from("l"));
                    g.add_edge(parent, rhs, String::from("r"));
                }
                Equation::Subtract(x, y) => {
                    let parent = g.node_indices().find(|i| g[*i] == *k).unwrap();
                    let lhs = g.node_indices().find(|i| g[*i] == x).unwrap();
                    let rhs = g.node_indices().find(|i| g[*i] == y).unwrap();
                    g.add_edge(parent, lhs, String::from("l"));
                    g.add_edge(parent, rhs, String::from("r"));
                }
                Equation::Multiply(x, y) => {
                    let parent = g.node_indices().find(|i| g[*i] == *k).unwrap();
                    let lhs = g.node_indices().find(|i| g[*i] == x).unwrap();
                    let rhs = g.node_indices().find(|i| g[*i] == y).unwrap();
                    g.add_edge(parent, lhs, String::from("l"));
                    g.add_edge(parent, rhs, String::from("r"));
                }
                Equation::Divide(x, y) => {
                    let parent = g.node_indices().find(|i| g[*i] == *k).unwrap();
                    let lhs = g.node_indices().find(|i| g[*i] == x).unwrap();
                    let rhs = g.node_indices().find(|i| g[*i] == y).unwrap();
                    g.add_edge(parent, lhs, String::from("l"));
                    g.add_edge(parent, rhs, String::from("r"));
                }
                Equation::Assign(x) => {
                    // do nothing
                }
            }
        }
        //println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
        let mut f = File::create("example1.dot").unwrap();
        let output = format!("{}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
        f.write_all(&output.as_bytes());
        excavate(&g, map);
        //println!();
        //println!("after: {:?}", g);
        //for (k, v) in map.iter() {
        //    //println!("{:?} {:?}", k, v);
        //}
        //println!("{:?}\n", g);

        Ok(())
    }
}

fn excavate_part2(graph: &Graph<&str, String>, h: HashMap<&str, Equation>) {
    let mut i: u32 = 0;
    let mut mapped_values: HashMap<&str, i64> = HashMap::new();
    let root_label: &str = &String::from("root");
    let start = graph.node_indices().find(|i| graph[*i] == root_label).unwrap();
    let dfs = DfsPostOrder::new(&graph, start);
    for node_id in dfs.iter(graph) {
        let node_label = graph.node_weight(node_id).unwrap();
        match h.get(node_label).unwrap() {
            &Equation::Assign(x) => {
                    mapped_values.insert(node_label, x);
            }
            Equation::Add(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs + rhs);
            }
            Equation::Subtract(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs - rhs);
            }
            Equation::Multiply(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs * rhs);
            }
            Equation::Divide(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs / rhs);
            }
            _ => {
                // do nothing
            }
        }
        let root_string: String = String::from("root");
        let root_label: &str = &root_string;
        //let root_value: &i64 = mapped_values.get(root_label).unwrap();
    }
    println!("root = {:?}", mapped_values);
    //println!("{:?}", mapped_values);
}

fn excavate(graph: &Graph<&str, String>, h: HashMap<&str, Equation>) {
    let mut i: u32 = 0;
    let mut mapped_values: HashMap<&str, i64> = HashMap::new();
    let root_label: &str = &String::from("root");
    let start = graph.node_indices().find(|i| graph[*i] == root_label).unwrap();
    let dfs = DfsPostOrder::new(&graph, start);
    for node_id in dfs.iter(graph) {
        let node_label = graph.node_weight(node_id).unwrap();
        match h.get(node_label).unwrap() {
            &Equation::Assign(x) => {
                mapped_values.insert(node_label, x);
            }
            Equation::Add(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs + rhs);
            }
            Equation::Subtract(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs - rhs);
            }
            Equation::Multiply(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs * rhs);
            }
            Equation::Divide(x, y) => {
                let lhs_label: &str = &x;
                let rhs_label: &str = &y;
                let lhs: &i64 = mapped_values.get(lhs_label).unwrap();
                let rhs: &i64 = mapped_values.get(rhs_label).unwrap();
                mapped_values.insert(node_label, lhs / rhs);
            }
            _ => {
                // do nothing
            }
        }
        let root_string: String = String::from("root");
        let root_label: &str = &root_string;
        //let root_value: &i64 = mapped_values.get(root_label).unwrap();
    }
    println!("root = {:?}", mapped_values);
    //println!("{:?}", mapped_values);
}

//3) Push the current node to S and set current = current->left until current is NULL
//4) If current is NULL and stack is not empty then
//     a) Pop the top item from stack.
//     b) Print the popped item, set current = popped_item->right
//     c) Go to step 3.
//5) If current is NULL and stack is empty then we are done.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assign_works() {
        let x: String = "root: 3".to_string();
        let mut curr = parse_monkey_jobs(&x).unwrap();
        //println!("curr = {:?}", curr);
        let actual = curr.1.pop().unwrap().1;
        let expected: Equation = Equation::Assign(3);
        assert_eq!(actual, expected);
    }

    #[test]
    fn add_works() {
        let x: String = "root: a + b".to_string();
        let mut curr = parse_monkey_jobs(&x).unwrap();
        //println!("curr = {:?}", curr);
        let actual = curr.1.pop().unwrap().1;
        let expected: Equation = Equation::Add("a".to_string(), "b".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn sub_works() {
        let x: String = "root: a - b".to_string();
        let mut curr = parse_monkey_jobs(&x).unwrap();
        //println!("curr = {:?}", curr);
        let actual = curr.1.pop().unwrap().1;
        let expected: Equation = Equation::Subtract("a".to_string(), "b".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn mult_works() {
        let x: String = "root: a * b".to_string();
        let mut curr = parse_monkey_jobs(&x).unwrap();
        //println!("curr = {:?}", curr);
        let actual = curr.1.pop().unwrap().1;
        let expected: Equation = Equation::Multiply("a".to_string(), "b".to_string());
        assert_eq!(actual, expected);
    }

    #[test]
    fn div_works() {
        let x: String = "root: a / b".to_string();
        let mut curr = parse_monkey_jobs(&x).unwrap();
        //println!("curr = {:?}", curr);
        let actual = curr.1.pop().unwrap().1;
        let expected: Equation = Equation::Divide("a".to_string(), "b".to_string());
        assert_eq!(actual, expected);
    }
}
