use std::collections::HashMap;

fn parse(input: &str) -> HashMap<String, usize> {
    let mut current_dir = vec![];
    let mut sizes = HashMap::<String, usize>::default();
    for line in input.lines() {
        if line == "$ cd /" {
            current_dir = vec![];
        } else if line == "$ ls" || line.starts_with("dir") {
        } else if line.starts_with("$ cd") {
            let cd = line.split(' ').nth(2).unwrap();
            if cd == ".." {
                current_dir.pop();
            } else {
                current_dir.push(cd);
            }
        } else {
            let size = line.split_once(' ').unwrap().0.parse::<usize>().unwrap();
            for i in 0..=current_dir.len() {
                let key = current_dir[..i].join("/");
                *sizes.entry(key).or_default() += size;
            }
        }
    }
    sizes
}

fn p1(input: &str) -> usize {
    parse(input)
        .values()
        .filter(|s| **s <= 100000)
        .sum::<usize>()
}

fn p2(input: &str) -> usize {
    let sizes = parse(input);
    let needed = sizes[""] - 40000000;
    *sizes.values().filter(|s| **s >= needed).min().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    dbg!(p1(&input));
    dbg!(p2(&input));
}

#[test]
fn t1() {
    assert_eq!(
        p1(r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#),
        95437
    )
}
