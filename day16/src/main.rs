use std::{cell::Cell, collections::HashMap, iter::once};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Id([char; 2]);

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

#[derive(Debug)]
struct Valve {
    rate: usize,
    tunnels: Vec<Id>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct State {
    agents: Vec<(usize, Id)>,
    turn: usize,
    score: usize,
    openable: Vec<Id>,
}

struct Maze {
    openable: Vec<Id>,
    valves: HashMap<Id, Valve>,
    dists: HashMap<(Id, Id), usize>,
}

impl State {
    fn potential(&self, maze: &Maze) -> usize {
        let mut valves: Vec<usize> = self.openable.iter().map(|v| maze.valves[&v].rate).collect();
        valves.sort();
        valves.reverse();
        valves
            .chunks(self.agents.len())
            .enumerate()
            .map(|(ix, valves)| -> usize {
                let remaning_turns = 30usize.saturating_sub(self.turn + 2 * ix);
                remaning_turns * valves.iter().sum::<usize>()
            })
            .sum::<usize>()
            + self.score
    }

    fn next(&self, maze: &Maze, to_best: &Cell<usize>) -> Vec<State> {
        if let Some(agent) = self.agents.iter().position(|a| a.0 == self.turn) {
            self.openable
                .iter()
                .filter_map(|target| {
                    let free = self.turn + maze.dists[&(self.agents[agent].1, *target)];
                    let mut agents = self.agents.clone();
                    agents[agent] = (free + 1, *target);
                    let turn = agents.iter().map(|p| p.0).min().unwrap();
                    let pos = State {
                        agents,
                        turn,
                        score: self.score + 30usize.saturating_sub(free) * maze.valves[target].rate,
                        openable: self
                            .openable
                            .iter()
                            .filter(|v| v != &target)
                            .cloned()
                            .collect(),
                    };
                    if pos.potential(&maze) >= to_best.get() {
                        Some(pos)
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            unreachable!();
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let maze = build(&input);
    dbg!(part1(&maze));
    dbg!(part2(&maze));
}

fn build(input: &str) -> Maze {
    let mut valves = HashMap::new();
    let mut dists = HashMap::new();
    for line in input.trim().lines() {
        let mut chars = line.split(" ").nth(1).unwrap().chars();
        let name = Id([chars.next().unwrap(), chars.next().unwrap()]);
        let rate = line
            .split([' ', '='])
            .nth(5)
            .unwrap()
            .trim_end_matches(";")
            .parse::<usize>()
            .unwrap();
        let tunnels = line
            .split(' ')
            .skip(9)
            .map(|s| {
                let mut chars = s.chars();
                Id([chars.next().unwrap(), chars.next().unwrap()])
            })
            .collect();
        valves.insert(name, Valve { rate, tunnels });
    }
    let openable: Vec<_> = valves
        .iter()
        .filter(|(_k, v)| v.rate > 0)
        .map(|(k, _v)| k.to_owned())
        .collect();
    for from in openable.iter().copied().chain(once(Id(['A', 'A']))) {
        for to in &openable {
            let dist = pathfinding::directed::dijkstra::dijkstra(
                &from,
                |v| valves[v].tunnels.iter().map(|s| (*s, 1)),
                |v| v == to,
            )
            .unwrap()
            .1;
            dists.insert((from, *to), dist);
        }
    }
    Maze {
        valves,
        dists,
        openable,
    }
}

fn optimize(maze: &Maze, start: State) -> usize {
    let best = Cell::new(0);
    for found in pathfinding::directed::dfs::dfs_reach(start.clone(), |pos| pos.next(&maze, &best))
    {
        if found.score > best.get() {
            best.set(found.score);
        }
    }
    return best.get();
}

fn part1(maze: &Maze) -> usize {
    let start = State {
        turn: 1,
        agents: vec![(1, Id(['A', 'A']))],
        score: 0,
        openable: maze.openable.clone(),
    };
    optimize(maze, start)
}

fn part2(maze: &Maze) -> usize {
    let start = State {
        turn: 5,
        agents: vec![(5, Id(['A', 'A'])), (5, Id(['A', 'A']))],
        score: 0,
        openable: maze.openable.clone(),
    };
    optimize(maze, start)
}

#[test]
fn t() {
    let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
    let maze = build(input);
    assert_eq!(part1(&maze), 1651);
    assert_eq!(part2(&maze), 1707);
}
