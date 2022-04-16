use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut numbers :Vec<i32> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let data = x.split("|").collect::<Vec<&str>>();
                if data.len() == 2 {
                    let signals: Vec<&str> = data[0].split_whitespace().collect::<Vec<&str>>();
                    let display: Vec<&str> = data[1].split_whitespace().collect::<Vec<&str>>();

                    let mapping = process_signal(signals);
                    let mut number :Vec<i32> = Vec::new();
                    for word in display {
                        if let Some(x) = mapping.get(&sort_word(word)) {
                            number.push(*x);
                        }
                    }
                    numbers.push(vec_to_num(number));
                }
            }
        }
        println!("sum: {}", numbers.iter().sum::<i32>());
    }
}

/// Returns an iterator to the reader of the lines of the file
/// The output is wrapped in Result for better error handling
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_signal(signals: Vec<&str>) -> HashMap<String, i32> {
    let mut mapping: HashMap<String, i32> = HashMap::new();
    let mut lights: HashMap<u8, Vec<char>> = HashMap::new();
    let mut leftovers1: Vec<String> = Vec::new();
    let mut leftovers2: Vec<String> = Vec::new();

    for signal in signals {
        let word = sort_word(signal);
        match signal.len() {
            2 => {
                mapping.insert(word, 1);
            }
            3 => {
                let l = &*word;
                lights.insert(7, l.chars().collect());
                mapping.insert(word, 7);
            }
            4 => {
                let l = &*word;
                lights.insert(4, l.chars().collect());
                mapping.insert(word, 4);
            }
            7 => {
                mapping.insert(word, 8);
            }
            _ => {
                leftovers1.push(word);
            }
        }
    }

    for leftover in leftovers1 {
        let mut four_chars = Vec::new();
        let mut seven_chars = Vec::new();

        if let Some(x) = lights.get(&4) {
            four_chars = x.to_vec();
        }
        if let Some(x) = lights.get(&7) {
            seven_chars = x.to_vec();
        }

        if leftover.len() == 6 {
            // find (0,6,9) mapping
            if check_substring(&leftover, four_chars) {
                mapping.insert(leftover, 9);
            } else if check_substring(&leftover, seven_chars) {
                mapping.insert(leftover, 0);
            } else {
                let l = &*leftover;
                lights.insert(6, l.chars().collect());
                mapping.insert(leftover, 6);
            }
        } else {
            leftovers2.push(leftover);
        }
    }

    for leftover in leftovers2 {
        let mut six = String::new();
        let mut seven_chars = Vec::new();

        if let Some(x) = lights.get(&6) {
            six = x.into_iter().collect();
        }
        if let Some(x) = lights.get(&7) {
            seven_chars = x.to_vec();
        }

        let lo_slice = &*leftover;
        let lo_vec = lo_slice.chars().collect();

        //only 5 chars strings left (2,3,5)
        if check_substring(&leftover, seven_chars) {
            mapping.insert(leftover, 3);
        } else if check_substring(&six, lo_vec) {
            mapping.insert(leftover, 5);
        } else {
            mapping.insert(leftover, 2);
        }
    }

    mapping
}

fn check_substring(signal: &String, ss: Vec<char>) -> bool {
    for c in ss {
        if !signal.contains(c) {
            return false;
        }
    }
    true
}

fn sort_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();

    let s = String::from_iter(chars);
    s
}

fn vec_to_num(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }

    let mut number :i32 = 0;

    for i in 0..len {
        let m = i32::pow(10, (len - (i + 1)) as u32);
        number += nums[i]*m;
    }
    number
}
