use std::collections::{HashMap, HashSet};

use util;

fn main() {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut to_check: HashSet<String> = HashSet::new();
    for line in util::read_lines("day23.txt") {
        let (left, right) = line.split_once("-").unwrap(); 
        graph.entry(left.to_owned()).and_modify(|neighbors| neighbors.push(right.to_owned())).or_insert(vec![right.to_owned()]);
        graph.entry(right.to_owned()).and_modify(|neighbors| neighbors.push(left.to_owned())).or_insert(vec![left.to_owned()]);
        if left[0..1] == *"t" {
            to_check.insert(left.to_owned()); 
        }
    }

    println!("Part 1: {}", part_1(&graph, &to_check));
    println!("Part 2: {}", part_2(&graph).join(","));
}

fn part_1(graph: &HashMap<String, Vec<String>>, to_check: &HashSet<String>) -> usize {
    // keep track of computers starting with t that we've handled, this prevents double counting if
    // a set of three contains two or more computers starting with t
    let mut checked: HashSet<String> = HashSet::new();
    let mut sum = 0;
    for computer in to_check.iter() {
        let connections = &graph[computer];
        for one in 0..connections.len() - 1 {
            if checked.contains(&connections[one]) { continue; }
            for two in one + 1..connections.len() {
                if checked.contains(&connections[two]) { continue; }
                if graph[&connections[one]].contains(&connections[two]) {
                    checked.insert(computer.to_owned());
                    sum += 1;
                }
            }
        } 
    }
    return sum;
}

fn part_2(graph: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut best_network: HashSet<&String> = HashSet::new();

    for computer in graph.keys() {
        // find this computer's largest network
        let mut best_inner_network: HashSet<&String> = HashSet::new();
        let connections = &graph[computer];

        // if number of connections is lower than our current best network this can't be any better
        if connections.len() < best_network.len() { continue; }

        for connection in connections {
            // track the network formed with this connection
            let mut inner_network: HashSet<&String> = HashSet::new();
            inner_network.insert(computer);
            inner_network.insert(connection);

            // if additional less than current best inner network this can't be any better
            if connections.len() - 2 < best_inner_network.len() { continue; }

            // see if any others can be added to the network
            'add_connection: for additional in connections {
                for existing in &inner_network {
                    if !graph[existing.to_owned()].contains(additional) { continue 'add_connection; }
                }
                inner_network.insert(additional);
            }
            
            // check if this new network beats the existing inner one
            if inner_network.len() > best_inner_network.len() {
                best_inner_network = inner_network;
            }
        }
        // check if this computer's best network beats the current overall best
        if best_inner_network.len() > best_network.len() {
            best_network = best_inner_network;
        }
    }
    
    let mut return_value: Vec<String> = best_network.into_iter().map(|it| it.to_owned()).collect();
    return_value.sort();
    return return_value;
}
