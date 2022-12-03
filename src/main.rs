mod helpers;
mod day1;
mod day2;
mod day3;

use helpers::Result;

fn main() -> Result<()> {
    day1::day1()?;
    day2::day2()?;
    day3::day3()?;

    return Ok(());
}
