type Stack = Vec<Vec<char>>;
type Move = (usize, usize, usize);
type Input = (Stack, Vec<Move>);

fn parse(input: &str) -> Input {
    let (stack_str, moves_str) = input.split_once("\n\n").unwrap();

    let mut stack_iter = stack_str.lines().rev();
    let mut stack = vec![vec![]; stack_iter.next().unwrap().len() / 4 + 1];

    stack_iter.for_each(|l| {
        l.chars().skip(1).enumerate().for_each(|(i, c)| {
            if i % 4 == 0 && c != ' ' {
                stack[i / 4].push(c);
            }
        });
    });

    let moves = moves_str
        .lines()
        .filter_map(|l| {
            let s: Vec<&str> = l.split_ascii_whitespace().collect();
            Some((s[1].parse().ok()?, s[3].parse().ok()?, s[5].parse().ok()?))
        })
        .collect();

    (stack, moves)
}

fn move_stacks(stack: &mut Stack, moves: &[Move], version: u16) {
    moves.iter().for_each(|(amount, from, to)| {
        let from = &mut stack[*from - 1];
        let crates = from.split_off(from.len() - amount);
        if version == 9000 {
            stack[*to - 1].extend(crates.iter().rev());
        } else {
            stack[*to - 1].extend(crates.iter());
        }
    });
}

fn get_top_row(stack: &Stack) -> String {
    let mut res = String::new();
    for ch in stack.iter().filter_map(|m| m.last()) {
        res.push(*ch);
    }
    res
}

fn part1(input: Input) -> String {
    let (mut stack, moves) = input;
    move_stacks(&mut stack, &moves, 9000);
    Some(get_top_row(&stack)).unwrap()
}

fn part2(input: Input) -> String {
    let (mut stack, moves) = input;
    move_stacks(&mut stack, &moves, 9001);
    Some(get_top_row(&stack)).unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    let _ = args.next();
    let input_file = args.next().unwrap();
    let input = std::fs::read_to_string(input_file)?;

    let result = part1(parse(&input));
    println!("Part 1: {result}");

    let result = part2(parse(&input));
    println!("Part 2: {result}");

    Ok(())
}
