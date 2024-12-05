use std::{collections::HashMap, fs};

use eyre::{eyre, Result};

type Rules = HashMap<usize, Vec<usize>>;
type Updates = Vec<Vec<usize>>;

fn parse_rules(input: &str) -> Result<Rules> {
    let mut rules: Rules = HashMap::new();
    for line in input.lines() {
            let (a, b) = line.split_once('|').ok_or(eyre!("line did not have delimiter"))?;
            let a = a.parse()?;
            let b = b.parse()?;

            (*rules.entry(a).or_default()).push(b);
    }

    Ok(rules)
}

fn parse_updates(input: &str) -> Result<Updates> {
    input
        .lines()
        .map(|line| line
            .split(',')
            // dont ask
            .map(|s| Ok(s.parse()?))
            .collect()
        )
        .collect()
}

fn fix_update<'a>(rules: &Rules, update: &mut Vec<usize>) -> bool {
    let mut broken = false;

    'outer: for (i, page) in update.clone().iter().enumerate() {
        let page_rules = rules.get(page).cloned().unwrap_or_default();

        for rule in page_rules {
            let rule_index = update.iter().position(|p| *p == rule).unwrap_or(usize::MAX);
            if rule_index < i {
                broken = true;
                update.swap(i, rule_index);
                fix_update(rules, update);
                break 'outer;
            }
        }
    }

    broken
}

fn center(update: &Vec<usize>) -> usize {
    update[update.len() / 2]
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");
    let (rule_input, update_input) = input.split_once("\n\n").expect("could not separate rules and updates");

    let rules = parse_rules(rule_input).expect("could not parse rules");
    let updates = parse_updates(update_input).expect("could not parse updates");

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for update in &updates {
        let update_fixed = &mut update.clone();
        let broken = fix_update(&rules, update_fixed);

        if broken {
            part2_sum += center(update_fixed);
        } else {
            part1_sum += center(update);
        }
    }

    println!("part 1: {}", part1_sum);
    println!("part 2: {}", part2_sum);
}
