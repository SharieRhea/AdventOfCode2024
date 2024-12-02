use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("src/resources/day02.txt");
    // open the file and match result enum in case of error
    let file = match File::open(&path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let mut reports: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        // panic if there is an error reading the line
        let line = line.unwrap(); 
        // split and map to a vector of ints
        let numbers: Vec<i32> = line.split_whitespace().flat_map(|x| x.parse::<i32>()).collect();
        reports.push(numbers);
    }


    let safe_reports = part_1(reports.clone());
    println!("Part 1: {}", safe_reports);
    let new_safe_reports = part_2(reports);
    println!("Part 2: {}", new_safe_reports);
}

fn part_1(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports: i32 = 0;
    for report in reports.iter() {
        if check_report(report.to_vec()) {
            safe_reports += 1;
        }
    }
    return safe_reports;
}

fn part_2(reports: Vec<Vec<i32>>) -> i32 {
    let mut safe_reports: i32 = 0;
    for report in reports.iter() {
        if check_report_with_dampener(report.to_vec()) {
            safe_reports += 1;
        }
    }
    return safe_reports;
}

fn check_report(report: Vec<i32>) -> bool {
    // a report is safe if it always increases or always decreases and change is at least 1 and
    // no more than 3
    
    let mut previous: i32 = *report.get(0).unwrap();
    let increasing: bool = *report.get(1).unwrap() - *report.get(0).unwrap() > 0; 

    // skip the first one because there's nothing to compare it to
    for level in report.iter().skip(1) {
        if ((level - previous).abs() < 1 || (level - previous).abs() > 3) ||
        (increasing && level - previous < 0 ) ||
        (!increasing && level - previous > 0) {
            return false;
        }
        previous = *level;
    }
    return true;
}

fn check_report_with_dampener(report: Vec<i32>) -> bool {
    if check_report(report.to_vec()) {
        return true;
    }

    // a report is safe if it always increases or always decreases and change is at least 1 and
    // no more than 3
    // additionally, one level may be removed by the dampener
    
    // edge case: check for removing the very first or second element
    // as these can determine the value of increasing
    let mut remove_two = report.to_vec();
    remove_two.remove(1);
    if check_report(report[1..].to_vec()) || check_report(remove_two) {
        return true;
    }

    let mut previous: i32 = *report.get(1).unwrap();
    let increasing: bool = *report.get(1).unwrap() - *report.get(0).unwrap() > 0; 

    // skip the first two because they have been checked already
    for (index, level) in report.iter().skip(2).enumerate() {
        if ((level - previous).abs() < 1 || (level - previous).abs() > 3) ||
        (increasing && level - previous < 0 ) ||
        (!increasing && level - previous > 0) {
            // try removing this element and the one before it, add 2 because of skip
            let mut attempt1 = report.to_vec();
            attempt1.remove(index + 2);
            let mut attempt2 = report.to_vec();
            attempt2.remove(index + 1);
            if !check_report(attempt1) && !check_report(attempt2) {
                return false;
            }
        }
        previous = *level;
    }
    return true;
}
