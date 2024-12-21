use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use util;

fn main() {
    // don't use the util because of start and end positions
    let path = Path::new("src/resources/day20.txt");
    // open the file and match result enum in case of error
    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];
    let mut start: (usize, usize) = (0, 0);
    
    for (y, line) in reader.lines().enumerate() {
        let characters = line.unwrap();
        // if start hasn't been found yet, check the line
        if start == (0, 0) {
            if let Some(x) = characters.clone().find(|it| it == 'S') {
                start = (x, y);
            }
        }
        grid.push(characters.chars().collect());
    }

    let height = grid.len();
    let width = grid[0].len();

    let mut scores: HashMap<(usize, usize), i32> = HashMap::new();
    find_base_scores(&grid, &mut scores, start, height, width); 
    println!("Part 1: {}", part_1(&mut grid, &mut scores, height, width));
    println!("Part 2: {}", part_2(&mut grid, &mut scores, height, width));
}

fn part_1(grid: &mut Vec<Vec<char>>, scores: &HashMap<(usize, usize), i32>, height: usize, width: usize) -> usize {
    // for every possible "cheat" (phase through walls for 2 seconds) compute how much time it
    // saves by calculating destination score - starting score
    let mut good_cheats = 0;
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)]; 
    for (x, y) in scores.keys() {
        for direction in directions {
            let destination1_int = (*x as i32 + direction.0, *y as i32 + direction.1);
            let destination2_int = (destination1_int.0 + direction.0, destination1_int.1 + direction.1); 
            if !util::check_bounds(destination1_int, height, width) || !util::check_bounds(destination2_int, height, width) { 
                // this cheat takes us out of bounds, skip
                continue; 
            }
            let destination1 = (destination1_int.0 as usize, destination1_int.1 as usize);
            let destination2 = (destination2_int.0 as usize, destination2_int.1 as usize);
            if grid[destination1.1][destination1.0] != '#' {
                // this cheat isn't actually a cheat
                continue;
            }
            // if we land on a valid tile
            if scores.contains_key(&destination2) {
                let initial_score = scores[&(*x, *y)];
                let destination_score = scores[&destination2];
                // remember to account for the 2 steps taken during the cheat
                if destination_score > initial_score && destination_score - initial_score - 2 >= 100 { 
                    good_cheats += 1; 
                }
            }
        }
    } 
    return good_cheats;
}

fn part_2(grid: &mut Vec<Vec<char>>, scores: &HashMap<(usize, usize), i32>, height: usize, width: usize) -> usize {
    // this time we need to consider all points in a 20 second radius
    let mut good_cheats = 0;
    for point in scores.keys() {
        for (destination, distance) in get_tiles(*point, height, width) {
            if grid[destination.1][destination.0] == '#' {
                // this cheat lands us in a wall
                continue;
            }
            // if we land on a valid tile
            if scores.contains_key(&destination) {
                let initial_score = scores[point];
                let destination_score = scores[&destination];
                // remember to account for the steps taken during the cheat
                if destination_score > initial_score && destination_score - initial_score - distance >= 100 { 
                    good_cheats += 1; 
                }
            }
        }
    } 

    return good_cheats;
}

fn find_base_scores(grid: &Vec<Vec<char>>, scores: &mut HashMap<(usize, usize), i32>, start: (usize, usize), height: usize, width: usize) {
    // first, do a normal path find to figure out how long it takes to get to each tile on the grid
    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    scores.insert(start, 0);
    queue.push(start);

    // pathfind
    while let Some(position) = queue.pop() {

        for point in util::get_points(position, height, width) {
            // skip walls and tiles we've already been on
            if scores.contains_key(&point) || grid[point.1][point.0] == '#' { continue; }
            scores.insert(point, scores[&position] + 1);
            queue.push(point);
        }
    }
}

fn get_tiles(point: (usize, usize), height: usize, width: usize) -> Vec<((usize, usize), i32)> {
    let mut list: HashSet<((i32, i32), i32)> = HashSet::new();
    // get all unique tiles and their distance within a 20 tile distance
    for x_distance in 0..=20 {
        for y_distance in 0..=20 {
            if x_distance + y_distance > 20 || x_distance + y_distance < 1 { continue; }
            list.insert(((point.0 as i32 + x_distance, point.1 as i32 + y_distance), x_distance + y_distance));
            list.insert(((point.0 as i32 - x_distance, point.1 as i32 + y_distance), x_distance + y_distance));
            list.insert(((point.0 as i32 + x_distance, point.1 as i32 - y_distance), x_distance + y_distance));
            list.insert(((point.0 as i32 - x_distance, point.1 as i32 - y_distance), x_distance + y_distance));
        }
    }
    // make sure each point is actually on the grid and convert to usize
    list.into_iter().filter(|it| util::check_bounds(it.0, height, width)).map(|it| ((it.0.0 as usize, it.0.1 as usize), it.1)).collect()
}
