use std::{io::{self, BufRead}, fs::File, collections::HashMap};
use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut current_dir = Vec::new();
    let mut dir_sizes: HashMap<Vec<String>, usize> = HashMap::new();

    for line in data_lines {
        if let Ok(x) = line {
            if x.starts_with("$ cd") {
                let mut command: Vec<&str> = x.split(" ").collect();

                if let Some(d) = command.pop() {

                    if d == ".." {
                        _ = current_dir.pop();
                    } else {
                        current_dir.push(d.to_string());
                    }
                    info!("dir: {:?}", current_dir);
                    let _ = dir_sizes.entry(current_dir.clone()).or_insert(0);
                }
            } else {
                let line_parts: Vec<&str> = x.split(" ").collect();
                if let Ok(file_size) = line_parts[0].parse::<usize>() {
                    let mut d = current_dir.clone();
                    loop {
                        if let Some(s) = dir_sizes.get_mut(&d) {
                            *s += file_size;
                        }
                        
                        if let None = d.pop() {
                            break;
                        }
                    }
                }
            }
        }
    }

    let mut acc = 0;
    for (_, v) in dir_sizes {
        if v <= 100000 {
            acc += v;
        }
    }

    println!("Result: {}", acc);
}
