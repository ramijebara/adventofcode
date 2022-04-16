use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut rules :HashMap<String, char> = HashMap::new();
        let mut polymer :Vec<char> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y :Vec<&str> = x.split(" -> ").collect::<Vec<_>>();
                if y.len() == 2 {
                    let k = y[0].to_string();
                    let v = y[1].chars().nth(0).unwrap();
                    rules.insert(k, v);
                } else {
                    let y = &*x;
                    if y.len() > 0 {
                        polymer = y.chars().collect::<Vec<_>>();
                    }
                }
            }
        }

        let steps = 10;
        let mut i = 1;
        while i <= steps {
            let mut j = 0;
            while j < (polymer.len() - 1) {
                let k = format!("{}{}", polymer[j], polymer[j+1]);
                let v = rules.get(&k).unwrap();
                polymer.insert(j+1, *v);
                j += 2;
            }
            println!("step {}: polymer length = {}", i, polymer.len());
            i += 1;
        }

        polymer.sort();
        let mut elements = polymer.clone();
        elements.dedup();
        let mut most :usize = 0;
        let mut least :usize = polymer.len() as usize;
        for e in elements {
            let count = polymer.iter().filter(|&c| *c == e).count();
            if count > most { most = count; }
            if count < least { least = count; }
        }

        println!("most - least = {}", most - least);
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
