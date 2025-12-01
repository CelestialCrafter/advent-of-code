const INPUT: &str = include_str!("../input");
const HIGH: u64 = 99;
const LOW: u64 = 0;
const START: u64 = 50;

#[inline]
fn wrap(mut value: u64, diff: i128) -> (u64, u64) {
    let mut zeros = 0;
    let negative = diff.is_negative();

    for _ in 0..diff.abs() {
        let (bound, result) = if negative {
            (HIGH, value.checked_sub(1))
        } else {
            (LOW, value.checked_add(1))
        };

        value = match result {
            Some(result) if result > HIGH => bound,
            Some(result) => result,
            None => bound,
        };

        zeros += (value == 0) as u64;
    }

    (value, zeros)
}

enum Behavior {
    Part1,
    Part2,
}

fn solve(behavior: Behavior) -> u64 {
    let (password, _) = INPUT
        .lines()
        .fold((0, START), |(mut zeros, mut dial), line| {
            let (direction, amount) = line.split_at(1);
            let amount: i128 = amount.parse().expect("should be able to parse amount");

            let (new_dial, new_zeros) = wrap(
                dial,
                match direction {
                    "L" => -amount,
                    "R" => amount,
                    _ => panic!("unexpected direction: {direction}"),
                },
            );

            zeros += match behavior {
                Behavior::Part1 => (new_dial == 0) as u64,
                Behavior::Part2 => new_zeros,
            };
            dial = new_dial;

            (zeros, dial)
        });

    password
}

fn main() {
    println!("part 1: {}", solve(Behavior::Part1));
    println!("part 2: {}", solve(Behavior::Part2));
}
