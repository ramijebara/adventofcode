use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const RADIX: u32 = 10; // number base (base 10 in this case)

    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data = Vec::new();
        for line in data_lines {
            if let Ok(x) = line {
                let y = &*x;
                let y: Vec<char> = y.chars().collect::<Vec<_>>();
                let y: Vec<u32> = y.iter().map(|c| c.to_digit(RADIX).unwrap()).collect();
                data.push(y);
            }
        }

        let mut accumulator = 0;

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if is_lowpoint(&data, i, j) {
                    accumulator += data[i][j] + 1;
                    println!("hit on data[{}][{}]: {}", i, j, data[i][j]);
                }
            }
        }

        println!("accumulator: {}", accumulator);
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

fn is_lowpoint(data: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let x_len = data.len() - 1;
    let y_len = if x_len > 0 {data[0].len() - 1 } else { 0 };

    // corners 
    if x == 0 && y == 0 {
        if data[x][y] < data[x + 1][y] && data[x][y] < data[x][y + 1] {
            return true;
        }
    }

    else if x == 0 && y == y_len {
        if data[x][y] < data[0][y - 1] && data[x][y] < data[x + 1][y] {
            return true;
        }
    }

    else if x == x_len && y == 0 {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x][y + 1] {
            return true;
        }
    }

    else if x == x_len && y == y_len {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x][y - 1] {
            return true;
        }
    }

    // edges

    else if x < x_len && y == 0 {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x + 1][y] && data[x][y] < data[x][y + 1] {
            return true;
        }
    }    

    else if x < x_len && y == y_len {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x + 1][y] && data[x][y] < data[x][y - 1] {
            return true;
        }
    } 

    else if x == 0 && y < y_len {
        if data[x][y] < data[x][y - 1] && data[x][y] < data[x][y + 1] && data[x][y] < data[x + 1][y] {
            return true;
        }
    }    

    else if x == x_len && y < y_len {
        if data[x][y] < data[x][y - 1] && data[x][y] < data[x][y + 1] && data[x][y] < data[x - 1][y] {
            return true;
        }
    }

    // body
    else {
        if data[x][y] < data[x][y - 1] && data[x][y] < data[x][y + 1] && data[x][y] < data[x - 1][y] && data[x][y] < data[x + 1][y] {
            return true;
        } 
    }

    false
}
