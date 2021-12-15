use anyhow::Result;

use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn solve_part1(f: File) -> Result<usize> {
  let nums = setup(f);

  let mut sum = 0;
  for i in 0..nums.len() {
    for j in 0..nums[i].len() {
      let risk_level = get_risk_level(&nums, i, j);
      sum += risk_level;
    }
  }

  Ok(sum)
}

pub fn solve_part2(f: File) -> Result<usize> {
  let mut nums = setup(f);

  let mut basin_sizes: Vec<usize> = vec![];
  for i in 0..nums.len() {
    for j in 0..nums[i].len() {
      basin_sizes.push(consume_and_calculate_basin(&mut nums, i, j));
    }
  }

  basin_sizes.sort_unstable();

  Ok(basin_sizes.iter().rev().take(3).product())
}

fn setup(f: File) -> Vec<Vec<usize>> {
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

fn get_risk_level(nums: &[Vec<usize>], i: usize, j: usize) -> usize {
  let height = nums[i][j];
  let is_hole = !((i > 0 && nums[i - 1][j] <= height)
    || (i + 1 < nums.len() && nums[i + 1][j] <= height)
    || (j > 0 && nums[i][j - 1] <= height)
    || (j + 1 < nums[i].len() && nums[i][j + 1] <= height));

  if is_hole {
    height + 1
  } else {
    0
  }
}

fn consume_and_calculate_basin(nums: &mut Vec<Vec<usize>>, i: usize, j: usize) -> usize {
  if nums[i][j] == 9 {
    return 0;
  }

  nums[i][j] = 9;

  let above = if i > 0 {
    consume_and_calculate_basin(nums, i - 1, j)
  } else {
    0
  };
  let below = if i + 1 < nums.len() {
    consume_and_calculate_basin(nums, i + 1, j)
  } else {
    0
  };
  let left = if j > 0 {
    consume_and_calculate_basin(nums, i, j - 1)
  } else {
    0
  };
  let right = if j + 1 < nums[i].len() {
    consume_and_calculate_basin(nums, i, j + 1)
  } else {
    0
  };

  1 + above + below + left + right
}
