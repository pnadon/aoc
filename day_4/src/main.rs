// 235741 < num < 706948
// minimum 1 repeating digit -> max 5 unique numbers
// non-decreasing
use std::time::{Instant};

fn main() {
    let mut num_passwords = 0;
    let smallest_num = 235741;
    let largest_num = 706948;
    let now = Instant::now();
    for num in smallest_num..largest_num {
        if num_contains_repeating_digits(num) && num_nondecreasing(num) {
            num_passwords = num_passwords + 1;
        }
    }

    println!("{} found in {}", num_passwords, now.elapsed().as_nanos());
}

fn num_contains_repeating_digits(num: i64) -> bool {
    let mut check_num = num;
    while check_num / 10 > 0 {
        if (check_num % 10) == ((check_num / 10) % 10) {
            if check_num / 100 % 10 != check_num % 10 {
                return true;
            }
            let prev_num = check_num % 10;
            while check_num / 10 % 10 == prev_num {
                check_num = check_num / 10;
            }
        }
        check_num = check_num / 10;
    }
    return false;
}
// 12345 -> 5,  12385 / 10 -> 1238 -> 8
fn num_nondecreasing(num: i64) -> bool {
    let mut check_num = num;
    while check_num / 10 > 0 {
        if (check_num % 10) < ((check_num / 10) % 10) {
            return false;
        }
        check_num = check_num / 10;
    }
    return true;
}
