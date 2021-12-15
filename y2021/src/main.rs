use std::error::Error;
use std::fs::{read_to_string, File};

mod days;
use days::*;

mod helpers;

fn main() -> Result<(), Box<dyn Error>> {
  // dbg!(day_2::solve(&read_to_string("y2021/inputs/2.txt")?));
  // dbg!(day_3::solve_part1(&read_to_string("y2021/inputs/3.txt")?)?);
  // dbg!(day_3::solve_part2(&read_to_string("y2021/inputs/3.txt")?)?);
  // dbg!(day_4::solve_part1(File::open("y2021/inputs/4.txt")?)?);
  // dbg!(day_4::solve_part2(File::open("y2021/inputs/4.txt")?)?);
  // dbg!(day_5a::solve_part1(File::open("y2021/inputs/5.txt")?)?);
  // dbg!(day_5a::solve_part2(File::open("y2021/inputs/5.txt")?)?);
  // dbg!(day_6::solve_part1(File::open("y2021/inputs/6.txt")?)?);
  // dbg!(day_6::solve_part2(File::open("y2021/inputs/6.txt")?)?);
  // dbg!(day_7::solve_part1(File::open("y2021/inputs/7.txt")?)?);
  // dbg!(day_7::solve_part2(File::open("y2021/inputs/7.txt")?)?);
  // dbg!(day_8::solve_part1(File::open("y2021/inputs/8.txt")?)?);
  // dbg!(day_8::solve_part2(File::open("y2021/inputs/8.txt")?)?);
  // dbg!(day_9::solve_part1(File::open("y2021/inputs/9.txt")?)?);
  // dbg!(day_9::solve_part2(File::open("y2021/inputs/9.txt")?)?);
  // dbg!(day_10::solve_part1(File::open("y2021/inputs/10.txt")?));
  // dbg!(day_10::solve_part2(File::open("y2021/inputs/10.txt")?));
  dbg!(day_11::solve_part1(File::open("y2021/inputs/11.txt")?));
  dbg!(day_11::solve_part2(File::open("y2021/inputs/11.txt")?));

  Ok(())
}
