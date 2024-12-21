use std::collections::{HashMap, HashSet};

use util;

fn main() {
    let contents = util::read_file("day19.txt");
    let (top, bottom) = contents.split_once("\n\n").unwrap();
    let stock: HashSet<&str> = top.split(", ").collect();
    let mut designs: Vec<&str> = bottom.split("\n").collect();
    // remove the empty entry at the end
    designs.remove(designs.len() - 1);
    // find the maximum length of a pattern in the stock
    let max_pattern_size = stock.clone().into_iter().map(|it| it.len()).max().unwrap();

    println!("Part 1: {}", part_1(&designs, &stock));
    println!("Part 2: {}", part_2(&designs, &stock, max_pattern_size));
}

fn part_1(designs: &Vec<&str>, stock: &HashSet<&str>) -> usize {
    // keep track of designs we've already checked to reduce recomputing
    let mut map: HashMap<String, bool> = HashMap::new();

    let mut possible_designs = 0;
    for design in designs {
        if find_design(design, stock, &mut map) {
            possible_designs += 1;
        }
    }
    return possible_designs;
}

fn part_2(designs: &Vec<&str>, stock: &HashSet<&str>, max_pattern_size: usize) -> usize {
    // keep track of how many ways to make a design, memoization!
    let mut map: HashMap<String, usize> = HashMap::new();

    let mut possible_arrangements = 0;
    for design in designs {
        possible_arrangements += find_designs(design, stock, &mut map, max_pattern_size);
    }
    return possible_arrangements;
}

fn find_design(design: &str, stock: &HashSet<&str>, map: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() { return true; }
    // check the map first
    if map.contains_key(design) {
        return map[design];
    }
    // if the design is only one in length, it either can be made or not
    if design.len() == 1 {
        let result = stock.contains(design);
        map.insert(design.to_owned(), result);
        return result;
    }
    // otherwise, try to match the biggest section possible
    // start by reducing the length by 1 each time
    for final_index in (1..=design.len()).rev() {
        // for each new size, we need to shift over to check all possibilities, segmenting the
        // design into 3 sections
        for shift in 0..=design.len() - final_index { 
            if stock.contains(&design[shift..final_index + shift]) {
                // make sure rest of design can be matched, aka segments 1 and 3
                if find_design(&design[..shift], stock, map) && find_design(&design[final_index + shift..], stock, map) { 
                    map.insert(design.to_owned(), true);
                    return true; 
                }
            }
        }
    }
    map.insert(design.to_owned(), false);
    return false;
}

fn find_designs(design: &str, stock: &HashSet<&str>, map: &mut HashMap<String, usize>, max_pattern_size: usize) -> usize {
    let mut possibilities = 0;
    // check the map first
    if map.contains_key(design) {
        return map[design];
    }
    // similar to part 1 but only segment into 2 sections because that makes way more sense
    for index in 0..design.len() {
        // if our length is longer than the maximum available pattern there will be no more matches
        if index > max_pattern_size - 1 { break; }
        // get the remaining section of the pattern
        let second = &design[..index + 1];
        if stock.contains(second) {
            // it can be made, check the first segment
            let first = &design[index + 1..];
            if stock.contains(first) {
                possibilities += 1;
            }
            if map.contains_key(first) {
                possibilities += map[first]; 
                // continue so we don't double count anything
                continue;
            }
            possibilities += find_designs(&design[index + 1..], stock, map, max_pattern_size);
        } 
    }
    map.insert(design.to_owned(), possibilities);
    return possibilities;
}
