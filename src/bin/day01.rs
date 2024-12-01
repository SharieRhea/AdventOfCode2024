use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("resources/day01.txt");
    // open the file and match result enum in case of error
    let file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // PART 1
    // create a buffered reader and initialize two empty vectors
    let reader = BufReader::new(file);
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in reader.lines() {
        // panic if there is an error reading the line
        let line = line.unwrap(); 
        // split and map to a vector of ints
        let numbers: Vec<i32> = line.split_whitespace().flat_map(|x| x.parse::<i32>()).collect();
        // add the ints to the lists
        list1.push(*numbers.get(0).unwrap());
        list2.push(*numbers.get(1).unwrap());
    }

    list1.sort();
    list2.sort();

    let mut sum: i32 = 0;
    for i in 0..list1.len() {
       sum += (list1.get(i).unwrap() - list2.get(i).unwrap()).abs()
    }
    println!("Part 1: {}", sum);

    // PART 2
    let mut map = HashMap::new();
    // create map from number -> occurrences of that number
    for number in list2.iter() {
        match map.get(number) {
            Some(result) => map.insert(number, result + 1),
            _ => map.insert(number, 1),
        };
    }

    let mut similarity: i32 = 0;
    // calculate similarity based on the number of occurrences of that number
    for number in list1.iter() {
        similarity += number * map.get(number).unwrap_or(&0);
    }
    println!("Part 2: {}", similarity);
}
