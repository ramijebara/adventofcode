use std::{collections::HashSet, fs::File, io::{self, BufRead}};
use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut common_items = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let midpoint = x.len()/2;
            let (split1, split2) = x.split_at(midpoint);
            
            let set1: HashSet<char> = HashSet::from_iter(split1.chars().into_iter());
            let set2: HashSet<char> = HashSet::from_iter(split2.chars().into_iter());
            let common_item: HashSet<_> = set1.intersection(&set2).collect();

            for x in common_item.into_iter() {
                common_items.push(*x);
            }
        } 
    }

    info!("common items: {:?}", common_items);

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut values = Vec::new();

    for item in common_items {
        let mut position_of = alphabet.iter().position(|x| x == &item).unwrap();
        position_of += 1; // add 1 since array index starts at 0
        values.push(position_of);
    }

    info!("values: {:?}", values);

    println!("Result: {}", values.iter().fold(0, |acc, x| acc + x));
}