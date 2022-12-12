use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

use crate::utils::slurp_file;

#[derive(Parser, Debug)]
pub struct Day6 {
    #[clap(long, short)]
    input: PathBuf,
}

pub fn get_start_of_packet(msg: String, offset: usize) -> usize {
    let mut start_of_packet: usize = 0;
    let mut stop_search: usize = msg.len() - offset;
    for start in 0..stop_search {
        let mut stop = start + offset;
        let slice = &msg[start..stop];
        let x = String::from(slice).to_owned();
        //let mut chars: Vec<char> = x.to_vec().sort().dedup();
        let mut chars: Vec<u8> = x.into_bytes();
        chars.sort();
        chars.dedup();
        let nchars = chars.len() as usize;
        if nchars == offset {
            start_of_packet = stop;
            break;
        }
    }
    start_of_packet
}

impl CommandImpl for Day6 {
    fn main(&self) -> Result<(), DynError> {
        let lines: Vec<String> = slurp_file(&self.input)?;
        for line in lines {
            //let packet_start: usize = get_start_of_packet(line, 4);
            //println!("start of packet {packet_start}");
            let message_start: usize = get_start_of_packet(line, 14);
            println!("start of message {message_start}");
        }
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
