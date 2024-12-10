use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // don't use the util because I want to keep track of 0's that are seen
    let path = Path::new("src/resources/day10.txt");
    // open the file and match result enum in case of error
    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<i32>> = vec![];
    let mut trailheads: Vec<(usize, usize)> = vec![];
    
    for (y, line) in reader.lines().enumerate() {
        let mut row: Vec<i32> = vec![];
        for (x, character) in line.unwrap().chars().enumerate() {
            row.push(character.to_digit(10).unwrap() as i32);
            if character == '0' {
               trailheads.push((x, y)); 
            }
        }
        grid.push(row);
    }

    let height = grid.len();
    let width = grid[0].len();

    let (part_1, part_2) = process(&grid, &trailheads, width, height);
    println!("Part 1: {}", part_1); 
    println!("Part 2: {}", part_2); 
}

fn process(grid: &Vec<Vec<i32>>, trailheads: &Vec<(usize, usize)>, width: usize, height: usize) -> (usize, usize) {
    let mut part_1 = 0;
    let mut part_2 = 0;
    for trailhead in trailheads {
        let mut set: HashSet<(usize, usize)> = HashSet::new();
        part_2 += find_hikes(&grid, &mut set, *trailhead, 0, width, height); 
        part_1 += set.len();
    }
    return (part_1, part_2);
}

fn find_hikes(grid: &Vec<Vec<i32>>, set: &mut HashSet<(usize, usize)>, location: (usize, usize), elevation: i32, width: usize, height: usize) -> usize {
    // set is used for part 1 to eliminate different paths to the same 9
    if elevation == 9 {
        set.insert(location);
        return 1;
    }
    else {
        let points = get_points(location, height, width);
        let mut hikes = 0;
        for point in points {
            if grid[point.1][point.0] == elevation + 1 {
                hikes += find_hikes(grid, set, point, elevation + 1, width, height);
            }
        }
        return hikes;
    }
}

fn get_points(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    // get points going [up, down, left, right]
    let list = vec![
        (point.0 as i32, point.1 as i32 - 1), (point.0 as i32, point.1 as i32 + 1), (point.0 as i32 - 1, point.1 as i32), (point.0 as i32 + 1, point.1 as i32)
    ];
    // make sure each point is actually on the grid and convert to usize
    list.into_iter().filter(|it| check_bounds(*it, height, width)).map(|it| (it.0 as usize, it.1 as usize)).collect()
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
