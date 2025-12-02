const INPUT: &str = include_str!("../input");

// set pattern to first character
// set counter to 1
// loop
//     set peeked to look ahead pattern.len() bytes
//     if we dont have that many bytes left, return counter
//     if peeked == pattern, counter += 1 && consume peeked && continue
//     pattern += peeked[0]

// 12341234
//
// counter: 1
// pat: 1
// peek: 2
// pattern = 12
//
// counter: 1
// pat: 12
// peek: 34
// pattern = 123
//
// counter: 1
// pat: 123
// peek: 412
// pat = 1234
//
// counter: 1
// pat: 1234
// peek: 1234
// counter = 2
//
// counter: 2
// pat: 1234
// peek: cant
// return 2

// 123123123
//
// counter: 1
// pat: 1
// peek: 2
// pat = 12
//
// counter: 1
// pat: 12
// peek: 34
// pat = 123
//
// counter: 1
// pat: 123
// peek: 123
// counter = 2
//
// counter: 2
// pat: 123
// peek: 123
// counter = 3
//
// counter: 3
// pat: 123
// peek: cant
// return 3

// 123456
//
// counter: 1
// pat: 1
// peek: 2
// pat = 12
//
// counter: 1
// pat: 12
// peek: 34
// pat = 123
//
// counter: 1
// pat: 123
// peek: 456
// pat = 1234
//
// counter: 1
// pat: 1234
// peek: cant
// return 1

fn invalid(id: String, behavior: Behavior) -> bool {
    let mut count = 1;

    let chars: Vec<_> = id.chars().collect();
    if chars.len() == 0 {
        return false;
    }

    let mut pat = vec![];

    let mut i = 0;
    while i < chars.len() {
        let peek_to = i + pat.len().max(1);
        if peek_to > chars.len() {
            return false;
        }
        let peeked = &chars[i..peek_to];

        if pat == peeked {
            count += 1;
            i += pat.len();
        } else {
            pat.push(chars[i]);
            i += 1;
        }
    }

    match behavior {
        Behavior::Part1 => count == 2,
        Behavior::Part2 => count >= 2,
    }
}

#[derive(Clone, Copy)]
enum Behavior {
    Part1,
    Part2,
}

fn solve(behavior: Behavior) -> u64 {
    INPUT
        .trim_end()
        .split(",")
        .map(|range| {
            let (left, right) = range.split_once("-").expect("range should have -");
            let left: u64 = left.parse().expect("left should parse");
            let right: u64 = right.parse().expect("right should parse");

            left..=right
        })
        .flatten()
        .map(|id| id * invalid(id.to_string(), behavior) as u64)
        .sum()
}

fn main() {
    println!("part 1: {}", solve(Behavior::Part1));
    println!("part 2: {}", solve(Behavior::Part2));
}
