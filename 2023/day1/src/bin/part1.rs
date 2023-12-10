use color_eyre::eyre::Result;
use log::{info, trace};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let mut data = Vec::new();
    let file = File::open("./src/bin/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();

    while let Some(Ok(data_line)) = data_lines.next() {
        let data_chars: Vec<char> = data_line.chars().collect::<Vec<char>>();
        let mut nums = Vec::new();

        for c in data_chars {
            if let Some(num) = c.to_digit(10) {
                nums.push(num);
            }
        }

        if nums.len() > 0 {
            let first = nums.iter().next().unwrap();
            let last = nums.iter().last().unwrap();
            let line_value = (first * 10) + last;
            trace!("{}", line_value);
            data.push(line_value);
        }
    }

    info!("Result: {}", data.iter().sum::<u32>());

    Ok(())
}
