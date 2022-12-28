use env_logger;
use log::{error, info, trace};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut start = Node::default();
    let mut end = Node::default();
    let mut line_number = 0;
    let mut map: HashMap<Node, i32> = HashMap::new();

    for data_line in data_lines {
        if let Ok(line) = data_line {
            let nodes: Vec<char> = line.chars().collect();

            for (e, c) in nodes.iter().enumerate() {
                match *c {
                    'S' => {
                        start = Node(line_number, e as i32);
                        map.insert(start.clone(), 0);
                    }
                    'E' => {
                        end = Node(line_number, e as i32);
                        map.insert(end.clone(), 25);
                    }
                    _ => {
                        let height = alphabet.iter().position(|x| x == c).unwrap() as i32;
                        let node = Node(line_number, e as i32);
                        map.insert(node, height);
                    }
                }
            }

            // increment y
            line_number += 1;
        }
    }

    info!("Start: {:?}, End: {:?}", start, end);
    info!("Map: {:?}", map);

    if let Some(path) = shortest_path(start, end, map) {
        println!("Result: {}", path.len);
        info!("Path: {:?}", path.path);
    } else {
        error!("No path found.");
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
struct Node(i32, i32);

impl Node {
    pub fn available_paths(&self, map: &HashMap<Node, i32>) -> Vec<Self> {
        let mut paths: Vec<Self> = Vec::new();

        // only send accessible neighbors if the node is in the map
        if let Some(cur_h) = map.get(self) {
            let neighbors = vec![
                // top, bottom, left, right
                Node(self.0 + 1, self.1),
                Node(self.0 - 1, self.1),
                Node(self.0, self.1 + 1),
                Node(self.0, self.1 - 1),
            ];

            for n in neighbors {
                if let Some(h) = map.get(&n) {
                    let d = h - cur_h;
                    if d <= 1 {
                        paths.push(n);
                    }
                }
            }
        }

        paths
    }
}

#[derive(Debug)]
struct Route {
    pos: Node,
    path: Option<Rc<Route>>,
    len: i32,
}

impl Route {
    pub fn start_rc(pos: Node) -> Rc<Self> {
        Rc::new(Route {
            pos: pos,
            path: None,
            len: 0,
        })
    }
}

fn shortest_path(start: Node, end: Node, map: HashMap<Node, i32>) -> Option<Rc<Route>> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut routes: Vec<Rc<Route>> = Vec::new();

    // start route tracking
    routes.push(Route::start_rc(start));

    loop {
        let current_route = routes.pop()?;
        trace!("Current Route: {:?}", current_route);

        if current_route.pos == end {
            // we have arrived
            info!("Arrived: {:?}", current_route);
            return Some(current_route);
        }

        if visited.contains(&current_route.pos) {
            // no point in visiting an already visited node
            continue;
        }

        visited.insert(current_route.pos.clone());

        let exits = current_route.pos.available_paths(&map);

        for exit in exits {
            trace!("Exit: {:?}", exit);

            let new_length = current_route.len + 1;

            let new_route = Rc::new(Route {
                pos: exit,
                len: new_length,
                path: Some(current_route.clone()),
            });

            // if this is the first step add the new routes to the routes vector
            // and continue
            if routes.len() == 0 {
                routes.push(new_route);
                continue;
            }

            // if there are routes insert new route into a the routes vector sorted by
            // length in descending order

            let mut insert_index = routes.len() - 1;

            loop {
                if routes[insert_index].len > new_length {
                    // lowest element last
                    routes.insert(insert_index + 1, new_route);
                    break;
                }

                if insert_index == 0 {
                    // reached end
                    routes.insert(0, new_route);
                    break;
                }

                insert_index -= 1;
            }
        }
    }
}
