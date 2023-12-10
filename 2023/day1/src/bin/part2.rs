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
        let parsed_line = string_to_num(data_line);
        let data_chars: Vec<char> = parsed_line.chars().collect::<Vec<char>>();
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

fn string_to_num(data_line: String) -> String {
    let l = data_line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    l
}
