use std::{collections::HashSet, fmt::{self, Display, Write}, fs, hint::unreachable_unchecked, thread, time::Duration};

const DISPLAY_RUNNING: bool = true;
const SIZE: (usize, usize) = (15, 30);
const DELAY: u64 = 100;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Guard {
    direction: Direction,
    row: usize,
    col: usize
}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Obstacle,
    Guard(Guard)
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let char = match self {
            Cell::Empty => '.',
            Cell::Obstacle => '#',
            Cell::Guard(g) => match g.direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>'
            }
        };

        f.write_char(char)
    }
}

type Map = Vec<Vec<Cell>>;
struct Game {
    map: Map,
    visited: HashSet<(usize, usize)>,
    guard: Guard
}

fn parse_map(input: String) -> Map {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line
                .chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '^' => Cell::Guard(Guard {
                        direction: Direction::Up,
                        row: i,
                        col: j
                    }),
                    '#' => Cell::Obstacle,
                    '.' => Cell::Empty,
                    _ => unsafe { unreachable_unchecked() }
                })
            .collect()
        })
    .collect()
}

fn display_map(game: &Game, i: usize) {
    // clear screen
    print!("\x1B[2J\x1B[1;1H");
    let display: String = game.map
        .iter()
        .enumerate()
        .filter(|(i, _)| i.abs_diff(game.guard.row) < SIZE.0)
        .map(|(i, line)|
            line
            .iter()
            .enumerate()
            .filter(|(i, _)| i.abs_diff(game.guard.col) < SIZE.1)
            .map(|(j, cell)| if matches!(cell, Cell::Empty) && game.visited.contains(&(i, j)) {
                "X".to_string()
            } else {
                match cell {
                    Cell::Guard(..) => format!("\x1b[93m{}\x1b[0m", cell),
                    _ => format!("{}", cell)
                }
            })
            .collect::<String>() + "\n")
        .collect();
    print!("{}", display);
    println!("Iteration {}", i);
    println!("Visited {}", game.visited.len());
}

fn find_guard(map: &Map) -> Option<Guard> {
    for line in map {
        for cell in line {
            match cell {
                Cell::Guard(g) => return Some(g.clone()),
                _ => continue
            }
        }
    }

    None
}

fn tick(game: &mut Game) -> bool {
    let (old_row, old_col) = (game.guard.row, game.guard.col);

    match game.guard.direction {
        Direction::Up => game.guard.row -= 1,
        Direction::Down => game.guard.row += 1,
        Direction::Left => game.guard.col -= 1,
        Direction::Right => game.guard.col += 1,
    };

    let (row, col) = (game.guard.row, game.guard.col);
    if row >= game.map.len() {
        return true
    }
    if col >= game.map[0].len() {
        return true
    }

    if matches!(game.map[row][col], Cell::Obstacle) {
        game.guard = Guard {
            direction: match game.guard.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up
            },
            row: old_row,
            col: old_col
        };
        game.map[old_row][old_col] = Cell::Guard(game.guard.clone());

        return false
    }

    game.map[row][col] = Cell::Guard(game.guard.clone());
    game.map[old_row][old_col] = Cell::Empty;
    false
}

fn part1(mut game: Game) {
    let mut i = 0;
    let mut finished = false;

    while !finished {
        if DISPLAY_RUNNING {
            display_map(&game, i);
        }
        i += 1;

        game.visited.insert((game.guard.row, game.guard.col));
        finished = tick(&mut game);
        if DISPLAY_RUNNING  && DELAY != 0 {
            thread::sleep(Duration::from_millis(DELAY));
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("could not read input");

    let map = parse_map(input);
    let guard = find_guard(&map).expect("could not find guard");
    let game = Game {
        map,
        guard,
        visited: HashSet::new()
    };

    part1(game);
}
