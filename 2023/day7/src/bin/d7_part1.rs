use color_eyre::eyre::Result;
use log::{debug, info, trace};
use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader, cmp::Ordering};

fn main() -> Result<()> {
    env_logger::init();
    let file = File::open("./data/day7/data.txt")?;
    let mut data_lines = BufReader::new(file).lines();
    let mut data: Vec<Hand> = Vec::new();

    while let Some(Ok(data_line)) = data_lines.next() {
        trace!("{}", data_line);
        let data_strs: Vec<&str> = data_line.splitn(2, " ").collect::<Vec<&str>>();

        let card_list_chars: Vec<char> = data_strs[0].chars().collect();
        let card_hist = card_hist(card_list_chars.clone());
        let card_list: Vec<usize> = card_list_chars.into_iter().map(|c|{ get_card_value(c) }).collect::<Vec<usize>>();
        let bid = data_strs[1].parse::<usize>().unwrap();

        let hand = Hand {
            card_list,
            card_hist,
            bid,
            ..Default::default()
        };

        data.push(hand);
    }

    data.iter_mut().for_each(|hand| {
        hand.game_type = get_game_type(&hand.card_hist);

        trace!(
            "cards: {:?}, bid: {}, map: {:?}, type: {}",
            hand.card_hist,
            hand.bid,
            hand.card_list,
            hand.game_type
        );
    });

    data.sort();
    trace!("{:#?}", data);

    let mut result = 0;

    data.into_iter().enumerate().for_each(|(i, h)| {
        let rank = i + 1;
        debug!("cards: {:?}, type: {}, map: {:?}, bid: {}, rank: {}", h.card_hist, h.game_type, h.card_list, h.bid, rank);
        result += h.bid * rank;
    });

    info!("result: {}", result);

    Ok(())
}

#[derive(Debug, Default, Eq)]
struct Hand {
    card_list: Vec<usize>,
    bid: usize,
    card_hist: Vec<usize>,
    game_type: usize,
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.game_type == other.game_type {
            for (a, b) in self.card_list.iter().zip(other.card_list.iter()) {
                if a != b {
                    return false;
                }
                return true;
            }
        }
        false
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.game_type.cmp(&other.game_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (a, b) in self.card_list.iter().zip(other.card_list.iter()) {
                    if a != b {
                        return a.cmp(&b);
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn card_hist(card_list: Vec<char>) -> Vec<usize> {
    let mut cards: HashMap<char, usize> = HashMap::new();

    card_list.iter().for_each(|c| {
        if let Some(v) = cards.get_mut(c) {
            *v += 1;
        } else {
            cards.insert(*c, 1);
        }
    });

    let mut hist = cards.values().cloned().collect::<Vec<usize>>();

    hist.sort();

    hist
}

fn get_game_type(card_counts: &Vec<usize>) -> usize {
    if card_counts == &vec![5] {
        7
    } else if card_counts == &vec![1, 4] {
        6
    } else if card_counts == &vec![2, 3] {
        5
    } else if card_counts == &vec![1, 1, 3] {
        4
    } else if card_counts == &vec![1, 2, 2] {
        3
    } else if card_counts == &vec![1, 1, 1, 2] {
        2
    } else if card_counts == &vec![1, 1, 1, 1, 1] {
        1
    } else {
        panic!("invalid game type");
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
