use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let p1 = input
        .as_bytes()
        .windows(4)
        .position(|it| {
            if let [a, b, c, d] = it {
                a != b && a != c && a != d && b != c && b != d && c != d
            } else {
                panic!()
            }
        })
        .unwrap()
        + 4;
    dbg!(p1);
    let p2 = input
        .as_bytes()
        .windows(14)
        .position(|it| it.iter().collect::<HashSet<_>>().len() == 14)
        .unwrap()
        + 14;
    dbg!(p2);
}
