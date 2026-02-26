use std::io::Read;

use crate::{Problem, read_file};

pub struct Input;

const PATH: &str = "./inputs/2025/2.txt";

enum State {
    Start,
    End,
}

impl Problem for Input {
    fn part_one(&self) -> u64 {
        let mut total = 0;
        if let Ok(file) = read_file(PATH) {
            let bytes = file.bytes();
            let mut state = State::Start;
            let mut start: u64 = 0;
            let mut end: u64 = 0;
            for byte in bytes {
                let b = byte.expect("Error while reading input.");
                match b {
                    b',' | b'\n' | b'\r' => {
                        total += find_invalids_half(start, end);
                        start = 0;
                        end = 0;
                        state = State::Start
                    }
                    b'-' => state = State::End,
                    b'0'..=b'9' => {
                        let digit = (b - b'0') as u64;
                        match state {
                            State::Start => start = start * 10 + digit,
                            State::End => end = end * 10 + digit,
                        }
                    }
                    _ => println!("Unexpected character"),
                }
            }
        }
        total
    }

    fn part_two(&self) -> u64 {
        let mut total = 0;
        if let Ok(file) = read_file(PATH) {
            let bytes = file.bytes();
            let mut state = State::Start;
            let mut start: u64 = 0;
            let mut end: u64 = 0;
            for byte in bytes {
                let b = byte.expect("Error while reading input.");
                match b {
                    b',' | b'\n' | b'\r' => {
                        total += find_invalids(start, end);
                        start = 0;
                        end = 0;
                        state = State::Start
                    }
                    b'-' => state = State::End,
                    b'0'..=b'9' => {
                        let digit = (b - b'0') as u64;
                        match state {
                            State::Start => start = start * 10 + digit,
                            State::End => end = end * 10 + digit,
                        }
                    }
                    _ => println!("Unexpected character"),
                }
            }
        }
        total
    }
}

fn find_invalids_half(start: u64, end: u64) -> u64 {
    let mut total = 0;
    for num in start..end {
        if is_repeat_half(num) {
            total += num
        }
    }
    total
}

fn find_invalids(start: u64, end: u64) -> u64 {
    let mut total = 0;
    for num in start..end {
        if is_repeat(num) {
            total += num
        }
    }
    total
}

fn is_repeat(num: u64) -> bool {
    if num == 0 {
        return false;
    }

    let len = num.ilog10() + 1;
    for block in 1..=len / 2 {
        if len % block != 0 {
            continue;
        }
        let pow = 10u64.pow(block);
        let repeating = num % pow;
        let mut valid = true;
        let mut partial_num = num / pow;

        while partial_num > 0 {
            if partial_num % pow != repeating {
                valid = false;
                break;
            }
            partial_num /= pow;
        }

        if valid {
            return true;
        }
    }
    false
}

fn is_repeat_half(num: u64) -> bool {
    if num == 0 {
        return false;
    }
    let len = num.ilog10() + 1;
    if len % 2 != 0 {
        return false;
    }
    let pow = 10u64.pow(len / 2);
    num / pow == num % pow
}
