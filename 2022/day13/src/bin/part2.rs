use std::{fs::File, io::{self, BufRead}};
use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/sample.txt") {
       io::BufReader::new(file).lines() 
    } else {
        error!("Error reading data");
        return;
    };

    for data_line in data_lines {
        if let Ok(line) = data_line {
            info!("{}", line);
        }
    }

    error!("Sample Error")

}
