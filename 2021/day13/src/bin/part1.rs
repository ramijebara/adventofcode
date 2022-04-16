use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut coordinates: Vec<(usize, usize)> = Vec::new();
        let mut folds: Vec<(String, usize)> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let y: Vec<&str> = x.split(",").collect::<Vec<_>>();

                if y.len() == 2 {
                    let (x, y): (usize, usize) = (
                        FromStr::from_str(y[0]).unwrap(),
                        FromStr::from_str(y[1]).unwrap(),
                    );
                    coordinates.push((x, y));
                    if x > max_x {
                        max_x = x;
                    }
                    if y > max_y {
                        max_y = y;
                    }
                } else if y.len() == 1 {
                    let f: Vec<&str> = y[0].split("=").collect::<Vec<_>>();
                    if f.len() == 2 {
                        let axis = f[0].replace("fold along ", "");
                        let line: usize = FromStr::from_str(f[1]).unwrap();
                        folds.push((axis, line));
                    }
                }
            }
        }

        // initialize paper adding 1 to max numbers because some values start at 0
        let mut paper: Vec<Vec<char>> = vec![vec!['.'; max_x + 1]; max_y + 1];

        // fill paper
        for (x, y) in coordinates {
            paper[y][x] = '#';
        }

        // fold
        let fold1 = fold(&paper, folds[0].0.clone(), folds[0].1);
        print_paper(&fold1);
        println!("\ncount: {}\n", count_hashtags(&fold1));
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

fn print_paper(paper: &Vec<Vec<char>>) {
    for i in 0..paper.len() {
        for j in 0..paper[0].len() {
            print!("{}", paper[i][j]);
        }
        println!();
    }
}

fn fold(paper: &Vec<Vec<char>>, axis: String, line: usize) -> Vec<Vec<char>> {
    let mut folded_paper: Vec<Vec<char>> = Vec::new();

    if paper.len() == 0 || paper[0].len() == 0 {
        return folded_paper;
    }

    if axis == "y" {
        // fold horizontally
        if line > paper.len() {
            return folded_paper;
        }

        folded_paper = vec![vec![]; line];
        let mut offset: usize = paper.len() - 1;

        for i in 0..line {
            for j in 0..paper[0].len() {
                let v = if paper[i][j] == '#' || paper[offset][j] == '#' {
                    '#'
                } else {
                    '.'
                };
                folded_paper[i].push(v);
            }
            offset -= 1;
        }
    } else {
        // fold vertically
        if line > paper[0].len() {
            return folded_paper;
        }

        folded_paper = vec![vec![]; paper.len()];

        for i in 0..folded_paper.len() {
            let mut offset: usize = paper[0].len() - 1;

            for j in 0..line {
                let v = if paper[i][j] == '#' || paper[i][offset] == '#' {
                    '#'
                } else {
                    '.'
                };
                folded_paper[i].push(v);
                offset -= 1;
            }
        }
    }

    folded_paper
}

fn count_hashtags(paper: &Vec<Vec<char>>) -> usize {
    let accumulator = paper
        .iter()
        .map(|x| x.iter().filter(|&y| *y == '#').count())
        .sum();
    accumulator
}
