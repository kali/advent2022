use std::cell::Cell;

use pathfinding::prelude::dfs_reach;

#[derive(Clone, Debug)]
struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

impl Blueprint {
    fn geodes(&self, steps: usize) -> usize {
        let state = State {
            turn: 1,
            robots: (1, 0, 0, 0),
            stock: (0, 0, 0, 0),
        };
        let best = Cell::new(0usize);
        for it in dfs_reach(state, |s| s.next(self, steps, &best)) {
            if it.stock.3 > best.get() {
                best.set(it.stock.3);
            }
        }
        best.get()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    turn: usize,
    robots: (usize, usize, usize, usize),
    stock: (usize, usize, usize, usize),
}

impl State {
    fn next(&self, bp: &Blueprint, steps: usize, must_best: &Cell<usize>) -> Vec<State> {
        if self.potential(steps) < must_best.get() || self.turn == steps + 1 {
            return vec![];
        }
        let mut wait = State {
            turn: self.turn + 1,
            robots: self.robots.clone(),
            stock: (
                self.stock.0 + self.robots.0,
                self.stock.1 + self.robots.1,
                self.stock.2 + self.robots.2,
                self.stock.3 + self.robots.3,
            ),
        };
        // if we can build a geode bot, build it.
        if self.stock.0 >= bp.geode.0 && self.stock.2 >= bp.geode.1 {
            wait.stock.0 -= bp.geode.0;
            wait.stock.2 -= bp.geode.1;
            wait.robots.3 += 1;
            return vec!(wait)
        }

        let mut states = vec![];
        if self.stock.0 >= bp.ore {
            let mut state = wait.clone();
            state.stock.0 -= bp.ore;
            state.robots.0 += 1;
            states.push(state);
        }
        if self.stock.0 >= bp.clay {
            let mut state = wait.clone();
            state.stock.0 -= bp.clay;
            state.robots.1 += 1;
            states.push(state);
        }
        if self.stock.0 >= bp.obsidian.0 && self.stock.1 >= bp.obsidian.1 {
            let mut state = wait.clone();
            state.stock.0 -= bp.obsidian.0;
            state.stock.1 -= bp.obsidian.1;
            state.robots.2 += 1;
            states.push(state);
        }
        // no reason to wait if we can build any bot.
        if states.len() < 3 {
            states.push(wait);
        }
        states
    }

    fn potential(&self, steps: usize) -> usize {
        let remaining = (steps + 1).saturating_sub(self.turn);
        self.stock.3 + remaining * self.robots.3 + remaining * remaining.saturating_sub(1) / 2
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let blueprints = parse(&input);
    dbg!(part1(&*blueprints));
    dbg!(part2(&*blueprints));
}

fn parse(input: &str) -> Vec<Blueprint> {
    input
        .trim()
        .lines()
        .map(|line| {
            let id = line.split(&[' ', ':']).nth(1).unwrap().parse().unwrap();
            let ore = line.split(&[' ', ':']).nth(7).unwrap().parse().unwrap();
            let clay = line.split(&[' ', ':']).nth(13).unwrap().parse().unwrap();
            let o1 = line.split(&[' ', ':']).nth(19).unwrap().parse().unwrap();
            let o2 = line.split(&[' ', ':']).nth(22).unwrap().parse().unwrap();
            let g1 = line.split(&[' ', ':']).nth(28).unwrap().parse().unwrap();
            let g2 = line.split(&[' ', ':']).nth(31).unwrap().parse().unwrap();
            Blueprint {
                id,
                ore,
                clay,
                obsidian: (o1, o2),
                geode: (g1, g2),
            }
        })
        .collect()
}

fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|bp| bp.geodes(24) * bp.id)
        .sum::<usize>()
}

fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|bp| bp.geodes(32))
        .product::<usize>()
}

#[cfg(test)]
mod t {
    use super::*;

    const TEST_INPUT: &str =
        "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn t1() {
        let bps = parse(&TEST_INPUT);
        assert_eq!(bps[0].geodes(24), 9);
        assert_eq!(bps[1].geodes(24), 12);
    }

    #[test]
    fn tp1() {
        let bps = parse(&TEST_INPUT);
        assert_eq!(part1(&bps), 33);
    }

    #[test]
    fn tp2() {
        let bps = parse(&TEST_INPUT);
        assert_eq!(bps[0].geodes(32), 56);
        assert_eq!(bps[1].geodes(32), 62);
    }
}
