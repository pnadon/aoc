const TREE: char = '#';
// const empty: char = '.';

pub fn part_1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.check_slope(3, 1)
}

pub fn part_2(input: &str) -> usize {
    let grid = Grid::new(input);

    grid.check_slope(1, 1)
        * grid.check_slope(3, 1)
        * grid.check_slope(5, 1)
        * grid.check_slope(7, 1)
        * grid.check_slope(1, 2)
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>(),
        }
    }

    fn check_slope(&self, right: usize, down: usize) -> usize {
        let width = self.grid[0].len();
        let height = self.grid.len();
        (down..height)
            .step_by(down)
            .filter(|row| self.grid[*row][((row / down) * right) % width] == TREE)
            .count()
    }
}
