use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const BOARD_SIZE: usize = 5;

pub fn solve_part1(f: File) -> Result<usize> {
  let (nums, mut boards) = setup(f)?;

  for num in nums.into_iter() {
    for board in &mut boards {
      if board.mark_num(num) {
        return Ok(board.sum_unmarked() * (num as usize));
      }
    }
  }

  Err(anyhow!("Game should have ended"))
}

pub fn solve_part2(f: File) -> Result<usize> {
  let (nums, mut boards) = setup(f)?;

  for num in nums.into_iter() {
    let mut i = 0;
    while i < boards.len() {
      if boards[i].mark_num(num) {
        if boards.len() == 1 {
          // final winner
          return Ok(boards[0].sum_unmarked() * (num as usize));
        } else {
          // pop the winner from the board
          let len = boards.len();
          boards.swap(i, len - 1);
          boards.pop();
        }
      } else {
        // else iterate, (only do it when not popping)
        i += 1;
      }
    }
  }

  Err(anyhow!("Game should have ended"))
}

fn setup(f: File) -> Result<(Vec<u8>, Vec<Board>)> {
  let mut lines = BufReader::new(f).lines();

  let nums_line = lines.next().ok_or(anyhow!(
    "expected a line at the top of the file, got nothing"
  ))??;

  let boards = lines
    .into_iter()
    .filter_map(|l| l.ok())
    .peekable() // we just filter out errors
    .batching(|it| {
      loop {
        match it.peek() {
          Some(v) => {
            if !v.is_empty() {
              break;
            } else {
              it.next();
            }
          }
          None => return None,
        }
      }
      Some(
        (0..BOARD_SIZE)
          .filter_map(|_| it.next())
          .map(|l| {
            l.trim()
              .split(' ')
              .flat_map(|num| num.parse::<u8>().ok())
              .collect::<Vec<u8>>()
          })
          .flatten()
          .collect::<Vec<u8>>()
          .try_into(),
      )
    })
    .collect::<Result<Vec<Board>>>()?;

  let nums_iter = nums_line
    .split(',')
    .map(|num| num.parse::<u8>().expect("couldn't parse number!"))
    .collect::<Vec<u8>>();

  Ok((nums_iter, boards))
}

#[derive(Debug)]
struct Board {
  lines: Vec<HashSet<u8>>, // would be better if we used an unordered accumulator hash function
}

impl Board {
  fn new(nums: Vec<u8>) -> Result<Self> {
    match nums.len() == BOARD_SIZE * BOARD_SIZE {
      true => {
        let mut lines: Vec<HashSet<u8>> = Vec::with_capacity(BOARD_SIZE * 2);
        (0..(BOARD_SIZE * 2)).for_each(|_| lines.push(HashSet::<u8>::new()));

        for dir1 in 0..BOARD_SIZE {
          for dir2 in 0..BOARD_SIZE {
            // lines is both rows and cols, we just need to check what numbers are unmarked in each
            // if no numbers are unmarked (ie. the set is empty) -> you win
            lines[dir1].insert(nums[dir1 * BOARD_SIZE + dir2]);
            lines[dir1 + 5].insert(nums[dir2 * BOARD_SIZE + dir1]);
          }
        }
        Ok(Self { lines })
      }
      false => Err(anyhow!(
        "Board needs to be 5x5, got {}\n{:?}",
        nums.len(),
        nums
      )),
    }
  }

  fn mark_num(&mut self, num: u8) -> bool {
    for line in &mut self.lines {
      if line.contains(&num) {
        line.remove(&num);
      }
    }
    self.has_won()
  }

  /// I need a way to hash an collection of u8s, s/t the order that they are combined to form the hash doesn't matter.
  /// Then, one could verify the row/col is filled by comparing the hashes
  /// Kind of, but not quite this https://stackoverflow.com/questions/3191020/commutative-accumulator-based-function-for-calculating-a-digest-of-multiple-has/3191596
  fn has_won(&self) -> bool {
    self.lines.iter().any(|line| line.is_empty())
  }

  fn sum_unmarked(&self) -> usize {
    // skipping half cuz rows & columns double count.
    self
      .lines
      .iter()
      .skip(self.lines.len() / 2)
      .map(|line| line.iter().fold(0, |acc, num| acc + (*num as usize)))
      .sum()
  }
}

impl TryFrom<Vec<u8>> for Board {
  type Error = anyhow::Error;
  fn try_from(nums: Vec<u8>) -> Result<Self> {
    Board::new(nums)
  }
}
