use std::collections::HashSet;

pub fn part_1(target_sum: u32, nums: &str) -> Option<u32> {
    let mut num_set = HashSet::new();
    nums.lines().for_each(|num| {
        num_set.insert(num.parse::<u32>().unwrap());
    });
    num_set.iter().find(|num| match target_sum > **num {
        true => num_set.contains(&(target_sum - *num)),
        false => false
    }).copied()
}

pub fn part_2(target_sum: u32, nums: &str) {
    let mut sorted_nums = nums.lines().map(|num| (num.parse::<u32>().unwrap())).collect::<Vec<u32>>();
    sorted_nums.sort();
    for num in sorted_nums {
        if num >= target_sum {
            continue;
        }
        if let Some(other_num) = part_1(target_sum - num, nums) {
            println!("Product of trio that sum to target: {:?}", num * other_num * (target_sum - (num + other_num)));
            return
        }
    }
}
