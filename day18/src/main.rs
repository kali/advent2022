use std::collections::HashSet;

const MOVES: [(isize, isize, isize); 6] = [
    (1isize, 0isize, 0isize),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn is_air(droplets: &HashSet<(isize, isize, isize)>, coords: (isize, isize, isize)) -> bool {
    !droplets.contains(&coords)
}

fn count_surface(
    droplets: &HashSet<(isize, isize, isize)>,
    criteria: impl Fn(&HashSet<(isize, isize, isize)>, (isize, isize, isize)) -> bool,
) -> usize {
    droplets
        .iter()
        .map(|(x, y, z)| {
            MOVES
                .iter()
                .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .map(|c| criteria(&droplets, c) as usize)
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let droplets: HashSet<(isize, isize, isize)> = input
        .lines()
        .map(|line| {
            let mut tokens = line.split(",").map(|x| x.parse::<isize>().unwrap());
            (
                tokens.next().unwrap(),
                tokens.next().unwrap(),
                tokens.next().unwrap(),
            )
        })
        .collect();
    let part1 = count_surface(&droplets, is_air);
    dbg!(part1);
    let max = droplets
        .iter()
        .map(|(x, y, z)| x.max(y).max(z))
        .max()
        .unwrap()
        + 1;
    let outside_air: HashSet<(isize, isize, isize)> =
        pathfinding::directed::bfs::bfs_reach((-1, -1, -1), |&(x, y, z)| {
            MOVES
                .iter()
                .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
                .filter(|(x, y, z)| {
                    *x >= -1 && *y >= -1 && *z >= -1 && *x <= max && *y <= max && *z <= max
                })
                .filter(|c| is_air(&droplets, *c))
        })
        .collect();
    let part2 = count_surface(&droplets, |_, c| outside_air.contains(&c));
    dbg!(part2);
}
