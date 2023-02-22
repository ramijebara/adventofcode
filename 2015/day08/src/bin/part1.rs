use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead},
};

use log::info;

fn main() -> Result<()> {
    env_logger::init();

    let file = File::open("src/bin/data.txt")?;
    let mut data_lines = io::BufReader::new(file).lines();

    let mut original_length = 0;
    let mut memory_length = 0;
    let mut line_number = 1;

    while let Some(Ok(mut line)) = data_lines.next() {
        if line.len() == 0 {
            continue;
        }
        info!("{} - Intial line: {}", line_number, line);

        original_length += line.chars().count();

        // clean begining and end
        if let Some(c) = line.chars().next() {
            if c == '"' {
                line.remove(0);
            }
        }
        if let Some(c) = line.chars().last() {
            if c == '"' {
                line.pop();
            }
        }

        // adjustments
        line = line.replace("\\\"", "\"");

        let encoded_ascii: Vec<_> = line.match_indices("\\x").collect();
        let mut replacements: Vec<(String, String)> = Vec::new();

        encoded_ascii.iter().for_each(|x| {
            if !is_special_case(&line, x.0) {
                let ea = &line[x.0..(x.0 + 4)];
                if let Ok(c) = u8::from_str_radix(&ea[2..], 16) {
                    replacements.push((ea.to_string(), (c as char).to_string()))
                }
            }
        });

        for r in replacements {
            line = line.replace(&r.0, &r.1);
        }

        line = line.replace("\\\\", "\\");

        info!("{} - Final line : {}", line_number, line);

        memory_length += line.chars().count();
        line_number += 1;
    }

    println!("Original length: {}", original_length);
    println!("Memory length: {}", memory_length);
    println!("Result: {}", original_length - memory_length);

    Ok(())
}

fn is_special_case(line: &String, index: usize) -> bool {
    let mut line_to_index = line[0..index].to_string();
    let mut slashes = 1;

    while let Some(c) = line_to_index.pop() {
        if c == '\\' {
            slashes += 1;
        } else {
            break;
        }
    }

    if slashes % 2 == 0 {
        return true;
    }

    false
}
