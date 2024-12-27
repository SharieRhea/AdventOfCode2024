use util;

fn main() {
    let mut locks: Vec<[i32; 5]> = vec![];
    let mut keys: Vec<[i32; 5]> = vec![];

    let mut lines = util::read_lines("day25.txt").into_iter();

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => { break; }
        };
        if line[0..1] == *"#" {
            // this is a lock
            let mut lock = [0; 5];
            for _ in 0..6 {
                for (index, character) in lines.next().unwrap().chars().enumerate() {
                    if character == '#' {
                        lock[index] += 1;
                    }
                }
            }
            locks.push(lock);
        }
        else {
            // this is a key
            let mut key = [0; 5];
            for _ in 0..5 {
                for (index, character) in lines.next().unwrap().chars().enumerate() {
                    if character == '#' {
                        key[index] += 1;
                    }
                }
            }
            // ignore the bottom row of the key
            lines.next();
            keys.push(key);
        }
        lines.next();
    }

    println!("Part 1: {}", part_1(&locks, &keys));
}

fn part_1(locks: &Vec<[i32; 5]>, keys: &Vec<[i32; 5]>) -> usize {
    let mut sum = 0;
    for one in 0..locks.len() {
        'pair: for two in 0..keys.len() {
            // check heights for each column, can't be greater than 5
            for column in 0..5 {
                if locks[one][column] + keys[two][column] > 5 {
                    continue 'pair; 
                }
            }
            // this pair was valid
            sum += 1;
        }
    }
    return sum;
}
