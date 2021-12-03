pub fn part_1(input: &str) -> u16 {
  input
    .lines()
    .map(|line| seat_to_id(line).unwrap())
    .max()
    .unwrap()
}

pub fn part_2(input: &str) -> u16 {
  let mut seats: [bool; 1024] = [false; 1024];
  for line in input.lines() {
    seats[seat_to_id(line).unwrap() as usize] = true;
  }
  seats
    .iter()
    .enumerate()
    .skip_while(|(_, i)| **i == false)
    .find(|(_, seat)| !**seat)
    .unwrap()
    .0 as u16
}

fn seat_to_id(input: &str) -> Result<u16, std::num::ParseIntError> {
  u16::from_str_radix(
    &input
      .chars()
      .map(|c| match c {
        'F' | 'L' => '0',
        'B' | 'R' => '1',
        _ => panic!("Invalid char {:?}", c),
      })
      .collect::<String>(),
    2,
  )
}
