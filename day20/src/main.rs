fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let initial: Vec<isize> = input.trim().lines().map(|l| l.parse().unwrap()).collect();
    dbg!(p1(&initial));
    dbg!(p2(&initial));
}

fn mix(input: &[isize], turns: usize) -> Vec<isize> {
    let mut mixed: Vec<(usize, isize)> = input.iter().cloned().enumerate().collect();
    let len = mixed.len();
    for _ in 0..turns {
        for i in 0..len {
            let pos = mixed.iter().position(|p| p.0 == i).unwrap();
            let value = mixed[pos].1;
            if value >= 0 {
                for j in 0..(value as usize % (len - 1)) {
                    mixed.swap((pos + j) % len, (pos + j + 1) % len);
                }
            } else {
                for j in 0..(-value as usize % (len - 1)) {
                    mixed.swap((pos + len - j % len) % len, (pos + len - j % len - 1) % len);
                }
            }
        }
    }
    mixed.iter().map(|p| p.1).collect()
}

fn extract(mixed: &[isize]) -> isize {
    let len = mixed.len();
    let zero_pos = mixed.iter().position(|p| *p == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|x| mixed[(x + zero_pos) % len])
        .sum::<isize>()
}

fn p1(initial: &[isize]) -> isize {
    extract(&mix(initial, 1))
}

fn p2(initial: &[isize]) -> isize {
    let keyed = initial.iter().map(|x| x * 811589153).collect::<Vec<_>>();
    let mixed = mix(&keyed, 10);
    extract(&mixed)
}

#[test]
fn tp1() {
    let input = [1, 2, -3, 3, -2, 0, 4];
    assert_eq!(p1(&input), 3);
}

#[test]
fn tp2() {
    let input = [1, 2, -3, 3, -2, 0, 4];
    assert_eq!(p2(&input), 1623178306);
}
