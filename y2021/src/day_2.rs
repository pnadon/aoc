use std::convert::TryFrom;
use std::num::ParseIntError;

pub fn solve(contents: &str) -> isize {
  let ans = contents
    .lines()
    .map(Dir::try_from)
    // .inspect(|v| println!("{:?}", v))
    .scan(PosState::default(), |state, dir| {
      let PosState { depth, hori, aim } = *state;
      let ans = match dir {
        Ok(Dir::Forward(v)) => Some(PosState {
          depth: depth + v * aim,
          hori: hori + v,
          aim,
        }),
        Ok(Dir::Up(v)) => Some(PosState {
          depth,
          hori,
          aim: aim - v,
        }),
        Ok(Dir::Down(v)) => Some(PosState {
          depth,
          hori,
          aim: aim + v,
        }),
        e => {
          println!("something went wrong: {:?}", e);
          None
        }
      };

      if let Some(v) = ans {
        *state = v;
      }
      ans
    })
    // .inspect(|v| println!("{:?}", v))
    // .take(5)
    .last()
    .unwrap();

  ans.depth * ans.hori
}

#[derive(Debug)]
enum Dir {
  Forward(isize),
  Up(isize),
  Down(isize),
}

impl TryFrom<&str> for Dir {
  type Error = ParseIntError;
  fn try_from(input: &str) -> Result<Self, Self::Error> {
    match input.split(' ').collect::<Vec<&str>>().as_slice() {
      ["forward", v] => v.parse().map(Dir::Forward),
      ["up", v] => v.parse().map(Dir::Up),
      ["down", v] => v.parse().map(Dir::Down),
      _ => panic!("invalid direction, this shouldnt happen"),
    }
  }
}

#[derive(Debug, Default, Copy, Clone)]
struct PosState {
  depth: isize,
  hori: isize,
  aim: isize,
}
