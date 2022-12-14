use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (usize, usize) {
    let mut universe: HashSet<(usize, usize)> = HashSet::default();
    let mut ymax = 0;
    for line in input.trim().lines() {
        let points: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|pt| {
                let (x, y) = pt.split_once(",").unwrap();
                (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect();
        for segment in points.windows(2) {
            let ((x1, y1), (x2, y2)) = (segment[0], segment[1]);
            ymax = ymax.max(y1).max(y2);
            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    universe.insert((x1, y));
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    universe.insert((x, y1));
                }
            } else {
                unreachable!();
            }
        }
    }
    let (mut x, mut y) = (500, 0);
    let mut p2 = 0;
    let mut p1 = 0;
    loop {
        (x, y) = if let Some(pt) = [x, x - 1, x + 1]
            .into_iter()
            .map(|x| (x, y + 1))
            .find(|pt| pt.1 <= ymax + 1 && !universe.contains(&pt))
        {
            pt
        } else {
            universe.insert((x, y));
            p2 += 1;
            if y == 0 {
                break;
            }
            (500, 0)
        };
        if y == ymax && p1 == 0 {
            p1 = p2;
        }
    }
    (p1, p2)
}

#[test]
fn t() {
    assert_eq!(
        run(r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#),
        (24, 93)
    )
}
