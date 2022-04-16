use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const RADIX: u32 = 10; // number base (base 10 in this case)

    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data: Vec<Vec<i32>> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y = &*x;
                let y: Vec<i32> = y
                    .chars()
                    .map(|c| c.to_digit(RADIX).unwrap() as i32)
                    .collect::<Vec<i32>>();
                data.push(y);
            }
        }

        // create hashmap from data
        let nodes :HashMap<(i32, i32), i32> = data.iter().enumerate().flat_map(|(i, line)| {
            line.iter().enumerate().map(
                move |(j, c)|
                ((i as i32, j as i32), *c)
            )
        }).collect();

        let risk_accumulator = calculate_risk(&nodes);
        println!("bottom right risk: {}", risk_accumulator);

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

// this calculates the lowest cost to all nodes in the matrix
// returns the max (lowest) cost to a node. which will corresond
// to the bottom right corner.
fn calculate_risk(nodes :&HashMap<(i32, i32), i32>) -> usize {
    let mut knowns = HashMap::new();
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0)]);

    while let Some((Reverse(total_risk), x, y)) = queue.pop() {
        let best_risk = knowns.entry((x, y)).or_insert(usize::MAX);

        if total_risk < *best_risk {
            *best_risk = total_risk;

            // check connections and calculate the best distance to 
            // a set of coordinates. Once calculated push to the queue.
            for (i, j) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let (x, y) = (x + i, y + j);
                if let Some(risk) = nodes.get(&(x, y)) {
                    queue.push((Reverse(total_risk + *risk as usize), x, y));
                }
            }
        }
    }
    // get the highest best entry which will correspond to the 
    // best code of bottom right
    knowns[knowns.keys().max().unwrap()]
}