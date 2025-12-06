use std::ops::RangeInclusive;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let ranges: Vec<_> = line.split(',').collect();
            let mut a = ranges[0].split('-');
            let mut b = ranges[1].split('-');
            let a1: usize = a.next().unwrap().parse().unwrap();
            let a2: usize = a.next().unwrap().parse().unwrap();
            let b1: usize = b.next().unwrap().parse().unwrap();
            let b2: usize = b.next().unwrap().parse().unwrap();
            let a_range: RangeInclusive<usize> = a1..=a2;
            let b_range: RangeInclusive<usize> = b1..=b2;
            if a_range.contains(&b1) && a_range.contains(&b2) {
                1
            } else if b_range.contains(&a1) && b_range.contains(&a2) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let ranges: Vec<_> = line.split(',').collect();
            let mut a = ranges[0].split('-');
            let mut b = ranges[1].split('-');
            let a1 = a.next().unwrap().parse::<u32>().unwrap();
            let a2 = a.next().unwrap().parse::<u32>().unwrap();
            let b1 = b.next().unwrap().parse::<u32>().unwrap();
            let b2 = b.next().unwrap().parse::<u32>().unwrap();
            let a_range: RangeInclusive<u32> = a1..=a2;
            let b_range: RangeInclusive<u32> = b1..=b2;
            if a_range.contains(&b1) | a_range.contains(&b2) {
                1
            } else if b_range.contains(&a1) | b_range.contains(&a2) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let _ = args.next();
    let input_file = args.next().unwrap();
    let input = std::fs::read_to_string(input_file)?;

    let result = part1(&input);
    println!("Part 1: {result}");

    let result = part2(&input);
    println!("Part 2: {result}");

    Ok(())
}
