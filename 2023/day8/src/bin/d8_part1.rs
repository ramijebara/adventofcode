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

    debug!("data: {:?}", data);
    debug!("{:?}", instructions);

    let mut result = 0;
    let mut key = "AAA".to_string();
    let mut i = 0;
    let i_max = instructions.len();

    while key != "ZZZ".to_string() {
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

    info!("result: {}", result);

    Ok(())
}
