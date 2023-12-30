use color_eyre::eyre::Result;
use log::{info, trace};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize,
};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day9/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data = Vec::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("input:  {}", data_line);
        let readings = data_line
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        trace!("parsed: {:?}", readings);
        data.push(readings);
    }

    let mut result = 0;

    data.iter().for_each(|l| {
        result += calculate_prediction(l);
        trace!("line result: {}", result);
    });

    info!("result: {}", result);

    Ok(())
}

fn calculate_prediction(line: &Vec<isize>) -> isize {
    let mut analysis = Vec::new();
    analysis.push(line.clone());
    let mut i = 0;

    while !all_zeros(&analysis[i]) {
        analysis.push(process_line(&analysis[i]));
        i += 1;
    }

    trace!("pre analysis: {:?}", analysis);

    // add 0 to last line
    analysis.last_mut().unwrap().push(0);

    analysis.reverse();

    for i in 1..analysis.len() {
        analysis[i].reverse();
        let prediction = analysis[i].last().unwrap() - analysis[i - 1].last().unwrap();
        analysis[i].push(prediction);
    }

    trace!("post analysis: {:?}", analysis);

    *analysis.last().unwrap().last().unwrap()
}

fn all_zeros(line: &Vec<isize>) -> bool {
    for i in 0..line.len() {
        if line[i] != 0 {
            return false;
        }
    }

    true
}

fn process_line(line: &Vec<isize>) -> Vec<isize> {
    let mut analysis: Vec<isize> = Vec::new();
    let mut prev_i = 0;

    for i in 1..line.len() {
        let diff = line[i] - line[prev_i];
        analysis.push(diff);
        prev_i = i;
    }

    analysis
}
