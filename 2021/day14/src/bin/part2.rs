use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut rules: HashMap<String, char> = HashMap::new();
        let mut polymer: Vec<char> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y: Vec<&str> = x.split(" -> ").collect::<Vec<_>>();
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

        let mut buckets :HashMap<String, usize> = HashMap::new();
        for i in 0..(polymer.len() - 1) {
            let mut pair = String::new();
            pair.push(polymer[i]);
            pair.push(polymer[i + 1]);
            let e = buckets.entry(pair).or_insert(0);
            *e += 1;
        }

        let steps = 40;
        let mut i = 1;

        while i <= steps {
            let mut new_buckets :HashMap<String, usize> = HashMap::new();
            for (k, v) in &buckets {
                let c = rules.get(k).unwrap();
                let s: Vec<char> = k.chars().collect::<Vec<_>>();

                // forked key 1
                let mut key1 = String::new();
                key1.push(s[0]);
                key1.push(c.clone());

                // forked key 2
                let mut key2 = String::new();
                key2.push(c.clone());
                key2.push(s[1]);

                let e = new_buckets.entry(key1).or_insert(0);
                *e += v;

                let e = new_buckets.entry(key2).or_insert(0);
                *e += v;
            }
            // assign the new bucket to the old bucket and start over
            buckets = new_buckets;
            i += 1;
        }

        let mut counts :HashMap<char, usize> = HashMap::new();
        for (k, v) in &buckets {
            let s: Vec<char> = k.chars().collect::<Vec<_>>();
            let e = counts.entry(s[0]).or_insert(0);
            *e += v;
        }

        // count the last element of the template exactly once
        let e = counts.entry(polymer[polymer.len() - 1]).or_insert(0);
        *e += 1;

        let max = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let min = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        
        println!("max - min: {:?}", max.1 - min.1);
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
