use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    const RADIX: u32 = 10; // number base (base 10 in this case)

    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut data = Vec::new();
        let mut basins: Vec<Basin> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y = &*x;
                let y: Vec<char> = y.chars().collect::<Vec<_>>();
                let y: Vec<u32> = y.iter().map(|c| c.to_digit(RADIX).unwrap()).collect();
                data.push(y);
            }
        }

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                if is_lowpoint(&data, i, j) {
                    let basin = Basin::new(i, j);
                    basins.push(basin);
                }
            }
        }

        for i in 0..basins.len() {
            let mut been_there: Vec<(usize, usize)> = Vec::new();
            basins[i].size = basin_crawler(basins[i].x, basins[i].y, &data, &mut been_there);
        }

        basins.sort_by_key(|b| b.size);

        let b_len = basins.len();
        if b_len >= 3 {
            println!(
                "top three multiplied: {}",
                (basins[b_len - 1].size * basins[b_len - 2].size * basins[b_len - 3].size)
            );
        } else {
            println!("less than three basins discovered.");
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
struct Basin {
    x: usize,
    y: usize,
    size: u32,
}

impl Basin {
    pub fn new(x: usize, y: usize) -> Self {
        Basin {
            x: x,
            y: y,
            size: 0,
        }
    }
}

fn basin_crawler(
    x: usize,
    y: usize,
    data: &Vec<Vec<u32>>,
    been_there: &mut Vec<(usize, usize)>,
) -> u32 {
    let x_len = data.len() - 1;
    let y_len = if x_len > 0 { data[0].len() - 1 } else { 0 };

    if x_len == 0 || y_len == 0 {
        return 0;
    }

    if been_there.contains(&(x, y)) {
        return 0;
    } else {
        been_there.push((x, y));
    }

    if data[x][y] == 9 {
        return 0;
    }

    if x == 0 && (y < y_len && y > 0) {
        return 1
            + basin_crawler(x, y + 1, data, been_there)
            + basin_crawler(x, y - 1, data, been_there)
            + basin_crawler(x + 1, y, data, been_there);
    }

    if x == x_len && (y < y_len && y > 0) {
        return 1
            + basin_crawler(x, y + 1, data, been_there)
            + basin_crawler(x, y - 1, data, been_there)
            + basin_crawler(x - 1, y, data, been_there);
    }

    if y == 0 && (x < x_len && x > 0) {
        return 1
            + basin_crawler(x + 1, y, data, been_there)
            + basin_crawler(x - 1, y, data, been_there)
            + basin_crawler(x, y + 1, data, been_there);
    }

    if y == y_len && (x < x_len && x > 0) {
        return 1
            + basin_crawler(x + 1, y, data, been_there)
            + basin_crawler(x - 1, y, data, been_there)
            + basin_crawler(x, y - 1, data, been_there);
    }

    if x == 0 && y == 0 {
        return 1
            + basin_crawler(x + 1, y, data, been_there)
            + basin_crawler(x, y + 1, data, been_there);
    }

    if x == x_len && y == y_len {
        return 1
            + basin_crawler(x - 1, y, data, been_there)
            + basin_crawler(x, y - 1, data, been_there);
    }

    if x == 0 && y == y_len {
        return 1
            + basin_crawler(x + 1, y, data, been_there)
            + basin_crawler(x, y - 1, data, been_there);
    }

    if x == x_len && y == 0 {
        return 1
            + basin_crawler(x - 1, y, data, been_there)
            + basin_crawler(x, y + 1, data, been_there);
    }

    return 1
        + basin_crawler(x + 1, y, data, been_there)
        + basin_crawler(x - 1, y, data, been_there)
        + basin_crawler(x, y + 1, data, been_there)
        + basin_crawler(x, y - 1, data, been_there);
}

fn is_lowpoint(data: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let x_len = data.len() - 1;
    let y_len = if x_len > 0 { data[0].len() - 1 } else { 0 };

    // corners
    if x == 0 && y == 0 {
        if data[x][y] < data[x + 1][y] && data[x][y] < data[x][y + 1] {
            return true;
        }
    } else if x == 0 && y == y_len {
        if data[x][y] < data[0][y - 1] && data[x][y] < data[x + 1][y] {
            return true;
        }
    } else if x == x_len && y == 0 {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x][y + 1] {
            return true;
        }
    } else if x == x_len && y == y_len {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x][y - 1] {
            return true;
        }
    }
    // edges
    else if x < x_len && y == 0 {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x + 1][y] && data[x][y] < data[x][y + 1]
        {
            return true;
        }
    } else if x < x_len && y == y_len {
        if data[x][y] < data[x - 1][y] && data[x][y] < data[x + 1][y] && data[x][y] < data[x][y - 1]
        {
            return true;
        }
    } else if x == 0 && y < y_len {
        if data[x][y] < data[x][y - 1] && data[x][y] < data[x][y + 1] && data[x][y] < data[x + 1][y]
        {
            return true;
        }
    } else if x == x_len && y < y_len {
        if data[x][y] < data[x][y - 1] && data[x][y] < data[x][y + 1] && data[x][y] < data[x - 1][y]
        {
            return true;
        }
    }
    // body
    else {
        if data[x][y] < data[x][y - 1]
            && data[x][y] < data[x][y + 1]
            && data[x][y] < data[x - 1][y]
            && data[x][y] < data[x + 1][y]
        {
            return true;
        }
    }

    false
}
