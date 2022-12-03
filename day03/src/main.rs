fn main() {
    let bags: Vec<Vec<usize>> = std::fs::read_to_string("input")
        .unwrap()
        .trim()
        .lines()
        .map(|bag| {
            bag.trim()
                .bytes()
                .map(|c| {
                    (if c <= b'Z' {
                        27 + c - b'A'
                    } else {
                        1 + c - b'a'
                    }) as usize
                })
                .collect()
        })
        .collect();
    let p1 = bags
        .iter()
        .map(|bag| {
            let (left, right) = (&bag[..bag.len() / 2], &bag[bag.len() / 2..]);
            left.iter().find(|c| right.contains(c)).unwrap()
        })
        .sum::<usize>();
    dbg!(p1);
    let p2 = bags
        .chunks(3)
        .map(|triple| {
            *triple[0]
                .iter()
                .find(|c| triple[1].contains(c) && triple[2].contains(c)).unwrap()
        })
        .sum::<usize>();
    dbg!(p2);
}
