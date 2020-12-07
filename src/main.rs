type Result = std::result::Result<(), Box<dyn std::error::Error>>;

mod day1;
mod day2;
mod day3;

fn main() -> Result {
    // day1::main()?;
    // day2::main()?;
    day3::main()?;

    Ok(())
}
