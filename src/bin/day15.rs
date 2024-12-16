use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("src/resources/day15.txt");
    // open the file and match result enum in case of error
    let file = match File::open(path) {
        Err(why) => panic!("Failed to open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut lines = BufReader::new(file).lines().into_iter();

    // read in the grid section first, use a hash map because why not
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut grid_2: HashMap<(i32, i32), char> = HashMap::new();
    let mut y: i32 = 0;
    let mut start: (i32, i32) = (0, 0);
    let mut start_2: (i32, i32) = (0, 0);
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() { break; }
       
        for (x, character) in line.chars().enumerate() {
            grid.insert((x as i32, y), character);
            // save the guard's starting position
            if character == '@' {
               start = (x as i32, y);
            }

            // for part 2, double width of everything
            match character {
                '@' => {
                    grid_2.insert((x as i32 * 2, y), '@');
                    grid_2.insert((x as i32 * 2 + 1, y), '.');
                    start_2 = (x as i32 * 2, y);
                },
                'O' => {
                    grid_2.insert((x as i32 * 2, y), '[');
                    grid_2.insert((x as i32 * 2 + 1, y), ']');
                },
                _ => {
                    grid_2.insert((x as i32 * 2, y), character);
                    grid_2.insert((x as i32 * 2 + 1, y), character);
                }
            }
        }
        y += 1;
    } 

    // read in the moves
    let mut moves: Vec<char> = vec![];
    while let Some(line) = lines.next() {
        for character in line.unwrap().chars() {
            moves.push(character);
        }
    }

    println!("Part 1: {}", part_1(start, &mut grid, &moves));
    println!("Part 2: {}", part_2(start_2, &mut grid_2, &moves));
}

fn part_1(start: (i32, i32), grid: &mut HashMap<(i32, i32), char>, moves: &Vec<char>) -> i32 {
    process(start, grid, moves); 
    
    // calculate the sum of box GPS coords
    let mut sum = 0;
    for (position, value) in grid.iter() {
        if *value == 'O' {
           sum += 100 * position.1 + position.0; 
        }
    }
    return sum;
}

fn part_2(start: (i32, i32), grid: &mut HashMap<(i32, i32), char>, moves: &Vec<char>) -> i32 {
    process(start, grid, moves); 
    
    // calculate the sum of box GPS coords
    let mut sum = 0;
    for (position, value) in grid.iter() {
        if *value == '[' {
            sum += 100 * position.1 + position.0; 
        }
    }
    return sum;
}

fn process(start: (i32, i32), grid: &mut HashMap<(i32, i32), char>, moves: &Vec<char>) {
    let mut position = start;
    for direction in moves {
        if check_space(position, *direction, grid) {
            position = move_stack(position, *direction, grid);
        }
    }
}

fn check_space(position: (i32, i32), direction: char, grid: &mut HashMap<(i32, i32), char>) -> bool {
    let vector = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid move found!")
    };

    let mut current_position = position;
    // traverse in the given direction until we find a blank space or hit edge
    loop {
        current_position = add_vector(current_position, vector);
        match grid.get(&current_position) {
            // found space
            Some(character) if *character == '.' => { 
                return true;
            },
            // wall, no room to move
            Some(character) if *character == '#' => { return false; },
            // left side of a box, need to check for the right side if moving up or down
            Some(character) if *character == '[' => {
                if direction == '^' || direction == 'v' {
                    if !check_space(add_vector(current_position, (1, 0)), direction, grid) {
                        return false;
                    }
                }
            }
            // right side of a box, need to check for the right side if moving up or down
            Some(character) if *character == ']' => {
                if direction == '^' || direction == 'v' {
                    if !check_space(add_vector(current_position, (-1, 0)), direction, grid) {
                        return false;
                    }
                }
            }
            // keep checking
            Some(_) => {},
            // went of the grid, no space found
            None => { return false; }
        }
    }
}

fn move_stack(position: (i32, i32), direction: char, grid: &mut HashMap<(i32, i32), char>) -> (i32, i32) {
    let vector = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid move found!")
    };

    let mut current_position = position;
    // store the next tile into a temp so we don't overwrite it
    let mut temporary = *grid.get(&current_position).unwrap();
    // the character to fill the next tile with
    let mut to_fill = '.';
    loop {
        grid.insert(current_position, to_fill);
        if temporary == '.' {
            // we've reached the space that was needed for this move, stop updating tiles
            break;
        }
        else if current_position != position && temporary == '[' && (direction == '^' || direction == 'v') {
            // also need to move the right side
            move_stack(add_vector(current_position, (1, 0)), direction, grid);
        }
        else if current_position != position && temporary == ']' && (direction == '^' || direction == 'v') {
            // also need to move the left side
            move_stack(add_vector(current_position, (-1, 0)), direction, grid);
        }
        to_fill = temporary;
        current_position = add_vector(current_position, vector);
        temporary = *grid.get(&current_position).unwrap();
    }
    // return the new position of the guard
    return add_vector(position, vector);
}

fn add_vector(position: (i32, i32), vector: (i32, i32)) -> (i32, i32) {
    (position.0 + vector.0, position.1 + vector.1)
}
