use aoc2021::read_lines;
use regex::Regex;
use std::io::Result;

fn part1() -> Result<usize> {
    let forward_pattern = Regex::new(r"forward (\d)").unwrap();
    let up_pattern = Regex::new(r"up (\d)").unwrap();
    let down_pattern = Regex::new(r"down (\d)").unwrap();

    let (hor_pos, depth) =
        read_lines("res/day2.txt")?.fold((0, 0), |accum, current| match current {
            Ok(line) => {
                if let Some(cap) = forward_pattern.captures(&line) {
                    (accum.0 + cap[1].parse::<usize>().unwrap(), accum.1)
                } else if let Some(cap) = up_pattern.captures(&line) {
                    (accum.0, accum.1 - cap[1].parse::<usize>().unwrap())
                } else if let Some(cap) = down_pattern.captures(&line) {
                    (accum.0, accum.1 + cap[1].parse::<usize>().unwrap())
                } else {
                    accum
                }
            }
            Err(_) => accum,
        });

    Ok(hor_pos * depth)
}

fn part2() -> Result<usize> {
    let forward_pattern = Regex::new(r"forward (\d)").unwrap();
    let up_pattern = Regex::new(r"up (\d)").unwrap();
    let down_pattern = Regex::new(r"down (\d)").unwrap();

    let (hor_pos, depth, _) =
        read_lines("res/day2.txt")?.fold((0, 0, 0), |accum, current| match current {
            Ok(line) => {
                if let Some(cap) = forward_pattern.captures(&line) {
                    let val = cap[1].parse::<usize>().unwrap();
                    (accum.0 + val, accum.1 + accum.2 * val, accum.2)
                } else if let Some(cap) = up_pattern.captures(&line) {
                    let val = cap[1].parse::<usize>().unwrap();
                    (accum.0, accum.1, accum.2 - val)
                } else if let Some(cap) = down_pattern.captures(&line) {
                    let val = cap[1].parse::<usize>().unwrap();
                    (accum.0, accum.1, accum.2 + val)
                } else {
                    accum
                }
            }
            Err(_) => accum,
        });

    Ok(hor_pos * depth)
}

fn main() -> Result<()> {
    println!("{:?}", part2());
    Ok(())
}
