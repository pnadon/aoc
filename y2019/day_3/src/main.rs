use std::env;
use std::fs::File;
use std::io::{Error, Read};

mod day_3;
use day_3::{find_min_intersection, find_min_travel, Move, Point};

fn parse_list(fname: &str) -> Result<Vec<Point>, Error> {
    let mut contents = String::new();
    File::open(fname)?.read_to_string(&mut contents)?;
    let moves: Vec<Move> = contents
        .split(",")
        .map(|e| day_3::Move::parse_input(e))
        .collect();
    let mut points: Vec<Point> = Vec::new();
    points.push(Point::new_point(0, 0));
    for movement in &moves {
        points.push(points.last().unwrap().parse_move(*movement));
    }
    Ok(points)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Incorrect number of arguments!");
    }
    let fname_1 = &args[1];
    let fname_2 = &args[2];
    let vec_1 = parse_list(fname_1).unwrap();
    let vec_2 = parse_list(fname_2).unwrap();
    // let ans = find_min_intersection(vec_1, vec_2);
    let ans = find_min_travel(vec_1, vec_2);

    println!("{}", ans);
}
