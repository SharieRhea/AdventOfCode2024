use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines(filename: &str) -> Vec<String> {
    let reader = get_reader(filename);
    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        // panic if there is an error reading the line
        lines.push(line.unwrap()); 
    }
    return lines;
}

pub fn read_file(filename: &str) -> String {
    let reader = get_reader(filename);
    let mut input: String = "".to_owned();
    for line in reader.lines() {
        // panic if there is an error reading the line
        input.push_str(&line.unwrap()); 
    }
    return input;
}

pub fn read_grid(filename: &str ) -> Vec<Vec<char>> {
    let reader = get_reader(filename);
    let mut grid: Vec<Vec<char>> = vec![];
    for line in reader.lines() {
        let mut row: Vec<char> = vec![];
        for character in line.unwrap().chars() {
            row.push(character);
        }
        grid.push(row);
    }
    return grid;
}

fn get_reader(filename: &str) -> BufReader<File> {
    let path_string: &str = &format!("src/resources/{filename}");
    let path = Path::new(path_string);
    // open the file and match result enum in case of error
    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    return BufReader::new(file);
}

pub fn get_points(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
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
