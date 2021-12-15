use anyhow::Result;
use std::fs::File;

use crate::helpers::comma_delimited_input;

pub fn solve_part1(f: File) -> Result<usize> {
  let nums = comma_delimited_input(f)?;
  let start = *nums.iter().min().unwrap();
  let end = *nums.iter().max().unwrap();

  Ok(
    (start..=end)
      .map(|destination| compute_cost_part1(&nums, destination))
      .min()
      .unwrap(),
  )
}

pub fn solve_part2(f: File) -> Result<usize> {
  let nums = comma_delimited_input(f)?;
  let start = *nums.iter().min().unwrap();
  let end = *nums.iter().max().unwrap();

  Ok(
    (start..=end)
      .map(|destination| compute_cost_part2(&nums, destination))
      .min()
      .unwrap(),
  )
}

fn compute_cost_part1(positions: &Vec<usize>, destination: usize) -> usize {
  positions
    .iter()
    .map(|pos| {
      if *pos > destination {
        pos - destination
      } else {
        destination - pos
      }
    })
    .sum()
}

fn compute_cost_part2(positions: &Vec<usize>, destination: usize) -> usize {
  positions
    .iter()
    .map(|pos| fuel_cost(*pos, destination))
    .sum()
}

fn fuel_cost(start: usize, end: usize) -> usize {
  let n = if start < end {
    end - start
  } else {
    start - end
  };
  (n * (n + 1)) / 2
}
