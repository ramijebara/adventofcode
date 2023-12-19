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

    let mut cards: Vec<Vec<i32>> = Vec::new();

    data.into_iter().for_each(|(_, w, n)| {
        let c = Vec::from([get_matches(w, n)]);
        cards.push(c);
    });

    trace!("{:?}", cards);

    let mut i = 0;
    loop {
        if i == cards.len() {
            break;
        }

        for _ in 0..cards[i].len() {
            let start = i + 1;
            let end = i + (cards[i][0] + 1) as usize;
            for k in start..end {
                let v = cards[k][0];
                cards[k].push(v);
            }
        }

        trace!("{:?}", cards);
        i += 1;
    }

    let mut result = 0;

    cards.iter().for_each(|x| {
        result += x.len();
    });

    info!("result: {}", result);
    Ok(())
}

fn get_matches(winning_numbers: Vec<i32>, numbers: Vec<i32>) -> i32 {
    let mut match_num = 0;

    winning_numbers.iter().for_each(|x| {
        if numbers.contains(x) {
            match_num += 1;
        }
    });

    match_num
}
