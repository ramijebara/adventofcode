use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day6/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut time: Vec<usize> = Vec::new();
    let mut distance: Vec<usize> = Vec::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("{}", data_line);
        if data_line.contains("Time:") {
            time = data_line
                .replace("Time:", "")
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        }

        if data_line.contains("Distance:") {
            distance = data_line
                .replace("Distance:", "")
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        }
    }

    debug!("time: {:?}", time);
    debug!("distance: {:?}", distance);

    let mut result = 1;

    time.iter().enumerate().for_each(|(i, x)| {
        let distance_record = distance[i];
        let game_time = x;
        let mut game_result = 0;

        for t in 0..x + 1 {
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
        result *= game_result;
    });

    info!("result: {}", result);
    Ok(())
}
