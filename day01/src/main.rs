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
    elves.sort();
    elves.reverse();
    println!("p1: {:?}", elves[0]);
    println!("p2: {:?}", elves.iter().take(3).sum::<usize>());
}
