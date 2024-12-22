use std::collections::HashMap;

use itertools::Itertools;
use util;

fn main() {
    let codes = util::read_lines("day21.txt");

    println!("Part 1: {}", part_1(&codes));
    println!("Part 2: {}", part_2(&codes));
}

fn part_1(codes: &Vec<String>) -> i64 {
    let mut complexity = 0;

    // these all get reset to the 'A' key by the end anyway so no need to clear
    let mut numeric_keypad: (i64, i64) = (2, 3);
    let mut directional_keypad1: (i64, i64) = (2, 0);
    let mut directional_keypad2: (i64, i64) = (2, 0);

    for code in codes {
        let mut numeric_path: Vec<char> = vec![];
        let mut directional1_path: Vec<char> = vec![];
        let mut directional2_path: Vec<char> = vec![];

        for digit in code.chars() {
            numeric_path.extend(find_moves(&mut numeric_keypad, get_button(digit), true));
        }
        for movement in &numeric_path {
            directional1_path.extend(find_moves(&mut directional_keypad1, get_button(*movement), false));
        }
        for movement in &directional1_path {
            directional2_path.extend(find_moves(&mut directional_keypad2, get_button(*movement), false));
        }

        // complexity is the numeric part of the code times the length of shortest path
        complexity += code[..3].parse::<i64>().unwrap() * directional2_path.len() as i64;
    }
    return complexity;
}

fn part_2(codes: &Vec<String>) -> i64 {
    let mut complexity: i64 = 0;

    let mut numeric_keypad: (i64, i64) = (2, 3);
    // cache so we don't recompute expansions we've already checked
    let mut cache: HashMap<(String, i64), i64> = HashMap::new();

    for code in codes {
        let mut path: Vec<char> = vec![];
        // do the numeric part manually
        for digit in code.chars() {
            path.extend(find_moves(&mut numeric_keypad, get_button(digit), true));
        }
        
        // 25 layers of directional robots
        let length = get_length(&path, &mut cache, 0, 25); 
        complexity += code[..3].parse::<i64>().unwrap() * length;
    }
    return complexity;
}

fn get_length(path: &Vec<char>, cache: &mut HashMap<(String, i64), i64>, depth: i64, max_depth: i64) -> i64 {
    // we've already calculated this, just reuse
    if cache.contains_key(&(path.iter().collect(), depth)) {
        return cache[&(path.iter().collect(), depth)];
    }
    // hit the max depth, this length is it
    if depth == max_depth {
        return path.len() as i64;
    }

    // need to keep going down
    let mut next_path: Vec<char> = vec![];
    let mut keypad: (i64, i64) = (2, 0);
    for character in path {
        next_path.extend(find_moves(&mut keypad, get_button(*character), false));
    }

    let mut length = 0;
    loop {
        if let Some((index, _)) = next_path.iter().find_position(|&it| *it == 'B') {
            // segment until the next 'B' press because that's when everyone gets reset back to the
            // starting position, solve in chunks essentially so that we can cache more efficiently
            let segment: Vec<char> = next_path[0..index + 1].to_vec();
            // trim that segment off the remaining path
            next_path = next_path[index + 1..].to_vec();
            length += get_length(&segment, cache, depth + 1, max_depth);
        }
        else { break; }
    }
    cache.insert((path.iter().collect(), depth), length);
    return length;
}

fn find_moves(keypad: &mut (i64, i64), goal: (i64, i64), numeric: bool) -> Vec<char> {
    let x_distance = goal.0 - keypad.0;
    let y_distance = goal.1 - keypad.1;

    let mut moves: Vec<char> = vec![];
    if x_distance < 0 {
        // moving left
        if y_distance < 0 {
            // moving up
            if numeric && keypad.1 == 3 && goal.0 == 0 {
                // moving all the way to the left from the bottom, have to move up first to avoid
                // the gap
                for _ in y_distance..0 { moves.push('^'); }
                for _ in x_distance..0 { moves.push('<'); }
            }
            else {
                for _ in x_distance..0 { moves.push('<'); }
                for _ in y_distance..0 { moves.push('^'); }
            }
        }
        else {
            // moving down
            if !numeric && keypad.1 == 0 && goal.0 == 0 && goal.1 == 1 {
                // moving all the way to bottom left from the top, so we have to move down first to
                // avoid the gap
                for _ in 0..y_distance { moves.push('v'); }
                for _ in x_distance..0 { moves.push('<'); }
            }
            else {
                for _ in x_distance..0 { moves.push('<'); }
                for _ in 0..y_distance { moves.push('v'); }
            }
        }
    }
    // must be moving right
    else if y_distance < 0 {
        // moving up
        if !numeric && keypad.0 == 0 {
            // on the bottom left of the dpad moving up so we have to move right first to avoid the
            // gap
            for _ in 0..x_distance { moves.push('>'); }
            for _ in y_distance..0 { moves.push('^'); }
        }
        else {
            for _ in y_distance..0 { moves.push('^'); }
            for _ in 0..x_distance { moves.push('>'); }
        }
    }
    else {
        // moving down
        if numeric && keypad.0 == 0 && goal.1 == 3 {
            // on the numeric keypad on the left and trying to get to bottom row,
            // have to move right first to avoid the gap
            for _ in 0..x_distance { moves.push('>'); }
            for _ in 0..y_distance { moves.push('v'); }
        }
        else {
            for _ in 0..y_distance { moves.push('v'); }
            for _ in 0..x_distance { moves.push('>'); }
        }
    }
    moves.push('B');
    // set the position to the goal's coordinates
    *keypad = goal;
    return moves;
}

fn get_button(character: char) -> (i64, i64) {
    // define static coordinates for each button on each keypad
    return match character {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        '^' => (1, 0),
        'v' => (1, 1),
        '<' => (0, 1),
        '>' => (2, 1),
        // use B for the directional A
        'B' => (2, 0),
        _ => unreachable!()
    };
}
