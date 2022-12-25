fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> String {
    let mut sum = vec![0i8; 0];
    for line in input.trim().lines() {
        let digits = sum.len().max(line.as_bytes().len()) + 1;
        while sum.len() < digits {
            sum.push(0);
        }
        let mut carry = 0;
        for ix in 0..digits {
            let c = line.bytes().rev().nth(ix).unwrap_or(b'0');
            let v = "=-012".bytes().position(|x| x == c).unwrap() as i8 - 2;
            let s = v + sum[ix] + carry;
            let (car, d) = if s > 2 {
                (1, s - 5)
            } else if s < -2 {
                (-1, s + 5)
            } else {
                (0, s)
            };
            sum[ix] = d;
            carry = car;
        }
        while sum.last().copied() == Some(0) {
            sum.pop();
        }
    }
    sum.iter()
        .rev()
        .map(|d| "=-012".as_bytes()[(*d + 2) as usize] as char)
        .collect()
}

#[test]
fn t() {
    let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
    assert_eq!(run(input), "2=-1=0");
}

#[test]
fn t0() {
    /*
     *     1
     * 1=-0-2
     *  12111
     * ------
     * 1-111=
     */
    let input = "1=-0-2\n12111";
    assert_eq!(run(input), "1-111=");
}
