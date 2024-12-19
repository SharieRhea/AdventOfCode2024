use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

use util;

#[derive(Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    direction: usize,
    score: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip so that lower scores are prioritized, compare positions on a tie 
        other.score.cmp(&self.score).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct State {
    position: (usize, usize),
    direction: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct Parent {
    score: usize,
    parents: Vec<State>
}

fn main() {
    let grid = util::read_grid("day16.txt");
   
    let min_score = part_1(&grid);
    println!("Part 1: {}", min_score);
    println!("Part 2: {}", part_2(min_score, &grid));
}

fn part_1(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    // queue up nodes to be visited, use heap to visit lowest scores first
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    // keep track of already visited nodes
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    // initiate with the starting point
    let starting_node = Node { position: (1, height - 2), direction: 3, score: 0 };
    queue.push(starting_node);

    while let Some(node) = queue.pop() {
        // reached the end, return the score which will always be lowest
        if grid[node.position.1][node.position.0] == 'E' {
            return node.score;
        }

        for (direction, point) in util::get_points(node.position, height, width).iter().enumerate() {
            // if we've seen this position or this is a wall skip
            if visited.contains(point) || grid[point.1][point.0] == '#'  { continue; }
            let mut next_score = node.score;
            // add 1 for forward steps add 1000 for turns
            next_score += if node.direction == direction { 1 } else { 1001 };
            let next_node = Node { position: *point, direction, score: next_score };
            queue.push(next_node);
            visited.insert(*point);
        }
    }
    return 0;
}

fn part_2(score: usize, grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    // queue up nodes to be visited, use heap to visit lowest scores first
    let mut queue: BinaryHeap<Node> = BinaryHeap::new();
    // keep track of visited points by position AND direction
    let mut visited: HashSet<State> = HashSet::new();
    // create a map from (point, direction) -> (lowest score, parents)
    let mut parents: HashMap<State, Parent> = HashMap::new();
    let mut final_state: State = State { position: (0, 0), direction: 0 };

    // initiate with the starting point
    let starting_node = Node { position: (1, height - 2), direction: 3, score: 0 };
    queue.push(starting_node);

    while let Some(node) = queue.pop() {
        // reached the end, set final state so we know where to traverse from
        if grid[node.position.1][node.position.0] == 'E' {
            final_state = State { position: node.position, direction: node.direction };
            break;
        }

        for (direction, point) in util::get_points(node.position, height, width).iter().enumerate() {
            let mut next_score = node.score;
            // add 1 for forward steps add 1000 for turns
            next_score += if node.direction == direction { 1 } else { 1001 };

            // create the next node to visit
            let next_node = Node { position: *point, direction, score: next_score };
            let parent_state: State = State { position: node.position, direction: node.direction };

            // check if this node has already been visited
            if let Some(parent) = parents.get_mut(&State { position: *point, direction }) {
                // check if the new score is lower
                if next_score < parent.score {
                    // reset all the parent info to be for this node
                    parent.score = next_score;
                    parent.parents = vec![parent_state];
                }
                else if next_score == parent.score {
                    // add this node as an additional parent
                    parent.parents.push(parent_state);
                }
                else {
                    continue;
                }
            }
            else {
                // create parent entry for the first time
                parents.insert(State { position: *point, direction }, Parent { score: next_score, parents: vec![parent_state] });
            }

            // if score is too high or we've visited or this is a wall skip
            if next_score > score || visited.contains(&State { position: *point, direction }) || grid[point.1][point.0] == '#'  { continue; }

            queue.push(next_node);
            visited.insert(State { position: *point, direction });
        }
    }

    // now traverse the parents of the final state to determine all possible tiles
    let mut seats: HashSet<(usize, usize)> = HashSet::new();
    let mut to_check: Vec<&State> = vec![&final_state];
    while let Some(state) = to_check.pop() {
        // add this state to seen states
        seats.insert(state.position);
        // add all the parent states to the to_check list
        if let Some(parent) = parents.get(&state) {
            for new_state in &parent.parents {
                to_check.push(new_state);
            }
        }
    }
    return seats.len();
}
