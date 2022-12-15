use std::ops::Range;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let sensors = parse(&input);
    dbg!(part1(&sensors, 2000000));
    dbg!(part2(&sensors, 4000000));
}

fn parse(input: &str) -> Vec<(isize, isize, isize, isize)> {
    input
        .trim()
        .lines()
        .map(|sensor| {
            scan_fmt::scan_fmt!(
                sensor,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                isize,
                isize,
                isize,
                isize
            )
            .unwrap()
        })
        .collect()
}

fn coverage(sensors: &[(isize, isize, isize, isize)], row: isize) -> Vec<Range<isize>> {
    let mut ranges = vec![];
    for &(sx, sy, bx, by) in sensors {
        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as isize;
        let length = dist - sy.abs_diff(row as isize) as isize;
        if length > 0 {
            ranges.push((sx - length)..(sx + length + 1));
        }
    }
    ranges
}

fn part1(sensors: &[(isize, isize, isize, isize)], row: isize) -> usize {
    let mut length = 0;
    let ranges = coverage(sensors, row);
    let beacons = sensors
        .iter()
        .filter(|p| p.3 == row)
        .map(|p| p.2)
        .unique()
        .collect::<Vec<_>>();
    for (start, end) in ranges
        .iter()
        .flat_map(|r| [r.start, r.end])
        .chain(beacons.iter().flat_map(|&b| [b, b+1].into_iter()))
        .sorted()
        .dedup()
        .tuple_windows()
    {
        if !beacons.contains(&start) && ranges.iter().any(|r| r.contains(&start)) {
            length += end - start;
        }
    }
    length as usize
}

fn part2(sensors: &[(isize, isize, isize, isize)], max: isize) -> isize {
    for row in 0..max {
        let ranges = coverage(sensors, row);
        for start in ranges
            .iter()
            .flat_map(|r| [r.start, r.end])
            .chain([0, max].into_iter())
            .filter(|&b| b >= 0 && b <= max)
            .sorted()
            .dedup()
        {
            if !ranges.iter().any(|r| r.contains(&start)) {
                return start * 4000000 + row;
            }
        }
    }
    unreachable!()
}

#[test]
fn t() {
    let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;
    assert_eq!(part1(&parse(&input), 10), 26);
    assert_eq!(part2(&parse(&input), 20), 56000011);
}
