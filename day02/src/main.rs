fn main() {
    let lines: Vec<(usize, usize)> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let a = (line.as_bytes()[0] - b'A') as usize;
            let b = (line.as_bytes()[2] - b'X') as usize;
            (a, b)
        })
        .collect();
    let p1 = lines
        .iter()
        .map(|(them, me)| (me + 1) + ((4 + me - them) % 3) * 3)
        .sum::<usize>();
    dbg!(p1);
    let p2 = lines
        .iter()
        .map(|(them, res)| {
            let me = (them + res + 2) % 3;
            (me + 1) + ((4 + me - them) % 3) * 3
        })
        .sum::<usize>();
    dbg!(p2);
}
