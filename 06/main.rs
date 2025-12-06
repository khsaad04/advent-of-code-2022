fn part1(input: &str) -> String {
    let result: usize = input
        .as_bytes()
        .windows(4)
        .position(|w| {
            let mut arr = [0u8; 4];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 4)
        .unwrap();
    result.to_string()
}

fn part2(input: &str) -> String {
    let result: usize = input
        .as_bytes()
        .windows(14)
        .position(|w| {
            let mut arr = [0u8; 14];
            let mut idx = 0;
            for x in w {
                for i in 0..idx {
                    if arr[i] == *x {
                        return false;
                    }
                }
                arr[idx] = *x;
                idx += 1;
            }
            return true;
        })
        .map(|x| x + 14)
        .unwrap();
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
