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
        let (command, num) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Missing space delimiter"))?;

        let num = num.parse()?;

        match command {
            "forward" => Ok(Self::Forward(num)),
            "up" => Ok(Self::Up(num)),
            "down" => Ok(Self::Down(num)),
            _ => Err(anyhow!("Unknown command: {}", command)),
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
        let action: Action = line
            .parse()
            .with_context(|| format!("Failed to parse action: {}", line))?;

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
    use super::{calc_position, calc_position2};
    use claim::assert_ok_eq;

    #[test]
    fn part_1() {
        assert_ok_eq!(
            calc_position(include_str!("../inputs/example-2.txt")),
            (15, 10)
        );
    }

    #[test]
    fn part_2() {
        assert_ok_eq!(
            calc_position2(include_str!("../inputs/example-2.txt")),
            (15, 60)
        );
    }
}
