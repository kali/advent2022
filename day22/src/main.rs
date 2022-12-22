/*         0  49 50 199 100 149
 *
 *      0        AAAAAA BBBBBBB
 *     49        AAAAAA BBBBBBB
 *
 *     50        CCCCCC
 *     99        CCCCCC
 *
 *    100  DDDDD EEEEEE
 *    149  DDDDD EEEEEE
 *
 *    150  FFFFF
 *    199  FFFFF
 */

const DIRS: [(isize, isize); 4] = [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)];

fn move_part1(x: usize, y: usize, dir: usize) -> (usize, usize, usize) {
    let (dx, dy) = DIRS[dir];
    match (x, y, dir) {
        (149, 0..=49, 0) => (50, y, dir),
        (99, 50..=99, 0) => (50, y, dir),
        (99, 99..=149, 0) => (0, y, dir),
        (49, 150..=199, 0) => (0, y, dir),
        (0..=49, 199, 1) => (x, 100, dir),
        (50..=99, 149, 1) => (x, 0, dir),
        (100..=149, 49, 1) => (x, 0, dir),
        (50, 0..=49, 2) => (149, y, dir),
        (50, 50..=99, 2) => (99, y, dir),
        (0, 100..=149, 2) => (99, y, dir),
        (0, 149..=199, 2) => (49, y, dir),
        (0..=49, 100, 3) => (x, 199, dir),
        (50..=99, 0, 3) => (x, 149, dir),
        (100..=149, 0, 3) => (x, 49, dir),
        _ => ((x as isize + dx) as usize, (y as isize + dy) as usize, dir),
    }
}

fn move_part2(x: usize, y: usize, dir: usize) -> (usize, usize, usize) {
    let (dx, dy) = DIRS[dir];
    match (x, y, dir) {
        (149, 0..=49, 0) => /* B> => E< */ (99, 149 - y, 2),
        (99, 50..=99, 0) => /* C> => B^ */ (50 + y, 49, 3),
        (99, 100..=149, 0) => /* E> => B< */ (149, 149 - y, 2),
        (49, 150..=199, 0) => /* F> => E^ */ (y - 100, 149, 3),
        (0..=49, 199, 1) => /* Fv => Bv */ (x + 100, 0, 1),
        (50..=99, 149, 1) => /* Ev => F< */ (49, x + 100, 2),
        (100..=149, 49, 1) => /* Bv => C< */ (99, x - 50, 2),
        (50, 0..=49, 2) => /* A< => D> */ (0, 149 - y, 0),
        (50, 50..=99, 2) => /* C< => Dv */ (y - 50, 100, 1),
        (0, 100..=149, 2) => /* D< => A> */ (50, 149 - y, 0),
        (0, 149..=199, 2) => /* F< => Av */ (y - 100, 0, 1),
        (0..=49, 100, 3) => /* D^ => C> */ (50, 50 + x, 0),
        (50..=99, 0, 3) => /* A^ => F> */ (0, x + 100, 0),
        (100..=149, 0, 3) => /* B^ => F^ */ (x - 100, 199, 3),
        _ => ((x as isize + dx) as usize, (y as isize + dy) as usize, dir),
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (maze, path) = input.split_once("\n\n").unwrap();
    let maze: Vec<Vec<u8>> = maze.lines().map(|l| l.as_bytes().to_vec()).collect();
    let path = path.replace('R', " R ").replace('L', " L ");
    for move_part in [move_part1, move_part2] {
        let mut dir = 0;
        let mut y = 0;
        let mut x = maze[0].iter().position(|c| *c != b' ').unwrap();
        for mv in path.split_whitespace() {
            match mv {
                "R" => dir = (dir + 1) % 4,
                "L" => dir = (dir + 3) % 4,
                _ => {
                    let len = mv.parse::<usize>().unwrap();
                    for _ in 0..len {
                        let (nx, ny, ndir) = move_part(x, y, dir);
                        if maze[ny][nx] == b'#' {
                            break;
                        }
                        (x, y, dir) = (nx, ny, ndir)
                    }
                }
            }
        }
        dbg!(1000 * (y + 1) + 4 * (x + 1) + dir);
    }
}

#[test]
fn test_loop() {
    fn checkloop(start: (usize, usize, usize)) -> bool {
        (0..200).fold(start, |acc, _| move_part2(acc.0, acc.1, acc.2)) == start
    }
    for (fx, fy) in [(50, 0), (100, 0), (50, 50), (0, 100), (50, 100), (0, 150)] {
        for dir in 0..3 {
            assert!(checkloop((fx + 5, fy + 10, dir)));
        }
    }
}

#[test]
fn t_fore_and_back() {
    for (fx, fy) in [(50, 0), (100, 0), (50, 50), (0, 100), (50, 100), (0, 150)] {
        for x in 0..50 {
            for y in 0..50 {
                for dir in 0..3 {
                    let x = fx + x;
                    let y = fy + y;
                    let (x1, y1, dir1) = move_part2(x, y, dir);
                    let (x2, y2, dir2) = move_part2(x1, y1, (dir1 + 2) % 4);
                    assert_eq!(
                        (x2, y2, (dir2 + 2) % 4),
                        (x, y, dir),
                        "From ({},{}) {} ; got to ({},{}) {} then ({},{}) {}.",
                        x,
                        y,
                        dir,
                        x1,
                        y1,
                        dir1,
                        x2,
                        y2,
                        dir2
                    );
                }
            }
        }
    }
}
