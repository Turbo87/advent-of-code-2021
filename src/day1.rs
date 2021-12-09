use anyhow::Context;
use itertools::Itertools;
use std::num::ParseIntError;

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../inputs/input-1.txt");

    let result_1 = count_increases(input)?;
    dbg!(result_1);

    let result_2 = count_increases2(input)?;
    dbg!(result_2);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

fn count_increases(input: &str) -> anyhow::Result<usize> {
    let lines = parse(input).context("Failed to parse input")?;

    let increases = lines
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    Ok(increases)
}

fn count_increases2(input: &str) -> anyhow::Result<usize> {
    let lines = parse(input).context("Failed to parse input")?;

    let increases = lines
        .into_iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    Ok(increases)
}

#[cfg(test)]
mod tests {
    use super::{count_increases, count_increases2};
    use claim::assert_ok_eq;

    #[test]
    fn part1() {
        assert_ok_eq!(count_increases(include_str!("../inputs/example-1.txt")), 7);
    }

    #[test]
    fn part2() {
        assert_ok_eq!(count_increases2(include_str!("../inputs/example-1.txt")), 5);
    }
}
