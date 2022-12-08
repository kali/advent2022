use std::iter::once;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(input.trim()));
}

fn view(tree: u8, iter: impl Iterator<Item = u8>) -> usize {
    for (ix, t) in iter.chain(once(u8::MAX)).enumerate() {
        if t == tree {
            return ix + 1;
        } else if tree < t {
            return ix;
        }
    }
    unreachable!()
}

fn run(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let (height, width) = (grid.len(), grid[0].len());
    let mut p1 = 0;
    let mut p2 = 0;
    for x in 0..width {
        for y in 0..height {
            let t = grid[y][x];
            if grid[y][0..x].iter().max().cloned().unwrap_or(0) < t
                || grid[y][x + 1..].iter().max().cloned().unwrap_or(0) < t
                || grid[0..y].iter().map(|l| l[x]).max().unwrap_or(0) < t
                || grid[y + 1..].iter().map(|l| l[x]).max().unwrap_or(0) < t
            {
                p1 += 1;
            }
            let left = view(t, grid[y][0..x].iter().rev().copied());
            let right = view(t, grid[y][x + 1..].iter().copied());
            let up = view(t, grid[0..y].iter().rev().map(|l| l[x]));
            let down = view(t, grid[y + 1..].iter().map(|l| l[x]));
            p2 = p2.max(up * down * left * right);
        }
    }
    (p1, p2)
}

#[test]
fn t1() {
    assert_eq!(
        run(r#"30373
25512
65332
33549
35390"#)
        .1,
        8
    );
}
