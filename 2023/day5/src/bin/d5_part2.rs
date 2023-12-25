use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day5/sample_1.txt")?;
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

        if let Some(seed_to_soil) = data.get("seed-to-soil map") {
            (seed_range.0..seed_range.1).into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, seed_to_soil) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                soil_list.push(mapped_value)
            });
        }

        if let Some(soil_to_fertilizer) = data.get("soil-to-fertilizer map") {
            soil_list.into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, soil_to_fertilizer) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                fertilizer_list.push(mapped_value)
            });
        }

        if let Some(fertilizer_to_water) = data.get("fertilizer-to-water map") {
            fertilizer_list.into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, fertilizer_to_water) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                water_list.push(mapped_value)
            });
        }

        if let Some(water_to_light) = data.get("water-to-light map") {
            water_list.into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, water_to_light) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                light_list.push(mapped_value)
            });
        }

        if let Some(light_to_temperature) = data.get("light-to-temperature map") {
            light_list.into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, light_to_temperature) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                temperature_list.push(mapped_value)
            });
        }

        if let Some(temperature_to_humidity) = data.get("temperature-to-humidity map") {
            temperature_list.into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, temperature_to_humidity) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                humidity_list.push(mapped_value)
            });
        }

        if let Some(humidity_to_location) = data.get("humidity-to-location map") {
            humidity_list.into_iter().for_each(|s| {
                let mapped_value = if let Some(value) = get_mapping(s, humidity_to_location) {
                    value
                } else {
                    s
                };

                trace!("{} maps to {}", s, mapped_value);
                location_list.push(mapped_value)
            });
        }

        lowest_locations.push(*location_list.iter().min().unwrap());
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
