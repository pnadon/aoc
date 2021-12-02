use regex::Regex;

const FIELDS: [&str; 7] = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

const ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|passport| FIELDS.iter().all(|field| passport.contains(*field)))
        .count()
}

pub fn part_2(input: &str) -> usize {
    let match_9_digits = Regex::new(r"^([0-9]{9})$").unwrap();
    let match_hex = Regex::new(r"^(#[0-9a-z]{6})$").unwrap();
    input
        .split("\n\n")
        .filter(|passport| {
            FIELDS.iter().all(|field| passport.contains(*field))
                && passport.split_whitespace().all(|field| match &field[..4] {
                    "byr:" => {
                        if let Ok(num) = field[4..].parse::<u32>() {
                            1920 <= num && num <= 2002
                        } else {
                            false
                        }
                    }
                    "iyr:" => {
                        if let Ok(num) = field[4..].parse::<u32>() {
                            2010 <= num && num <= 2020
                        } else {
                            false
                        }
                    }
                    "eyr:" => {
                        if let Ok(num) = field[4..].parse::<u32>() {
                            2020 <= num && num <= 2030
                        } else {
                            false
                        }
                    }
                    "hgt:" => match_measurement(&field[4..]),
                    "hcl:" => match_hex.is_match(&field[4..]),
                    "ecl:" => ECLS.contains(&&field[4..]),
                    "pid:" => match_9_digits.is_match(&field[4..]),
                    _ => true,
                })
        })
        .count()
}

fn match_measurement(input: &str) -> bool {
    if input.ends_with("cm") {
        if let Some(num_end) = input.find("cm") {
            if let Ok(num) = input[..num_end].parse::<u32>() {
                return 150 <= num && num <= 193;
            }
        }
    } else if input.ends_with("in") {
        if let Some(num_end) = input.find("in") {
            if let Ok(num) = input[..num_end].parse::<u32>() {
                return 59 <= num && num <= 76;
            }
        }
    }
    false
}
