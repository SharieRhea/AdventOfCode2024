use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() {
    let path = Path::new("src/resources/day03.txt");
    // open the file and match result enum in case of error
    let file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut input: String = "".to_owned();
    for line in reader.lines() {
        // panic if there is an error reading the line
        input.push_str(&line.unwrap()); 
    }

    // PART 1
    // match any valid mul() operation and save the two numbers into capture classes
    let pattern: Regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut sum: i32 = 0;
    for (_, [number1, number2]) in  pattern.captures_iter(&input).map(|it| it.extract()) {
        sum += number1.parse::<i32>().unwrap() * number2.parse::<i32>().unwrap();
    }
    println!("Part 1: {}", sum);

    // PART 2
    // same as before but also add "do" and "don't" as captures, to match capture number place both
    // numbers into one capture and split later
    let pattern: Regex = Regex::new(r"mul\(([0-9]{1,3},[0-9]{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();
    let mut sum: i32 = 0;
    // begin with multiplication enabled until we see a "don't"
    let mut enabled = true;
    for (_, [capture]) in  pattern.captures_iter(&input).map(|it| it.extract()) {
        // check for a do or don't
        if capture == "do" { enabled = true; }
        else if capture == "don't" { enabled = false; }
        else if enabled {
            // only add to the sum if multiplication is currently "enabled"
            let mut numbers = capture.split(",");
            sum += numbers.next().unwrap().parse::<i32>().unwrap() * numbers.next().unwrap().parse::<i32>().unwrap();
        }
    }
    println!("Part 2: {}", sum);
}
