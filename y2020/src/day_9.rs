use std::collections::HashSet;

pub fn part_1(input: &str) -> u64 {
    let nums = input.lines().map(|l| l.parse::<u64>().unwrap()).collect::<Vec<u64>>();

    *nums.iter().enumerate().skip(25)
        .take_while(|(i, n)| {
            let set = nums[(i-25)..*i].iter()
                .collect::<HashSet<&u64>>();

            set.iter()
                .any(|s| set.contains(&&(**s - **n)))
    })
    .last().unwrap().1
}
