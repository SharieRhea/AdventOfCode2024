use std::collections::{HashMap, HashSet};

use util;

fn main() {
    let grid = util::read_grid("day12.txt");

    let regions = process_regions(&grid);
    println!("Part 1: {:?}", part_1(&regions));
    println!("Part 2: {:?}", part_2(&regions));
}

fn part_1(regions: &HashMap<(usize, usize), (usize, usize, usize)>) -> usize {
    // calculate cost with area * perimeter for each region
    let mut cost = 0;
    for region in regions.values() {
        cost += region.0 * region.1;
    }
    return cost;
}

fn part_2(regions: &HashMap<(usize, usize), (usize, usize, usize)>) -> usize {
    // calculate cost with area * sides for each region 
    let mut cost = 0;
    for region in regions.values() {
        cost += region.0 * region.2;
    }
    return cost;
}

fn process_regions(grid: &Vec<Vec<char>>) -> HashMap<(usize, usize), (usize, usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    // this keeps track of every tile visited so we know where to start the next flood fill
    let mut master_visited: HashSet<(usize, usize)> = HashSet::new();
    // map for different regions: start tile -> (area, perimeter, sides)
    let mut regions: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            // we've seen this tile, move on
            if master_visited.contains(&(x, y)) { continue; } 

            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            // start a flood fill for this plot
            flood_fill(grid[y][x], (x, y), grid, &mut visited, height, width);
            let area = visited.len();
            let perimeter = calculate_perimeter(&visited, width, height);
            let sides = calculate_sides(grid[y][x], grid, &visited, width, height);

            // add to the map if this is plot type that's already been seen, otherwise add it
            regions.insert((x, y), (area, perimeter, sides));

            // finally, update master_visited to include the tiles from this flood fill
            master_visited.extend(visited);
        }
    }
    return regions;
}

fn flood_fill(
    letter: char, 
    start: (usize, usize),
    grid: &Vec<Vec<char>>, 
    visited: &mut HashSet<(usize, usize)>,
    height: usize, 
    width: usize
) {
    // make sure we don't ever recount this tile
    visited.insert(start);
    let new_points: Vec<(usize, usize)> = get_points(start, height, width).into_iter()
        // points are only new if we haven't visited them yet and they have the right value
        .filter(|coords| !visited.contains(coords) && grid[coords.1][coords.0] == letter).collect();

    // no more points to visit
    if new_points.is_empty() {
        return;
    }

    for point in new_points {
        // recursive call for each new point to visit
        flood_fill(letter, point, grid, visited, height, width);
    } 
}

fn calculate_perimeter(visited: &HashSet<(usize, usize)>, width: usize, height: usize) -> usize {
    let mut perimeter = 0;
    for point in visited {
        // contributing perimeter for this tile is 4 - number of adjacent tiles in the region
        perimeter += 4 - get_points(*point, width, height).into_iter().filter(|point| visited.contains(point)).count();
    }
    return perimeter;
}

fn calculate_sides(letter: char, grid: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>, width: usize, height: usize) -> usize {
    // number of sides is actually number of corners, so look for that
    let mut sides = 0;
    for point in visited {
        let letters: Vec<char> = get_sides_points(*point).into_iter().map(|it| 
            if check_bounds(it, height, width) {
                grid[it.1 as usize][it.0 as usize]
            }
            // use a nonsense character for anything out of bounds
            else { '!' }
            ).collect();

        // if 3 in a row vertical or horizontal then no corners at all
        if letters[0] == letter && letters[4] == letter && letters[2] != letter && letters[6] != letter { continue; }
        if letters[2] == letter && letters[6] == letter && letters[0] != letter && letters[4] != letter { continue; }

        // check interior corners
        // upper left, upper right, lower left, lower right
        if letters[0] == letter && letters[6] == letter && letters[7] != letter { sides += 1; }
        if letters[0] == letter && letters[2] == letter && letters[1] != letter { sides += 1; }
        if letters[4] == letter && letters[6] == letter && letters[5] != letter { sides += 1; }
        if letters[4] == letter && letters[2] == letter && letters[3] != letter { sides += 1; }

        // check exterior corners
        // upper left, upper right, lower left, lower right
        if letters[0] != letter && letters[6] != letter { sides += 1; }
        if letters[0] != letter && letters[2] != letter { sides += 1; }
        if letters[4] != letter && letters[6] != letter { sides += 1; }
        if letters[4] != letter && letters[2] != letter { sides += 1; }
    }
    return sides;
}

fn get_points(point: (usize, usize), height: usize, width: usize) -> Vec<(usize, usize)> {
    // get points going [up, down, left, right]
    let list = vec![
        (point.0 as i32, point.1 as i32 - 1), (point.0 as i32, point.1 as i32 + 1), (point.0 as i32 - 1, point.1 as i32), (point.0 as i32 + 1, point.1 as i32)
    ];
    // make sure each point is actually on the grid and convert to usize
    list.into_iter().filter(|it| check_bounds(*it, height, width)).map(|it| (it.0 as usize, it.1 as usize)).collect()
}

fn get_sides_points(point: (usize, usize)) -> [(i32, i32); 8] {
    // get points going [north, northeast, east, southeast, south, southwest, west, northwest]
    [
        (point.0 as i32, point.1 as i32 - 1), (point.0 as i32 + 1, point.1 as i32 - 1),
        (point.0 as i32 + 1, point.1 as i32), (point.0 as i32 + 1, point.1 as i32 + 1),
        (point.0 as i32, point.1 as i32 + 1), (point.0 as i32 - 1, point.1 as i32 + 1),
        (point.0 as i32 - 1, point.1 as i32), (point.0 as i32 - 1, point.1 as i32 - 1)
    ]
}

fn check_bounds(point: (i32, i32), height: usize, width: usize) -> bool {
    point.0 >= 0 && point.0 < width as i32 && point.1 >= 0 && point.1 < height as i32
}
