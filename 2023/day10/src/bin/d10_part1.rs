use color_eyre::eyre::Result;
use log::{trace, info};
use std::{
    char,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day10/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: Vec<Vec<char>> = Vec::new();
    let south = vec!['|', 'J', 'L'];
    let north = vec!['|', 'F', '7'];
    let east = vec!['-', 'J', '7'];
    let west = vec!['-', 'F', 'L'];

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("{}", data_line);
        data.push(data_line.chars().collect());
    }

    let (start_row, start_col) = find_start(&data).unwrap();
    let mut direction = Direciton::East; // initilize direction
    let mut visited = Vec::new();

    // Set initial direction
    if start_row < (data.len() - 1) && south.contains(&data[start_row + 1][start_col]) {
        direction = Direciton::South;
    } else if start_row > 1 && north.contains(&data[start_row - 1][start_col]) {
        direction = Direciton::North;
    } else if start_col < (data[0].len() - 1) && east.contains(&data[start_row][start_col + 1]) {
        direction = Direciton::East;
    } else if start_col > 1 && west.contains(&data[start_row][start_col - 1]) {
        direction = Direciton::West;
    }

    let mut cur_row = start_row;
    let mut cur_col = start_col;

    loop {
        match direction {
            Direciton::North => {
                cur_row -= 1;
            }
            Direciton::South => {
                cur_row += 1;
            }
            Direciton::East => {
                cur_col += 1;
            }
            Direciton::West => {
                cur_col -= 1;
            }
        }

        let cur_direction = direction.clone();

        direction = match data[cur_row][cur_col] {
            'L' => {
                if direction == Direciton::South {
                    Direciton::East
                } else {
                    Direciton::North
                }
            }
            'J' => {
                if direction == Direciton::South {
                    Direciton::West
                } else {
                    Direciton::North
                }
            }
            '7' => {
                if direction == Direciton::North {
                    Direciton::West
                } else {
                    Direciton::South
                }
            }
            'F' => {
                if direction == Direciton::North {
                    Direciton::East
                } else {
                    Direciton::South
                }
            }

            _ => { cur_direction }
        };

        visited.push((cur_row, cur_col));

        if cur_row == start_row && cur_col == start_col {
            break;
        }
    }

    trace!("visited: {:?}", {});
    let result = visited.len() / 2 as usize + (visited.len() % 2);
    info!("result: {}", result);

    Ok(())
}

fn find_start(data: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for row in 0..data.len() {
        for col in 0..data[row].len() {
            if data[row][col] == 'S' {
                return Some((row, col));
            }
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
enum Direciton {
    North,
    South,
    East,
    West,
}
