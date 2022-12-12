fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input, 1));
    dbg!(run(&input, 2));
}

fn run(input: &str, part: usize) -> usize {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let find = |lookup| -> Vec<(usize, usize)> {
        grid.iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .filter_map(move |(x, c)| if *c == lookup { Some((x, y)) } else { None })
            })
            .collect()
    };
    let start = find(b'S')[0];
    let end = find(b'E')[0];
    let (h, w) = (grid.len() as isize, grid[0].len() as isize);

    let at = |pair: (usize, usize)| -> u8 {
        if pair == start {
            b'a'
        } else if pair == end {
            b'z'
        } else {
            grid[pair.1][pair.0]
        }
    };

    let all_as = find(b'a');

    let successors = |(x, y)| {
        if part == 2 && (x, y) == start {
            all_as.iter().map(|&pair| (pair, 0)).collect()
        } else {
            [(0, 1), (0, -1), (1, 0), (-1, 0)]
                .iter()
                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter(|&(x2, y2)| x2 >= 0 && x2 < w && y2 >= 0 && y2 < h)
                .map(|(x2, y2)| (x2 as usize, y2 as usize))
                .filter(|&(x2, y2): &(usize, usize)| at((x, y)) + 1 >= at((x2, y2)))
                .map(|pair| (pair, 1))
                .collect::<Vec<_>>()
        }
    };

    let path =
        pathfinding::directed::dijkstra::dijkstra(&start, |pair| successors(*pair), |&p| p == end)
            .unwrap();
    path.1
}

#[test]
fn t() {
    let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    assert_eq!(run(&input, 1), 31);
    assert_eq!(run(&input, 2), 29);
}
