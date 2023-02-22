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
    let mut encoded_length = 0;
    let mut line_number = 1;

    while let Some(Ok(mut line)) = data_lines.next() {
        if line.len() == 0 {
            continue;
        }
        info!("{} - Intial line: {}", line_number, line);

        original_length += line.chars().count();

        // adjustments
        line = line.replace("\\", "\\\\");
        line = line.replace("\"", "\\\"");
        line = format!("\"{}\"", line);

        info!("{} - Final line : {}", line_number, line);

        encoded_length += line.chars().count();
        line_number += 1;
    }

    println!("Original length: {}", original_length);
    println!("Encoded length: {}", encoded_length);
    println!("Result: {}", encoded_length - original_length);

    Ok(())
}

