fn main() {
    let lines: Vec<(usize, usize, usize, usize)> = std::fs::read_to_string("input")
        .unwrap()
        .lines()
        .map(|line| {
            let mut vs = line.split(&[',', '-']).map(|v| v.parse::<usize>().unwrap());
            (
                vs.next().unwrap(),
                vs.next().unwrap(),
                vs.next().unwrap(),
                vs.next().unwrap(),
            )
        })
        .collect();
    let p1 = lines
        .iter()
        .filter(|(s1, e1, s2, e2)| (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2))
        .count();
    dbg!(p1);
    let p2 = lines
        .iter()
        .filter(|(s1, e1, s2, e2)| (*s1..=*e1).any(|x| (*s2..=*e2).contains(&x)))
        .count();
    dbg!(p2);
}
