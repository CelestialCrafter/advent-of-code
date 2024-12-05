use std::fs;

use strum::{EnumIter, IntoEnumIterator};

type Input = Vec<Vec<char>>;

#[derive(Debug, EnumIter, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

fn parse_input(input: String) -> Input {
    input.trim().split('\n').map(|s| s.chars().collect()).collect()
}

fn get_node_from_offset(input: &Input, offset: usize, mut x: usize, mut y: usize, direction: Direction) -> Option<char> {
    match direction {
        Direction::Right => x = x + offset,
        Direction::Left => x = x.checked_sub(offset)?,
        Direction::Down => y = y + offset,
        Direction::Up => y = y.checked_sub(offset)?,
        Direction::TopLeft => {
            y = y.checked_sub(offset)?;
            x = x.checked_sub(offset)?;
        },
        Direction::TopRight => {
            y = y.checked_sub(offset)?;
            x = x + offset;
        },
        Direction::BottomLeft => {
            y = y + offset;
            x = x.checked_sub(offset)?;
        },
        Direction::BottomRight => {
            y = y + offset;
            x = x + offset;
        }
    }

    return input
        .get(y)
        .map(|v| v.get(x))
        .unwrap_or(None)
        .copied()
}

fn check_node_part1(input: &Input, x: usize, y: usize) -> usize {
    let target = ['X', 'M', 'A', 'S'];
    let mut sum = 0;

    let current = input[y][x];
    if current != target[0] {
        return 0;
    }

    for direction in Direction::iter() {
        for i in 1..target.len() + 1 {
            if i == target.len() {
                sum += 1;
                break
            }

            let node = match get_node_from_offset(input, i, x, y, direction.clone()) {
                None => break,
                Some(c) => c
            };

            if node != target[i] {
                break
            }
        }
    }

    sum
}

fn check_node_part2(input: &Input, x: usize, y: usize) -> usize {
    let target = ['M', 'A', 'S'];

    let current = input[y][x];
    if current != target[1] {
        return 0
    }

    let mut nodes = vec![];
    for directions in [
        Direction::TopLeft,
        Direction::TopRight,
        Direction::BottomLeft,
        Direction::BottomRight
    ] {
        nodes.push(match get_node_from_offset(input, 1, x, y, directions) {
            None => return 0,
            Some(n) => n
        });
    }

    for node in &nodes {
        if *node != 'M' && *node != 'S' {
            return 0
        }
    }

    let matching_horiz = nodes[0] == nodes[1] && nodes[2] == nodes[3];
    let matching_vert = nodes[0] == nodes[2] && nodes[1] == nodes[3];

    if matching_horiz ^ matching_vert {
        1
    } else {
        0
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");
    let input = parse_input(input);

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            part1_sum += check_node_part1(&input, x, y);
            part2_sum += check_node_part2(&input, x, y);
        }
    }

    println!("part 1: {}", part1_sum);
    println!("part 2: {}", part2_sum);
}
