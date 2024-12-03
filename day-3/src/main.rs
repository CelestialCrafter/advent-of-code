// i dont wana use a regex, thats boring.
// but also its like 3am.....
// whatever...

use std::{char, fs};

#[derive(Clone, Debug)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont
}

fn parse_mul(input: &str) -> Vec<(usize, Instruction)> {
    let mut found = "".to_string();
    let mut acc = 0;

    let target: Vec<char> = "mul(_,_)".chars().collect();
    let mut output = vec![];

    for (i, c) in input.char_indices() {
        // handle match
        if acc == target.len() {
            let (a, b) = found.split_once(',').unwrap();
            let a = filter_numbers(a);
            let b = filter_numbers(b);

            output.push((i, Instruction::Mul(a, b)));
            acc = 0;
            found = "".to_string();
        }

        let mut next = target[acc];

        // numbers
        if next == '_' {
            if char::is_numeric(c) {
                found += &c.to_string();
                continue;
            }

            acc += 1;
            next = target[acc];
        }

        // normal characters
        if c == next {
            found += &c.to_string();
            acc += 1;
            continue;
        }

        // reset
        acc = 0;
        found = "".to_string();
    }

    output
}

fn parse(input: &str, target: &str, instruction: Instruction) -> Vec<(usize, Instruction)> {
    let target: Vec<char> = target.chars().collect();
    let mut found = "".to_string();
    let mut output = vec![];

    for (i, c) in input.char_indices() {
        if found.len() == target.len() {
            output.push((i, instruction.clone()));
            found = "".to_string();
        }

        let next = target[found.len()];

        if c == next {
            found += &c.to_string();
            continue;
        }

        // reset
        found = "".to_string();
    }

    output
}

fn compute_sum(instructions: Vec<Instruction>) -> usize {
    let mut multiply = true;
    let mut sum = 0;

    for inst in instructions {
        match inst {
            Instruction::Do => multiply = true,
            Instruction::Dont => multiply = false,
            Instruction::Mul(a, b) => if multiply {
                sum += a * b;
            }
        }
    }

    sum
}

fn remove_positions(input: Vec<(usize, Instruction)>) -> Vec<Instruction> {
    input.into_iter().map(|(_, i)| i).collect()
}

fn filter_numbers(string: &str) -> usize {
    return string
        .chars()
        .filter(|c| char::is_numeric(*c))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");
    
    let muls = parse_mul(&input);
    println!("part 1: {}", compute_sum(remove_positions(muls.clone())));

    let dos = parse(&input, "do()", Instruction::Do);
    let donts = parse(&input, "don't()", Instruction::Dont);

    let mut instructions: Vec<(usize, Instruction)> = muls
        .into_iter()
        .chain(dos.into_iter())
        .chain(donts.into_iter())
        .collect();

    instructions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("part 2: {}", compute_sum(remove_positions(instructions)));
}
