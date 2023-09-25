use clap::Parser;

use glam::IVec2;

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
    let mut occupied_spaces: Vec<IVec2> = Vec::new();
    let mut space: Vec<Vec<bool>> = get_empty_valley(vecs.len(), vecs[0].len());
    for (y, l) in vecs.iter().enumerate() {
        for (x, p) in l.iter().enumerate() {
            if *p == '#' {
                let elf: IVec2 = IVec2::new(x as i32, y as i32);
                occupied_spaces.push(elf);
                space[y][x] = true;
            }
        }
    }
    let valley = Valley::new(occupied_spaces, space);
    Ok((input, valley))
}

fn get_empty_valley(length: usize, width: usize) -> Vec<Vec<bool>> {
    vec![vec![false; width]; length]
}

#[derive(Debug)]
struct Valley {
    occupied_spaces: Vec<IVec2>,
    spaces: Vec<Vec<bool>>,
    directions: Vec<Direction>,
}

impl Valley {
    fn new(occupied_spaces: Vec<IVec2>, spaces: Vec<Vec<bool>>) -> Valley {
        let directions = vec![Direction::North, Direction::South, Direction::West, Direction::East];
        Valley { occupied_spaces, spaces, directions }
    }

    fn dimensions(&self) -> Option<IVec2> {
        Some(IVec2::new(self.spaces[0].len() as i32, self.spaces.len() as i32))
    }

    fn n_elves(&self) -> Option<usize> {
        Some(self.occupied_spaces.len())
    }

    fn isolated(&self, elf: usize) -> bool {
        let pos = self.occupied_spaces[elf];
        let x: usize = pos.x as usize;
        let y: usize = pos.y as usize;
        let xlim: usize = self.spaces[0].len() - 1;
        let ylim: usize = self.spaces.len() - 1;

        !((x > 0 && self.spaces[y][x - 1])
            || (x != xlim && self.spaces[y][x + 1])
            || (y > 0 && self.spaces[y - 1][x])
            || (y != ylim && self.spaces[y + 1][x])
            || (x > 0 && y > 0 && self.spaces[y - 1][x - 1])
            || (x > 0 && y != ylim && self.spaces[y + 1][x - 1])
            || (x != xlim && y > 0 && self.spaces[y - 1][x + 1])
            || (x != xlim && y != ylim && self.spaces[y + 1][x + 1]))
    }

    fn next_move(&self, elf: usize) -> Option<IVec2> {
        let pos = self.occupied_spaces[elf];
        let x: usize = pos.x as usize;
        let y: usize = pos.y as usize;

        let blocked_north = self.blocked_above(elf);
        let blocked_south = self.blocked_below(elf);
        let blocked_left = self.blocked_left(elf);
        let blocked_right = self.blocked_right(elf);

        let mut p: Option<IVec2> = None;
        if !self.isolated(elf) {
            for dxn in self.directions.iter() {
                if *dxn == Direction::North && !(blocked_north) {
                    p = Some(IVec2::new(x as i32, (y - 1) as i32));
                    break;
                } else if *dxn == Direction::South && !(blocked_south) {
                    p = Some(IVec2::new(x as i32, (y + 1) as i32));
                    break;
                } else if *dxn == Direction::West && !(self.blocked_left(elf)) {
                    p = Some(IVec2::new((x - 1) as i32, y as i32));
                    break;
                } else if *dxn == Direction::East && !(self.blocked_right(elf)) {
                    p = Some(IVec2::new((x + 1) as i32, y as i32));
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

    fn blocked_above(&self, elf: usize) -> bool {
        let x: usize = self.occupied_spaces[elf].x as usize;
        let y: usize = self.occupied_spaces[elf].y as usize;
        let xlim: usize = self.spaces[0].len();

        return (y == 0)
            || (x != 0 && self.spaces[y - 1][x - 1])
            || (x != (xlim - 1) && self.spaces[y - 1][x + 1])
            || self.spaces[y - 1][x];
    }

    fn blocked_below(&self, elf: usize) -> bool {
        let x: usize = self.occupied_spaces[elf].x as usize;
        let y: usize = self.occupied_spaces[elf].y as usize;
        let xlim: usize = self.spaces[0].len();
        let ylim: usize = self.spaces.len();

        (y == (ylim - 1))
            || (x != 0 && self.spaces[y + 1][x - 1])
            || (x != (xlim - 1) && self.spaces[y + 1][x + 1])
            || self.spaces[y + 1][x]
    }

    fn blocked_left(&self, elf: usize) -> bool {
        let x: usize = self.occupied_spaces[elf].x as usize;
        let y: usize = self.occupied_spaces[elf].y as usize;
        let xlim: usize = self.spaces[0].len();
        let ylim: usize = self.spaces.len();

        (x == 0)
            || (y != 0 && self.spaces[y - 1][x - 1])
            || (y != (ylim - 1) && self.spaces[y + 1][x - 1])
            || self.spaces[y][x - 1]
    }

    fn blocked_right(&self, elf: usize) -> bool {
        let x: usize = self.occupied_spaces[elf].x as usize;
        let y: usize = self.occupied_spaces[elf].y as usize;
        let xlim: usize = self.spaces[0].len();
        let ylim: usize = self.spaces.len();

        (x == (xlim - 1))
            || (y != 0 && self.spaces[y - 1][x + 1])
            || (y != (ylim - 1) && self.spaces[y + 1][x + 1])
            || self.spaces[y][x + 1]
    }

    fn plan_moves(&mut self) -> HashMap<IVec2, Vec<usize>> {
        let mut dict: HashMap<IVec2, Vec<usize>> = HashMap::new();
        println!("{:?}", self.directions);
        for (id, _) in self.occupied_spaces.iter().enumerate() {
            match self.next_move(id) {
                Some(p) => match dict.entry(p) {
                    Entry::Vacant(e) => {
                        e.insert(vec![id]);
                    }
                    Entry::Occupied(e) => {
                        e.into_mut().push(id);
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
        let minx = self.occupied_spaces.iter().map(|x| x.x).min();
        let maxx = self.occupied_spaces.iter().map(|x| x.x).max();
        let miny = self.occupied_spaces.iter().map(|y| y.y).min();
        let maxy = self.occupied_spaces.iter().map(|y| y.y).max();
        (IVec2::new(minx.unwrap(), miny.unwrap()), IVec2::new(maxx.unwrap(), maxy.unwrap()))
    }

    fn nempty(&self) -> usize {
        let (upper_left, lower_right) = self.rectangle();
        let dim =
            ((lower_right.x - upper_left.x + 1) * (lower_right.y - upper_left.y + 1)) as usize;
        dim - self.occupied_spaces.len()
    }

    fn execute_moves(&mut self, planned_moves: HashMap<IVec2, Vec<usize>>) {
        for (key, value) in planned_moves {
            if value.len() == 1 {
                let elf: usize = value[0];
                let oldx: usize = self.occupied_spaces[elf].x as usize;
                let oldy: usize = self.occupied_spaces[elf].y as usize;
                self.spaces[oldy][oldx] = false;
                let newx: usize = key.x as usize;
                let newy: usize = key.y as usize;
                self.occupied_spaces[elf] = key;
                self.spaces[newy][newx] = true;
            }
        }
    }
}

impl fmt::Display for Valley {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for v in &self.spaces {
            for b in v {
                let c = if *b { "#" } else { "." };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl CommandImpl for Day23 {
    fn main(&self) -> Result<(), DynError> {
        let characters = fs::read_to_string(&self.input).unwrap();
        let (_, mut valley) = parse_valley(&characters).unwrap();
        let nelves = if let Some(nelves) = valley.n_elves() { nelves } else { 0 };
        println!("{valley} : dimensiones {:?} with {nelves} elves", valley.dimensions());
        for i in 1..11 {
            let planned_moves = valley.plan_moves();
            valley.execute_moves(planned_moves);
            println!("== End of Round {i} ==");
            println!("{valley}");
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
