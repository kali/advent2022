fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let states = run(&input);
    dbg!(p1(&states));
    p2(&states);
}

fn run(input: &str) -> Vec<isize> {
    let mut states = vec![1];
    for line in input.trim().lines() {
        let state = *states.last().unwrap();
        states.push(state);
        if let Some((_, i)) = line.split_once(' ') {
            states.push(state + i.parse::<isize>().unwrap());
        }
    }
    states
}

fn p1(states: &[isize]) -> isize {
    [20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&i| i as isize * states[i - 1])
        .sum()
}

fn p2(states: &[isize]) {
    for line in 0..6 {
        for pixel in 0..40 {
            if (states[line * 40 + pixel] - pixel as isize).abs() < 2 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[test]
fn t1() {
    let input = std::fs::read_to_string("test-input").unwrap();
    assert_eq!(p1(&run(&input)), 13140);
}
