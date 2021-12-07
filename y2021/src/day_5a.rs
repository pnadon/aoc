use anyhow::{anyhow, Result};

use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

pub fn solve_part1(f: File) -> Result<usize> {
  let lines = BufReader::new(f)
    .lines()
    .map(|l| anyhow::Ok(parse_line(&l?))?)
    .filter(|l| {
      if let Ok(l) = l {
        l.p1.x == l.p2.x || l.p1.y == l.p2.y
      } else {
        false
      }
    })
    .collect::<Result<Vec<Line>>>()?;

  // lines.iter().for_each(|l| println!("{}", l));
  // println!("{}", lines.len());

  let mut sparse_matrix: HashMap<Point, usize> = HashMap::new();

  for line in lines.iter() {
    line.intermediate_points().into_iter().for_each(|p| {
      let prev = *sparse_matrix.get(&p).unwrap_or(&0);
      sparse_matrix.insert(p, 1 + prev);
    })
  }

  // println!("{:?}", sparse_matrix);

  Ok(sparse_matrix.into_values().filter(|n| n > &1).count())
}

pub fn solve_part2(f: File) -> Result<usize> {
  let lines = BufReader::new(f)
    .lines()
    .map(|l| anyhow::Ok(parse_line(&l?))?)
    .collect::<Result<Vec<Line>>>()?;

  // lines.iter().for_each(|l| println!("{}", l));
  // println!("{}", lines.len());

  let mut sparse_matrix: HashMap<Point, usize> = HashMap::new();

  for line in lines.iter() {
    line.intermediate_points().into_iter().for_each(|p| {
      let prev = *sparse_matrix.get(&p).unwrap_or(&0);
      sparse_matrix.insert(p, 1 + prev);
    })
  }

  // println!("{:?}", sparse_matrix);

  Ok(sparse_matrix.into_values().filter(|n| n > &1).count())
}

fn parse_line(input: &str) -> Result<Line> {
  match &input.split(" -> ").collect::<Vec<&str>>()[..] {
    &[p1, p2] => match (
      &p1.split(',').collect::<Vec<&str>>()[..],
      &p2.split(',').collect::<Vec<&str>>()[..],
    ) {
      (&[p1x, p1y], &[p2x, p2y]) => {
        let (p1x, p1y, p2x, p2y) = (p1x.parse()?, p1y.parse()?, p2x.parse()?, p2y.parse()?);
        match [p1x, p1y, p2x, p2y].iter().all(|v| *v >= 0) {
          true => Ok(Line::new(Point::new(p1x, p1y), Point::new(p2x, p2y))),
          false => Err(anyhow!(
            "Expected all numbers parsed to be non-negative, got: {} {} {} {}",
            p1x,
            p1y,
            p2x,
            p2y
          )),
        }
      }
      (p1, p2) => Err(anyhow!(
        "Expected 2 elements per point, got\np1 = {:?}\np2 = {:?}",
        p1,
        p2
      )),
    },
    v => Err(anyhow!("Expected 2 elements from split, got {:?}", v)),
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Line {
  p1: Point,
  p2: Point,
}

impl Line {
  fn new(p1: Point, p2: Point) -> Self {
    Self { p1, p2 }
  }

  fn distance(&self) -> Distance {
    self.p2 - self.p1
  }

  fn intermediate_points(&self) -> Vec<Point> {
    let (step, total) = match self.distance() {
      dist if dist.x == 0 && dist.y != 0 => (Distance::new(0, dist.y / dist.y.abs()), dist.y.abs()),
      dist if dist.x != 0 && dist.y == 0 => (Distance::new(dist.x / dist.x.abs(), 0), dist.x.abs()),
      dist if dist.x != 0 && dist.y != 0 => (
        Distance::new(dist.x / dist.x.abs(), dist.y / dist.y.abs()),
        dist.x.abs(),
      ),
      dist => panic!(
        "expected one of the distance axis to be zero, got {:?}",
        dist
      ),
    };

    (0..=total)
      .scan(self.p1, |state, _| {
        let prev_state = *state;
        *state = *state + step;
        Some(prev_state)
      })
      .collect::<Vec<Point>>()
  }
}

impl Display for Line {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{}", self.p1, self.p2)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
  x: isize,
  y: isize,
}

impl Point {
  fn new(x: isize, y: isize) -> Self {
    if x >= 0 && y >= 0 {
      Self { x, y }
    } else {
      // panic instead of Result so that we can use new in +/- ops
      panic!("Point coordinates must be >= 0, got {}, {}", x, y)
    }
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

#[derive(Clone, Copy, Debug)]
struct Distance {
  x: isize,
  y: isize,
}

impl Distance {
  fn new(x: isize, y: isize) -> Self {
    Self { x, y }
  }
}

impl Add<Distance> for Point {
  type Output = Self;

  fn add(self, rhs: Distance) -> Point {
    Self::new(self.x + rhs.x, self.y + rhs.y)
  }
}

impl Sub<Distance> for Point {
  type Output = Self;

  fn sub(self, rhs: Distance) -> Self::Output {
    Self::new(self.x - rhs.x, self.y - rhs.y)
  }
}

impl Sub for Point {
  type Output = Distance;

  fn sub(self, rhs: Point) -> Self::Output {
    Distance::new(self.x - rhs.x, self.y - rhs.y)
  }
}
