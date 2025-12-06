use std::collections::HashMap;

fn get_letters() -> HashMap<char, usize> {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>()
}

fn part1(input: &str) -> usize {
    let letters = get_letters();
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let common_char = a.chars().find(|c| b.contains(*c)).unwrap();
            letters.get(&common_char).unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let letters = get_letters();

    let groups: Vec<_> = input.lines().collect();

    groups
        .chunks(3)
        .into_iter()
        .map(|group| {
            let common_char = group[0]
                .chars()
                .find(|c: &char| group[1].contains(*c) && group[2].contains(*c))
                .unwrap();
            letters.get(&common_char).unwrap()
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
