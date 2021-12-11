use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn solve_part1(f: File) -> Result<usize> {
  let nums = BufReader::new(f)
    .lines()
    .map(|line| {
      let output_vals = get_output_vals(line.unwrap())?;
      let num_unique = output_vals
        .iter()
        .filter(|word| match word.len() {
          2 | 4 | 3 | 7 => true,
          _ => false,
        })
        .count();

      Ok(num_unique)
    })
    .collect::<Result<Vec<usize>>>()?;

  Ok(nums.into_iter().sum())
}

fn get_output_vals(raw_input: String) -> Result<Vec<String>> {
  let raw_output_vals = raw_input
    .split('|')
    .nth(1)
    .ok_or(anyhow!("nothing on the right side of `|`!"))?;
  Ok(raw_output_vals.split(' ').map(|s| s.to_owned()).collect())
}

pub fn solve_part2(f: File) -> Result<usize> {
  let nums = BufReader::new(f)
    .lines()
    .map(|line| compute_output_num(line.unwrap()))
    .sum::<Result<usize>>()?;

  Ok(nums)
}

fn compute_output_num(input: String) -> Result<usize> {
  let (signals, outputs) = input
    .split('|')
    .map(|nums| {
      nums
        .split(' ')
        .filter(|v| !v.is_empty())
        .map(chars_to_set)
        .collect::<Result<Vec<u8>>>()
    })
    .tuples()
    .next()
    .ok_or(anyhow!("could not split on `|`"))?;

  let (mut signals, outputs) = (signals?, outputs?);

  // num 1s
  // 2, 3, 4, 5, 5, 5, 6, 6, 6, 7
  signals.sort_by(|num1, num2| num1.count_ones().cmp(&num2.count_ones()));
  assert_eq!(signals.len(), 10);

  let mut digits = [0; 10];
  let mut segments = [0; 10];

  // n<number> -> the digit
  // s<number> -> the segment, numbered top-down, then left-right
  // #<number> -> some digit with that many segments

  // our knowns
  digits[1] = signals[0];
  digits[4] = signals[2];
  digits[7] = signals[1];
  digits[8] = signals[9];

  const BIT_MASK: u8 = (1 << 7) - 1;

  // n7 - n1 == s1
  segments[1] = digits[7] & (!digits[1] & BIT_MASK);

  // s1 + n4 + s7 == n9
  for num in signals[6..=8].iter() {
    let diff = num & (!(segments[1] | digits[4]) & BIT_MASK);
    if diff.count_ones() == 1 {
      digits[9] = *num;
      segments[7] = diff;
    }
  }

  assert_ne!(digits[9], 0);

  // n8 == n9 + s5
  segments[5] = digits[8] & (!digits[9] & BIT_MASK);

  // n9 - #6 == s5 -> #6 == n6, other is n0
  for num in signals[6..=8].iter() {
    if *num != digits[9] {
      if num | digits[1] == *num {
        digits[0] = *num;
      } else {
        digits[6] = *num;
      }
    }
  }

  assert_ne!(digits[6], 0);
  assert_ne!(digits[0], 0);

  // n6 - s5 == n5
  digits[5] = digits[6] & (!segments[5] & BIT_MASK);

  // s5 in #5 -> #5 == n3
  // rem #5 == n2
  for num in signals[3..=5].iter() {
    if num & segments[5] != 0 {
      digits[2] = *num;
    } else if *num != digits[5] {
      digits[3] = *num;
    }
  }

  let num_outputs = outputs.len();

  let ans = outputs
    .iter()
    .enumerate()
    .map(|(i, c)| {
      digits
        .iter()
        .enumerate()
        .find(|(_, val)| *val == c)
        .ok_or(anyhow!("missing digit at {}", i))
        .map(|(j, _)| j * 10_usize.pow((num_outputs - i - 1) as u32))
    })
    .sum::<Result<usize>>()?;

  Ok(ans)
}

fn chars_to_set(chars: &str) -> Result<u8> {
  assert!(chars.len() > 0);
  chars
    .chars()
    .map(|c| match c {
      'a' => Ok(1 << 0),
      'b' => Ok(1 << 1),
      'c' => Ok(1 << 2),
      'd' => Ok(1 << 3),
      'e' => Ok(1 << 4),
      'f' => Ok(1 << 5),
      'g' => Ok(1 << 6),
      c => Err(anyhow!("invalid char found: {}", c))?,
    })
    .reduce(|a, b| Ok(a? | b?))
    .ok_or(anyhow!("chars shouldn't be empty"))?
}

#[allow(dead_code)]
fn print_bits(ns: &[u8]) {
  println!("[");
  ns.iter().for_each(|n| println!("  {:08b},", n));
  println!("]");
}
