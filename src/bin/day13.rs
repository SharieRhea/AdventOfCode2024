use util;

struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn main() {
    let mut lines = util::read_lines("day13.txt").into_iter();
    let mut machines: Vec<Machine> = vec![];
    let mut machines_2: Vec<Machine> = vec![];

    loop {
        let a = extract_values(lines.next().unwrap());
        let b = extract_values(lines.next().unwrap());
        let prize = extract_values(lines.next().unwrap());
        machines.push(Machine { a_x: a.0, a_y: a.1, b_x: b.0, b_y: b.1, prize_x: prize.0, prize_y: prize.1 });
        machines_2.push(Machine { a_x: a.0, a_y: a.1, b_x: b.0, b_y: b.1, prize_x: prize.0 + 10000000000000, prize_y: prize.1 + 10000000000000 });
        // if there's no blank line, this was the last entry
        if lines.next() == None { break; }
    }

    println!("Part 1: {}", compute_tokens(&machines));
    println!("Part 2: {}", compute_tokens(&machines_2));
}

fn compute_tokens(machines: &Vec<Machine>) -> i64 {
    let mut tokens = 0;
    
    for machine in machines {
        // use Cramer's rule to solve a system of equations with 2 equations and 2 unknowns
        // find value for a
        let a: f64 = (machine.prize_x * machine.b_y - machine.prize_y * machine.b_x) as f64 / (machine.a_x * machine.b_y - machine.a_y * machine.b_x) as f64;
        // use a to find b by plugging into first equation
        let b: f64 = (machine.prize_x as f64 - a * machine.a_x as f64) / machine.b_x as f64;
        if a.fract() != 0.0 || b.fract() != 0.0 { 
            // not a whole number, there is no way to win on this machine
            continue; 
        }
        // 3 tokens to push a, 1 token for b 
        tokens += a as i64 * 3 + b as i64;
    }

    return tokens;
}

fn extract_values(line: String) -> (i64, i64) {
    let (_, right_side) = line.split_once(": ").unwrap();
    let (x_string, y_string) = right_side.split_once(", ").unwrap();
    let x: i64 = x_string[2..].parse().unwrap();
    let y: i64 = y_string[2..].parse().unwrap();
    return (x, y);
}
