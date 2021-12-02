#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vector {
    head: Point,
    tail: Point,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    direction: Direction,
    value: i64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new_point(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }

    pub fn parse_move(self, instruction: Move) -> Point {
        match instruction.direction {
            Direction::UP => Point {
                x: self.x,
                y: self.y + instruction.value,
            },
            Direction::DOWN => Point {
                x: self.x,
                y: self.y - instruction.value,
            },
            Direction::LEFT => Point {
                x: self.x - instruction.value,
                y: self.y,
            },
            Direction::RIGHT => Point {
                x: self.x + instruction.value,
                y: self.y,
            },
        }
    }

    pub fn manhatten_distance(self, point: Point) -> i64 {
        (self.x - point.x).abs() + (self.y - point.y).abs()
    }
}

impl Move {
    pub fn parse_input(input: &str) -> Move {
        Move {
            direction: match input.chars().nth(0) {
                Some('U') => Direction::UP,
                Some('D') => Direction::DOWN,
                Some('L') => Direction::LEFT,
                Some('R') => Direction::RIGHT,
                _ => panic!("invalid input {:?}", input.chars().nth(0)),
            },
            value: input[1..].parse::<i64>().unwrap(),
        }
    }
}

pub fn find_min_intersection(vec_1: Vec<Point>, vec_2: Vec<Point>) -> i64 {
    find_intersections(&vec_1, &vec_2)
        .iter()
        .map(|x| x.manhatten_distance(Point::new_point(0, 0)))
        .min()
        .unwrap()
}

pub fn find_min_travel(vec_1: Vec<Point>, vec_2: Vec<Point>) -> i64 {
    let intersections = find_intersections(&vec_1, &vec_2);
    get_shortest_travel(vec_1, vec_2, intersections)
}

pub fn find_intersections(vec_1: &Vec<Point>, vec_2: &Vec<Point>) -> Vec<Point> {
    let mut pt_1 = 1;
    let mut intersections: Vec<Point> = Vec::new();
    while pt_1 < vec_1.len() {
        let mut pt_2 = 1;
        while pt_2 < vec_2.len() {
            match find_intersection(vec_1[pt_1 - 1], vec_1[pt_1], vec_2[pt_2 - 1], vec_2[pt_2]) {
                Some(res) if res.x != 0 || res.y != 0 => intersections.push(res),
                _ => (),
            };
            pt_2 += 1;
        }
        pt_1 += 1;
    }
    intersections
}

pub fn find_intersection(
    tail_1: Point,
    head_1: Point,
    tail_2: Point,
    head_2: Point,
) -> Option<Point> {
    let u_a = ((head_2.x - tail_2.x) * (tail_1.y - tail_2.y)
        - (head_2.y - tail_2.y) * (tail_1.x - tail_2.x)) as f64
        / ((head_2.y - tail_2.y) * (head_1.x - tail_1.x)
            - (head_2.x - tail_2.x) * (head_1.y - tail_1.y)) as f64;

    let u_b = ((head_1.x - tail_1.x) * (tail_1.y - tail_2.y)
        - (head_1.y - tail_1.y) * (tail_1.x - tail_2.x)) as f64
        / ((head_2.y - tail_2.y) * (head_1.x - tail_1.x)
            - (head_2.x - tail_2.x) * (head_1.y - tail_1.y)) as f64;

    if u_a <= 1.0 && u_a >= 0.0 && u_b <= 1.0 && u_b >= 0.0 {
        let x_pos = tail_1.x as f64 + u_a * (head_1.x - tail_1.x) as f64;
        let y_pos = tail_1.y as f64 + u_a * (head_1.y - tail_1.y) as f64;
        let intersection = Point::new_point(x_pos as i64, y_pos as i64);
        Some(intersection)
    } else {
        None
    }
}

/// Returns the sum of the distances from each wire's origin to the point
/// which has the shortest sum of distances.
///
pub fn get_shortest_travel(wire_1: Vec<Point>, wire_2: Vec<Point>, points: Vec<Point>) -> i64 {
    points
        .iter()
        .map(|point| {
            get_distance_to_point(&wire_1, *point) + get_distance_to_point(&wire_2, *point)
        })
        .min()
        .unwrap()
}

pub fn get_distance_to_point(wire: &Vec<Point>, point: Point) -> i64 {
    let mut pt_1 = 1;
    let mut sum = 0;
    while pt_1 < wire.len() {
        match point_intersects_line(wire[pt_1 - 1], wire[pt_1], point) {
            Some(x) => {
                println!("reached end: {} at {}, for point {:?}", sum, pt_1, point);
                sum += x;
                break;
            }
            None => sum += wire[pt_1].manhatten_distance(wire[pt_1 - 1]),
        }
        println!("segment: {} at {}, for point {:?}", sum, pt_1, point);
        pt_1 += 1;
    }
    sum
}

pub fn point_intersects_line(tail: Point, head: Point, point: Point) -> Option<i64> {
    let t_x;
    let t_y;
    if (head.x - tail.x) == 0 && head.x == point.x {
        t_x = 0.0;
    } else {
        t_x = (point.x - tail.x) as f64 / (head.x - tail.x) as f64;
    }
    if (head.y - tail.y) == 0 && head.y == point.y {
        t_y = 0.0;
    } else {
        t_y = (point.y - tail.y) as f64 / (head.y - tail.y) as f64;
    }
    println!(
        "line {:?} -> {:?} vs point {:?}, leads to {}, {}",
        tail, head, point, t_x, t_y
    );
    if t_x >= 0.0 && t_x <= 1.0 && t_y >= 0.0 && t_y <= 1.0 {
        Some(point.manhatten_distance(tail))
    } else {
        None
    }
}
