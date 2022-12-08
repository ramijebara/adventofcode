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

            let player_move = play_selector(turn[0], turn[1]);

            info!("Suggested move: {}", player_move);

            let win_score = win_loss_draw(turn[0], &player_move);
            
            let choice_score = match &player_move.to_owned()[..] {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => {
                    error!("invalid play value detected!");
                    0
                }
            };

            let total_score = win_score + choice_score;
            info!("total score: {}", total_score);
            game_scores.push(total_score);
        }
    }

    let final_score = game_scores.iter().fold(0, |acc, x| acc + x);

    println!("final score: {}", final_score);
}

fn play_selector(opponent: &str, strategy: &str) -> String {
    match strategy {
        "X" => {
            match opponent {
                "A" => { "C".to_string() },
                "B" => { "A".to_string() },
                "C" => { "B".to_string() },
                _ => {
                    error!("Invalid opponent move");
                    "ERROR".to_string()        
                }
            }
        },
        "Y" => {
            match opponent {
                "A" => { "A".to_string() },
                "B" => { "B".to_string() },
                "C" => { "C".to_string() },
                _ => {
                    error!("Invalid opponent move");
                    "ERROR".to_string()        
                }
            }
        },
        "Z" => {
            match opponent {
                "A" => { "B".to_string() },
                "B" => { "C".to_string() },
                "C" => { "A".to_string() },
                _ => {
                    error!("Invalid opponent move");
                    "ERROR".to_string()        
                }
            }
        },
        _ => {
            error!("Invalid strategy");
            "ERROR".to_string()
        }

    }    
}

fn win_loss_draw(opponent: &str, player: &str) -> usize {

    match player {
        "A" => {
            match opponent {
                "C" => 6,
                "A" => 3,
                _ => 0,
            }
        },
        "B" => {
            match opponent {
                "A" => 6,
                "B" => 3,
                _ => 0,
            }
        },
        "C" => {
            match opponent {
                "B" => 6,
                "C" => 3,
                _ => 0,
            }
        },
        _ => 0
    }
}
