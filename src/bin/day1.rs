use aoc2021::read_lines;

fn part1() -> std::io::Result<()> {
    let current_iter = read_lines("res/day1.txt")?;
    let next_iter = read_lines("res/day1.txt")?.skip(1);

    println!(
        "{:?}",
        current_iter
            .zip(next_iter)
            .filter(|(current, next)| {
                match (current, next) {
                    (Ok(current_num), Ok(next_num)) => {
                        next_num.parse::<i32>().unwrap() > current_num.parse::<i32>().unwrap()
                    }
                    (_, _) => false,
                }
            })
            .count()
    );
    Ok(())
}

fn part2() -> std::io::Result<usize> {
    let lines: Vec<i32> = read_lines("res/day1.txt")?
        .map(|num| num.unwrap().parse::<i32>().unwrap())
        .collect();

    let current_iter = lines.windows(3);
    let next_iter = lines.windows(3).skip(1);

    let result = current_iter
        .zip(next_iter)
        .filter(|(current_window, next_window)| {
            next_window.into_iter().sum::<i32>() > current_window.into_iter().sum()
        })
        .count();

    Ok(result)
}

fn main() -> std::io::Result<()> {
    println!("{}", part2()?);
    Ok(())
}
