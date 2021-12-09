use anyhow::{anyhow, Context};
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("forward ") {
            let num = num
                .parse()
                .context("Failed to parse action argument as number")?;

            Ok(Self::Forward(num))
        } else if let Some(num) = s.strip_prefix("up ") {
            let num = num
                .parse()
                .context("Failed to parse action argument as number")?;

            Ok(Self::Up(num))
        } else if let Some(num) = s.strip_prefix("down ") {
            let num = num
                .parse()
                .context("Failed to parse action argument as number")?;

            Ok(Self::Down(num))
        } else {
            Err(anyhow!("Failed to parse action"))
        }
    }
}

pub fn run() -> anyhow::Result<()> {
    let (horizontal, depth) = calc_position(include_str!("../inputs/input-2.txt"))?;
    dbg!(horizontal);
    dbg!(depth);

    let result_1 = horizontal * depth;
    dbg!(result_1);

    let (horizontal, depth) = calc_position2(include_str!("../inputs/input-2.txt"))?;
    dbg!(horizontal);
    dbg!(depth);

    let result_2 = horizontal * depth;
    dbg!(result_2);

    Ok(())
}

fn calc_position(input: &str) -> anyhow::Result<(u32, u32)> {
    let mut horizontal = 0;
    let mut depth = 0;

    for line in input.lines() {
        let action: Action = line.parse()?;
        match action {
            Action::Forward(num) => horizontal += num,
            Action::Down(num) => depth += num,
            Action::Up(num) => depth -= num,
        }
    }

    Ok((horizontal, depth))
}

fn calc_position2(input: &str) -> anyhow::Result<(u32, u32)> {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in input.lines() {
        let action: Action = line.parse()?;
        match action {
            Action::Forward(num) => {
                horizontal += num;
                depth += num * aim
            }
            Action::Down(num) => aim += num,
            Action::Up(num) => aim -= num,
        }
    }

    Ok((horizontal, depth))
}

#[cfg(test)]
mod tests {
    use claim::assert_ok_eq;
    use super::{calc_position,calc_position2};

    #[test]
    fn part_1() {
        assert_ok_eq!(calc_position(include_str!("../inputs/example-2.txt")), (15, 10));
    }

    #[test]
    fn part_2() {
        assert_ok_eq!(calc_position2(include_str!("../inputs/example-2.txt")), (15, 60));
    }}