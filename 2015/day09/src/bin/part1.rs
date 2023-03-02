use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use anyhow::Result;
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    let file = File::open("src/bin/sample.txt")?;
    let mut data_lines = io::BufReader::new(file).lines();

    let mut cities: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    while let Some(Ok(line)) = data_lines.next() {
        info!("{}", line);
        let mut line_part: Vec<&str> = line.split("=").collect();
        let Some(right) = line_part.pop() else {continue;};
        let Some(left) = line_part.pop() else {continue; };

        let mut city: Vec<String> = left.split("to").map(|c| c.trim().to_string()).collect();
        let distance = right.trim().parse::<u32>()?;

        city.sort();

        let Some(city_0) = city.pop() else { info!("Could not pop city 0"); continue;};
        let Some(city_1) = city.pop() else { info!("Could not pop city 1"); continue;};

        distances.insert((city_0.clone(), city_1.clone()), distance);
        cities.insert(city_0);
        cities.insert(city_1);
    }

    info!("\n{:#?}\n{:#?}\n", cities, distances);

    for city in cities {
        println!("{}", city);
    }

    Ok(())
}

