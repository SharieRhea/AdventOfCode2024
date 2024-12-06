use std::collections::HashSet;
use util;

fn main() {
    let grid = util::read_grid("day06.txt");
    // find the starting point
    let mut start: (i32, i32) = (0, 0);
    'rows: for (y, row) in grid.iter().enumerate() {
        for (x, _column) in row.iter().enumerate() {
            if grid[y][x] == '^' {
                start = (x as i32, y as i32);
                break 'rows;
            }
        }
    }
    
    println!("Part 1: {}", part_1(grid.clone(), start));
    println!("Part 2: {}", part_2(grid, start));

}

fn part_1(mut grid: Vec<Vec<char>>, start: (i32, i32)) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut position = start;
    let directions: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction = 0; 

    while check_bounds(position, height, width) {
        // mark the current position
        grid[position.1 as usize][position.0  as usize] = 'X';
        let next_position = (position.0 + directions[direction].0, position.1 + directions[direction].1);
        if check_bounds(next_position, height, width) && grid[next_position.1 as usize][next_position.0 as usize] == '#' {
            // hit an obstacle, turn
            direction = (direction + 1) % 4;
        }
        else {
            position = next_position;
        }
    }

    // count the number of marked tiles
    let mut sum = 0;
    for row in grid.iter() {
        sum += row.iter().filter(|character| **character == 'X').count();
    }

    return sum;
}

fn part_2(grid: Vec<Vec<char>>, start: (i32, i32)) -> usize {
    let mut sum = 0;

    // replace every possible blank space with an obstacle and check
    // computer go brrr :)
    for (y, row) in grid.iter().enumerate() {
        for (x, _column) in row.iter().enumerate() {
            if grid[y][x] == '.' {
                let mut new_grid = grid.clone();
                new_grid[y][x] = '#';
                if check_cycle(new_grid, start) {
                    sum += 1;
                }
            }
        }
    }

    return sum;
}

fn check_cycle(grid: Vec<Vec<char>>, start: (i32, i32)) -> bool {
    let width = grid[0].len();
    let height = grid.len();

    let mut position = start;
    let directions: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction = 0; 

    // keep track of tiles we've visited and what direction we were moving
    let mut visited: HashSet<((i32, i32), usize)> = HashSet::new();

    while check_bounds(position, height, width) {
        if !visited.insert((position, direction)) {
            // we are on the same tile moving in the same direction as before
            // will always result in a cycle
            return true;
        }
        let next_position = (position.0 + directions[direction].0, position.1 + directions[direction].1);
        if check_bounds(next_position, height, width) && grid[next_position.1 as usize][next_position.0 as usize] == '#' {
            // hit an obstacle, turn
            direction = (direction + 1) % 4;
        }
        else {
            position = next_position;
        }
    }
    
    // we've exited the map, no cycle created
    return false;
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
