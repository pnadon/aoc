pub fn part_1(input: &str) -> usize {
  input.split("\n\n").map(unique_choices).sum()
}

pub fn part_2(input: &str) -> usize {
  input.split("\n\n").map(num_all_chosen).sum()
}

fn unique_choices(input: &str) -> usize {
  let mut exists = [false; 26];
  for chr in input.chars().filter(|c| c.is_alphabetic()) {
    exists[letter_to_idx(chr)] = true;
  }
  exists.iter().filter(|c| **c).count()
}

fn letter_to_idx(chr: char) -> usize {
  chr as usize - 'a' as usize
}

fn num_all_chosen(input: &str) -> usize {
  input
    .lines()
    .map(|line| {
      line
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(0_u32, |acc, chr| acc | (1 << (letter_to_idx(chr) as u32)))
    })
    .fold(((1 << 26) - 1) as u32, |acc, choices| acc & choices)
    .count_ones() as usize
}
