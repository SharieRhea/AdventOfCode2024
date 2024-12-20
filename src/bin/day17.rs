use util;

struct Registers {
    a: i64,
    b: i64,
    c: i64
}

/* my program input simplifies to
*   b <- a % 8
*   b <- b ^ 1
*   c <- a / 2^b
*   a <- a / 8
*   b <- b ^ c
*   output b % 8
*   jump 0 if a != 0
*/

fn main() {
    let mut lines = util::read_lines("day17.txt").into_iter();
    // populate registers
    let a = parse_register(lines.next().unwrap()); 
    let b = parse_register(lines.next().unwrap()); 
    let c = parse_register(lines.next().unwrap()); 
    let mut registers = Registers { a, b, c };
    // consume the blank line
    lines.next();
    // populate the list of instructions
    let line = lines.next().unwrap();
    let (_, program) = line.split_once(": ").unwrap();
    let instructions: Vec<i64> = program.split(",").map(|it| it.parse::<i64>().unwrap()).collect();

    println!("Part 1: {}", part_1(&mut registers, &instructions).iter().map(|it| it.to_string()).collect::<Vec<_>>().join(","));
    println!("Part 2: {}", part_2(&instructions, instructions.as_slice()));
}

fn part_1(registers: &mut Registers, instructions: &Vec<i64>) -> Vec<i64> {
    let mut output: Vec<i64> = vec![];
    let mut instruction_pointer: i64 = 0;

    while let Some(opcode) = instructions.get(instruction_pointer as usize) {
        let operand = instructions.get(instruction_pointer as usize + 1).unwrap();
        match opcode {
            0 => { adv(registers, *operand, &mut instruction_pointer); },
            1 => { bxl(registers, *operand, &mut instruction_pointer); },
            2 => { bst(registers, *operand, &mut instruction_pointer); },
            3 => { jnz(registers, *operand, &mut instruction_pointer); },
            4 => { bxc(registers, *operand, &mut instruction_pointer); },
            5 => { output.push(out(registers, *operand, &mut instruction_pointer)); },
            6 => { bdv(registers, *operand, &mut instruction_pointer); },
            7 => { cdv(registers, *operand, &mut instruction_pointer); },
            _ => { panic!("Unknown opcode"); },
        }
    }
    return output;
}

fn part_2(instructions: &Vec<i64>, target: &[i64]) -> i64 {
    // only value of a matters for each iteration
    // last 3 bits of a are removed each iteration
    
    // start from 0 if we're looking for one number
    // otherwise, start from 8 * answer for list with first item removed
    let mut value = if target.len() == 1 { 0 } else { 8 * part_2(instructions, &target[1..]) };
    
    while target != part_1(&mut Registers { a: value, b: 0, c: 0 }, instructions) {
        value += 1;
    }
    
    return value;
}

fn adv(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) {
    // performs division of a and combo operand and writes into a
    let numerator = registers.a;
    let denominator = 2i64.pow(get_combo_operand(registers, operand) as u32);
    registers.a = numerator / denominator;
    *instruction_pointer += 2;
}

fn bxl(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) {
    // performs bitwise xor of b and operand and writes into b
    registers.b = registers.b ^ operand;
    *instruction_pointer += 2;
}

fn bst(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) {
    // calculates combo operand mod 8 and writes into b
    registers.b = get_combo_operand(registers, operand) % 8;
    *instruction_pointer += 2;
}

fn jnz(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) {
    // do nothing if a is 0, otherwise jump to literal operand
    if registers.a != 0 {
        *instruction_pointer = operand;
    }
    else {
        *instruction_pointer += 2;
    }
}

fn bxc(registers: &mut Registers, _operand: i64, instruction_pointer: &mut i64) {
    // performs bitwise xor of b and c and writes into c
    // ignores the operand
    registers.b = registers.b ^ registers.c;
    *instruction_pointer += 2;
}

fn out(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) -> i64 {
    // calculates combo operand mod 8 and outputs it
    *instruction_pointer += 2;
    return get_combo_operand(registers, operand) % 8;
}

fn bdv(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) {
    // adv but store into b
    let numerator = registers.a;
    let denominator = 2i64.pow(get_combo_operand(registers, operand) as u32);
    registers.b = numerator / denominator;
    *instruction_pointer += 2;
}

fn cdv(registers: &mut Registers, operand: i64, instruction_pointer: &mut i64) {
    // adv but store into c
    let numerator = registers.a;
    let denominator = 2i64.pow(get_combo_operand(registers, operand) as u32);
    registers.c = numerator / denominator;
    *instruction_pointer += 2;
}

fn get_combo_operand(registers: &Registers, operand: i64) -> i64 {
    return match operand {
        // 0-3 are literal values
        0 | 1 | 2 | 3 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Unknown combo operator")
    }
}

fn parse_register(line: String) -> i64 {
    let (_, register) = line.split_once(": ").unwrap();
    return register.parse().unwrap();
}
