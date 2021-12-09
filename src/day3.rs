use anyhow::anyhow;

const INPUT: &str = include_str!("../inputs/input-3.txt");

pub fn run() -> anyhow::Result<()> {
    let (gamma, epsilon) = calc_rates(INPUT)?;
    dbg!(gamma);
    dbg!(epsilon);

    let result_1 = gamma * epsilon;
    dbg!(result_1);

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

#[cfg(test)]
mod tests {
    use super::calc_rates;
    use claim::assert_ok_eq;

    const EXAMPLE: &str = include_str!("../inputs/example-3.txt");

    #[test]
    fn test() {
        assert_ok_eq!(calc_rates(EXAMPLE), (22, 9));
    }
}
