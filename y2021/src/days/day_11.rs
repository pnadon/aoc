use core::panic;
use std::{cmp::min, fs::File};

use crate::helpers::digit_grid;

// max number reachable by flashes is 18 (10 + 1*8 neighbours)
const ALREADY_FLASHED: usize = 255;

pub fn solve_part1(f: File) -> usize {
  let mut nums = digit_grid(f);

  let num_iterations = 100;
  let mut score = 0;
  for _ in 0..num_iterations {
    // println!("iter {}", i);
    // print_matrix(&nums);
    increment(&mut nums);
    flash_cells(&mut nums);
    score += count_and_reset_flashed_cells(&mut nums);
  }
  // print_matrix(&nums);

  score
}

pub fn solve_part2(f: File) -> usize {
  let mut nums = digit_grid(f);

  for i in 0.. {
    increment(&mut nums);
    flash_cells(&mut nums);
    if nums.iter().all(|l| l.iter().all(|v| *v == ALREADY_FLASHED)) {
      return i + 1;
    }
    count_and_reset_flashed_cells(&mut nums);
  }
  panic!("should be unreachable");
}

// first increment values
fn increment(nums: &mut [Vec<usize>]) {
  for row in nums.iter_mut() {
    for cell in row.iter_mut() {
      *cell += 1;
    }
  }
}

fn flash_cells(nums: &mut [Vec<usize>]) {
  loop {
    // println!("loop");
    // print_matrix(nums);
    let mut new_flash = false;
    for i in 0..nums.len() {
      for j in 0..nums[i].len() {
        let res = try_flash_cell(nums, i, j);
        new_flash = new_flash || res;
        // println!("{} {} {} {}", i, j, res, new_flash);
      }
    }
    if !new_flash {
      // println!("end");
      break;
    }
  }
  // print_matrix(nums);
}

// "flash" if cell > 9
// returns true if it flashed
fn try_flash_cell(nums: &mut [Vec<usize>], i: usize, j: usize) -> bool {
  // println!("{:?} {:?} {:?} {:?}", i, j, i.saturating_sub(1)..=min(i+1, nums.len() - 1), j.saturating_sub(1)..=min(j+1, nums[i].len() - 1));
  if nums[i][j] <= 9 || nums[i][j] == ALREADY_FLASHED {
    return false;
  }

  for row in i.saturating_sub(1)..=min(i + 1, nums.len() - 1) {
    for col in j.saturating_sub(1)..=min(j + 1, nums[i].len() - 1) {
      if (row != i || col != j) && nums[row][col] != ALREADY_FLASHED {
        nums[row][col] += 1;
      }
    }
  }

  nums[i][j] = ALREADY_FLASHED;
  true
}

fn count_and_reset_flashed_cells(nums: &mut [Vec<usize>]) -> usize {
  let mut counter = 0;
  for row in nums.iter_mut() {
    for cell in row.iter_mut() {
      if cell == &ALREADY_FLASHED {
        counter += 1;
        *cell = 0;
      }
    }
  }

  counter
}

fn print_matrix(nums: &[Vec<usize>]) {
  nums.iter().for_each(|l| {
    l.iter().for_each(|v| {
      if v == &ALREADY_FLASHED {
        print!(" ");
      } else {
        print!("{:X}", v)
      }
    });
    println!("")
  });
  println!("");
}
