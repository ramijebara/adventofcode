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
            trace!("game number: {}", game_number);
            if is_game_possible(game_data_split[1]) {
                result += game_number;
            }
        }
    }

    info!("result: {}", result);
    Ok(())
}

fn is_game_possible(game_string: &str) -> bool {
    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let set_list: Vec<&str> = game_string.split(';').collect();

    for s in set_list {
        let cubes: Vec<&str> = s.split(',').collect();

        for c in cubes {
            let mut cube_number = c.trim().splitn(2, ' ');
            let v = cube_number.next().unwrap().parse::<u32>().unwrap();
            let k = cube_number.next().unwrap();

            match k {
                "red" => {
                    if v > red_limit {
                        return false;
                    }
                }
                "green" => {
                    if v > green_limit {
                        return false;
                    }
                }
                "blue" => {
                    if v > blue_limit {
                        return false;
                    }
                }
                _ => {
                    trace!("Unexpected input");
                }
            }
        }
    }
    true
}
