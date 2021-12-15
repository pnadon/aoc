pub fn part_1(input: &str) -> usize {
  input
    .lines()
    .filter(|line| {
      let reqs = PasswordReqs::parse(line);
      let char_count = reqs.password.chars().filter(|c| *c == reqs.chr).count();
      (reqs.start <= char_count) && (char_count <= reqs.end)
    })
    .count()
}

pub fn part_2(input: &str) -> usize {
  input
    .lines()
    .filter(|line| {
      let reqs = PasswordReqs::parse(line);
      (reqs.password.chars().nth(reqs.start - 1).unwrap() == reqs.chr)
        ^ (reqs.password.chars().nth(reqs.end - 1).unwrap() == reqs.chr)
    })
    .count()
}

struct PasswordReqs {
  start: usize,
  end: usize,
  chr: char,
  password: String,
}

impl PasswordReqs {
  fn parse(input: &str) -> Self {
    let params = input.split(|c| c == ' ' || c == '-').collect::<Vec<&str>>();
    let start = params[0].parse::<usize>().unwrap();
    let end = params[1].parse::<usize>().unwrap();
    let chr = params[2].chars().next().unwrap();
    Self {
      start,
      end,
      chr,
      password: params[3].to_owned(),
    }
  }
}
