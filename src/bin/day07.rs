use itertools::Itertools;
use std::iter;
use util;

fn main() {
    let lines = util::read_lines("day07.txt");

    let mut part_1 = 0;
    let mut part_2 = 0;
    
    for line in lines.iter() {
        let mut halves = line.split(": ");
        let expected: usize = halves.next().unwrap().parse().unwrap();
        let numbers: Vec<usize> = halves.next().unwrap().split(" ").into_iter().map(|number| number.parse::<usize>().unwrap()).collect();
   
        if valid_equation(expected, &numbers, 1) { part_1 += expected; }
        if valid_equation(expected, &numbers, 2) { part_2 += expected; }

    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn valid_equation(expected: usize, numbers: &Vec<usize>, part: usize) -> bool {
    let operators = if part == 1 { vec!['+', '*'] } else { vec!['+', '*', '|'] };

    // get every permutation with replacement
    // there is always one fewer operation than the number of numbers
    let permutations = iter::repeat_n(operators.into_iter(), numbers.len() - 1).multi_cartesian_product();
    for permutation in permutations {
        let mut sum = numbers[0];
        for i in 0..permutation.len() {
            sum = match permutation[i] {
                '+' => sum + numbers[i + 1],
                '*' => sum * numbers[i + 1],
                '|' => format!("{}{}", sum, numbers[i + 1]).parse().unwrap(),
                _ => sum
            };
            if sum > expected {
                // already too high, abandon this permutation
                break;
            }
        }
        if sum == expected { 
            return true; 
        } 
    }
    return false;
}
