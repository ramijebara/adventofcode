use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut caves: HashMap<String, Cave> = HashMap::new();

        for line in data_lines {
            if let Ok(x) = line {
                // classify caves
                let cave_link: Vec<&str> = x.split("-").collect();
                let big_cave = Regex::new(r"^.*[A-Z]$").unwrap();
                if cave_link.len() == 2 {
                    // left side cave
                    if !caves.contains_key(&cave_link[0].to_string()) {
                        caves.insert(
                            cave_link[0].to_string(),
                            Cave::new(big_cave.is_match(cave_link[0])),
                        );
                    }

                    // right side cave
                    if !caves.contains_key(&cave_link[1].to_string()) {
                        caves.insert(
                            cave_link[1].to_string(),
                            Cave::new(big_cave.is_match(cave_link[1])),
                        );
                    }

                    // link left to right
                    if let Some(c) = caves.get_mut(&cave_link[0].to_string()) {
                        if !c.edges.contains(&cave_link[1].to_string()) {
                            c.add_edge(cave_link[1].to_string());
                        }
                    }

                    // link right to left
                    if let Some(c) = caves.get_mut(&cave_link[1].to_string()) {
                        if !c.edges.contains(&cave_link[0].to_string()) {
                            c.add_edge(cave_link[0].to_string());
                        }
                    }
                }
            }
        }
        let mut been_there: Vec<(String, u32)> = Vec::new();
        let mut routes: Vec<Vec<String>> = Vec::new();
        let start: Vec<String> = vec!["start".to_string()];

        print_routes(
            &caves,
            start,
            &"end".to_string(),
            &mut been_there,
            &mut routes,
        );

        let mut valid_routes = 0;

        for route in routes
            .iter()
            .filter(|&r| r[r.len() - 1] == "end".to_string())
        {
            println!("{:?}", route);
            valid_routes += 1;
        }
        println!("valid routes: {}", valid_routes);

        println!("\nCaves : {}", caves.len());
        for cave in caves {
            println!("{:?}", cave);
        }
    }
}

/// Returns an iterator to the reader of the lines of the file
/// The output is wrapped in Result for better error handling
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Cave {
    is_big: bool,
    edges: Vec<String>,
}

impl Cave {
    pub fn new(is_big: bool) -> Self {
        Cave {
            is_big: is_big,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: String) {
        self.edges.push(edge);
    }
}

fn print_routes(
    caves: &HashMap<String, Cave>,
    start: Vec<String>,
    end: &String,
    been_there: &mut Vec<(String, u32)>,
    routes: &mut Vec<Vec<String>>,
) {
    // we reached the end
    if start[start.len() - 1] == end.to_string() {
        if start[start.len() - 1] == end.clone() {
            routes.push(start)
        };
        return;
    }

    // we are back to a small cave we have seen before
    if check_criteria(&been_there, start[start.len() - 1].clone()) {
        if start[start.len() - 1] == end.clone() {
            routes.push(start)
        };
        return;
    }

    if let Some(a) = caves.get(&start[start.len() - 1]) {
        // add small caves to been_there vector
        if &a.is_big == &false {
            if (start[start.len() - 1] != "start".to_string()
                || start[start.len() - 1] != "end".to_string())
                && been_there.contains(&(start[start.len() - 1].clone(), 1))
                && been_there.iter().filter(|x| x.1 == 2).count() == 0
            {
                been_there.push((start[start.len() - 1].clone(), 2));
            } else {
                been_there.push((start[start.len() - 1].clone(), 1));
            }
        }

        // we reached a dead end
        if &a.edges.len() == &0 {
            if start[start.len() - 1] == end.clone() {
                routes.push(start)
            };
            return;
        }

        for e in &a.edges {
            // clone start and been there so context for route is not lose
            let mut new_start = start.clone();
            let mut new_been_there = been_there.clone();
            new_start.push(e.clone());
            print_routes(caves, new_start, end, &mut new_been_there, routes);
        }
    }
}

fn check_criteria(been_there: &Vec<(String, u32)>, node: String) -> bool {
    let start = "start".to_string();
    let end = "end".to_string();

    if (node == start || node == end) && been_there.iter().any(|x| x.0 == start || x.0 == end) {
        return true;
    }

    if been_there.contains(&(node.clone(), 2)) {
        return true;
    }

    if been_there.contains(&(node.clone(), 1))
        && been_there.iter().filter(|x| x.1 == 2).count() == 1
    {
        return true;
    }

    false
}
