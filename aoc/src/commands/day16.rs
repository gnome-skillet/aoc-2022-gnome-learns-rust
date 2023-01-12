use std::path::PathBuf;

use clap::Parser;

use std::{fmt::Display, fmt::Formatter, fs::File};

use super::{CommandImpl, DynError};

use std::fs;

use std::io::Write;

use std::collections::HashMap;

use std::slice::Iter;

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
    multi::separated_list0,
    sequence::{delimited, preceded},
    *,
};

#[derive(Parser, Debug)]
pub struct Day16 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct Valve<'a> {
    valve: String,
    flowrate: u32,
    valves: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn connected_valves(&self) -> std::slice::Iter<&str> {
        let mut iterator = self.valves.iter();
        iterator
    }

    fn valve_edges(&self) -> Vec<(&str, &str)> {
        let edges: Vec<(&str, &str)> = vec![];
        let from: &str = &self.valve;
        for to in self.connected_valves() {
            edges.push((from, to));
        }
        edges
    }
}

fn valve_connections(valves: Vec<Valve<'a>>) -> Vec<(&<'a>str, &<'a>str)> {
    let edges = valves.map(|node| node.valve_edges()).collect();
    edges
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, valve) = alpha1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flowrate) = digit1(input)?;
    let flowrate: u32 = flowrate.parse().unwrap();
    let (input, _) =
        alt((tag("; tunnels lead to valves "), tag("; tunnel leads to valve ")))(input)?;
    let (input, valves) = separated_list0(tag(", "), alpha1)(input)?;
    let valve: String = String::from(valve);
    Ok((input, Valve { valve, flowrate, valves }))
}

fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
    separated_list0(newline, parse_valve)(input)
}

impl CommandImpl for Day16 {
    fn main(&self) -> Result<(), DynError> {
        let file = fs::read_to_string(&self.input).unwrap();
        let valves = parse_valves(&file).unwrap().1;
        for valve in valves.iter() {
            println!("Valve: {:?}", valve);
        }
        let mut g: Graph<&str, String> = Graph::new();
        println!("EX: {:?}", self.input);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_valve() {
        let x: String = "Valve XB has flow rate=0; tunnels lead to valves WZ, LE".to_string();
        let mut curr = parse_valve(&x).unwrap();
        let actual = curr.1;
        let valve = String::from("XB");
        let flowrate: u32 = 0;
        let valves: Vec<&str> = vec!["WZ", "LE"];
        let expected: Valve = Valve { valve, flowrate, valves };
        assert_eq!(actual.valve, expected.valve);
        assert_eq!(actual.flowrate, expected.flowrate);
        assert_eq!(actual.valves, expected.valves);
    }
}
