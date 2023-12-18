use color_eyre::eyre::Result;
use log::{info, trace};
use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day2/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut result = 0;

    while let Some(Ok(data_line)) = data_lines.next() {
        let game_data_split: Vec<&str> = data_line.split(':').collect();

        if game_data_split.len() > 1 {
            let game: Vec<&str> = game_data_split[0].split(" ").collect();
            let game_number = game[1].parse::<u32>().unwrap();
            let power = calculate_power(game_data_split[1]);
            trace!("game: {}, power: {}", game_number, power);
            result += power;
        }
    }

    info!("result: {}", result);
    Ok(())
}

fn calculate_power(game_string: &str) -> u32 {
    let mut red = Vec::new();
    let mut green = Vec::new();
    let mut blue = Vec::new();

    let set_list: Vec<&str> = game_string.split(';').collect();

    for s in set_list {
        let cubes: Vec<&str> = s.split(',').collect();

        for c in cubes {
            let mut cube_number = c.trim().splitn(2, ' ');
            let v = cube_number.next().unwrap().parse::<u32>().unwrap();
            let k = cube_number.next().unwrap();

            match k {
                "red" => {
                    red.push(v);
                }
                "green" => {
                    green.push(v);
                }
                "blue" => {
                    blue.push(v);
                }
                _ => {
                    trace!("Unexpected input");
                }
            }
        }
    }

    trace!("red   {:?}", red);
    trace!("green {:?}", green);
    trace!("blue  {:?}", blue);
    let red_min = if let Some(r) = red.into_iter().max() {
        r
    } else {
        1
    };
    let green_min = if let Some(g) = green.into_iter().max() {
        g
    } else {
        1
    };
    let blue_min = if let Some(b) = blue.into_iter().max() {
        b
    } else {
        1
    };

    red_min * green_min * blue_min
}
