fn part1(input: &str) -> String {
    let edge = &input.lines().count() * 2 + input.lines().last().unwrap().len() - 4;
    let result = edge;
    result.to_string()
}

fn part2(input: &str) -> String {
    let result = 0;
    result.to_string()
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
