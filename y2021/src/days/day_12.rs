use std::{
  fs::File,
  io::{BufRead, BufReader},
};

pub fn solve_part1(f: File) -> usize {
  let paths = BufReader::new(f)
  .lines()
  .map(|l| line_to_path(&l.unwrap()))
  .collect::<Vec<Path>>();

  0
}

pub fn solve_part2(f: File) -> usize {
  0
}

enum Cave {
  Small(String),
  Large(String),
  Start,
  End,
}

impl Cave {
  fn from_string(input: String) -> Cave {
    match input.as_ref() {
      "start" => Cave::Start,
      "end" => Cave::End,
      _ if input.to_uppercase() == input => Cave::Large(input),
      _ => Cave::Small(input),
    }
  }
}

struct Path {
  from: Cave,
  to: Cave,
}

fn line_to_path(line: &str) -> Path {
  let parts = line.split('-').collect::<Vec<&str>>();
  assert!(parts.len() == 2);
  Path {
    from: Cave::from_string(parts[0].to_string()),
    to: Cave::from_string(parts[1].to_string()),
  }
}
