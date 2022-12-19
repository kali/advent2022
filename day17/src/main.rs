use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input, 2022));
    dbg!(run(&input, 1000000000000));
}

fn height(tower: &[[bool; 7]]) -> usize {
    heights(tower).into_iter().max().unwrap()
}

fn heights(tower: &[[bool; 7]]) -> [usize; 7] {
    (0..7)
        .map(|x| {
            tower.len()
                - tower
                    .iter()
                    .rev()
                    .position(|line| line[x])
                    .unwrap_or(tower.len())
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn hull(tower: &[[bool; 7]]) -> [usize; 7] {
    let mut heights = heights(tower);
    let max = heights.iter().copied().max().unwrap();
    for h in heights.iter_mut() {
        *h = max - *h
    }
    heights
}

fn run(input: &str, pieces: usize) -> usize {
    let dash: Vec<(isize, usize)> = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus: Vec<(isize, usize)> = vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)];
    let el: Vec<(isize, usize)> = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let bar: Vec<(isize, usize)> = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let square: Vec<(isize, usize)> = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
    let shapes = vec![dash, plus, el, bar, square];
    let winds = input.trim().as_bytes();

    let mut tower: Vec<[bool; 7]> = vec![];
    let mut wind = 0;
    let mut history: HashMap<([usize; 7], usize, usize), (usize, usize)> = Default::default();
    let mut end = None;
    let mut from_cycle = 0;
    let mut piece = 0;
    loop {
        if Some(piece) == end || piece == pieces {
            break;
        }
        let h = height(&tower);
        while tower.len() < h + 10 {
            tower.push([false; 7]);
        }
        let (mut x, mut y) = (2isize, h + 3);
        let shape = &shapes[piece % shapes.len()];
        loop {
            let mov = if winds[wind % winds.len()] == b'<' {
                -1
            } else {
                1
            };
            if shape.iter().all(|(dx, dy)| {
                x + dx + mov >= 0
                    && x + dx + mov < 7
                    && !tower[(y + dy) as usize][(x + dx + mov) as usize]
            }) {
                x += mov;
            }
            wind = (wind + 1) % winds.len();
            if shape
                .iter()
                .all(|(dx, dy)| (y + dy) > 0 && !tower[(y + dy) as usize - 1][(x + dx) as usize])
            {
                y -= 1;
            } else {
                for &(dx, dy) in shape {
                    tower[(y + dy) as usize][(x + dx) as usize] = true;
                }
                if end.is_none() {
                    if let Some(before) = history.insert(
                        (hull(&tower), piece % shapes.len(), wind % winds.len()),
                        (piece, height(&tower)),
                    ) {
                        let done = piece;
                        let pieces_in_loop = piece - before.0;
                        let remain = pieces - done;
                        let cycles = (remain / pieces_in_loop) - 1;
                        end = Some(pieces - cycles * pieces_in_loop);
                        from_cycle = cycles * (height(&tower) - before.1);
                    }
                }
                break;
            }
        }
        piece += 1;
    }
    height(&tower) + from_cycle
}

#[cfg(test)]
#[test]
fn t() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(run(&input, 2022), 3068);
    assert_eq!(run(&input, 1000000000000), 1514285714288);
}
