use std::collections::{HashSet, VecDeque};

use util;

#[derive(Debug)]
struct Node {
    position: (usize, usize),
    direction: usize,
    score: usize,
    path: Vec<(usize, usize)>
}

fn main() {
    let grid = util::read_grid("test.txt");
   
    let score = part_1(&grid);
    println!("Part 1: {}", part_1(&grid));
    println!("Part 2: {}", part_2(score, &grid));
}

fn part_1(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    // queue up nodes to be visited
    let mut queue: VecDeque<Node> = VecDeque::new();
    // keep track of already visited nodes
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // initiate with the starting point
    queue.push_back(Node { position: (1, height - 2), direction: 3, score: 0, path: vec![] });

    while let Some(node) = queue.pop_front() {
        if grid[node.position.1][node.position.0] == 'E' {
            return node.score;
        }
        for (direction, point) in util::get_points(node.position, height, width).iter().enumerate() {
            if visited.contains(point) || grid[point.1][point.0] == '#'  { continue; }
            // prefer going in the same direction rather than turning
            if node.direction == direction {
                queue.push_front(Node { position: *point, direction, score: node.score + 1, path: vec![] });
            } 
            else {
                queue.push_back(Node { position: *point, direction, score: node.score + 1001, path: vec![] });
            }
            visited.insert(*point);
        }
    }
    return 0;
}

// TODO: seems to loop infinitely for some inputs, or at the very least take forever
// works for the two examples but not real input
fn part_2(score: usize, grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    // queue up nodes to be visited
    let mut queue: VecDeque<Node> = VecDeque::new();
    // keep track of already visited nodes
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut seats: HashSet<(usize, usize)> = HashSet::new();

    // initiate with the starting point
    queue.push_back(Node { position: (1, height - 2), direction: 3, score: 0, path: vec![(1, height -2)] });

    while let Some(node) = queue.pop_front() {
        if grid[node.position.1][node.position.0] == 'E' && node.score == score {
            println!("{:?}", node.path);
            println!("{}", node.score);
            seats.extend(node.path.clone());
            continue;
        }
        for (direction, point) in util::get_points(node.position, height, width).iter().enumerate() {
            if node.path.contains(point) || grid[point.1][point.0] == '#'  { continue; }
            let mut path = node.path.clone();
            path.push(*point);
            // prefer going in the same direction rather than turning
            if node.direction == direction {
                queue.push_front(Node { position: *point, direction, score: node.score + 1, path });
            } 
            else {
                queue.push_back(Node { position: *point, direction, score: node.score + 1001, path });
            }
            visited.insert(*point);
        }
    }
    return seats.len();
}
