use clap::Parser;

use glam::u32::UVec4;

use std::collections::VecDeque;

use std::fs;

use super::{CommandImpl, DynError};

use std::path::PathBuf;

use strum::IntoEnumIterator; // 0.17.1
                             //
use strum_macros::EnumIter;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space0, space1},
    multi::separated_list1,
    sequence::delimited,
    *,
};

const N_MINUTES: u8 = 32;

#[derive(Parser, Debug)]
pub struct Day19 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, PartialEq, Eq, EnumIter)]
pub enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
    Undefined,
}

impl ResourceType {
    fn new(label: &str) -> Option<ResourceType> {
        match label {
            "ore" => Some(ResourceType::Ore),
            "clay" => Some(ResourceType::Clay),
            "obsidian" => Some(ResourceType::Obsidian),
            "geode" => Some(ResourceType::Geode),
            _ => None,
        }
    }

    fn index(&self) -> Option<usize> {
        match *self {
            ResourceType::Ore => Some(0usize),
            ResourceType::Clay => Some(1usize),
            ResourceType::Obsidian => Some(2usize),
            ResourceType::Geode => Some(3usize),
            ResourceType::Undefined => None,
        }
    }
}

fn get_type(label: &str) -> Option<ResourceType> {
    match label {
        "ore" => Some(ResourceType::Ore),
        "clay" => Some(ResourceType::Clay),
        "obsidian" => Some(ResourceType::Obsidian),
        "geode" => Some(ResourceType::Geode),
        _ => None,
    }
}

trait Factory {
    fn purchase_robot(&mut self, resource_type: &ResourceType);
    fn gather_resources(&mut self);
    fn score(&self) -> u32;
    fn minute(&self) -> u8;
}

#[derive(Debug, Clone)]
/// Factory builds robots and collects resources based on resources gathered
pub struct RobotFactory {
    /// the cost of each robot type in resources
    resource_costs: Vec<UVec4>,
    /// the count of each robot type (categorized by resource)
    robot_counts: UVec4,
    /// the resources gathered
    gathered_resources: UVec4,
    minute: u8,
    first_minute: u8,
}

/// Factory builds robots based collected resources
impl RobotFactory {
    fn new(labeled_resource_costs: Vec<(ResourceType, UVec4)>) -> RobotFactory {
        let mut resource_costs: Vec<UVec4> = Vec::with_capacity(4);
        let robot_counts: glam::UVec4 = glam::UVec4::new(1, 0, 0, 0);
        let gathered_resources: UVec4 = glam::UVec4::new(0, 0, 0, 0);
        let minute: u8 = 0;
        let first_minute: u8 = 0;

        for labeled_resource_cost in labeled_resource_costs {
            let (_, resource_cost) = labeled_resource_cost;
            resource_costs.push(resource_cost);
        }

        RobotFactory { resource_costs, robot_counts, gathered_resources, minute, first_minute }
    }

    fn n_opening_moves(&self) -> usize {
        if self.resource_costs[0].x < self.resource_costs[1].x {
            self.resource_costs[0].x as usize
        } else {
            self.resource_costs[1].x as usize
        }
    }

    fn floor(&self) -> u32 {
        let nminutes: u32 = N_MINUTES as u32 - self.minute() as u32;
        self.gathered_resources.w + nminutes * self.robot_counts.w
    }

    fn ceiling(&self) -> u32 {
        let mut first_minute: u32 = self.first_minute as u32;
        if self.first_minute == 0 {
            first_minute = self.minute() as u32 + 1;
        }
        let nminutes: u32 = N_MINUTES as u32 - first_minute + 1;
        self.floor() + (nminutes + (nminutes + 1)) / 2
    }

    fn skip_turn(&self, cutoff: &u32) -> bool {
        ((self.robot_counts.x > self.resource_costs[0].x)
            && (self.robot_counts.x > self.resource_costs[1].x)
            && (self.robot_counts.x > self.resource_costs[2].x)
            && (self.robot_counts.x > self.resource_costs[3].x))
            || (self.robot_counts.y > self.resource_costs[2].y)
            || (self.robot_counts.z > self.resource_costs[3].z)
            || (self.minute <= 20 && (self.robot_counts.w < *cutoff))
    }

    fn n_robots_produced(&self, resource_type: ResourceType) -> Option<usize> {
        match resource_type {
            ResourceType::Ore => Some(self.robot_counts.x as usize),
            ResourceType::Clay => Some(self.robot_counts.y as usize),
            ResourceType::Obsidian => Some(self.robot_counts.z as usize),
            ResourceType::Geode => Some(self.robot_counts.w as usize),
            _ => None,
        }
    }

    fn n_robots_needed(&self, resource_type: ResourceType) -> Option<usize> {
        match resource_type {
            ResourceType::Ore => Some(self.robot_counts.x as usize),
            ResourceType::Clay => Some(self.robot_counts.y as usize),
            ResourceType::Obsidian => Some(self.robot_counts.z as usize),
            ResourceType::Geode => Some(self.robot_counts.w as usize),
            _ => None,
        }
    }

    fn robot_tally(&self) -> UVec4 {
        self.robot_counts
    }

    fn get_gathered_resources(&self) -> Option<UVec4> {
        Some(self.gathered_resources)
    }

    fn affordable(&self, resource_type: &ResourceType) -> bool {
        if let Some(index) = resource_type.index() {
            return self.gathered_resources.x >= self.resource_costs[index].x
                && (self.gathered_resources.y >= self.resource_costs[index].y)
                && (self.gathered_resources.z >= self.resource_costs[index].z);
        }
        true
    }

    fn fun_score(&self) -> u32 {
        let mut xscore: u32 = (self.gathered_resources.x + 1) * (self.robot_counts.x);
        xscore = xscore * (25 - self.minute as u32);
        let mut yscore = (self.gathered_resources.y + 1) * (self.robot_counts.y);
        yscore = yscore * (25 - self.minute as u32);
        let mut zscore = (self.gathered_resources.z + 1) * (self.robot_counts.z);
        zscore = zscore * (25 - self.minute as u32);
        let mut wscore = (self.gathered_resources.w + 1) * (self.robot_counts.w);
        wscore = wscore.pow(25 - self.minute as u32);
        xscore + yscore.pow(2) + zscore.pow(3) + wscore.pow(4)
    }

    fn next_move(&self, resource_type: &ResourceType) -> Option<RobotFactory> {
        let mut x: RobotFactory = self.clone();
        x.gather_resources();
        if !self.affordable(resource_type) {
            return None;
        }
        x.purchase_robot(resource_type);
        Some(x)
    }

    fn robot_cost(&self, resource_type: ResourceType) -> Option<UVec4> {
        let index: usize = resource_type.index()?;
        Some(self.resource_costs[index])
    }

    fn log_gathering(&self) {
        if self.robot_counts.x > 0 {
            println!(
                "minute({:?}): {:?} ore-collecting robot collects {:?} ore; you now have {:?} ore.",
                self.minute, self.robot_counts.x, self.robot_counts.x, self.gathered_resources.x
            );
        }
        if self.robot_counts.y > 0 {
            println!("minute({:?}): {:?} clay-collecting robot collects {:?} clay; you now have {:?} clay.",
                   self.minute,
                   self.robot_counts.y,
                   self.robot_counts.y,
                   self.gathered_resources.y);
        }
        if self.robot_counts.z > 0 {
            println!("minute({:?}): {:?} obsidian-collecting robot collects {:?} obsidian; you now have {:?} obsidian.",
                   self.minute,
                   self.robot_counts.z,
                   self.robot_counts.z,
                   self.gathered_resources.z);
        }
        if self.robot_counts.w > 0 {
            println!("minute({:?}): {:?} geode-collecting robot collects {:?} geode; you now have {:?} geode.",
                   self.minute,
                   self.robot_counts.w,
                   self.robot_counts.w,
                   self.gathered_resources.w);
        }
    }
}

impl Factory for RobotFactory {
    fn purchase_robot(&mut self, resource_type: &ResourceType) {
        if let Some(index) = resource_type.index() {
            self.gathered_resources -= self.resource_costs[index];
            match resource_type {
                ResourceType::Ore => {
                    self.robot_counts.x += 1;
                }
                ResourceType::Clay => {
                    self.robot_counts.y += 1;
                }
                ResourceType::Obsidian => {
                    self.robot_counts.z += 1;
                }
                ResourceType::Geode => {
                    self.robot_counts.w += 1;
                }
                _ => panic!(),
            }
        }
        //println!("robot_counts {:?}", self.robot_counts);
    }

    fn gather_resources(&mut self) {
        self.minute += 1;
        self.gathered_resources = self.gathered_resources + self.robot_counts;
        if self.gathered_resources.w == 1 {
            self.first_minute = self.minute();
        }
    }

    fn score(&self) -> u32 {
        self.gathered_resources.w
    }

    fn minute(&self) -> u8 {
        self.minute
    }
}

impl CommandImpl for Day19 {
    fn main(&self) -> Result<(), DynError> {
        let characters = fs::read_to_string(&self.input).unwrap();
        let (_, robot_factories) = parse_robot_factories(&characters).unwrap();
        let mut scores: VecDeque<u32> = VecDeque::new();

        for mut factory in robot_factories {
            let mut factory_vec: VecDeque<RobotFactory> = VecDeque::new();
            let mut top_score: u32 = 0;
            let mut highest_floor: u32 = 0;
            let mut cutoff: u32 = 0;
            let mut last_top_score: u32 = 0;
            for _ in 0..factory.n_opening_moves() {
                factory.gather_resources();
            }
            factory_vec.push_back(factory);
            while !factory_vec.is_empty() {
                let factory = factory_vec.pop_front().unwrap();
                for resource_type in ResourceType::iter() {
                    if let Some(f) = factory.next_move(&resource_type) {
                        if f.skip_turn(&cutoff) {
                            continue;
                        }
                        //f.log_gathering();
                        if f.score() > top_score {
                            cutoff = top_score;
                            last_top_score = top_score;
                            top_score = f.score();
                            println!("score updated to {top_score} at minute {:?}", f.minute());
                        }
                        if f.minute() < N_MINUTES && f.ceiling() >= highest_floor {
                            if f.floor() > highest_floor {
                                highest_floor = f.floor();
                                let lowest_floor: u32 = f.ceiling();
                                println!("minute({:?}): floor = {highest_floor:?}, ceiling = {lowest_floor:?}", f.minute());
                            }
                            factory_vec.push_back(f);
                        }
                    }
                }
                if top_score > last_top_score {
                    //println!("top score: {:?}", top_score);
                    last_top_score = top_score;
                }
            }
            scores.push_back(top_score);
        }

        let final_score: u32 = scores.iter().enumerate().map(|(i, x)| (i as u32 + 1) * *x).sum();

        println!("length: {:?}", scores.len());
        println!("length: {:?}", scores);
        println!("scores: {:?}", final_score);
        let elements: u32 = scores.into_iter().sum();
        println!("elements: {:?}", elements);

        Ok(())
    }
}

fn parse_resource_type(input: &str) -> IResult<&str, ResourceType> {
    let (input, resource_type) =
        alt((tag("ore"), tag("clay"), tag("obsidian"), tag("geode")))(input)?;
    let resource_type =
        get_type(resource_type).expect("expect valid type (ore, clay, obsidian, geode)");
    Ok((input, resource_type))
}

pub struct Robot {
    label: String,
    cost: u32,
}

fn parse_robot_cost(input: &str) -> IResult<&str, (ResourceType, u32)> {
    let (input, _) = space0(input)?;
    let (input, cost) = nom::character::complete::digit1(input)?;
    let cost = cost.to_string();
    let cost = cost.parse::<u32>().expect("expect valid positive integer");
    let (input, _) = space0(input)?;
    let (input, resource_type) = parse_resource_type(input)?;
    Ok((input, (resource_type, cost)))
}

fn parse_robot_costs(input: &str) -> IResult<&str, Vec<(ResourceType, u32)>> {
    let (input, costs) = separated_list1(tag(" and "), parse_robot_cost)(input)?;
    Ok((input, costs))
}

fn calculate_robot_resources(resource_costs: Vec<(ResourceType, u32)>) -> Option<UVec4> {
    let (mut x, mut y, mut z, mut w) = (0, 0, 0, 0);
    for cost in resource_costs {
        match cost.0 {
            ResourceType::Ore => {
                x = cost.1;
            }
            ResourceType::Clay => {
                y = cost.1;
            }
            ResourceType::Obsidian => {
                z = cost.1;
            }
            ResourceType::Geode => {
                w = cost.1;
            }
            _ => panic!(),
        }
    }

    Some(UVec4::new(x, y, z, w))
}

fn parse_robot_resource_cost(input: &str) -> IResult<&str, (ResourceType, UVec4)> {
    let (input, _) = tag("Each")(input)?;
    let (input, _) = space0(input)?;
    let (input, resource_type) = parse_resource_type(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("robot costs")(input)?;
    let (input, _) = space0(input)?;
    let (input, resource_costs) = parse_robot_costs(input)?;
    let (input, _) = tag(".")(input)?;

    let resource_costs =
        calculate_robot_resources(resource_costs).expect("expect valid resource cost vec4");
    Ok((input, (resource_type, resource_costs)))
}

fn parse_robot_blueprint(input: &str) -> IResult<&str, Vec<(ResourceType, UVec4)>> {
    let (input, blueprint) = separated_list1(space1, parse_robot_resource_cost)(input)?;
    Ok((input, blueprint))
}

fn parse_robot_factory(input: &str) -> IResult<&str, RobotFactory> {
    let (input, _) = delimited(tag("Blueprint "), nom::character::complete::u64, tag(":"))(input)?;
    //let (input, _) = space1(input)?;
    //let (input, cost) = nom::character::complete::digit1(input)?;
    //let (input, _) = tag(":")(input)?;
    let (input, _) = space0(input)?;
    let (input, resource_costs) = parse_robot_blueprint(input)?;
    let robot_factory: RobotFactory = RobotFactory::new(resource_costs);

    Ok((input, robot_factory))
}

fn parse_robot_factories(input: &str) -> IResult<&str, Vec<RobotFactory>> {
    let (input, factories) = separated_list1(newline, parse_robot_factory)(input)?;
    Ok((input, factories))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_robot_cost() {
        let (_, cost) = parse_robot_cost("2 ore").unwrap();
        let resource_type: ResourceType = cost.0;
        let cost: u32 = cost.1;
        assert_eq!(cost, 2, "cost evaluation");
        assert_eq!(resource_type, ResourceType::Ore, "resource evaluation");
    }

    #[test]
    fn test_parse_robot_costs_single() {
        let (_, costs) = parse_robot_costs("2 ore").unwrap();
        let resource_type: &ResourceType = &costs[0].0;
        let cost: &u32 = &costs[0].1;
        assert_eq!(*cost, 2, "cost evaluation");
        assert_eq!(*resource_type, ResourceType::Ore, "resource evaluation");
    }

    #[test]
    fn test_parse_robot_costs_multiple() {
        let (_, costs) = parse_robot_costs("2 ore and 6 obsidian.").unwrap();
        let resource_type: &ResourceType = &costs[0].0;
        let cost: &u32 = &costs[0].1;
        assert_eq!(*cost, 2, "cost evaluation");
        assert_eq!(*resource_type, ResourceType::Ore, "resource evaluation");

        let resource_type: &ResourceType = &costs[1].0;
        let cost: &u32 = &costs[1].1;
        assert_eq!(*cost, 6, "cost evaluation");
        assert_eq!(*resource_type, ResourceType::Obsidian, "resource evaluation");
    }

    #[test]
    fn test_parse_robot_resource_cost() {
        let msg: &str = "Each obsidian robot costs 3 ore and 14 clay.";
        let (_, observed_cost) = parse_robot_resource_cost(msg).unwrap();
        let expected_cost = UVec4::new(3, 14, 0, 0);

        let observed_cost: UVec4 = observed_cost.1;
        assert_eq!(expected_cost, observed_cost);
    }

    #[test]
    fn test_parse_robot_blueprint() {
        let msg: &str = "Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 1 ore and 1 obsidian.";
        let (_, blueprint) = parse_robot_blueprint(msg).unwrap();
        let expected_cost = UVec4::new(2, 0, 0, 0);

        let observed_cost: UVec4 = blueprint[0].1;
        assert_eq!(expected_cost, observed_cost);
    }

    #[test]
    fn test_parse_robot_factory() {
        let msg: &str = "Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 1 ore and 1 obsidian.";
        let (_, factory) = parse_robot_factory(msg).unwrap();
        let expected_cost = UVec4::new(1, 0, 1, 0);

        let observed_cost: UVec4 = factory.robot_cost(ResourceType::Geode).unwrap();
        assert_eq!(expected_cost, observed_cost);
    }

    #[test]
    fn test_factory_iterator_no_clay() {
        let costs: Vec<(ResourceType, UVec4)> = vec![
            (ResourceType::Ore, UVec4::new(2, 0, 0, 0)),
            (ResourceType::Clay, UVec4::new(2, 0, 0, 0)),
            (ResourceType::Obsidian, UVec4::new(2, 7, 0, 0)),
            (ResourceType::Geode, UVec4::new(4, 0, 6, 0)),
        ];
        let mut robot_factory = RobotFactory::new(costs);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        let mut observed: Vec<RobotFactory> = Vec::new();
        for resource_type in ResourceType::iter() {
            if let Some(factory) = robot_factory.next_move(&resource_type) {
                observed.push(factory);
            }
        }
        assert_eq!(observed.len(), 3usize);
    }

    #[test]
    fn test_factory_iterator_no_obsidian() {
        let costs: Vec<(ResourceType, UVec4)> = vec![
            (ResourceType::Ore, UVec4::new(2, 0, 0, 0)),
            (ResourceType::Clay, UVec4::new(2, 0, 0, 0)),
            (ResourceType::Obsidian, UVec4::new(2, 7, 0, 0)),
            (ResourceType::Geode, UVec4::new(4, 0, 6, 0)),
        ];
        let mut robot_factory = RobotFactory::new(costs);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Clay);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Clay);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        let mut observed: Vec<RobotFactory> = Vec::new();
        for resource_type in ResourceType::iter() {
            if let Some(factory) = robot_factory.next_move(&resource_type) {
                observed.push(factory);
            }
        }
        assert_eq!(observed.len(), 4usize);
    }

    #[test]
    fn test_factory_iterator_no_geode() {
        let costs: Vec<(ResourceType, UVec4)> = vec![
            (ResourceType::Ore, UVec4::new(2, 0, 0, 0)),
            (ResourceType::Clay, UVec4::new(2, 0, 0, 0)),
            (ResourceType::Obsidian, UVec4::new(2, 7, 0, 0)),
            (ResourceType::Geode, UVec4::new(4, 0, 6, 0)),
        ];
        let mut robot_factory = RobotFactory::new(costs);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Clay);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Clay);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Ore);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Obsidian);
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.gather_resources();
        robot_factory.purchase_robot(&ResourceType::Obsidian);
        let mut observed: Vec<RobotFactory> = Vec::new();
        for resource_type in ResourceType::iter() {
            if let Some(factory) = robot_factory.next_move(&resource_type) {
                observed.push(factory);
            }
        }
        assert_eq!(observed.len(), 5usize);
    }
}
