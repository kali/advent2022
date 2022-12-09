use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input, 1));
    dbg!(run(&input, 9));
}

fn run(input: &str, rope_len: usize) -> usize {
    let mut rope = vec![(0, 0); rope_len + 1];
    let mut visited = HashSet::new();
    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();
        let dir = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!(),
        };
        for _ in 0..count {
            rope[0] = (rope[0].0 + dir.0, rope[0].1 + dir.1);
            for knot in 1..=rope_len {
                let prev = rope[knot - 1];
                let curr = &mut rope[knot];
                let diff: (i32, i32) = (prev.0 - curr.0, prev.1 - curr.1);
                if diff.0.abs() > 1 || diff.1.abs() > 1 {
                    curr.0 = curr.0 + diff.0.signum();
                    curr.1 = curr.1 + diff.1.signum();
                }
            }
            visited.insert(rope[rope_len]);
        }
    }
    visited.len()
}

#[test]
fn t() {
    assert_eq!(
        run(
            r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#,
            9
        ),
        36
    )
}
