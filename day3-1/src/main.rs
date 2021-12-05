use anyhow::anyhow;

#[cfg(test)]
use claim::assert_ok_eq;

fn main() -> anyhow::Result<()> {
    let (gamma, epsilon) = calc_rates(include_str!("input.txt"))?;
    dbg!(gamma);
    dbg!(epsilon);

    let solution = gamma * epsilon;
    dbg!(solution);

    Ok(())
}

fn calc_rates(input: &str) -> anyhow::Result<(u32, u32)> {
    let first_line = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("Failed to read first line"))?;

    let mut num_lines = 0;
    let mut ones = vec![0; first_line.len()];

    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            if char == '1' {
                ones[index] += 1;
            }
        }

        num_lines += 1;
    }

    let gamma = ones
        .iter()
        .map(|num_ones| {
            if num_ones > &(num_lines - num_ones) {
                1
            } else {
                0
            }
        })
        .enumerate()
        .map(|(index, digit)| digit << (ones.len() - index - 1))
        .sum();

    let epsilon = ones
        .iter()
        .map(|num_ones| {
            if num_ones > &(num_lines - num_ones) {
                0
            } else {
                1
            }
        })
        .enumerate()
        .map(|(index, digit)| digit << (ones.len() - index - 1))
        .sum();

    Ok((gamma, epsilon))
}

#[test]
fn test() {
    assert_ok_eq!(calc_rates(include_str!("example.txt")), (22, 9));
}
