const DIRS: [(isize, isize); 5] = [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];

struct Winds {
    height: usize,
    width: usize,
    winds: Vec<(usize, usize, usize)>,
}

impl Winds {
    fn wind_free(&self, x: usize, y: usize, t: usize) -> bool {
        !self.winds.iter().any(|w| {
            (y == w.1 && w.2 == 0 && (w.0 + t) % self.width == x)
                || (y == w.1 && w.2 == 2 && (w.0 + t * (self.width - 1)) % self.width == x)
                || (x == w.0 && w.2 == 1 && (w.1 + t) % self.height == y)
                || (x == w.0 && w.2 == 3 && (w.1 + t * (self.height - 1)) % self.height == y)
        })
    }
}

fn find_path(winds: &Winds, pos0: (usize, usize), t0: usize, pos1: (usize, usize)) -> usize {
    pathfinding::directed::astar::astar(
        &(pos0, t0),
        |&((x, y), t)| {
            DIRS.into_iter()
                .map(move |(dx, dy)| {
                    (
                        ((x as isize + dx) as usize, (y as isize + dy) as usize),
                        t + 1,
                    )
                })
                .filter(|&((x, y), t)| {
                    ((1..=winds.width).contains(&x)
                        && (1..=winds.height).contains(&y)
                        && winds.wind_free(x - 1, y - 1, t))
                        || [pos0, pos1].contains(&(x, y))
                })
                .map(|state| (state, 1))
        },
        |&((x, y), _)| x.abs_diff(pos1.0) + y.abs_diff(pos1.1),
        |&(pos, _)| pos == pos1,
    )
    .unwrap()
    .1
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (usize, usize) {
    let width = input.lines().next().unwrap().len() - 2;
    let height = input.trim().lines().count() - 2;
    let hole = |line: &str| line.bytes().position(|c| c == b'.').unwrap();
    let entry = (hole(input.lines().next().unwrap()), 0);
    let exit = (hole(input.lines().last().unwrap()), height + 1);
    let winds = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            {
                l.chars().enumerate().filter_map(move |(x, c)| match c {
                    '>' => Some((x - 1, y - 1, 0)),
                    'v' => Some((x - 1, y - 1, 1)),
                    '<' => Some((x - 1, y - 1, 2)),
                    '^' => Some((x - 1, y - 1, 3)),
                    _ => None,
                })
            }
        })
        .collect();
    let winds = Winds {
        height,
        width,
        winds,
    };
    let p1 = find_path(&winds, entry, 0, exit);
    let back = find_path(&winds, exit, p1, entry);
    let again = find_path(&winds, entry, p1 + back, exit);
    (p1, p1 + back + again)
}

#[test]
fn t() {
    assert_eq!(
        run("#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"),
        (18, 54)
    );
}

#[test]
fn t0() {
    assert_eq!(
        run("#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#")
        .0,
        10
    );
}
