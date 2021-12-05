fn main() {
    let num_increases = count_increases(include_str!("input.txt"));
    println!(
        "the number of times a depth measurement increases: {}",
        num_increases
    );
}

fn count_increases(input: &str) -> u32 {
    let heights: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    heights
        .windows(4)
        .map(|window| {
            let sum1 = window[0] + window[1] + window[2];
            let sum2 = window[1] + window[2] + window[3];
            if sum2 > sum1 {
                1
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test() {
    assert_eq!(count_increases(include_str!("example.txt")), 5);
}
