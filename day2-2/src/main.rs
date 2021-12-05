use anyhow::{anyhow, Context};
use std::str::FromStr;

#[cfg(test)]
use claim::assert_ok_eq;

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

fn main() -> anyhow::Result<()> {
    let (horizontal, depth) = calc_position(include_str!("input.txt"))?;
    dbg!(horizontal);
    dbg!(depth);

    let solution = horizontal * depth;
    dbg!(solution);

    Ok(())
}

fn calc_position(input: &str) -> anyhow::Result<(u32, u32)> {
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

#[test]
fn test() {
    assert_ok_eq!(calc_position(include_str!("example.txt")), (15, 60));
}
