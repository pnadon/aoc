use std::error::Error;
use std::fs::read_to_string;

mod day_2;
mod day_3;

fn main() -> Result<(), Box<dyn Error>> {
  dbg!(day_2::solve(&read_to_string("y2021/inputs/2.txt")?));
  dbg!(day_3::solve_part1(&read_to_string("y2021/inputs/3.txt")?)?);
  dbg!(day_3::solve_part2(&read_to_string("y2021/inputs/3.txt")?)?);

  Ok(())
}
