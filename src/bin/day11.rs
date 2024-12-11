use std::collections::HashMap;

use util;

fn main() {
    let line = util::read_file("day11.txt");
    let mut stones: HashMap<usize, usize> = HashMap::new();
    line.split(" ").for_each(|number| { stones.insert(number.parse().unwrap(), 1); });

    println!("Part 1: {}", process(stones.clone(), 25));
    println!("Part 2: {}", process(stones, 75));
}

fn process(mut stones: HashMap<usize, usize>, blinks: usize) -> usize {
    for _ in 0..blinks {
        // make a new map each iteration to avoid dealing with repeats
        let mut new_stones: HashMap<usize, usize> = HashMap::new();
        for key in stones.keys() {
            let results = blink(*key); 
            for result in results {
                // if there weren't any in the map yet, default to 0 so only the new ones are added
                new_stones.insert(result, new_stones.get(&result).unwrap_or(&0) + stones[key]);
            }
        }
        // update to use the new map for the next iteration
        stones = new_stones;
    }
    return stones.values().sum();
}

fn blink(number: usize) -> Vec<usize> {
    if number == 0 {
        return vec![1];
    }
    // convert to string to find number of digits because I'm lazy
    let digits = number.to_string();
    if digits.len() % 2 == 0 {
        return vec![digits[0..digits.len() / 2].parse().unwrap(), digits[digits.len() / 2..].parse().unwrap()];        
    }
    return vec![number * 2024];
}
