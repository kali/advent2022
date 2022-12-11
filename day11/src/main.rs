#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Monkey {
    op: Op,
    operand: Option<usize>,
    divide: usize,
    then: usize,
    or_else: usize,
}

fn play(
    initial: &[Vec<usize>],
    rules: &[Monkey],
    rounds: usize,
    anxiety: impl Fn(usize) -> usize,
) -> u64 {
    let mut state: Vec<Vec<usize>> = initial.to_vec();
    let mut scores = vec![0; rules.len()];
    for _round in 0..rounds {
        for (ix, monkey) in rules.iter().enumerate() {
            while let Some(item) = state[ix].pop() {
                scores[ix] += 1;
                let item = match monkey.op {
                    Op::Add => monkey.operand.unwrap_or(item) + item,
                    Op::Mul => monkey.operand.unwrap_or(item) * item,
                };
                let item = anxiety(item);
                let dest = if item % monkey.divide == 0 {
                    monkey.then
                } else {
                    monkey.or_else
                };
                state[dest].push(item);
            }
        }
    }
    scores.sort();
    scores.reverse();
    scores[0] * scores[1]
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut rules = vec![];
    let mut initial = vec![];
    for monkey in input.trim().split("\n\n") {
        let mut lines = monkey.lines();
        let items: Vec<usize> = lines
            .nth(1)
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        initial.push(items);
        let op_line: Vec<&str> = lines.next().unwrap().trim().split(" ").collect();
        let mut number_at_end =
            lines.map(|line| line.trim().split(" ").last().unwrap().parse().unwrap());
        rules.push(Monkey {
            op: if op_line[4] == "*" { Op::Mul } else { Op::Add },
            operand: op_line[5].parse().ok(),
            divide: number_at_end.next().unwrap(),
            then: number_at_end.next().unwrap(),
            or_else: number_at_end.next().unwrap(),
        })
    }

    dbg!(play(&initial, &rules, 20, |x| x / 3));
    let ring = rules.iter().map(|m| m.divide).product::<usize>();
    dbg!(play(&initial, &rules, 10000, |x| x % ring));
}
