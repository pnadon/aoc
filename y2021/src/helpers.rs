use anyhow::Result;
use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn comma_delimited_input(f: File) -> Result<Vec<usize>> {
  let mut buf = String::new();
  BufReader::new(f).read_line(&mut buf)?;
  let nums = buf
    .trim()
    .split(',')
    .into_iter()
    .map(|num| Ok(num.parse::<usize>()?))
    .collect::<Result<Vec<usize>>>()?;

  Ok(nums)
}

pub fn digit_grid(f: File) -> Vec<Vec<usize>> {
  BufReader::new(f)
    .lines()
    .map(|l| {
      l.unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
    })
    .collect::<Vec<Vec<usize>>>()
}
