use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

use util;

#[derive(Eq, PartialEq)]
struct State {
    position: (usize, usize),
    cost: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // flip so that lower costs are prioritized, compare positions on a tie 
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut lines = util::read_lines("day18.txt").into_iter();
    let mut corrupted_bytes: Vec<(usize, usize)> = vec![];
    while let Some(line) = lines.next() {
        let (x, y) = line.split_once(",").unwrap();
        corrupted_bytes.push((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    }

    println!("Part 1: {}", part_1(corrupted_bytes[0..1024].iter().collect()));
    let coordinate = part_2(corrupted_bytes);
    println!("Part 2: {},{}", coordinate.0, coordinate.1);
}

fn part_1(corrupted_bytes: HashSet<&(usize, usize)>) -> usize {
    // more pathfinding!
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    // grid dimensions are given
    let height = 71;
    let width = 71;
    // start in top left corner
    queue.push(State { position: (0, 0), cost: 0 });
    visited.insert((0, 0));

    while let Some(state) = queue.pop() {
        // goal tile is bottom right tile
        if state.position == (width - 1, height - 1) {
           return state.cost; 
        }

        for point in util::get_points(state.position, height, width) {
            if corrupted_bytes.contains(&point) || visited.contains(&point) { continue; }
            queue.push(State { position: point, cost: state.cost + 1 });
            visited.insert(point);
        }
    }
    return 0;
}

fn part_2(corrupted_bytes: Vec<(usize, usize)>) -> (usize, usize) {
    // start from the end and work backwards
    let mut nanosecond = corrupted_bytes.len() - 1;
    while part_1(corrupted_bytes[0..nanosecond].iter().collect()) == 0 {
        nanosecond -= 1;
    }
    return corrupted_bytes[nanosecond];
}
