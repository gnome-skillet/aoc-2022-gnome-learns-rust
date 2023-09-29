use clap::Parser;

use glam::f32::Vec3;

use itertools::Itertools;

use super::{CommandImpl, DynError};

use std::path::PathBuf;

use std::fs;

use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::newline,
    multi::separated_list1, *,
};

#[derive(Parser, Debug)]
pub struct Day18 {
    #[clap(long, short)]
    input: PathBuf,
}

fn distance(this: Vec3, that: Vec3) -> f32 {
    let xdiff = this.x - that.x;
    let ydiff = this.y - that.y;
    let zdiff = this.z - that.z;
    let w: f32 = xdiff.powi(2) + ydiff.powi(2) + zdiff.powi(2);
    w.sqrt()
}

fn connected(this: Vec3, that: Vec3) -> usize {
    if distance(this, that) < 1.05 {
        println!("|{:?}, {:?}|", this, that);
        1
    } else {
        0
    }
}

fn cube(input: &str) -> IResult<&str, Vec3> {
    let (input, dims) = separated_list1(tag(","), digit1)(input)?;
    if dims.len() != 3 {
        panic!("expected 3 numbers");
    }
    let v1: f32 = dims[0].parse::<f32>().unwrap();
    let v2: f32 = dims[1].parse::<f32>().unwrap();
    let v3: f32 = dims[2].parse::<f32>().unwrap();
    let dim3: Vec3 = Vec3::new(v1, v2, v3);
    Ok((input, dim3))
}

fn parse_cubes(input: &str) -> IResult<&str, Vec<Vec3>> {
    let (input, cubes) = separated_list1(newline, cube)(input)?;
    Ok((input, cubes))
}

fn pairwise_comparison(cubes: &Vec<Vec3>) -> usize {
    cubes.iter().tuple_combinations().map(|(x, y)| connected(*x, *y)).sum()
}

impl CommandImpl for Day18 {
    fn main(&self) -> Result<(), DynError> {
        let characters = fs::read_to_string(&self.input).unwrap();
        let (_, cubes) = parse_cubes(&characters).unwrap();
        let nmatches: usize = pairwise_comparison(&cubes);
        let ncubes: usize = cubes.len();
        let nsurfaces = ncubes * 6 - nmatches * 2;
        println!("there are {nmatches} matches across {ncubes} cubes leaving {nsurfaces} exposed surfaces");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn string_compare_after_1_turn() {
    //}
}
