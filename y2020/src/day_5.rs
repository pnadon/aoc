pub fn part_1(input: &str) -> usize {
    input.lines().map(|line| 
        usize::from_str_radix(
            &line.chars().map(|c| match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => panic!("Invalid char {:?}", c)
            }).collect::<String>(),
            2
        ).unwrap()
    ).max().unwrap()
}

pub fn part_2(input: &str) -> u16 {
    0
}