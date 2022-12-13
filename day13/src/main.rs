use std::cmp::Ordering;
use std::fmt::Debug;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;

#[derive(Clone, PartialEq, Eq, Ord)]
enum Value {
    I(i64),
    List(Vec<Value>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        match self {
            I(i) => write!(f, "{}", i),
            List(l) => {
                write!(f, "[")?;
                for (ix, i) in l.iter().enumerate() {
                    if ix > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{:?}", i)?;
                }
                write!(f, "]")
            }
        }
    }
}

fn parse(input: &str) -> IResult<&str, Value> {
    alt((
        map(map_res(digit1, |s: &str| s.parse::<i64>()), Value::I),
        map(
            delimited(tag("["), separated_list0(tag(","), parse), tag("]")),
            Value::List,
        ),
    ))(input)
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Value::*;
        match (self, other) {
            (I(l), I(r)) => l.partial_cmp(&r),
            (List(l), List(r)) => Some(l
                .iter()
                .zip(r.iter())
                .map(|(l, r)| l.partial_cmp(r).unwrap())
                .find(|&it| it != Ordering::Equal)
                .unwrap_or(l.partial_cmp(&r).unwrap())),
            (I(_), List(_)) => List(vec![self.clone()]).partial_cmp(other),
            (List(_), I(_)) => self.partial_cmp(&List(vec![other.clone()])),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(run(&input));
}

fn run(input: &str) -> (usize, usize) {
    let pairs: Vec<(Value, Value)> = input
        .trim()
        .split("\n\n")
        .map(|l| {
            let (left, right) = l.split_once("\n").unwrap();
            (parse(left).unwrap().1, parse(right).unwrap().1)
        })
        .collect();
    let p1: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(ix, (l, r))| {
            if l <= r && !(l<r)  {
                println!("{:?}\n{:?}", l, r);
            }
            if l <= r {
                Some(ix + 1)
            } else {
                None
            }
        })
        .sum();
    let a = parse("[[2]]").unwrap().1;
    let b = parse("[[6]]").unwrap().1;
    let mut packets:Vec<Value> = pairs.into_iter().flat_map(|p| [p.0, p.1]).collect();
    packets.push(a.clone());
    packets.push(b.clone());
    packets.sort();
    let a = packets.iter().position(|x| x == &a).unwrap() + 1;
    let b = packets.iter().position(|x| x == &b).unwrap() + 1;
    (p1, a*b)
}

#[test]
fn t() {
    let input = r#"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
    assert_eq!(run(&input), (13, 140));
}
