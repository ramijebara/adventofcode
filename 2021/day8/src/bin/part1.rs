use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {

        let mut known_segments = 0;

        for line in data_lines {
            if let Ok(x) = line {
                let data = x.split("|").collect::<Vec<&str>>();
                if data.len() == 2 {
                    let display :Vec<&str> = data[1].split_whitespace().collect::<Vec<&str>>();

                    for word in display {
                        let len = word.len();
                        if len == 2 || len == 3 || len == 4 || len == 7 {
                            known_segments += 1;
                        }
                    }
                }
            }
        }

        println!("known signals: {}", known_segments);
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
