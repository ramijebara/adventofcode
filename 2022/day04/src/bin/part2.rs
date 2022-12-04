use std::{fs::File, io::{self, BufRead}, collections::HashSet};
use log::{error, info};

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

            if partial_overlap(&r1, &r2) {
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

fn partial_overlap(r1: &(usize, usize), r2: &(usize, usize)) -> bool {
    let set1: HashSet<usize> = HashSet::from_iter(r1.0..(r1.1 + 1));
    let set2: HashSet<usize> = HashSet::from_iter(r2.0..(r2.1 + 1));

    info!("Set1: {:?}", set1);
    info!("Set2: {:?}", set2);

    if set1.intersection(&set2).into_iter().collect::<Vec<_>>().len() > 0 {
        return true;
    }

    false
}
