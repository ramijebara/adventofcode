use color_eyre::eyre::Result;
use log::{info, trace};
use std::{fs::File, io::BufRead, io::BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day4/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: Vec<(String, Vec<i32>, Vec<i32>)> = Vec::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        let left_right: Vec<&str> = data_line.splitn(2, '|').collect();

        let mut lr = left_right.iter();

        let (left, right) = (lr.next().unwrap(), lr.next().unwrap());
        trace!("{} - {}", left, right);

        let l_split: Vec<&str> = left.splitn(2, ':').collect();

        let mut ls = l_split.iter();
        let (card, winning_numbers) = (
            ls.next().unwrap(),
            ls.next()
                .unwrap()
                .split(" ")
                .into_iter()
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
        let game_numbers = right
            .trim()
            .split(' ')
            .into_iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        data.push((card.to_string(), winning_numbers, game_numbers));
    }

    let mut result = 0;

    data.into_iter().for_each(|(_, w, n)| {
        let mut score = 0;

        w.iter().for_each(|x| {
            if n.contains(x) {
                score = if score == 0 { 1 } else { score * 2 };
            }
        });

        result += score;
    });

    info!("result: {}", result);

    Ok(())
}
