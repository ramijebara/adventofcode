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


    for line in data_lines {
        if let Ok(x) = line {
            let data_stream: Vec<char> = x.chars().collect();
            info!("{:?}", data_stream);

            let mut marker = 0;

            loop {
                let mut uniq_four = Vec::new();

                for i in marker..(marker + 4) {
                    if i < data_stream.len() {
                        if uniq_four.contains(&data_stream[i]) {
                            break;
                        }
                        uniq_four.push(data_stream[i]);
                    }                     
                }

                if uniq_four.len() == 4 {
                    println!("Result: {}, sequence: {:?}", marker + 4, uniq_four);
                    break;
                }

                if marker >= data_stream.len() - 4  {
                    break;
                }

                marker += 1;
            }
        }
    }
}
