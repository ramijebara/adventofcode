use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(data_lines) = read_lines("./src/bin/data.txt") {
        let mut bingo_numbers: Vec<u8> = Vec::new();
        let mut line_number = 0;
        let mut boards: Vec<BingoBoard> = Vec::new();
        let mut board_tracker = 0;
        let mut board = BingoBoard::new();

        for line in data_lines {
            if let Ok(x) = line {
                // read bingo input
                if line_number == 0 {
                    bingo_numbers = x.split(",").map(|n| n.parse().unwrap()).collect();
                }

                // read bingo boards
                if line_number > 1 {
                    match board_tracker {
                        t if t <= 4 => {
                            // fill the board row with index t
                            let board_line: Vec<u8> =
                                x.split_whitespace().map(|n| n.parse().unwrap()).collect();

                            for i in 0..5 {
                                board.board[t][i].0 = board_line[i];
                            }
                            board_tracker += 1;
                        }
                        t if t == 5 => {
                            // clone and push board to board array
                            board_tracker = 0;
                            let board1 = board.clone();
                            boards.push(board1);
                        }
                        _ => { /* else do nothing */ }
                    }
                }

                line_number += 1;
            }
        }

        for num in bingo_numbers {
            let mut bingo = false;

            for i in 0..boards.len() {
                boards[i].update_board(num);

                if boards[i].check_bingo() == true {
                    println!("{} BINGO!", num);
                    boards[i].print();
                    println!("{}", boards[i].calculate_result(num));

                    bingo = true;
                    break;
                }
            }

            if bingo {
                break;
            }
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

#[derive(Debug, Clone, Copy)]
struct BingoBoard {
    board: [[(u8, bool); 5]; 5],
}

impl BingoBoard {
    pub fn new() -> Self {
        BingoBoard {
            board: [
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
                [(0, false), (0, false), (0, false), (0, false), (0, false)],
            ],
        }
    }

    pub fn update_board(&mut self, number: u8) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].0 == number {
                    self.board[i][j].1 = true;
                }
            }
        }
    }

    pub fn check_bingo(&self) -> bool {
        //check rows and cols
        for r in 0..5 {
            if self.board[r][0].1
                && self.board[r][1].1
                && self.board[r][2].1
                && self.board[r][3].1
                && self.board[r][4].1
            {
                return true;
            }
        }

        for c in 0..5 {
            let board = self.board;
            let col = board
                .iter()
                .map(|s| s.iter().nth(c).unwrap())
                .collect::<Vec<_>>();
            if col[0].1 && col[1].1 && col[2].1 && col[3].1 && col[4].1 {
                return true;
            }
        }
        false
    }

    pub fn calculate_result(&self, num: u8) -> usize {
        let mut sum_unmarked: usize = 0;

        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j].1 == false {
                    sum_unmarked += self.board[i][j].0 as usize
                }
            }
        }

        sum_unmarked * num as usize
    }

    pub fn print(&self) {
        for i in 0..5 {
            println!(
                "{:?} {:?} {:?} {:?} {:?}",
                self.board[i][0],
                self.board[i][1],
                self.board[i][2],
                self.board[i][3],
                self.board[i][4]
            );
        }
    }
}
