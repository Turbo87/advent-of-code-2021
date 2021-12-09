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

    #[derive(Default, Copy, Clone)]
    struct Counter {
        zeros: usize,
        ones: usize,
    }

    let mut counters = vec![Counter::default(); first_line.len()];

    for line in input.lines() {
        for (index, char) in line.chars().enumerate() {
            let mut counter = counters
                .get_mut(index)
                .ok_or_else(|| anyhow!("Unexpected line length: {}", line))?;

            if char == '0' {
                counter.zeros += 1;
            } else if char == '1' {
                counter.ones += 1;
            }
        }
    }

    let gamma = counters
        .iter()
        .map(|counter| if counter.ones > counter.zeros { 1 } else { 0 })
        .fold(0, |acc, x| (acc << 1) | x);

    let epsilon = counters
        .iter()
        .map(|counter| if counter.ones > counter.zeros { 0 } else { 1 })
        .fold(0, |acc, x| (acc << 1) | x);

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
