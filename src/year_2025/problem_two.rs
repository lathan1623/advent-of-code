use std::io::Read;

use crate::{Problem, read_file};

pub struct Input;

const PATH : &str = "./inputs/2025/2.txt";

impl Problem for Input {
    fn part_one(&self) -> u64 {
        let mut total = 0;
        if let Ok(file) = read_file(PATH) {
            let bytes = file.bytes();
            let mut switch = false;
            let mut start: u64 = 0;
            let mut end: u64 = 0;
            for byte in bytes {
                let b = byte.expect("Error while reading input.");
                match b {
                    b',' | b'\n' | b'\r' => {
                        total += find_invalids(start, end);
                        start = 0;
                        end = 0;
                        switch = false;
                    },
                    b'-' => switch = true,
                    b'0'..=b'9' => {
                        let digit = (b - b'0') as u64;
                        if switch { 
                            end = end * 10 + digit; 
                        } else {
                            start = start * 10 + digit;
                        }
                    } 
                    _ => println!("Unexpected character")
                }
            }
        }
        total
    }

    fn part_two(&self) -> u64 {
        10
    }
} 

fn find_invalids(start : u64, end : u64) -> u64 {
    let mut total = 0;
    for num in start..end {
        if is_repeat_half(num) {
            total += num;
        }
    }
    total
}

fn is_repeat_half(num: u64) -> bool {
    if num == 0 {
        return false;
    }
    let len = (num.ilog10() + 1) as u32;
    if len % 2 != 0 {
        return false;
    }
    let pow = 10u64.pow(len / 2);
    num / pow == num % pow
}
