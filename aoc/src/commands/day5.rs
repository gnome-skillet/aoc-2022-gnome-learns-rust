use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use regex::Regex;

#[derive(Parser, Debug)]
pub struct Day5 {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct Port {
    stacks: Vec<Vec<char>>,
    n_stacks: usize,
}

impl Port {
    fn load_crate(&mut self, stack_index: usize, crate_id: char) {
        self.stacks[stack_index].push(crate_id);
    }

    fn insert_crate(&mut self, stack_index: usize, crate_id: char) {
        if self.stacks.is_empty() {
            self.stacks[stack_index].push(crate_id);
        } else {
            self.stacks[stack_index].insert(0, crate_id);
        }
    }

    fn unload_crate(&mut self, stack_index: usize) -> char {
        self.stacks[stack_index].pop().unwrap()
    }

    fn load_crates(&mut self, stack_index: usize, crates: &mut Vec<char>) {
        loop {
            match crates.pop() {
                Some(crate_id) => {
                    self.stacks[stack_index].push(crate_id);
                }
                _ => break,
            }
        }
    }

    fn unload_crates(&mut self, stack_index: usize, n_crates: usize) -> Vec<char> {
        let mut crates = Vec::new();
        for _ in 0..n_crates {
            let item = self.stacks[stack_index].pop().unwrap();
            crates.push(item);
        }
        crates
    }

    fn create_port(n_stacks: usize) -> Port {
        Port { stacks: vec![], n_stacks }
    }

    fn new_port(n_stacks: usize) -> Port {
        let mut port: Port = Port::create_port(n_stacks);
        for _i in 0..port.n_stacks {
            let mut x: Vec<char> = vec![];
            port.stacks.push(x);
        }
        port
    }

    fn tos(&mut self, stack_index: usize) -> Option<&char> {
        match self.stacks[stack_index].len() {
            0 => None,
            n => Some(&self.stacks[stack_index][n - 1]),
        }
    }

    fn top_crates(&mut self) -> Vec<char> {
        let mut tos: Vec<char> = vec![];
        for i in 0..self.n_stacks {
            let top = self.tos(i).unwrap();
            tos.push(*top);
        }
        tos
    }
}

#[derive(Debug)]
pub struct Crane {
    port: Port,
}

impl Crane {
    fn new_crane(n_stacks: usize) -> Crane {
        let mut port: Port = Port::new_port(n_stacks);
        println!("Initialize port {:?}", port);

        Crane { port }
    }

    fn add_to_bottom(&mut self, stack_index: usize, item: char) {
        //println!("Insert {item} into crate({stack_index})");

        self.port.insert_crate(stack_index - 1, item);
    }

    fn move_crate(&mut self, from: usize, to: usize) {
        let x = self.port.unload_crate(from - 1);
        self.port.load_crate(to - 1, x);
    }

    fn move_crates(&mut self, from: usize, to: usize, n_crates: usize) {
        let mut crates: Vec<char> = self.port.unload_crates(from - 1, n_crates);
        println!("move {n_crates} from({from}) to({to})");
        loop {
            if crates.is_empty() {
                break;
            }
            let crate_id: char = crates.pop().unwrap();
            self.port.load_crate(to - 1, crate_id);
        }
    }

    fn top_crates(&mut self) -> String {
        self.port.top_crates().iter().cloned().collect::<String>()
    }
}

#[derive(Debug)]
pub struct CraneSimulator {
    crane: Crane,
}

impl CraneSimulator {
    fn new_crane_simulator(n_stacks: usize) -> CraneSimulator {
        let mut crane: Crane = Crane::new_crane(n_stacks);

        CraneSimulator { crane }
    }

    fn run_simulation(&mut self, commands: Vec<String>) {
        let offset: usize = 4;
        let mut start_commands: usize = 0;

        let re = Regex::new(r"^\[([A-Z])\]\s*$").unwrap();
        let re2 = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

        for (i, cmd) in commands.iter().enumerate() {
            if cmd.is_empty() {
                start_commands = i;
                break;
            }
            for stack_index in 0..self.crane.port.n_stacks {
                let start: usize = stack_index * offset;
                let stop: usize =
                    if (start + offset) < cmd.len() { start + offset } else { cmd.len() };
                let slice = &cmd[start..stop];
                if re.is_match(&slice) {
                    let cap = re.captures(&slice).unwrap();
                    let item: char = cap[1].parse().unwrap();
                    self.crane.add_to_bottom(stack_index + 1, item);
                }
            }
        }

        for i in start_commands..commands.len() {
            if re2.is_match(&commands[i]) {
                let cap = re2.captures(&commands[i]).unwrap();
                let n_items: usize = cap[1].parse().unwrap();
                let from: usize = cap[2].parse().unwrap();
                let to: usize = cap[3].parse().unwrap();
                for _j in 0..n_items {
                    self.crane.move_crate(from, to);
                }
            }
        }
    }

    fn run_second_simulation(&mut self, commands: Vec<String>) {
        let offset: usize = 4;
        let mut start_commands: usize = 0;

        let re = Regex::new(r"^\[([A-Z])\]\s*$").unwrap();
        let re2 = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

        for (i, cmd) in commands.iter().enumerate() {
            if cmd.is_empty() {
                start_commands = i;
                break;
            }
            for stack_index in 0..self.crane.port.n_stacks {
                let start: usize = stack_index * offset;
                let stop: usize =
                    if (start + offset) < cmd.len() { start + offset } else { cmd.len() };
                let slice = &cmd[start..stop];
                if re.is_match(&slice) {
                    let cap = re.captures(&slice).unwrap();
                    let item: char = cap[1].parse().unwrap();
                    self.crane.add_to_bottom(stack_index + 1, item);
                }
            }
        }

        for i in start_commands..commands.len() {
            if re2.is_match(&commands[i]) {
                println!("{}", commands[i]);
                let cap = re2.captures(&commands[i]).unwrap();
                let n_items: usize = cap[1].parse().unwrap();
                let from: usize = cap[2].parse().unwrap();
                let to: usize = cap[3].parse().unwrap();
                self.crane.move_crates(from, to, n_items);
            }
        }
    }
}

impl CommandImpl for Day5 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        let n_crates: usize = lines[0].len() / 4 + 1;
        //let mut crane_simulator: CraneSimulator = CraneSimulator::new_crane_simulator(n_crates);
        let mut crane_simulator_2: CraneSimulator = CraneSimulator::new_crane_simulator(n_crates);
        //crane_simulator.run_simulation(lines);
        crane_simulator_2.run_second_simulation(lines);
        // load items onto crates

        //let mut top1 = crane_simulator.crane.top_crates();
        let mut top2 = crane_simulator_2.crane.top_crates();

        println!("Top of stacks {top2}");
        // SHQWSRBDL
        // SHQWSRBDL
        Ok(())
    }
}
