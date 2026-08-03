use crate::{Problem, read_file};
use std::io::BufRead;

pub struct Input;

const PATH: &str = "./inputs/3.txt";

impl Problem for Input {
    fn part_one(&self) -> u64 {
        let mut sum: u64 = 0;
        if let Ok(file) = read_file(PATH) {
            for line in file.lines().map_while(Result::ok) {
                let slice = line.as_bytes();
                let mut first: u64 = 0;
                let mut second: u64 = 0;

                for i in 0..slice.len() {
                    let curr = u64::from(slice[i] - b'0');
                    if curr > first && i != slice.len() - 1 {
                        first = curr;
                        second = 0;
                        continue;
                    }
                    if curr > second {
                        second = curr;
                    }
                }
                sum += (first * 10) + second;
            }
        }
        sum
    }

    fn part_two(&self) -> u64 {
        let mut sum = 0u64;
        if let Ok(file) = read_file(PATH) {
            for line in file.lines().map_while(Result::ok) {
                let line = line.as_bytes();
                let mut curr_sum = 0u64;
                let mut current_index = 0usize;
                
                for remaining in (0..12).rev() {
                    let mut largest_digit = 0u8;
                    for index in current_index..line.len() - remaining {
                        if line[index] > largest_digit {
                            largest_digit = line[index];
                            current_index = index + 1;
                        }
                    }
                    curr_sum = curr_sum * 10 + u64::from(largest_digit - b'0');
                }
                sum += curr_sum;
            }
        }
        sum
    }
}
