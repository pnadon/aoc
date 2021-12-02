use std::env;
use std::fs::File;
use std::io::{Error, Read};

fn main() {
    let mut contents = String::new();
    File::open("./in.txt")
        .unwrap()
        .read_to_string(&mut contents);
    let items: Vec<char> = contents.chars().collect();
    let (min_zeros_at, min_zeros) = items
        .chunks(6 * 25)
        .map(|x| x.iter().filter(|x| **x == '0').count())
        .enumerate()
        .min_by(|&(_, item), _| item.cmp(&0))
        .unwrap();
    println!("{} {}", min_zeros_at, min_zeros);
}
