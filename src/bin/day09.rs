use std::collections::BinaryHeap;

use itertools::Itertools;
use util;

fn main() {
    // NOTE: this is trash code but it's 1AM so I will come back and fix it tomorrow

    let line = util::read_file("day09.txt");

    println!("Part 1: {}", part_1(&line));
    println!("Part 2: {}", part_2(&line));
}


fn part_1(line: &str) -> i64 {
    let mut checksum: i64 = 0;

    let mut disk: Vec<i64> = vec![];
    let mut heap: BinaryHeap<i64> = BinaryHeap::new();
    let mut id: i32 = 0;
    let chars = line.chars().collect_vec();
    for index in (0..line.len()).step_by(2) {
        for _ in 0..chars[index].to_digit(10).unwrap() {
            disk.push(id.into());
            heap.push(id.into());
        }
        if chars.get(index + 1) != None {
            for _ in 0..chars[index + 1].to_digit(10).unwrap() {
                disk.push(-1);
            }
        }
        id += 1;
    }

    for index in 0..disk.len() {
        if disk[index] == -1 {
            let number = match heap.pop() {
                Some(number) => number,
                None => break
            };
            disk[index] = number;
            let position = disk.iter().rposition(|it| *it == number).unwrap();
            disk[position] = -1;
        }
    }

    for (index, number) in disk.iter().enumerate() {
        if *number != -1 {
            checksum += index as i64 * *number;
        }
    }

    return checksum;
}

fn part_2(line: &str) -> i64 {
    let mut checksum: i64 = 0;

    let mut disk: Vec<(i64, i64)> = vec![];
    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let mut id: i32 = 0;
    let chars = line.chars().collect_vec();
    for index in (0..line.len()).step_by(2) {
        disk.push((id.into(), chars[index].to_digit(10).unwrap().into()));
        heap.push((id.into(), chars[index].to_digit(10).unwrap().into()));
        if chars.get(index + 1) != None {
            disk.push((-1, chars[index + 1].to_digit(10).unwrap().into()));
        }
        id += 1;
    }

    loop {
        if heap.peek() == None { break; }

        let (id, length) = heap.pop().unwrap();
        if let Some(index) = disk.iter().position(|it| it.0 == -1 && it.1 >= length) {
           let space = disk[index].1;
            disk.splice(index..index + 1, [(id, length), (-1, space - length)]);
            let position = disk.iter().rposition(|it| it.0 == id).unwrap();
            disk.splice(position..position + 1, [(-1, length)]);
        }
    }

    let mut index: i64 = 0;
    for segment in disk.iter() {
        if segment.0 == -1 { 
            index += segment.1;
        }
        else {
            for _ in 0..segment.1 {
                checksum += segment.0 * index;
                index += 1;
            }
        }
    }

    return checksum;
}
