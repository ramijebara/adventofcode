use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader, usize};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day8/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut instructions = Vec::new();
    let mut data: HashMap<String, [String; 2]> = HashMap::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("input: {}", data_line);
        if data_line.contains('=') {
            let mut kv_split = data_line.splitn(2, '=');
            let (key, value_str) = (
                kv_split.next().unwrap().trim().to_string(),
                kv_split
                    .next()
                    .unwrap()
                    .trim()
                    .replace("(", "")
                    .replace(")", "")
                    .to_string(),
            );
            let mut value_split = value_str.splitn(2, ',');
            let (left, right) = (
                value_split.next().unwrap().trim().to_string(),
                value_split.next().unwrap().trim().to_string(),
            );

            trace!("key: {}, val: [{}, {}]", key, left, right);

            data.insert(key, [left, right]);
        } else if !data_line.is_empty() {
            let directions_string = data_line.replace("R", "1").replace("L", "0");
            instructions = directions_string
                .chars()
                .into_iter()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
        }
    }

    trace!("data: {:?}", data);
    trace!("{:?}", instructions);

    let mut starting_points: Vec<String> = Vec::new();

    data.keys().cloned().for_each(|k| {
        if k.ends_with("A") {
            starting_points.push(k.to_string())
        }
    });

    trace!("starting_points: {:?}", starting_points);
    debug!("number of starting points: {}", starting_points.len());

    let mut results = Vec::new();

    starting_points.iter().for_each(|s| {
        results.push(walk_path(s, &data, &instructions));
        debug!("{:?}", results);
    });

    info!("path results: {:?}", results);

    let mut prev_i = 0;

    for i in 1..results.len() {
        results[i] = lcm(results[prev_i], results[i]);
        prev_i = i;
    }

    info!("final result: {}", results.last().unwrap());

    Ok(())
}

fn walk_path(start: &String, data: &HashMap<String, [String; 2]>, instructions: &Vec<usize>) -> usize {
    let mut result = 0;
    let mut key = start.to_string();
    let mut i = 0;
    let i_max = instructions.len();

    while !key.ends_with("Z") {
        let v = data.get(&key).unwrap();
        let step = instructions[i];
        key = v[step].clone();
        result += 1;

        i += 1;
        if i == i_max {
            trace!("ran out of instructions, repeating");
            i = 0;
            continue;
        }
    }

    result
}

// find Greatest Common Divisor (GCD) then Find 
// Least Common Multiple using Euclidean method
// ref: https://rustp.org/number-theory/lcm/

// Find GCD
fn gcd(mut a:usize, mut b:usize) -> usize{
    if a==b { return a; }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b>0 {
        let temp = a;
        a = b;
        b = temp%b;
    }
    return a;
}

fn lcm(a:usize, b:usize) -> usize{
    // LCM = a*b / gcd
    return a * (b/gcd(a,b));
}

