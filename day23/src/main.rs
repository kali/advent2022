use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

use itertools::Itertools;

const MOVES: [usize; 4] = [0, 4, 6, 2];
const DIRS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Clone)]
struct State {
    elves: HashSet<(isize, isize)>,
    turn: usize,
}

impl FromStr for State {
    type Err = ();
    fn from_str(input: &str) -> Result<State, ()> {
        let mut elves = HashSet::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                if c == b'#' {
                    elves.insert((x as isize, y as isize));
                }
            }
        }
        Ok(State { elves, turn: 0 })
    }
}

impl State {
    fn run(&self) -> Option<State> {
        let mut plan: Vec<Option<(isize, isize)>> = vec![];
        for (x, y) in &self.elves {
            let occupied: Vec<bool> = DIRS
                .iter()
                .map(|(dx, dy)| self.elves.contains(&(x + dx, y + dy)))
                .collect();
            let mut mov = None;
            if occupied.iter().any(|x| *x) {
                for mv in 0..4 {
                    let mv = MOVES[(mv + self.turn) % 4];
                    if !occupied[(mv + 7) % 8] && !occupied[mv] && !occupied[(mv + 1) % 8] {
                        mov = Some((x + DIRS[mv].0, y + DIRS[mv].1));
                        break;
                    }
                }
            }
            plan.push(mov);
        }
        let mut targets = HashMap::<(isize, isize), usize>::default();
        for it in plan.iter().filter_map(|it| *it) {
            *targets.entry(it).or_default() += 1;
        }
        if targets.len() > 1 {
            Some(State {
                turn: self.turn + 1,
                elves: self
                    .elves
                    .iter()
                    .copied()
                    .zip(plan.iter())
                    .map(|(mut elf, target)| {
                        if let Some(target) = target {
                            if targets[target] == 1 {
                                (elf.0, elf.1) = (target.0, target.1);
                            }
                        }
                        elf
                    })
                    .collect(),
            })
        } else {
            None
        }
    }

    fn x_boundaries(&self) -> RangeInclusive<isize> {
        let r = self
            .elves
            .iter()
            .map(|e| e.0)
            .minmax()
            .into_option()
            .unwrap();
        r.0..=r.1
    }

    fn y_boundaries(&self) -> RangeInclusive<isize> {
        let r = self
            .elves
            .iter()
            .map(|e| e.1)
            .minmax()
            .into_option()
            .unwrap();
        r.0..=r.1
    }

    fn score(&self) -> isize {
        let x = self.x_boundaries();
        let y = self.y_boundaries();
        (x.end() - x.start() + 1) * (y.end() - y.start() + 1) - self.elves.len() as isize
    }

    fn turns(&self) -> isize {
        let mut state = self.clone();
        for i in 0.. {
            if let Some(s) = state.run() {
                state = s;
            } else {
                return i + 1;
            }
        }
        unreachable!()
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut state = input.parse::<State>().unwrap();
    for _ in 0..10 {
        state = state.run().unwrap_or(state);
    }
    dbg!(state.score());
    let state = input.parse::<State>().unwrap();
    dbg!(state.turns());
}

#[test]
fn t1() {
    let mut state = r#".......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......"#
        .parse::<State>()
        .unwrap();
    for _ in 0..10 {
        state = state.run().unwrap_or(state);
    }
    assert_eq!(state.score(), 110);
}

#[test]
fn t2() {
    let state = r#".......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......"#
        .parse::<State>()
        .unwrap();
    assert_eq!(state.turns(), 20);
}
