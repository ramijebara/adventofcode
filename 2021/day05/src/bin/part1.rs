use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut coordinates :Vec<Line> = Vec::new();

        for line in data_lines {
            if let Ok(x) = line {
                let points_vec = x.split(" -> ").collect::<Vec<&str>>();

                // make sure to get 2 points
                if points_vec.len() == 2 {
                    //parse points to a line
                    let mut line = Line::new();

                    let start_point = points_vec[0].split(',').map(|p| p.parse().unwrap()).collect::<Vec<u16>>();
                    if start_point.len() == 2 { line.start = (start_point[0], start_point[1]); }

                    let end_point = points_vec[1].split(',').map(|p| p.parse().unwrap()).collect::<Vec<u16>>();
                    if end_point.len() == 2 { line.end = (end_point[0], end_point[1]); }

                    // only work with horizontal or vertical lines
                    if line.is_horizontal() || line.is_vertical() {
                        coordinates.push(line);
                    }
                }
            }
        }

        // initialize grid of a certain size
        let mut grid = Grid::new(1000);

        for coordinate in coordinates {
            grid.add_line(coordinate);
        }

        //grid.draw();
        
        println!("Overlap: {}", grid.overlap());
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

#[derive(Debug, Clone, Copy)]
struct Line {
    start: (u16, u16),
    end: (u16, u16)
}

impl Line {
    pub fn new() -> Self {
        Line {
            start: (0,0),
            end: (0,0)
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn is_vertical(&self) -> bool {
        self.start.1 == self.end.1
    }
}

#[derive(Debug)]
struct Grid {
    canvas: Vec<Vec<u16>>
}

impl Grid {
    pub fn new(size: usize) -> Self {
        let mut c :Vec<Vec<u16>> = Vec::new();

        for _ in 0..size {
            c.push(vec![0; size]);
        }

        Grid {
            canvas: c,
        }
    }

    // pub fn draw(&self) {
    //     for i in 0..self.canvas.len() {
    //         println!{"{:?}", self.canvas[i]}
    //     }
    // }

    pub fn add_line(&mut self, line :Line) {
        if line.is_horizontal() {
            let (start_x, end_x) = if line.end.1 > line.start.1 {
                (line.start.1, line.end.1 + 1)
            } else {
                (line.end.1, line.start.1 + 1)
            };

            for i in start_x..end_x {
                self.canvas[line.start.0 as usize][i as usize] += 1; 
            }
        }

        if line.is_vertical() {
            let (start_y, end_y) = if line.end.0 > line.start.0 {
                (line.start.0, line.end.0 + 1)
            } else {
                (line.end.0, line.start.0 + 1)
            };

            for j in start_y..end_y {
                self.canvas[j as usize][line.start.1 as usize] += 1;
            }
        }
    }

    pub fn overlap(&self) -> usize {
        let length = self.canvas.len();
        let mut overlap = 0;

        for i in 0..length {
            for j in 0..length {
                if self.canvas[i][j] > 1 {
                    overlap += 1;
                }
            }
        }

        overlap
    }
}