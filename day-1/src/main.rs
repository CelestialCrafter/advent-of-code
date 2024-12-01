use std::{collections::HashMap, fs};

use eyre::{OptionExt, Result};

fn parse_input(input: String) -> Result<(Vec<u32>, Vec<u32>)> {
    let mut left: Vec<u32> = vec![];
    let mut right: Vec<u32> = vec![];

    for line in input.lines() {
        let (l, r) = line.split_once("   ").ok_or_eyre("input did not have delimiter")?;
        left.push(l.parse()?);
        right.push(r.parse()?);
    }

    Ok((left, right))
}

fn part_1(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    let diff: u32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    diff
}

fn part_2(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let mut counts = HashMap::new();

    for num in right {
        (*counts.entry(num).or_default()) += 1;
    }

    let similarity = left
        .into_iter()
        .map(|num| num * (counts.get(&num).unwrap_or(&0)))
        .sum();

    similarity
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");

    let (left, right) = parse_input(input).expect("could not parse input");
    println!("part 1: {}", part_1(left.clone(), right.clone()));
    println!("part 2: {}", part_2(left, right));
}
