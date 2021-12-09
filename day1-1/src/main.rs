use anyhow::Context;
use itertools::Itertools;
use std::num::ParseIntError;

fn main() -> anyhow::Result<()> {
    let num_increases = count_increases(include_str!("input.txt"))?;
    println!(
        "the number of times a depth measurement increases: {}",
        num_increases
    );

    Ok(())
}

fn parse(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.lines().map(|line| line.parse()).collect()
}

pub fn count_increases(input: &str) -> anyhow::Result<usize> {
    let lines = parse(input).context("Failed to parse input")?;

    let increases = lines
        .into_iter()
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count();

    Ok(increases)
}

#[cfg(test)]
mod tests {
    use super::count_increases;
    use claim::assert_ok_eq;

    #[test]
    fn test() {
        assert_ok_eq!(count_increases(include_str!("example.txt")), 7);
    }
}
