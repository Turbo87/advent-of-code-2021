mod day1;
mod day2;

fn main() -> anyhow::Result<()> {
    day1::run()?;
    println!();

    day2::run()?;

    Ok(())
}
