use std::{io::{self, BufRead}, fs::File, collections::HashMap};

fn main() {
    let data_file = "./src/bin/sample.txt";

    let data_lines = if let Ok(file) = File::open(data_file) {
        io::BufReader::new(file).lines()
    } else {
        return;
    };

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            let x_chars: Vec<char> = x.chars().collect();
            map.push(x_chars);
        }
    }

    println!("Starting state:");
    print_map(&map);

    // parse into a vector of nodes (coords, data)
    // generate graph from data
    // move nodes until we get to desired state

    let mut nodes: HashMap<(usize, usize), Node> = HashMap::new();

    for r in 0..map.len() {
        for c in 0..map[r].len() {
                let cost: Option<usize> = match map[r][c] {
                    'A' => { Some(1) },
                    'B' => { Some(10) },
                    'C' => { Some(100) },
                    'D' => { Some(1000) }
                    _ => None

                };
                nodes.insert((r, c), Node::new(map[r][c], cost));
        }
    }

    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if let Some(node) = &nodes.get(&(r, c)) {
                println!("{:?}", node);
            }
        }
    }

}

pub fn print_map(map: &Vec<Vec<char>>) {
    for r in 0..map.len() {
        for c in 0..map[r].len() {
                print!("{}", map[r][c]);
        }
        println!();
    }
}

#[derive(Debug)]
struct Node {
    data: char,
    cost: Option<usize>
}

impl Node {
    pub fn new(data: char, cost: Option<usize>) -> Self {
        Node { data, cost }
    }
}

