use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read(fname: &str) -> Result<Vec<i64>, Error> {
    let io = File::open(fname)?;
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(
            |e| Error::new(ErrorKind::InvalidData, e)
        )))
        .collect()
}

fn parse_list(fname: &str) -> Result<Vec<i64>, Error> {
    let mut contents = String::new();
    File::open(fname)?.read_to_string(&mut contents)?;
    contents.split(",").map(|v| v.parse().map_err(
        |e| Error::new(ErrorKind::InvalidData, e)
    )).collect()
}

fn compute_day_1(vec: Vec<i64>) -> i64 {
    vec.iter().map(|num| compute_fuel_cost( *num)).sum()
}

fn compute_fuel_cost(cost: i64) -> i64 {
    match cost / 3 - 2 {
        res if res <= 0 => 0,
        res => res + compute_fuel_cost(res)
    }
}

fn compute_day_2(input_1: usize, input_2: usize, vec: &Vec<i64>) -> Vec<i64> {
    let mut code: Vec<usize> = vec.iter().map( |&e| { e as usize }).collect();
    let mut index = 0;
    code[1] = input_1;
    code[2] = input_2;
    loop {
        let operation = code[index];
        let input_1 = code[index + 1];
        let input_2 = code[index + 2];
        let output = code[index + 3];
        match operation {
            1 => code[output] = code[input_1] + code[input_2],
            2 => code[output] = code[input_1] * code[input_2],
            99 => break,
            _ => panic!("invalid input"),
        };
        index += 4;
    }
    code.iter().map( |&e| { e as i64 }).collect()
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let fname = &args[1];
    // let vec = read(fname)?;
    let vec = parse_list(fname)?;
    // let result: i64 = compute_day_1(vec);
    // let result = compute_day_2(vec);
    let result = find_noun_verb(19690720, vec);
    println!("{:?}", result);
    Ok(())
}

fn find_noun_verb(ans: usize, vec: Vec<i64>) -> (usize, usize) {
    for input_1 in 0..99 as usize {
        for input_2 in 0..99 as usize {
            let res = compute_day_2(input_1, input_2, &vec)[0] as usize;
            if res == ans {
                return (input_1, input_2);
            }
        }
    }
    (0, 0)
}
