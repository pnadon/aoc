use std::env::args;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
fn main() {
    let day = args().nth(1)
        .expect("Must pass at least 1 param")
        .parse::<u8>().expect("First param must be an int representing the day to run");
    match day {
        1 => {
            let target_sum = args().nth(2).unwrap()
                .parse::<u32>().unwrap();
            let input = args().nth(3).unwrap();
            println!("{:?}", day_1::part_2(target_sum, &input));
        },
        2 => {
            let input = args().nth(2).expect("Must pass in input string");
            println!("{:?}", day_2::part_1(&input));
            println!("{:?}", day_2::part_2(&input));
        },
        3 => {
            let input = args().nth(2).unwrap();
            println!("{:?}", day_3::part_1(&input));
            println!("{:?}", day_3::part_2(&input));
        },
        4 => {
            let input = args().nth(2).unwrap();
            println!("{:?}", day_4::part_1(&input));
            println!("{:?}", day_4::part_2(&input));
        },
        _ => {
            println!("Invalid input");
        }
    }
}