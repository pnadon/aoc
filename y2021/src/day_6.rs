use std::{fs::File, collections::VecDeque};

use anyhow::Result;
use crate::helpers::comma_delimited_input;

const START_TIMER: usize = 8;
const RESET_TIMER: usize = 6;

pub fn solve_part1(f: File) -> Result<usize> {
  let nums = comma_delimited_input(f)?;

  Ok(count_jellyfish_after_days(80, nums))
}

pub fn solve_part2(f: File) -> Result<u128> {
  let nums = comma_delimited_input(f)?;

  Ok(compute_jellyfish_after_days(256, nums))
}



fn count_jellyfish_after_days(days: usize, mut timers: Vec<usize>) -> usize {
  let mut nums = vec![];
  // simulate for n days
  for _ in 0..days {
    // count finished timers and reset them to 6
    let mut num_zero = 0;
    for timer in timers.iter_mut().filter(|t| **t == 0) {
      *timer = RESET_TIMER + 1;
      num_zero += 1;
    }
    
    // decrement timer
    for timer in timers.iter_mut() {
      *timer -= 1;
    }

    // push n new timers, n == number of previously finished timers
    for _ in 0..num_zero {
      timers.push(START_TIMER);
    }
    nums.push(timers.len());
  }

  // println!("{:?}", nums);

  timers.len()
}

fn compute_jellyfish_after_days(days: usize, timers: Vec<usize>) -> u128 {
  
  // 0 to 6 incl., in rotation
  let mut pools: VecDeque<u128> = VecDeque::from([0; 7]);
  
  // enqueued to be added to the pools
  let mut num_at_8: u128 = 0;
  let mut num_at_7: u128 = 0;

  for timer in timers {
    pools[timer] += 1;
  }

  for _ in 0..days {
    let num_at_zero = pools.pop_front().unwrap();

    pools.push_back(num_at_7 + num_at_zero);

    num_at_7 = num_at_8;
    num_at_8 = num_at_zero;
  }

  num_at_8 + num_at_7 + pools.into_iter().sum::<u128>()
}
