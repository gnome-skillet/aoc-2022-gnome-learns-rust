use clap::Parser;

use glam::IVec2;

use itertools::iproduct;

use super::{CommandImpl, DynError};

use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};

use std::path::PathBuf;

use std::mem::swap;

use std::fmt;

use std::fs;

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    *,
};

#[derive(Parser, Debug)]
pub struct Day23 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn parse_valley(input: &str) -> IResult<&str, Valley> {
    let (input, vecs) = separated_list1(newline, many1(one_of(".#")))(input)?;
    let mut occupied_vec: Vec<IVec2> = Vec::new();
    let mut space: Vec<Vec<bool>> = get_empty_valley(vecs.len(), vecs[0].len());
    for (y, l) in vecs.iter().enumerate() {
        for (x, p) in l.iter().enumerate() {
            if *p == '#' {
                let elf: IVec2 = IVec2::new(x as i32, y as i32);
                occupied_vec.push(elf);
                space[y][x] = true;
            }
        }
    }
    let occupied: HashSet<IVec2> = HashSet::from_iter(occupied_vec.iter().cloned());
    let valley = Valley::new(occupied);
    Ok((input, valley))
}

fn get_empty_valley(length: usize, width: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; length]
}

#[derive(Debug)]
struct Valley {
    occupied: HashSet<IVec2>,
    directions: Vec<Direction>,
}

impl Valley {
    fn new(occupied: HashSet<IVec2>) -> Valley {
        let directions = vec![Direction::North, Direction::South, Direction::West, Direction::East];
        Valley { occupied, directions }
    }

    fn n_elves(&self) -> Option<usize> {
        Some(self.occupied.len())
    }

    fn isolated(&self, elf: IVec2) -> bool {
        let xvec: Vec<i32> = vec![elf.x - 1, elf.x, elf.x + 1];
        let yvec: Vec<i32> = vec![elf.y - 1, elf.y, elf.y + 1];
        let product: Vec<IVec2> = iproduct!(xvec, yvec)
            .map(|(a, b)| IVec2::new(a, b))
            .filter(|&z| z != elf)
            .collect::<Vec<IVec2>>();

        for e in product.iter() {
            if self.occupied.contains(e) {
                return false;
            }
        }
        true
    }

    fn next_move(&self, elf: IVec2) -> Option<IVec2> {
        let mut p: Option<IVec2> = None;
        if !self.isolated(elf) {
            for dxn in self.directions.iter() {
                if *dxn == Direction::North && !(self.blocked_above(elf)) {
                    p = Some(IVec2::new(elf.x, elf.y - 1));
                    break;
                } else if *dxn == Direction::South && !(self.blocked_below(elf)) {
                    p = Some(IVec2::new(elf.x, elf.y + 1));
                    break;
                } else if *dxn == Direction::West && !(self.blocked_left(elf)) {
                    p = Some(IVec2::new(elf.x - 1, elf.y));
                    break;
                } else if *dxn == Direction::East && !(self.blocked_right(elf)) {
                    p = Some(IVec2::new(elf.x + 1, elf.y));
                    break;
                }
            }
        }

        p
    }

    fn shuffle_directions(&mut self) {
        let first_direction = self.directions[0];
        self.directions.remove(0);
        self.directions.push(first_direction);
    }

    fn blocked_above(&self, elf: IVec2) -> bool {
        for x in (elf.x - 1)..(elf.x + 2) {
            let p: IVec2 = IVec2::new(x, elf.y - 1);
            if self.occupied.contains(&p) {
                return true;
            }
        }
        false
    }

    fn blocked_below(&self, elf: IVec2) -> bool {
        for x in (elf.x - 1)..(elf.x + 2) {
            let p: IVec2 = IVec2::new(x, elf.y + 1);
            if self.occupied.contains(&p) {
                return true;
            }
        }
        false
    }

    fn blocked_left(&self, elf: IVec2) -> bool {
        for y in (elf.y - 1)..(elf.y + 2) {
            let p: IVec2 = IVec2::new(elf.x - 1, y);
            if self.occupied.contains(&p) {
                return true;
            }
        }
        false
    }

    fn blocked_right(&self, elf: IVec2) -> bool {
        for y in (elf.y - 1)..(elf.y + 2) {
            let p: IVec2 = IVec2::new(elf.x + 1, y);
            if self.occupied.contains(&p) {
                return true;
            }
        }
        false
    }

    fn plan_moves(&mut self) -> HashMap<IVec2, Vec<IVec2>> {
        let mut dict: HashMap<IVec2, Vec<IVec2>> = HashMap::new();
        println!("{:?}", self.directions);
        for elf in self.occupied.iter() {
            match self.next_move(*elf) {
                Some(p) => match dict.entry(p) {
                    Entry::Vacant(e) => {
                        e.insert(vec![*elf]);
                    }
                    Entry::Occupied(e) => {
                        e.into_mut().push(*elf);
                    }
                },
                None => {
                    // do nothing
                }
            }
        }
        self.shuffle_directions();
        dict
    }

    fn rectangle(&self) -> (IVec2, IVec2) {
        let minx = self.occupied.iter().map(|x| x.x).min();
        let maxx = self.occupied.iter().map(|x| x.x).max();
        let miny = self.occupied.iter().map(|y| y.y).min();
        let maxy = self.occupied.iter().map(|y| y.y).max();
        (IVec2::new(minx.unwrap(), miny.unwrap()), IVec2::new(maxx.unwrap(), maxy.unwrap()))
    }

    fn nempty(&self) -> usize {
        let (upper_left, lower_right) = self.rectangle();
        let dim =
            ((lower_right.x - upper_left.x + 1) * (lower_right.y - upper_left.y + 1)) as usize;
        dim - self.occupied.len()
    }

    fn execute_moves(&mut self, planned_moves: HashMap<IVec2, Vec<IVec2>>) -> usize {
        let mut nmoves: usize = 0;

        for (key, value) in planned_moves {
            if value.len() == 1 {
                let oldelf: IVec2 = value[0];
                self.occupied.remove(&oldelf);
                self.occupied.insert(key);
                nmoves += 1;
            }
        }
        nmoves
    }
}

impl CommandImpl for Day23 {
    fn main(&self) -> Result<(), DynError> {
        let characters = fs::read_to_string(&self.input).unwrap();
        let (_, mut valley) = parse_valley(&characters).unwrap();
        let mut round: usize = 1;
        loop {
            let planned_moves = valley.plan_moves();
            let nmoves: usize = valley.execute_moves(planned_moves);
            if nmoves == 0 {
                println!("no elf moved at move {round}");
                break;
            }
            round += 1;
        }
        let dim: usize = valley.nempty();

        println!("{dim} empty spaces");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_compare_after_1_turn() {
        let initial_state = String::from(
            "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............",
        );
        let turn1 = String::from(
            "..............
.......#......
.....#...#....
...#..#.#.....
.......#..#...
....#.#.##....
..#..#.#......
..#.#.#.##....
..............
....#..#......
..............
..............
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn1, "compare first turn");
    }

    #[test]
    fn string_compare_after_2_turns() {
        let initial_state = String::from(
            "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............",
        );
        let turn2 = String::from(
            "..............
.......#......
....#.....#...
...#..#.#.....
.......#...#..
...#..#.#.....
.#...#.#.#....
..............
..#.#.#.##....
....#..#......
..............
..............
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn2, "compare second turn");
    }

    #[test]
    fn string_compare_after_3_turns() {
        let initial_state = String::from(
            "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............",
        );
        let turn3 = String::from(
            "..............
.......#......
.....#....#...
..#..#...#....
.......#...#..
...#..#.#.....
.#..#.....#...
.......##.....
..##.#....#...
...#..........
.......#......
..............
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn3, "compare second turn");
    }

    #[test]
    fn string_compare_after_4_turns() {
        let initial_state = String::from(
            "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............",
        );
        let turn4 = String::from(
            "..............
.......#......
......#....#..
..#...##......
...#.....#.#..
.........#....
.#...###..#...
..#......#....
....##....#...
....#.........
.......#......
..............
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn4, "compare fourth turn");
    }

    #[test]
    fn string_compare_after_5_turns() {
        let initial_state = String::from(
            "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............",
        );
        let turn5 = String::from(
            ".......#......
..............
..#..#.....#..
.........#....
......##...#..
.#.#.####.....
...........#..
....##..#.....
..#...........
..........#...
....#..#......
..............
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn5, "compare fifth turn");
    }

    #[test]
    fn string_compare_after_1_turn_toy() {
        let initial_state = String::from(
            ".....
..##.
..#..
.....
..##.
.....
",
        );
        let turn1 = String::from(
            "..##.
.....
..#..
...#.
..#..
.....
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn1, "compare first turn");
    }

    #[test]
    fn string_compare_after_2_turn_toy() {
        let initial_state = String::from(
            ".....
..##.
..#..
.....
..##.
.....
",
        );
        let turn2 = String::from(
            ".....
..##.
.#...
....#
.....
..#..
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn2, "compare second turn");
    }

    #[test]
    fn string_compare_after_3_turn_toy() {
        let initial_state = String::from(
            ".....
..##.
..#..
.....
..##.
.....
",
        );
        let turn3 = String::from(
            "..#..
....#
#....
....#
.....
..#..
",
        );

        let (_, mut valley) = parse_valley(&initial_state).unwrap();
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        let planned_moves = valley.plan_moves();
        valley.execute_moves(planned_moves);
        assert_eq!(valley.to_string(), turn3, "compare first turn");
    }
}
