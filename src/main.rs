mod day1;
mod day2;
mod day3;

fn main() -> anyhow::Result<()> {
    day1::run()?;
    println!();

    day2::run()?;
    println!();

    day3::run()?;
    println!();

    Ok(())
}
