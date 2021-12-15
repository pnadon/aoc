use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn solve_part1(f: File) -> usize {
  BufReader::new(f)
    .lines()
    .map(|l| {
      let chrs = l.unwrap().chars().collect::<Vec<char>>();

      get_first_incorrect_char(&chrs)
        .map(incorrect_char_score)
        .unwrap_or(0)
    })
    .sum()
}

pub fn solve_part2(f: File) -> usize {
  let mut scores = BufReader::new(f)
    .lines()
    .map(|l| {
      let chrs = l.unwrap().chars().collect::<Vec<char>>();

      get_autocomplete_score(&chrs)
    })
    .filter(|v| v != &0)
    .collect::<Vec<usize>>();

  // println!("{:#?}", scores);
  scores.sort_unstable();

  scores[scores.len() / 2]
}

fn get_first_incorrect_char(chrs: &[char]) -> Option<char> {
  let mut stack = vec![];
  for c in chrs.iter() {
    match c {
      '[' | '(' | '{' | '<' => stack.push(c),
      ']' | ')' | '}' | '>' => {
        if stack.pop() != Some(&match_brace(*c)) {
          return Some(*c);
        }
      }
      e => panic!("invalid character: {}", e),
    }
  }
  None
}

fn get_autocomplete_score(chrs: &[char]) -> usize {
  let mut stack = vec![];
  for c in chrs.iter() {
    match c {
      '[' | '(' | '{' | '<' => stack.push(c),
      ']' | ')' | '}' | '>' => {
        if stack.pop() != Some(&match_brace(*c)) {
          return 0; // ignore invalid lines
        }
      }
      e => panic!("invalid character: {}", e),
    }
  }

  stack
    .into_iter()
    .map(autocomplete_char_score)
    .rev()
    .fold(0, |acc, x| acc * 5 + x)
}

fn match_brace(c: char) -> char {
  match c {
    ']' => '[',
    ')' => '(',
    '}' => '{',
    '>' => '<',
    _ => panic!("invalid character: {}", c),
  }
}

fn incorrect_char_score(c: char) -> usize {
  match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    _ => panic!("invalid character: {}", c),
  }
}

fn autocomplete_char_score(c: &char) -> usize {
  match c {
    '(' => 1,
    '[' => 2,
    '{' => 3,
    '<' => 4,
    _ => panic!("invalid character: {}", c),
  }
}
