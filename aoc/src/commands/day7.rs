use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

use regex::Regex;

use std::collections::HashMap;

#[derive(Parser, Debug)]
pub struct Day7 {
    #[clap(long, short)]
    input: PathBuf,
}

//fn tos(vec: Vec<String>) -> Option<String> {
//    match vec.len() {
//        0 => None,
//        n => Some(vec[n - 1]),
//    }
//}

impl CommandImpl for Day7 {
    fn main(&self) -> Result<(), DynError> {
        let pop_stack = Regex::new(r"^\$ cd \.\.$").unwrap();
        let change_directory = Regex::new(r"^\$ cd (.*)$").unwrap();

        let list_contents = Regex::new(r"^\$ ls$").unwrap();
        let directory = Regex::new(r"^dir (.*)$").unwrap();
        let file = Regex::new(r"^(\d+) (.*)$").unwrap();
        let lines: Vec<String> = slurp_file(&self.input)?;
        let mut op_stack: Vec<String> = vec![];
        let mut directory_stack: Vec<String> = vec![];
        let mut directory_memory: HashMap<String, u32> = HashMap::new();
        let mut smallest_big_directory: Vec<u32> = vec![];

        let mut accumulator: u32 = 0;

        for line in lines {
            if pop_stack.is_match(&line) {
                let full_path: String = directory_stack.iter().map(|x| x.to_string() + "/").collect::<String>();
                let relative_dir: String = directory_stack.pop().unwrap();
                //println!("popped directory {:?}", curr_dir);
                loop {
                    let token = op_stack.pop().unwrap();
                    if token == "(" {
                        println!("dir {} memory is {}", full_path, accumulator);
                        directory_memory.insert(full_path, accumulator);
                        op_stack.push(accumulator.to_string());
                        accumulator = 0;
                        break;
                    } else {
                        let value = token.parse::<u32>().unwrap();
                        accumulator += value;
                    }
                }
            } else if change_directory.is_match(&line) {
                let cap = change_directory.captures(&line).unwrap();
                let item: String = cap[1].parse().unwrap();
                directory_stack.push(item.to_string());
                //println!("push directory {}", item);
                op_stack.push("(".to_string());
            } else if list_contents.is_match(&line) {
                //println!("list contents");
            } else if directory.is_match(&line) {
                //println!("skip directory");
            } else if file.is_match(&line) {
                let cap = file.captures(&line).unwrap();
                let item: String = cap[1].parse().unwrap();
                //println!("push file {}", item);
                op_stack.push(item.to_string());
            } else {
                //println!("unhandled condition {accumulator}");
            }
        }
        accumulator = 0;
        //println!("directory_stack: {:?}", directory_stack);
        while !directory_stack.is_empty() {
            let full_path: String = directory_stack.concat();
            let curr_dir: String = directory_stack.pop().unwrap();
            loop {
                let token = op_stack.pop().unwrap();
                if token == "(" {
                    //println!("dir {} memory is {}", full_path, accumulator);
                    directory_memory.insert(full_path, accumulator);
                    if !directory_stack.is_empty() {
                        op_stack.push(accumulator.to_string());
                    }
                    accumulator = 0;
                    break;
                } else {
                    let value = token.parse::<u32>().unwrap();
                    accumulator += value;
                }
            }
        }

        accumulator = 0;
        let total_disk_space = 70000000u32;
        let needed_space = 30000000u32;
        let mut used_space = directory_memory.get("/").unwrap(); 
        let mut unused_space = total_disk_space - used_space; 
        let mut space_still_needed = needed_space - unused_space;
        let mut best_answer = used_space;
        println!("still need: {space_still_needed} memory");
        println!("best answer: {best_answer} memory");
        for (key, val) in directory_memory.iter() {
            //println!("directory: {key} has memory {val}");
            if val <= &100000u32 {
                accumulator += val;
            }
            if (val < &best_answer) && (*val > space_still_needed) {
                best_answer = val;
                println!("best answer: {best_answer} memory");
            }
        }

        //285116 is too low
        println!("total part a {accumulator}");
        println!("smallest big directory {best_answer}");
        println!("selected directories: {accumulator}");
        Ok(())
    }
}
