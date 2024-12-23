use std::collections::{HashMap, HashSet};

use util;

fn main() {
    let numbers: Vec<i64> = util::read_lines("day22.txt").iter().map(|it| it.parse::<i64>().unwrap()).collect();

    println!("Part 1: {}", part_1(&numbers));
    println!("Part 2: {}", part_2(&numbers));
}

fn part_1(numbers: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for number in numbers {
        let mut secret = *number;
        for _ in 0..1999 {
           secret = next_secret(secret);
        }
        sum += next_secret(secret);
    }
    return sum;
}

fn part_2(numbers: &Vec<i64>) -> i64 {
    // collect what bananas will be bought for each sequence of changes
    let mut monkeys: HashSet<(usize, [i64; 4])> = HashSet::new();
    let mut map: HashMap<[i64; 4], i64> = HashMap::new();

    for (id, number) in numbers.into_iter().enumerate() {
        // set up the first 4 price changes
        let mut changes: Vec<i64> = vec![];
        let mut current = *number;
        let mut change;
        for _ in 0..4 {
            (current, change) = calculate_price_change(current);  
            changes.push(change);
        }
        // do the remaining secret generations
        for _ in 0..1996 {
            // only bother with sequences that have a price increase at the end
            if changes[3] > 0 {
                let array: [i64; 4] = changes.clone().try_into().expect("invalid length");
                // only count the first time this sequence is seen
                if !monkeys.contains(&(id, array)) {
                    monkeys.insert((id, array));
                    // add to the total if it exists, start new if not
                    if let Some(bananas) = map.get_mut(&array) {
                        *bananas += current % 10;
                    }
                    else {
                        map.insert(array, current % 10);
                    }
                }
            }
            changes.remove(0);
            (current, change) = calculate_price_change(current);
            changes.push(change);
        }
    }

    // see what the max was
    return *map.values().into_iter().max().unwrap();
}

fn calculate_price_change(number: i64) -> (i64, i64) {
    let next_secret = next_secret(number);
    let change = next_secret % 10 - number % 10;
    return (next_secret, change);
}

fn next_secret(secret: i64) -> i64 {
    let step1 = secret * 64;
    let step2 = step1 ^ secret;
    let step3 = step2 % 16777216;
    let step4 = step3 / 32;
    let step5 = step4 ^ step3;
    let step6 = step5 * 2048;
    let step7 = step6 ^ step5;
    let step8 = step7 % 16777216;
    return step8;
}
