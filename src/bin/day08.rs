use core::f32;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // don't use the util because I want to keep track of symbols as they are seen 
    let path = Path::new("src/resources/day08.txt");
    // open the file and match result enum in case of error
    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    
    
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        height += 1;
        if y == 0 {
            width = line.len();
        }
        for (x, character) in line.chars().enumerate() {
            if character != '.' && antennas.contains_key(&character) {
                antennas.get_mut(&character).unwrap().push((x as i32, y as i32));
            }
            else {
               antennas.insert(character, vec![(x as i32, y as i32)]); 
            }
        }
    }

    println!("Part 1: {}", part_1(&antennas, width, height));
    println!("Part 2: {}", part_2(&antennas, width, height));
}

fn part_1(antennas: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    // use a HashSet for antinodes because we want unique number
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // for every antenna type
    for locations in antennas.values() {
        for antenna_1 in 0..locations.len() - 1 {
            for antenna_2 in antenna_1 + 1..locations.len() {
                // for every unique combination of two antennas
                let (x_1, y_1) = locations[antenna_1];
                let (x_2, y_2) = locations[antenna_2];
                let slope: f32 = (y_2 - y_1) as f32 / (x_2 - x_1) as f32;
                // insert both antinodes in opposite directions
                let x_distance = x_2 - x_1;
                antinodes.insert((x_1 - x_distance, y_1 - (slope * x_distance as f32) as i32));
                antinodes.insert((x_2 + x_distance, y_2 + (slope * x_distance as f32) as i32));
            }
        }
    }

    // make sure antinodes are in the map boundaries
    return antinodes.into_iter().filter(|it| check_bounds(*it, height, width)).count();
}

fn part_2(antennas: &HashMap<char, Vec<(i32, i32)>>, width: usize, height: usize) -> usize {
    // use a HashSet for antinodes because we want unique number
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    // for every antenna type
    for locations in antennas.values() {
        for antenna_1 in 0..locations.len() - 1 {
            for antenna_2 in antenna_1 + 1..locations.len() {
                // for every unique combination of two antennas
                let (x_1, y_1) = locations[antenna_1];
                let (x_2, y_2) = locations[antenna_2];
                let slope: f32 = (y_2 - y_1) as f32 / (x_2 - x_1) as f32;
                let x_distance = x_2 - x_1;

                // insert antinodes going left until we run off the map
                let mut multiplier: i32 = 0;
                loop {
                    let antinode = (x_1 -  x_distance * multiplier, y_1 - (slope * (x_distance * multiplier) as f32) as i32); 
                    if check_bounds(antinode, height, width) { 
                        antinodes.insert(antinode); 
                        multiplier += 1;
                    } 
                    else { 
                        break; 
                    }
                } 
                // insert antinodes going right until we run off the map
                multiplier = 1;
                loop {
                    let antinode = (x_1 + x_distance * multiplier, y_1 + (slope * (x_distance * multiplier) as f32) as i32); 
                    if check_bounds(antinode, height, width) { 
                        antinodes.insert(antinode); 
                        multiplier += 1;
                    } 
                    else { 
                        break; 
                    }
               }
            }
        }
    }
    return antinodes.len();
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
