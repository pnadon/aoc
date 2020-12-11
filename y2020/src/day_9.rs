use std::collections::HashSet;

pub fn part_1(input: &str) -> u32 {
    let mut preamble = input.lines().map(|l| l.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let nums = preamble.split_off(25);

    let set = preamble.into_iter().collect::<HashSet<u32>>();

    for num in nums {
        if !set.iter().any(|n| num > *n && set.contains(&(num - n))) {
            return num
        }
    }
    return 0
}