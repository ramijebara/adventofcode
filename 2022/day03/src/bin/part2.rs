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
    let mut count = 1;
    let mut group_data: Vec<HashSet<char>> = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            info!("Line count: {}", count);

            let items = HashSet::from_iter(x.chars().into_iter());
            group_data.push(items);

            if count % 3 == 0 {
                let set1 = group_data[0].clone();
                let set2 = group_data[1].clone();
                let set3 = group_data[2].clone();

                let common_value1: Vec<&char> = set1.intersection(&set2).into_iter().collect();
                let common_value2: Vec<&char> = set2.intersection(&set3).into_iter().collect();
            
                let hs1: HashSet<&char> = HashSet::from_iter(common_value1.into_iter());
                let hs2: HashSet<&char> = HashSet::from_iter(common_value2.into_iter());

                let common_item: HashSet<_> = hs1.intersection(&hs2).collect();
                
                for x in common_item.into_iter() {
                    common_items.push(**x);
                }          

                group_data = Vec::new();
            }

            count += 1;
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