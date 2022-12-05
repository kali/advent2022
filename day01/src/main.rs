fn main() {
    let mut elves: Vec<usize> = std::fs::read_to_string("input")
        .unwrap()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(|s| s.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect();
    println!("p1: {:?}", elves.iter().max().unwrap());
    elves.select_nth_unstable_by_key(3, |x| -(*x as isize));
    println!("p2: {:?}", elves.iter().take(3).sum::<usize>());
}
