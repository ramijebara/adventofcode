use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};
use rayon::prelude::*;
use std::sync::mpsc::{self, Sender, Receiver};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day5/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
    let mut seed_list: Vec<usize> = Vec::new();
    let mut lowest_locations: Vec<usize> = Vec::new();
    let mut cur_key = String::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        if data_line.contains(':') {
            let line = data_line.split(':').collect::<Vec<&str>>();
            trace!("line: {:?}, length: {}", line, line.len());
            cur_key = String::from(*line.iter().next().unwrap());

            if cur_key.contains("seeds") {
                seed_list = line[1]
                    .trim()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                continue;
            } else {
                data.insert(cur_key.clone(), Vec::new());
                trace!("new key: {}", cur_key);
                continue;
            }
        } else {
            let num_line = data_line.split(' ').collect::<Vec<&str>>();

            if num_line.len() == 3 {
                trace!("numbers: {:?}", num_line);

                let nums = num_line
                    .iter()
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                if let Some(v) = data.get_mut(&cur_key) {
                    v.push((nums[0], nums[1], nums[2]));
                }
            }
        }
    }

    data.iter().for_each(|(k, v)| {
        trace!("{}: {:?}", k, v);
    });

    let seed_ranges = build_seed_ranges(seed_list);

    for seed_range in seed_ranges {
        debug!("seed range: {:?}", seed_range);
        let mut soil_list: Vec<usize> = Vec::new();
        let mut fertilizer_list: Vec<usize> = Vec::new();
        let mut water_list: Vec<usize> = Vec::new();
        let mut light_list: Vec<usize> = Vec::new();
        let mut temperature_list: Vec<usize> = Vec::new();
        let mut humidity_list: Vec<usize> = Vec::new();
        let mut location_list: Vec<usize> = Vec::new();

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

        if let Some(seed_to_soil) = data.get("seed-to-soil map") {
            (seed_range.0..seed_range.1).into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, seed_to_soil) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });

            soil_list = rx.into_iter().collect();
        }

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

        if let Some(soil_to_fertilizer) = data.get("soil-to-fertilizer map") {
            soil_list.into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, soil_to_fertilizer) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });

            fertilizer_list = rx.into_iter().collect();
        }

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();

        if let Some(fertilizer_to_water) = data.get("fertilizer-to-water map") {
            fertilizer_list.into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, fertilizer_to_water) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });

            water_list = rx.into_iter().collect();
        }

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
        if let Some(water_to_light) = data.get("water-to-light map") {
            water_list.into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, water_to_light) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });
            
            light_list = rx.into_iter().collect();
        }

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
        if let Some(light_to_temperature) = data.get("light-to-temperature map") {
            light_list.into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, light_to_temperature) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });

            temperature_list = rx.into_iter().collect();
        }

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
        if let Some(temperature_to_humidity) = data.get("temperature-to-humidity map") {
            temperature_list.into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, temperature_to_humidity) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });

            humidity_list = rx.into_iter().collect();
        }

        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
        if let Some(humidity_to_location) = data.get("humidity-to-location map") {
            humidity_list.into_par_iter().for_each_with(tx, |tx, s| {
                if let Some(value) = get_mapping(s, humidity_to_location) {
                    tx.send(value).unwrap();
                } else {
                    tx.send(s).unwrap();
                };
            });

            location_list = rx.into_iter().collect();
        }

        lowest_locations.push(*location_list.par_iter().min().unwrap());
        debug!("lowest locations: {:?}", lowest_locations);
    }

    info!(
        "lowest location result: {}",
        lowest_locations.iter().min().unwrap()
    );

    Ok(())
}

fn get_mapping(source_value: usize, mapping: &Vec<(usize, usize, usize)>) -> Option<usize> {
    for (destination, source, length) in mapping {
        let range_start = source;
        let range_end = source + length - 1;
        trace!("range: {}, {}", range_start, range_end);
        if source_value >= *range_start && source_value <= range_end {
            let step = source_value - range_start;
            let mapped_value = destination + step;
            trace!("{} maps to {}", source_value, mapped_value);
            return Some(mapped_value);
        }
    }

    None
}

fn build_seed_ranges(seed_list: Vec<usize>) -> Vec<(usize, usize)> {
    let mut seed_map: Vec<(usize, usize)> = Vec::new();

    let evens = seed_list
        .iter()
        .enumerate()
        .filter(|(i, _x)| (i % 2) == 0)
        .collect::<Vec<_>>();

    let odds = seed_list
        .iter()
        .enumerate()
        .filter(|(i, _x)| (i % 2) != 0)
        .collect::<Vec<_>>();

    evens
        .iter()
        .zip(odds.iter())
        .for_each(|(e, o)| seed_map.push((*e.1, *e.1 + *o.1)));

    seed_map
}
