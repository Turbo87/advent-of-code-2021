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
        .windows(2)
        .map(|window| if window[1] > window[0] { 1 } else { 0 })
        .sum()
}

#[test]
fn test() {
    assert_eq!(count_increases(include_str!("example.txt")), 7);
}
