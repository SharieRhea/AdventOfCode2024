use std::collections::HashSet;
use std::thread;
use std::sync::mpsc;
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
   
    let visited = part_1(grid.clone(), start);
    println!("Part 1: {}", visited.len());
    println!("Part 2: {}", part_2(grid, start, visited));

}

fn part_1(grid: Vec<Vec<char>>, start: (i32, i32)) -> HashSet<(usize, usize)> {
    let width = grid[0].len();
    let height = grid.len();

    let mut position = start;
    let directions: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while check_bounds(position, height, width) {
        visited.insert((position.0 as usize, position.1 as usize));
        let next_position = (position.0 + directions[direction].0, position.1 + directions[direction].1);
        if check_bounds(next_position, height, width) && grid[next_position.1 as usize][next_position.0 as usize] == '#' {
            // hit an obstacle, turn
            direction = (direction + 1) % 4;
        }
        else {
            position = next_position;
        }
    }

    return visited;
}

fn part_2(grid: Vec<Vec<char>>, start: (i32, i32), visited: HashSet<(usize, usize)>) -> usize {
    let mut sum = 0;
    let thread_count = 100;
    // the number of obstacles that each thread will tackle
    let size = visited.len() / thread_count;
    let mut to_check: Vec<(usize, usize)> = visited.into_iter().collect();

    // set up send and receive channel for message passing
    let (tx, rx) = mpsc::channel();

    for thread_id in 0..thread_count {
        // clone these so ownership isn't moved into the thread
        let thread_grid = grid.clone();
        let thread_tx = tx.clone();

        // remove a segment from the main list with length size (or shorter if  this is the last
        // thread
        let segment: Vec<(usize, usize)> = if thread_id == thread_count - 1 {
            to_check.clone()
        }
        else { 
            to_check.drain(0..size).collect()
        };

        thread::spawn(move || {
            thread_tx.send(check_obstacles(thread_grid, start, segment)).unwrap();
        });
    }

    for _thread_id in 0..thread_count {
        // recv is blocking so this will receive from any thread that has finished until they have
        // all sent something back
        // this means joining on join handles is unnecessary
        sum += rx.recv().unwrap();
    }

    return sum;
}

fn check_obstacles(grid: Vec<Vec<char>>, start: (i32, i32), visited: Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    // replace each tile that was visited with an obstacle to see if it creates a cycle
    // computer go brrr :)
    for (x, y) in visited.iter() {
        if grid[*y][*x] == '.' {
            let mut new_grid = grid.clone();
            new_grid[*y][*x] = '#';
            if check_cycle(new_grid, start) {
                sum += 1;
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
