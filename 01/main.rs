fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut result: Vec<_> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<usize>().unwrap()).sum())
        .collect();
    result.sort();
    result.iter().rev().take(3).sum()
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
