use anyhow::{anyhow, Result};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};

// doesnt work :( see part5a
#[allow(dead_code)]
pub fn solve_part1(f: File) -> Result<usize> {
  let mut intersections = HashSet::new();
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

  lines.iter().for_each(|l| println!("{}", l));
  println!("{}", lines.len());

  for line1 in lines.iter() {
    for line2 in lines.iter().filter(|l| l != &line1) {
      if let Some(p) = line1.intersects(line2) {
        println!("{} intersects {} at {}", line1, line2, p);
        intersections.insert(p);
      }
    }
  }

  Ok(intersections.len())
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
  a: isize,
  b: isize,
  c: isize,
}

impl Line {
  fn new(p1: Point, p2: Point) -> Self {
    let Distance { x: b, y: a } = p2 - p1;
    Self {
      p1,
      p2,
      a,
      b,
      c: a * p1.x + b * p1.y,
    }
  }

  fn det(&self, other: &Self) -> isize {
    self.a * other.b - other.a * self.b
  }

  fn intersects(&self, other: &Self) -> Option<Point> {
    match self.det(other) {
      0 => None, // parallel
      determinant => {
        let x = other.b * self.c - self.b * other.c / determinant;
        let y = self.a * other.c - other.a * self.c / determinant;
        if x >= 0 && y >= 0 {
          let p = Point::new(x, y);
          if self.lies_within(p) {
            return Some(p);
          }
        }
        None
      }
    }
  }

  fn lies_within(&self, p: Point) -> bool {
    p.x >= min(self.p1.x, self.p2.x)
      && p.x <= max(self.p1.x, self.p2.x)
      && p.y >= min(self.p1.y, self.p2.y)
      && p.y <= max(self.p1.y, self.p2.y)
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
    Self::new(dbg!(self.x + rhs.x), dbg!(self.y + rhs.y))
  }
}

impl Sub<Distance> for Point {
  type Output = Self;

  fn sub(self, rhs: Distance) -> Self::Output {
    Self::new(dbg!(self.x - rhs.x), dbg!(self.y - rhs.y))
  }
}

impl Sub for Point {
  type Output = Distance;

  fn sub(self, rhs: Point) -> Self::Output {
    Distance::new(self.x - rhs.x, self.y - rhs.y)
  }
}
