fn parse(input: &mut std::str::Lines) -> Vec<u64> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", dir]) if *dir != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);
    subdirs
}

fn part1(input: &str) -> String {
    let result: u64 = parse(&mut input.lines())
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum();
    result.to_string()
}

fn part2(input: &str) -> String {
    let mut sizes = parse(&mut input.lines());
    let missing = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    sizes.sort_unstable();
    let result = sizes.into_iter().find(|&s| s >= missing).unwrap();
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
