use std::fs;

#[derive(Debug)]
enum State {
    UNDETERMINED,
    INCREASING,
    DECREASING
}

fn inc_dec_rule(report: &Vec<u32>) -> bool {
    let mut state = State::UNDETERMINED;

    for (i, current) in report.iter().enumerate() {
        let prev = match i.checked_sub(1) {
            None => continue,
            Some(i) => report[i]
        };
        
        let diff = (*current as i32) - (prev as i32);
        if diff == 0 {
            return false;
        }

        if diff > 0 {
            if matches!(state, State::DECREASING) {
                return false;
            }
            state = State::INCREASING;
        } else {
            if matches!(state, State::INCREASING) {
                return false;
            }
            state = State::DECREASING;
        }
    }

    !matches!(state, State::UNDETERMINED)
}

fn diff_rule(report: &Vec<u32>) -> bool {
    for (i, current) in report.iter().enumerate() {
        let prev = match i.checked_sub(1) {
            None => continue,
            Some(i) => report[i]
        };

        let diff = current.abs_diff(prev);
        if diff < 1 || diff > 3 {
            return false;
        }
    }

    true
}

fn safe(report: &Vec<u32>) -> bool {
        return diff_rule(&report) && inc_dec_rule(&report);
}

fn safe_with_removal(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);

        if safe(&new_report) {
            return true
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");
    let mut part1_safe = 0;
    let mut part2_safe = 0;

    for line in input.lines() {
        let report = line
            .split(" ")
            .map(|s| s.parse().expect("could not parse level"))
            .collect();

        if safe(&report) {
            part1_safe += 1;
        }

        if safe_with_removal(&report) {
            part2_safe += 1;
        }
    }

    println!("part 1 safe: {}", part1_safe);
    println!("part 2 safe: {}", part2_safe);
}
