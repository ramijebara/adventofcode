use log::{error, info};
use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut packets: Vec<(String, String)> = Vec::new();
    let mut packet: (String, String) = (String::new(), String::new());
    let mut marker = 0;

    for data_line in data_lines {
        if let Ok(line) = data_line {
            match marker {
                0 => {
                    packet.0 = line;
                    marker += 1;
                }
                1 => {
                    packet.1 = line;
                    marker += 1;
                }
                _ => {
                    marker = 0;
                    packets.push(packet.clone());
                }
            }
        }
    }

    // push the last packet
    packets.push(packet.clone());

    for p in packets {
        info!("Left: {}, Right: {}", p.0, p.1);

        let left = p.0.chars().collect::<Vec<_>>();
        let right = p.1.chars().collect::<Vec<_>>();
    }
}
