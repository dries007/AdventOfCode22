mod day1;
mod day2;
mod helpers;

use helpers::Result;

fn main() -> Result<()> {
    day1::day1()?;
    day2::day2()?;

    return Ok(());
}
