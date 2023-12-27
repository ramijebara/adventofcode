use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day7/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: Vec<Hand> = Vec::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("{}", data_line);
        let data_strs: Vec<&str> = data_line.splitn(2, " ").collect::<Vec<&str>>();

        let card_list: Vec<char> = data_strs[0].chars().collect();
        let cards = card_hist(card_list.clone());
        let game: Vec<usize> = card_list.into_iter().map(|c|{ get_card_value(c) }).collect::<Vec<usize>>();
        let bid = data_strs[1].parse::<usize>().unwrap();

        let hand = Hand {
            game,
            card_hist: cards,
            bid,
            ..Default::default()
        };

        data.push(hand);
    }

    data.iter_mut().for_each(|hand| {
        let score = calculate_score(hand);
        hand.score = score;

        trace!(
            "cards: {:?}, bid: {}, map: {:?}, score: {}",
            hand.card_hist,
            hand.bid,
            hand.game,
            hand.score
        );
    });

    data.sort_by_key(|h| (h.score, h.game.clone()) );
    trace!("{:#?}", data);

    let mut result = 0;

    data.into_iter().enumerate().for_each(|(i, h)| {
        let rank = i + 1;
        debug!("cards: {:?}, score: {}, map: {:?}, bid: {}, rank: {}", h.card_hist, h.score, h.game, h.bid, rank);
        result += h.bid * rank;
    });

    info!("result: {}", result);

    Ok(())
}

#[derive(Debug, Default)]
struct Hand {
    game: Vec<usize>,
    bid: usize,
    card_hist: HashMap<char, usize>,
    score: usize,
}

fn card_hist(card_list: Vec<char>) -> HashMap<char, usize> {
    let mut cards: HashMap<char, usize> = HashMap::new();

    card_list.iter().for_each(|c| {
        if let Some(v) = cards.get_mut(c) {
            *v += 1;
        } else {
            cards.insert(*c, 1);
        }
    });

    cards
}

fn calculate_score(hand: &Hand) -> usize {
    let mut card_counts = Vec::new();

    hand.card_hist.iter().for_each(|(_, v)| {
        card_counts.push(*v);
    });

    card_counts.sort();

    trace!("{:?}", card_counts);

    get_boost(&card_counts)
}

fn get_boost(card_counts: &Vec<usize>) -> usize {
    if card_counts == &vec![5] {
        1000000
    } else if card_counts == &vec![1, 4] {
        100000
    } else if card_counts == &vec![2, 3] {
        10000
    } else if card_counts == &vec![1, 1, 3] {
        1000
    } else if card_counts == &vec![1, 2, 2] {
        100
    } else if card_counts == &vec![1, 1, 1, 2] {
        10
    } else if card_counts == &vec![1, 1, 1, 1, 1] {
        1
    } else {
        0
    }
}

fn get_card_value(card: char) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => { panic!("invalid input") },
    }
}
