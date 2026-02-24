use std::io::BufRead;

use crate::{Problem, read_file};

pub struct Input;

const PATH: &str = "./inputs/1.txt";

impl Problem for Input {
    fn part_one(&self) -> u64 {
        let mut count: u64 = 0;
        let mut amount = 50;
        if let Ok(file) = read_file(PATH) {
            for line in file.lines().map_while(Result::ok) {
                let bytes: Vec<u8> = line.bytes().collect();
                let direction = bytes[0];

                let mut i = 1;
                let mut spin = 0;
                while let Some(ch) = bytes.get(i) {
                    let curr_amount = (ch - b'0') as i32;
                    spin = spin * 10 + curr_amount;
                    i += 1
                }

                if spin > 100 {
                    count += (spin / 100) as u64;
                    spin %= 100;
                }

                println!("{}", spin);

                let dir = direction as char;
                println!("{}", dir);

                if direction == b'L' {
                    if spin > amount && amount != 0  {
                        count += 1
                    }
                    amount = (amount - spin + 100) % 100;
                    println!("{}", amount);
                } else {
                    let temp = amount + spin;
                    if temp > 100 {
                        count += 1
                    }
                    amount = temp % 100;
                    println!("{}", amount);
                }

                println!("break");

                if amount == 0 {
                    count += 1
                }
            }
        }
        count
    }

    fn part_two(&self) -> u64 {
        10
    }
} 
