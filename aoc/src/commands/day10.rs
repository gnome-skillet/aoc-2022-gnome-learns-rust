use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_file;

use super::{CommandImpl, DynError};

use std::collections::VecDeque;

const CYCLE_COLLECT: i32 = 20;
const CYCLE_WIDTH: i32 = 40;

use nom::{branch::alt, bytes::complete::tag, character::complete, sequence::separated_pair, *};

#[derive(Parser, Debug)]
pub struct Day10 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
enum RegisterCmd {
    Noop,
    Addx(i32),
    None,
}

fn addx_cmd(input: &str) -> IResult<&str, RegisterCmd> {
    let (input, (_, value)) = separated_pair(tag("addx"), tag(" "), complete::i32)(input)?;

    Ok((input, RegisterCmd::Addx(value)))
}

fn noop_cmd(input: &str) -> IResult<&str, RegisterCmd> {
    let (input, _) = tag("noop")(input)?;

    Ok((input, RegisterCmd::Noop))
}

fn parse_cmd(input: &str) -> IResult<&str, RegisterCmd> {
    let (input, cmd) = alt((addx_cmd, noop_cmd))(input)?;

    Ok((input, cmd))
}

pub struct Clock {
    registerx: i32,
    queue: VecDeque<i32>,
    cycle: i32,
    signal_strength: Vec<i32>,
    crt: String,
}

impl Clock {
    fn new() -> Clock {
        let registerx: i32 = 1;
        let queue: VecDeque<i32> = VecDeque::new();
        let cycle: i32 = 1;
        let signal_strength: Vec<i32> = vec![];
        let crt: String = String::from("");
        Clock { registerx, queue, cycle, signal_strength, crt }
    }

    fn exec_cycle(&mut self, cmd: RegisterCmd) {
        // initiate command
        match cmd {
            RegisterCmd::Addx(s) => {
                self.queue.push_back(0); // add dummy to simulate 2 cycles
                self.queue.push_back(s);
            }
            RegisterCmd::Noop => {
                self.queue.push_back(0); // add dummy to simulate 1 cycle
            }
            RegisterCmd::None => {
                //println!("cycle({:?}) None: registerx({:?})", self.cycle, self.registerx);
            }
        }
    }

    fn sprite_position(&self) -> String {
        let mut sprite = String::from("");
        for i in 0..40 {
            if (self.registerx - i).abs() <= 1 {
                sprite.push('#');
            } else {
                sprite.push('.');
            }
        }
        sprite
    }

    fn simulate_clock(&mut self) {
        println!("Sprite position: {}", self.sprite_position());
        println!("");
        while !self.queue.is_empty() {
            let top: i32 = self.queue.remove(0).unwrap();
            let pixel: i32 = self.cycle.rem_euclid(CYCLE_WIDTH) - 1;
            println!(
                "During cycle  {:?}: CRT draws pixel in position {:?}",
                self.cycle, self.registerx
            );
            if self.registerx.abs_diff(pixel) <= 1 {
                self.crt.push('#');
            } else {
                self.crt.push('.');
            }
            let start: usize = self.cycle as usize / 40;
            let end: usize = self.crt.len().min(start + 40);
            //println!("Current CRT row: {}", &self.crt[(start * 40)..end]);
            if top != 0 {
                self.registerx += top;
                println!(
                    "End of cycle\t{:?}: finish executing addx {top}: (registerx is now {:?})",
                    self.cycle, self.registerx
                );
                println!("Sprite position: {}", self.sprite_position());
            } else {
                match self.queue.front() {
                    Some(x) => {
                        if *x != 0 {
                            println!("Start cycle   {:?}: begin executing addx {:?}", self.cycle, x,)
                        }
                    }
                    None => print!(""),
                };
            }

            if self.cycle % CYCLE_WIDTH == CYCLE_COLLECT {
                let signal: i32 = self.registerx * self.cycle;
                self.signal_strength.push(signal);
            }
            self.cycle += 1;
            println!("");
        }
    }

    fn print_crt(&self) {
        println!("print crt");
        let mut reg_start: usize = 0;
        let mut reg_end: usize = 40;
        let last_cycle: usize = self.cycle as usize;
        while reg_start < self.crt.len() {
            let end_fix: usize = reg_end.min(self.crt.len());
            let slice = &self.crt[reg_start..end_fix];
            println!("{}", slice);
            reg_start += 40;
            reg_end += 40;
        }
    }

    fn sum_signal_strength(&self) -> i32 {
        self.signal_strength.iter().sum()
    }
}

impl CommandImpl for Day10 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut clock: Clock = Clock::new();
        for line in lines {
            let cmd = parse_cmd(&line).unwrap().1;
            clock.exec_cycle(cmd);
        }
        clock.simulate_clock();
        clock.print_crt();
        println!("signal strength sum: {:?}", clock.sum_signal_strength());
        println!("signal strength: {:?}", clock.signal_strength);
        println!("last cycle: {:?}", clock.cycle);
        println!("EX: {:?}", self.input);
        Ok(())
    }
}

fn process_part1(input: &str) -> i32 {
    let commands: String = input.to_string();
    let lines: Vec<&str> = commands.split("\n").collect::<Vec<&str>>();
    let mut clock: Clock = Clock::new();
    for line in lines {
        let cmd = parse_cmd(line).unwrap().1;
        clock.exec_cycle(cmd);
    }
    clock.simulate_clock();
    //clock.print_crt();
    clock.sum_signal_strength()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_works() {
        let expected_value: i32 = 13140;
        assert_eq!(process_part1(INPUT), expected_value);
    }
}

//    #[test]
//    fn part2_works() {
//assert_eq!(
//    process_part2(INPUT),
//"##..##..##..##..##..##..##..##..##..##..
//###...###...###...###...###...###...###.
//####....####....####....####....####....
//#####.....#####.....#####.....#####.....
//######......######......######......####
//#######.......#######.......#######....."
//        );
//    }
