use clap::Parser;
use std::error::Error;
use std::fs::{read_to_string, File};

mod days;
use days::*;

mod helpers;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
  #[clap(short, long, default_value_t = 99)]
  day: usize,
  #[clap(short, long, default_value_t = 1)]
  part: usize,
}

#[derive(Debug)]
enum Answer {
  ISize(isize),
  USize(usize),
  U128(u128),
}

impl From<isize> for Answer {
  fn from(item: isize) -> Self {
    Self::ISize(item)
  }
}

impl From<usize> for Answer {
  fn from(item: usize) -> Self {
    Self::USize(item)
  }
}

impl From<u128> for Answer {
  fn from(item: u128) -> Self {
    Self::U128(item)
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let Args { day, part } = Args::parse();

  let day_input = File::open(format!("y2021/inputs/{}.txt", day))?;
  let ans: Answer = match (day, part) {
    (2, _) => day_2::solve(&read_to_string("y2021/inputs/2.txt")?).into(),
    (3, 1) => day_3::solve_part1(&read_to_string("y2021/inputs/3.txt")?)?.into(),
    (3, 2) => day_3::solve_part2(&read_to_string("y2021/inputs/3.txt")?)?.into(),
    (4, 1) => day_4::solve_part1(day_input)?.into(),
    (4, 2) => day_4::solve_part2(day_input)?.into(),
    (5, 1) => day_5::solve_part1(day_input)?.into(),
    (5, 2) => day_5::solve_part2(day_input)?.into(),
    (6, 1) => day_6::solve_part1(day_input)?.into(),
    (6, 2) => day_6::solve_part2(day_input)?.into(),
    (7, 1) => day_7::solve_part1(day_input)?.into(),
    (7, 2) => day_7::solve_part2(day_input)?.into(),
    (8, 1) => day_8::solve_part1(day_input)?.into(),
    (8, 2) => day_8::solve_part2(day_input)?.into(),
    (9, 1) => day_9::solve_part1(day_input)?.into(),
    (9, 2) => day_9::solve_part2(day_input)?.into(),
    (10, 1) => day_10::solve_part1(day_input).into(),
    (10, 2) => day_10::solve_part2(day_input).into(),
    (_, 1) => day_11::solve_part1(day_input).into(),
    (_, 2) => day_11::solve_part2(day_input).into(),
    _ => return Err(anyhow::anyhow!("you must select a valid problem").into()),
  };

  println!("day {} part {}: {:?}", day, part, ans);

  Ok(())
}
