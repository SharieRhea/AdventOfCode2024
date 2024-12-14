use std::usize::MAX;

use util;

#[derive(Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let lines = util::read_lines("day14.txt");

    let mut robots: Vec<Robot> = vec![];
    for line in lines {
        // create each robot
        let (left_side, right_side) = line.split_once(" ").unwrap();
        let (x_position_string, y_position_string) = left_side[2..].split_once(",").unwrap();
        let (x_velocity_string, y_velocity_string) = right_side[2..].split_once(",").unwrap();
        let x_position: i32 = x_position_string.parse().unwrap();
        let y_position: i32 = y_position_string.parse().unwrap();
        let x_velocity: i32 = x_velocity_string.parse().unwrap();
        let y_velocity: i32 = y_velocity_string.parse().unwrap();
        robots.push(Robot { position: (x_position, y_position), velocity: (x_velocity, y_velocity) });        
    }
    
    println!("Part 1: {}", part_1(&mut robots.clone(), 100));
    println!("Part 2: {}", part_2(&mut robots));
}

fn part_1(robots: &mut Vec<Robot>, seconds: usize) -> usize {
    // move each robot for the given number of seconds 
    for _ in 0..seconds {
        for index in 0..robots.len() {
            move_robot(&mut robots[index]);
        }
    }

    return calculate_safety(robots);
}

fn part_2(robots: &mut Vec<Robot>) -> usize {
    // look through every second and see when the safety factor is the lowest
    // the rationale behind this is that to form the christmas tree most robots must be together in
    // the same quadrant, therefore lowering the safety factor
    let seconds = 10000;
    let mut lowest: usize = MAX;
    let mut second_seen = 0;

    // start from 1 second since the first move happens on 1 not 0
    for second in 1..seconds {
        for index in 0..robots.len() {
            move_robot(&mut robots[index]);
        }
        let safety = calculate_safety(robots);
        if safety < lowest {
            lowest = safety;
            second_seen = second;
        }
    }

    return second_seen;
}

fn move_robot(robot: &mut Robot) {
    robot.position.0 = robot.position.0 + robot.velocity.0;
    robot.position.1 = robot.position.1 + robot.velocity.1;

    // ensure that robots wrap around to other side of grid
    if robot.position.0 >= WIDTH {
        robot.position.0 = robot.position.0 - WIDTH ;
    }
    else if robot.position.0 < 0 {
        robot.position.0 = WIDTH + robot.position.0;
    }
    if robot.position.1 >= HEIGHT {
        robot.position.1 = robot.position.1 - HEIGHT;
    }
    else if robot.position.1 < 0 {
        robot.position.1 = HEIGHT + robot.position.1;
    }
}

fn calculate_safety(robots: &Vec<Robot>) -> usize {
    let mut quadrants: [usize; 4] = [0, 0, 0, 0];
    let x_midline: i32 = HEIGHT / 2;
    let y_midline: i32 = WIDTH / 2;
    for robot in robots.iter() {
        // left side
        if robot.position.0 < y_midline {
            if robot.position.1 < x_midline {
                quadrants[0] += 1;
            }
            else if robot.position.1 > x_midline {
                quadrants[2] += 1;
            }
        } 
        // right side
        else if robot.position.0 > y_midline {
            if robot.position.1 < x_midline {
                quadrants[1] += 1;
            }
            else if robot.position.1 > x_midline {
                quadrants[3] += 1;
            }
        }
    }
    return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
}
