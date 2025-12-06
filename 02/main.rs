fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|turn| {
            let moves: Vec<&str> = turn.split(' ').collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => 1 + 3,
                    "Y" => 2 + 6,
                    "Z" => 3,
                    _ => unreachable!(),
                },
                "B" => match moves[1] {
                    "X" => 1,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => unreachable!(),
                },
                "C" => match moves[1] {
                    "X" => 1 + 6,
                    "Y" => 2,
                    "Z" => 3 + 3,
                    _ => 0,
                },
                _ => unreachable!(),
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|turn| {
            let moves: Vec<&str> = turn.split(' ').collect();
            match moves[0] {
                "A" => match moves[1] {
                    "X" => 3,
                    "Y" => 1 + 3,
                    "Z" => 2 + 6,
                    _ => unreachable!(),
                },
                "B" => match moves[1] {
                    "X" => 1,
                    "Y" => 2 + 3,
                    "Z" => 3 + 6,
                    _ => unreachable!(),
                },
                "C" => match moves[1] {
                    "X" => 2,
                    "Y" => 3 + 3,
                    "Z" => 1 + 6,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
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
