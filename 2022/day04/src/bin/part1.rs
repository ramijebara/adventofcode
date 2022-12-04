use std::{fs::File, io::{self, BufRead}};
use log::{error};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut full_overlaps = 0;

    for line in data_lines {
        if let Ok(x) = line {
            let line_ranges = x.split(",").into_iter().collect::<Vec<&str>>();

            let r1 = get_range_numbers(line_ranges[0]);
            let r2 = get_range_numbers(line_ranges[1]);

            if full_overlap(&r1, &r2) {
                full_overlaps += 1;
            }
        }
    }

    println!("Result: {}", full_overlaps);
}

fn get_range_numbers(input_range: &str) -> (usize, usize) {
    let range: Vec<&str> = input_range.split("-").into_iter().collect();
    return (range[0].parse::<usize>().unwrap(), range[1].parse::<usize>().unwrap());
}

fn full_overlap(r1: &(usize, usize), r2: &(usize, usize)) -> bool {
    if r1.0 >= r2.0 && r1.1 <= r2.1 {
        return true;
    }

    if r2.0 >= r1.0 && r2.1 <= r1.1 {
        return true;
    }

    false
}
