use std::{io::{self, BufRead}, fs::File};

fn main() {
    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        println!("Error reading data");
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
    calories.sort();

    let mut top3 = Vec::new();

    for _ in 0..3 {
        match calories.pop() {
            Some(x) => top3.push(x),
            None => break
        }
    }
    println!("top3: {:?}", top3.iter().fold(0, |acc, x| acc + x));
}
