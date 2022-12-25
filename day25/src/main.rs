use std::iter::repeat;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> String {
    let mut sum = vec![0i8; 100];
    for line in input.trim().lines() {
        let mut carry = 0;
        for (acc, c) in sum.iter_mut().zip(line.bytes().rev().chain(repeat(b'0'))) {
            let s = *acc + carry + "=-012".bytes().position(|x| x == c).unwrap() as i8 - 2;
            (carry, *acc) = if s > 2 {
                (1, s - 5)
            } else if s < -2 {
                (-1, s + 5)
            } else {
                (0, s)
            };
        }
    }
    sum.into_iter()
        .rev()
        .skip_while(|c| *c == 0)
        .map(|d| "=-012".as_bytes()[(d + 2) as usize] as char)
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
