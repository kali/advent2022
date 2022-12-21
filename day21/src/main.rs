use evalexpr::{eval_int_with_context_mut, Context, HashMapContext};

fn solve(equations: &[String], target: &str) -> i64 {
    let mut ctx = HashMapContext::new();
    loop {
        for line in equations {
            let _ = eval_int_with_context_mut(&line, &mut ctx);
        }
        if let Some(target) = ctx.get_value(target) {
            return target.as_int().unwrap();
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let equations: Vec<String> = input.lines().map(|l| l.replace(":", " =")).collect();
    dbg!(solve(&equations, "root"));
    let mut root = equations
        .iter()
        .find(|l| l.starts_with("root"))
        .unwrap()
        .split_whitespace();
    let (a, b) = (root.nth(2).unwrap(), root.nth(1).unwrap());
    let root_exprs = [format!("{} = {}", a, b), format!("{} = {}", b, a)];
    let exprs: Vec<String> = equations
        .clone()
        .into_iter()
        .filter(|l| !l.starts_with("root") && !l.starts_with("humn"))
        .chain(
            equations
                .into_iter()
                .filter(|l| !l.starts_with("root") && !l.starts_with("humn"))
                .filter(|l| !l.chars().last().unwrap().is_digit(10))
                .flat_map(|eq| {
                    {
                        let t: Vec<&str> = eq.split_whitespace().collect();
                        let (r, a, op, b) = (t[0], t[2], t[3], t[4]);
                        match op {
                            "+" => vec![format!("{a} = {r} - {b}"), format!("{b} = {r} - {a}")],
                            "-" => vec![format!("{a} = {r} + {b}"), format!("{b} = {a} - {r}")],
                            "*" => vec![format!("{a} = {r} / {b}"), format!("{b} = {r} / {a}")],
                            "/" => vec![format!("{a} = {r} * {b}"), format!("{b} = {a} / {r}")],
                            _ => panic!(),
                        }
                    }
                    .into_iter()
                }),
        )
        .chain(root_exprs.into_iter())
        .collect();
    dbg!(solve(&exprs, "humn"));
}
