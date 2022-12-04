mod helpers;
mod day1;
mod day2;
mod day3;
mod day4;

use helpers::Result;

fn main() -> Result<()> {
    day1::day1()?;
    day2::day2()?;
    day3::day3()?;
    day4::day4()?;

    return Ok(());
}
