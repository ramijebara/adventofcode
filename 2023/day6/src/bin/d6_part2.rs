use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day6/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut game_time = 0;
    let mut distance_record = 0;

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("{}", data_line);
        if data_line.contains("Time:") {
            game_time = data_line
                .replace(" ", "")
                .splitn(2, ":")
                .collect::<Vec<&str>>()
                .iter()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }

        if data_line.contains("Distance:") {
            distance_record = data_line
                .replace(" ", "")
                .splitn(2, ":")
                .collect::<Vec<&str>>()
                .iter()
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
        }
    }

    debug!("time: {}", game_time);
    debug!("distance: {}", distance_record);

    let mut game_result = 0;

    for t in 0..game_time + 1 {
        let time_remaining = game_time - t;
        let distance = time_remaining * t;
        if distance > distance_record {
            trace!(
                "distance: {} is greater than record {}",
                distance,
                distance_record
            );
            game_result += 1;
        }
    }
    debug!("game result: {}", game_result);
    info!("result: {}", game_result);

    Ok(())
}
