const DIRS: [(isize, isize); 5] = [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];

struct Winds {
    height: usize,
    width: usize,
    down: Vec<Vec<usize>>,
    up: Vec<Vec<usize>>,
    left: Vec<Vec<usize>>,
    right: Vec<Vec<usize>>,
}

impl Winds {
    fn wind_free(&self, x: usize, y: usize, t: usize) -> bool {
        !self.right[y].iter().any(|x2| (x2 + t) % self.width == x)
            && !self.down[x].iter().any(|y2| (y2 + t) % self.height == y)
            && !self.left[y]
                .iter()
                .any(|x2| (x2 + (self.width - 1) * t) % self.width == x)
            && !self.up[x]
                .iter()
                .any(|y2| (y2 + (self.height - 1) * t) % self.height == y)
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
    let mut down = vec![vec!(); width];
    let mut up = vec![vec!(); width];
    let mut left = vec![vec!(); height];
    let mut right = vec![vec!(); height];
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                '>' => right[y - 1].push(x - 1),
                '<' => left[y - 1].push(x - 1),
                'v' => down[x - 1].push(y - 1),
                '^' => up[x - 1].push(y - 1),
                _ => (),
            }
        }
    }
    let winds = Winds {
        height,
        width,
        down,
        left,
        right,
        up,
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
