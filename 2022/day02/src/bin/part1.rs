use std::{io::{self, BufRead}, fs::File};
use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut game_scores: Vec<usize> = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let turn = x.split(" ").collect::<Vec<&str>>();
            info!("turn raw data {:?}", turn);

            let win_score = win_loss_draw(turn[0], turn[1]);
            let choice_score = match turn[1] {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => {
                    error!("invalid play value detected!");
                    0
                }
            };
            game_scores.push(win_score + choice_score);
        }
    }

    let final_score = game_scores.iter().fold(0, |acc, x| acc + x);

    println!("final score: {}", final_score);
}

fn win_loss_draw(opponent: &str, player: &str) -> usize {

    match player {
        "X" => {
            match opponent {
                "C" => 6,
                "A" => 3,
                _ => 0,
            }
        },
        "Y" => {
            match opponent {
                "A" => 6,
                "B" => 3,
                _ => 0,
            }
        },
        "Z" => {
            match opponent {
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        },
        _ => 0
    }
}