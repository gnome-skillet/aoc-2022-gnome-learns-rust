use clap::Parser;

use glam::f32::Vec3;

use itertools::Itertools;

use super::{CommandImpl, DynError};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

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

fn smallest_cube(cubes: &Vec<Vec3>) -> Option<Vec3> {
    let xmin: f32 = cubes.iter().map(|x| x.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let ymin: f32 = cubes.iter().map(|x| x.y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let zmin: f32 = cubes.iter().map(|x| x.z).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    Some(Vec3::new(xmin - 1.0, ymin - 1.0, zmin - 1.0))
}

fn largest_cube(cubes: &Vec<Vec3>) -> Option<Vec3> {
    let xmax: f32 = cubes.iter().map(|x| x.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let ymax: f32 = cubes.iter().map(|x| x.y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let zmax: f32 = cubes.iter().map(|x| x.z).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    Some(Vec3::new(xmax + 1.0, ymax + 1.0, zmax + 1.0))
}

fn surface_area(cubes: &Vec<Vec3>) -> Option<usize> {
    let nmatches: usize = pairwise_comparison(cubes);
    let ncubes: usize = cubes.len();
    Some(ncubes * 6 - nmatches * 2)
}

fn adjacent_air(cubes: &Vec<Vec3>) -> Option<HashMap<(i32, i32, i32), u8>> {
    let cube_map: HashSet<(i32, i32, i32)> =
        HashSet::from_iter(cubes.iter().map(|x| (x.x as i32, x.y as i32, x.z as i32)).clone());
    println!("cube_map: {:?}", cube_map);
    let mut left_air: Vec<(i32, i32, i32)> = cubes
        .iter()
        .map(|x| ((x.x - 1.0) as i32, x.y as i32, x.z as i32))
        .filter(|x| !cube_map.contains(x))
        .collect_vec();
    let right_air: Vec<(i32, i32, i32)> = cubes
        .iter()
        .map(|x| ((x.x + 1.0) as i32, x.y as i32, x.z as i32))
        .filter(|x| !cube_map.contains(x))
        .collect_vec();
    let front_air: Vec<(i32, i32, i32)> = cubes
        .iter()
        .map(|x| (x.x as i32, (x.y + 1.0) as i32, x.z as i32))
        .filter(|x| !cube_map.contains(x))
        .collect_vec();
    let back_air: Vec<(i32, i32, i32)> = cubes
        .iter()
        .map(|x| (x.x as i32, (x.y - 1.0) as i32, x.z as i32))
        .filter(|x| !cube_map.contains(x))
        .collect_vec();
    let top_air: Vec<(i32, i32, i32)> = cubes
        .iter()
        .map(|x| (x.x as i32, x.y as i32, (x.z + 1.0) as i32))
        .filter(|x| !cube_map.contains(x))
        .collect_vec();
    let bottom_air: Vec<(i32, i32, i32)> = cubes
        .iter()
        .map(|x| (x.x as i32, x.y as i32, (x.z - 1.0) as i32))
        .filter(|x| !cube_map.contains(x))
        .collect_vec();
    left_air.extend(&right_air);
    left_air.extend(&front_air);
    left_air.extend(&back_air);
    left_air.extend(&top_air);
    left_air.extend(&bottom_air);
    let air_map: HashSet<(i32, i32, i32)> =
        HashSet::from_iter(left_air.iter().map(|x| (x.0, x.1, x.2)).clone());
    let mut air_map = air_map.iter().map(|x| (*x, 0)).collect::<HashMap<_, _>>();
    for (key, value) in air_map {}
    Some(air_map)
}

impl CommandImpl for Day18 {
    fn main(&self) -> Result<(), DynError> {
        let characters = fs::read_to_string(&self.input).unwrap();
        let (_, cubes) = parse_cubes(&characters).unwrap();
        let nsurfaces =
            if let Some(nsurfaces) = surface_area(&cubes) { nsurfaces } else { todo!() };
        println!("the obsidian has {nsurfaces} surfaces exposed to the air");
        let min_obsidian =
            if let Some(min_obsidian) = smallest_cube(&cubes) { min_obsidian } else { todo!() };
        let max_obsidian =
            if let Some(max_obsidian) = largest_cube(&cubes) { max_obsidian } else { todo!() };
        let adjacent = if let Some(adjacent) = adjacent_air(&cubes) { adjacent } else { todo!() };

        println!("the smallest point is {:?}", min_obsidian);
        println!("the largest point is {:?}", max_obsidian);
        println!("the adjacent cubes are {:?}", adjacent);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn string_compare_after_1_turn() {
    //}the
}
