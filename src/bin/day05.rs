use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use util;

fn main() {
    let mut lines = util::read_lines("day05.txt").into_iter();
    let mut line = lines.next().unwrap();
    
    // PART 1
    // create a map from each page number to all page numbers that must be printed BEFORE it
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    while !line.is_empty() {
        let mut numbers = line.trim().split("|");
        let value: i32 = numbers.next().unwrap().parse().unwrap();
        let key: i32 = numbers.next().unwrap().parse().unwrap();

        match rules.get_mut(&key) {
            // add new page to set or create set if needed
            Some(set) => { set.insert(value); },
            None => { rules.insert(key, HashSet::from([value])); },
        }

        line = lines.next().unwrap();
    }

    let mut sum: i32 = 0;
    let mut invalid_updates: Vec<Vec<i32>> = vec![];
    'update: for update in lines {
        // get a Vec of the pages as ints
        let pages: Vec<i32> = update.trim().split(",").map(|number| number.parse::<i32>().unwrap()).collect();
        let middle: i32 = *pages.get(pages.len() / 2).unwrap();

        let mut banned_pages: HashSet<i32> = HashSet::new();
        for page in pages.iter() {
            if banned_pages.contains(&page) {
                // ordering is invalid, stop processing this update but save this list for part 2
                invalid_updates.push(pages.clone());
                continue 'update;
            }
            if let Some(banned_page_set) = rules.get(&page) {
                // ordering valid so far, add new banned pages
                banned_pages.extend(banned_page_set);
            }
        }
        // this update was good, add its middle page
        sum += middle;
    }

    println!("Part 1: {}", sum);

    // PART 2
    let mut sum2: i32 = 0;
    for invalid_update in invalid_updates.iter_mut() {
        // custom comparator to sort such that a number is greater than all numbers that appear in
        // its banned set
        // if a number does not appear in the rules map, it has irrelevant ordering
        invalid_update.sort_by(|one, two| {
            if rules.contains_key(&one) && rules.get(&one).unwrap().contains(&two) { return Ordering::Greater; }
            else if rules.contains_key(&two) && rules.get(&two).unwrap().contains(&one) { return Ordering::Less; }
            else { return Ordering::Equal; }
        });
        sum2 += invalid_update.get(invalid_update.len() / 2).unwrap();
    }

    println!("Part 2: {}", sum2);
}
