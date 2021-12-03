use std::error::Error;

const NUM_BITS: usize = 12;

pub fn solve_part1(contents: &str) -> Result<isize, Box<dyn Error>> {
  let num_str: String = contents
    .split('\n')
    .filter(|line| !line.is_empty())
    .map(|line| line.chars().collect::<Vec<char>>())
    .fold(vec![0; NUM_BITS], |mut acc, digits| {
      assert_eq!(
        digits.len(),
        NUM_BITS,
        "should have length equal to NUM_BITS"
      );
      for (i, v) in digits.iter().enumerate() {
        acc[i] += match v {
          '1' => 1,
          '0' => -1,
          _ => panic!("should only be 1s and 0s!"),
        };
      }
      acc
    })
    .into_iter()
    .map(|digit| match digit < 0 {
      true => '0',
      false => '1',
    })
    .collect::<String>();

  let num = isize::from_str_radix(&num_str, 2)?;
  let not_num = !num & ((1 << 12) - 1);

  println!("num 1: {:0>12b}\nnum 2: {:0>12b}", num, not_num);

  Ok(num * not_num)
}

type Line = [char; NUM_BITS];

pub fn solve_part2(contents: &str) -> Result<isize, Box<dyn Error>> {
  // for each bit, if number of remaining numbers > 1:
  //  O: determine most common bit -> eliminate numbers that dont have that bit
  // CO2: determine least common bit -> ''
  // move to next bit, repeat
  //
  // final number is what you're looking for.
  let lines = contents
    .split('\n')
    .filter(|line| !line.is_empty())
    .map(str_to_line)
    .collect::<Result<Vec<Line>, anyhow::Error>>()?;

  let (oxygen_reading, c02_reading) = get_oxygen_and_c02_readings(lines);

  let oxygen_reading = isize::from_str_radix(&oxygen_reading.iter().collect::<String>(), 2)?;
  let c02_reading = isize::from_str_radix(&c02_reading.iter().collect::<String>(), 2)?;

  println!(
    "num 1: {:0>12b}\nnum 2: {:0>12b}",
    oxygen_reading, c02_reading
  );

  Ok(oxygen_reading * c02_reading)
}

fn str_to_line(line: &str) -> Result<Line, anyhow::Error> {
  line
    .chars()
    .collect::<Vec<char>>()
    .try_into()
    .map_err(|_| anyhow::anyhow!("Failed to fit vec into array"))
}

fn get_oxygen_and_c02_readings(lines: Vec<Line>) -> (Line, Line) {
  let (oxygen, c02) = sort_oxygen_and_c02(0, lines);

  let oxygen_reading = (1..NUM_BITS).fold(oxygen, |acc, i| {
    if acc.len() > 1 {
      sort_oxygen_and_c02(i, acc).0
    } else {
      acc
    }
  });

  let c02_reading = (1..NUM_BITS).fold(c02, |acc, i| {
    if acc.len() > 1 {
        sort_oxygen_and_c02(i, acc).1
    } else {
      acc
    }
  });

  assert_eq!(oxygen_reading.len(), 1);
  assert_eq!(c02_reading.len(), 1);

  (oxygen_reading[0], c02_reading[0])
}

fn sort_oxygen_and_c02(idx: usize, lines: Vec<Line>) -> (Vec<Line>, Vec<Line>) {
  let (zeros, ones) = partition_0_and_1_digits(idx, lines);
  match zeros.len() > ones.len() {
    true => (zeros, ones),
    false => (ones, zeros),
  }
}

fn partition_0_and_1_digits(idx: usize, input: Vec<Line>) -> (Vec<Line>, Vec<Line>) {
  input.into_iter().partition(|line| line[idx] == '0')
}
