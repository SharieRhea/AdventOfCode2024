use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    // don't use the util because I want to keep track of X's and A's that are seen
    let path = Path::new("src/resources/day04.txt");
    // open the file and match result enum in case of error
    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = vec![];
    let mut x_list: Vec<(usize, usize)> = vec![];
    let mut a_list: Vec<(usize, usize)> = vec![];
    
    for (y, line) in reader.lines().enumerate() {
        let mut row: Vec<char> = vec![];
        for (x, character) in line.unwrap().chars().enumerate() {
            row.push(character);
            // keep track of X's and A's to avoid interating over entire grid
            if character == 'X' {
               x_list.push((x, y)); 
            }
            else if character == 'A' {
                a_list.push((x, y));
            }
        }
        grid.push(row);
    }

    println!("Part 1: {}", part_1(x_list, grid.clone()));
    println!("Part 2: {}", part_2(a_list, grid));
}

fn part_1(x_list: Vec<(usize, usize)>, grid: Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    // define all possible directions from the X
    let directions: [(i32, i32); 8] = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)];
    let mut sum: i32 = 0;

    for point in x_list.iter() {
        for direction in directions.iter() {
            if let Some(result) = get_points_in_line(*point, *direction, height, width) {
                // rest of the letters must spell "MAS" in order
                if grid[result[0].1][result[0].0] == 'M' 
                && grid[result[1].1][result[1].0] == 'A' 
                && grid[result[2].1][result[2].0] == 'S' {
                    sum += 1;
                }
            };
        }
    }
    return sum;
}

fn part_2(a_list: Vec<(usize, usize)>, grid: Vec<Vec<char>>) -> i32 {
    let height = grid.len();
    let width = grid[0].len();
    let mut sum: i32 = 0;

    for point in a_list.iter() {
        if let Some(result) = get_points_in_x(*point, height, width) {
            // "MAS" can be forwards or backwords when forming the 'X'
            // check \ diagonal first
            if ((grid[result[0].1][result[0].0] == 'M' && grid[result[3].1][result[3].0] == 'S')  ||
            (grid[result[0].1][result[0].0] == 'S' && grid[result[3].1][result[3].0] == 'M'))
            &&
            // check / diagonal next
            ((grid[result[1].1][result[1].0] == 'M' && grid[result[2].1][result[2].0] == 'S')  ||
            (grid[result[1].1][result[1].0] == 'S' && grid[result[2].1][result[2].0] == 'M')) { 
                sum += 1;
            };
        }
    }
    return sum;
}

fn get_points_in_line(point: (usize, usize), direction: (i32, i32), height: usize, width: usize) -> Option<[(usize, usize); 3]> {
    // given the position of an X and the direction to go in, get the next 3 points in the line
    let m_point: (i32, i32) = (point.0 as i32 + direction.0, point.1 as i32 + direction.1);
    let a_point: (i32, i32) = (m_point.0 + direction.0, m_point.1 + direction.1);
    let s_point: (i32, i32) = (a_point.0 + direction.0, a_point.1 + direction.1);

    let list = [m_point, a_point, s_point];
    // make sure each point is actually on the grid
    for item in list.iter() {
        if !check_bounds(*item, height, width) {
            return None;
        }
    }
    // convert to usize now that bounds have been checked and return
    Some(list.map(|it| (it.0 as usize, it.1 as usize)))
}

fn get_points_in_x(point: (usize, usize), height: usize, width: usize) -> Option<[(usize, usize); 4]> {
    // given the position of an A, get the points in the corners that form an 'X' shape 
    let upper_left: (i32, i32) = (point.0 as i32 - 1, point.1 as i32 + 1);
    let upper_right: (i32, i32) = (point.0 as i32 + 1, point.1 as i32 + 1);
    let lower_left: (i32, i32) = (point.0 as i32 - 1, point.1 as i32 - 1);
    let lower_right: (i32, i32) = (point.0 as i32 + 1, point.1 as i32 - 1);

    let list = [upper_left, upper_right, lower_left, lower_right];
    // make sure each point is actually on the grid
    for item in list.iter() {
        if !check_bounds(*item, height, width) {
            return None;
        }
    }
    // convert to usize now that bounds have been checked and return
    Some(list.map(|it| (it.0 as usize, it.1 as usize)))
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
