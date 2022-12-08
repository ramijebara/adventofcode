use std::{io::{self, BufRead}, fs::File, collections::HashMap};
use log::{error, info};

fn main() {

    const DISK_SIZE: usize = 70_000_000;
    const UPDATE_SIZE: usize = 30_000_000;

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

    let root_dir = vec!["/".to_string()];

    if let Some(current_size) = dir_sizes.get(&root_dir) {
        let free_space = DISK_SIZE - current_size;
        let needed = UPDATE_SIZE - free_space;
        let mut compatible: Vec<usize> = Vec::new();

        for (_, v) in dir_sizes {
            if v >= needed {
                compatible.push(v);
            }
        } 

        info!("free space: {}, needed: {}", free_space, needed);

        println!("Result: {:?}", compatible.iter().min());
    }

}
