use std::{io::{self, BufRead}, fs::File};

fn main() {
    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        println!("Error reading line");
        return;
    };

    let mut calories: Vec<usize> = Vec::new();
    let mut running_sum = 0;
    for line in data_lines {
        if let Ok(x) = line {
            if let Ok(line_value) = x.parse::<usize>() {
                running_sum += line_value;
            } else {
                calories.push(running_sum);
                running_sum = 0;
            }
        }
    }
    println!("max: {}", calories.iter().max().unwrap());
}
