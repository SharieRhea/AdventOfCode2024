use std::collections::BinaryHeap;
use itertools::Itertools;

use util;

fn main() {
    let line = util::read_file("day09.txt");

    let mut disk: Vec<(i64, i64)> = vec![];
    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();

    // file ids increase starting from 0
    let mut id: i32 = 0;
    let chars = line.chars().collect_vec();
    for index in 0..chars.len() {
        if index % 2 == 0 {
            disk.push((id.into(), chars[index].to_digit(10).unwrap().into()));
            heap.push((id.into(), chars[index].to_digit(10).unwrap().into()));
            id += 1;
        }
        else {
            disk.push((-1, chars[index].to_digit(10).unwrap().into()));
        }
    }

    println!("Part 1: {}", part_1(disk.clone(), heap.clone()));
    println!("Part 2: {}", part_2(disk, heap));
}


fn part_1(mut disk: Vec<(i64, i64)>, mut heap: BinaryHeap<(i64, i64)>) -> i64 {
    loop {
        // no more files to move
        if heap.peek() == None { break; }

        let (id, length) = heap.pop().unwrap();
        if let Some(free_space_index) = disk.iter().position(|it| it.0 == -1) {
            let free_space = disk[free_space_index].1;
            let remaining = free_space - length;
            if remaining == 0 { 
                // file matching a gap exactly, just move it and delete old file from end
                disk.splice(free_space_index..free_space_index + 1, [(id, length)]); 
                let position = disk.iter().rposition(|it| it.0 == id).unwrap();
                disk.remove(position);
            }
            else if remaining > 0 { 
                // more free space than the file size, move the file, add the remaining space,
                // delete old file from end
                disk.splice(free_space_index..free_space_index + 1, [(id, length), (-1, remaining)]); 
                let position = disk.iter().rposition(|it| it.0 == id).unwrap();
                disk.remove(position);
            }
            else { 
                // not enough space for whole file, move the portion that will fit, update the file
                // on disk and in the heap to have the new shorter length
                disk.splice(free_space_index..free_space_index + 1, [(id, free_space)]); 
                heap.push((id, length - free_space));
                let position = disk.iter().rposition(|it| it.0 == id).unwrap();
                disk[position] = (id, length - free_space);
            }

            // remove any trailing free space from the end of the disk
            loop {
                let index = disk.len() - 1;
                if disk[index].0 == -1 {
                    disk.remove(index);
                }
                else { break; }
            }

        }
        else { break; }
    }

    let mut checksum: i64 = 0;
    let mut index: i64 = 0;
    for segment in disk.iter() {
        if segment.0 == -1 { index += segment.1; }
        else {
            for _ in 0..segment.1 {
                checksum += segment.0 * index;
                index += 1;
            }
        }
    }

    return checksum;
}

fn part_2(mut disk: Vec<(i64, i64)>, mut heap: BinaryHeap<(i64, i64)>) -> i64 {
    loop {
        if heap.peek() == None { break; }

        let (id, length) = heap.pop().unwrap();
        // find a gap that can accomodate this file
        if let Some(index) = disk.iter().position(|it| it.0 == -1 && it.1 >= length) {
           let space = disk[index].1;
            // move the file and update and remaining space
            disk.splice(index..index + 1, [(id, length), (-1, space - length)]);
            let position = disk.iter().rposition(|it| it.0 == id).unwrap();
            // remove the file from the end of disk
            disk.splice(position..position + 1, [(-1, length)]);
        }
    }

    let mut checksum: i64 = 0;
    let mut index: i64 = 0;
    for segment in disk.iter() {
        if segment.0 == -1 { index += segment.1; }
        else {
            for _ in 0..segment.1 {
                checksum += segment.0 * index;
                index += 1;
            }
        }
    }

    return checksum;
}
