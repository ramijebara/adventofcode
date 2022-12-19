use std::{
    fs::File,
    io::{self, BufRead},
};

use log::{error, info};

fn main() {
    env_logger::init();

    let data_lines = if let Ok(file) = File::open("./src/bin/data.txt") {
        io::BufReader::new(file).lines()
    } else {
        error!("Error reading data");
        return;
    };

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut inspected_item_count = Vec::new();

    for line in data_lines {
        if let Ok(x) = line {
            if x.starts_with("Monkey") {
                let mut new_monkey = Monkey::default();

                // monkey number
                let mn = x.replace(":", "");
                let mn = mn.split(" ").collect::<Vec<_>>();
                if let Some(n) = mn.last() {
                    new_monkey.id = n.parse::<usize>().unwrap();
                }
                monkeys.push(new_monkey);
                inspected_item_count.push(0 as usize);
            }

            if let Some(cur_monkey) = monkeys.last_mut() {
                // split the lines into key/value pairs
                let sl = x.split(":").collect::<Vec<_>>();

                if x.contains("Starting items") {
                    if let Some(v) = sl.last() {
                        let items: Vec<usize> = v
                            .split(",")
                            .collect::<Vec<_>>()
                            .iter()
                            .map(|i| {
                                let number = i.replace(" ", "");
                                number.parse::<usize>().unwrap()
                            })
                            .collect();

                        *cur_monkey.items = items;
                    }
                }

                if x.contains("Operation") {
                    if let Some(v) = sl.last() {
                        let operation = v.replace(" new = ", "").to_string();

                        *cur_monkey.operation = operation;
                    }
                }

                if x.contains("Test") {
                    if let Some(v) = sl.last() {
                        let test_devisor =
                            v.replace(" divisible by ", "").parse::<usize>().unwrap();

                        cur_monkey.test_divisor = test_devisor;
                    }
                }

                if x.contains("If true") {
                    if let Some(v) = sl.last() {
                        let true_route =
                            v.replace(" throw to monkey ", "").parse::<usize>().unwrap();

                        cur_monkey.true_route = true_route;
                    }
                }

                if x.contains("If false") {
                    if let Some(v) = sl.last() {
                        let false_route =
                            v.replace(" throw to monkey ", "").parse::<usize>().unwrap();

                        cur_monkey.false_route = false_route;
                    }
                }
            }
        }
    }

    let common_multiple = get_common_multiple(&monkeys);

    // 10,000 rounds
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            info!("Before: {:#?}", monkeys[i]);

            monkeys[i].items.reverse();
            while let Some(item) = monkeys[i].items.pop() {
                // increment inspected item count
                inspected_item_count[i] += 1;

                // do the math and item routing
                let expression = monkeys[i].operation.replace("old", &item.to_string());
                let worry_level = meval::eval_str(expression).unwrap() as usize;
                let worry_level = worry_level % common_multiple;

                info!("worry_level: {}", worry_level);

                if worry_level % monkeys[i].test_divisor == 0 {
                    info!("(True route) Throw to: {}", monkeys[i].true_route);
                    let next_monkey = monkeys[i].true_route;
                    monkeys[next_monkey].items.push(worry_level);
                } else {
                    info!("(False route) Throw to: {}", monkeys[i].false_route);
                    let next_monkey = monkeys[i].false_route;
                    monkeys[next_monkey].items.push(worry_level);
                }
            }

            info!("After: {:#?}", monkeys[i]);
        }
    }

    // inspection counts
    for i in 0..inspected_item_count.len() {
        println!(
            "Monkey {} inspected items {} times.",
            i, inspected_item_count[i]
        );
    }

    inspected_item_count.sort();
    inspected_item_count.reverse();

    println!(
        "Result: {}",
        inspected_item_count[0] * inspected_item_count[1]
    );
}

#[derive(Debug, Default)]
struct Monkey {
    id: usize,
    items: Box<Vec<usize>>,
    operation: Box<String>,
    test_divisor: usize,
    true_route: usize,
    false_route: usize,
}

fn get_common_multiple(monkeys: &Vec<Monkey>) -> usize {
    monkeys.iter().fold(1, |acc, m| m.test_divisor * acc)
}
