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
