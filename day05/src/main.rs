fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (stacks, steps) = input.split_once("\n\n").unwrap();
    let mut iter = stacks.lines().rev();
    let count = iter
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut state = vec![vec!(); count];
    for line in iter {
        for i in 0..state.len() {
            let Some(b) = line.as_bytes().get(1 + i * 4) else { continue };
            if b.is_ascii_alphabetic() {
                state[i].push(*b as char);
            }
        }
    }
    let steps = steps
        .lines()
        .map(|l| {
            let v = l
                .split(' ')
                .filter_map(|t| t.parse().ok())
                .collect::<Vec<usize>>();
            (v[0], v[1] - 1, v[2] - 1)
        })
        .collect::<Vec<_>>();
    let initial = state.clone();
    for &(count, from, to) in &steps {
        for _ in 0..count {
            let it = state[from].pop().unwrap();
            state[to].push(it);
        }
    }
    let p1 = state.iter().map(|s| s.last().unwrap()).collect::<String>();
    dbg!(p1);

    let mut state = initial;
    for &(count, from, to) in &steps {
        let (keep, mv) = state[from].split_at(state[from].len() - count);
        let (keep, mv) = (keep.to_vec(), mv.to_vec());
        state[from] = keep;
        state[to].extend(mv);
    }
    let p2 = state.iter().map(|s| s.last().unwrap()).collect::<String>();
    dbg!(p2);
}
