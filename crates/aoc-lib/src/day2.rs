use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Solution { vals: Vec<i32> }

impl Solution {
    pub fn new() -> Self {
        Self { vals: Vec::new() }
    }

    pub fn read_file(&mut self, filename: &str) {
        let file = File::open(filename).expect("Problem encountered opening file.");
        let reader = BufReader::new(file);

        for (_index, line) in reader.lines().enumerate() {
            let line = line.expect("Problem encountered unpacking line.");
            let val = line.parse::<i32>().expect("Could not parse line as number.");
            self.vals.push(val);
        };
    }

    pub fn solve_full(&self) -> (i32, i32) {
        (self.solve_part1(), self.solve_part2())
    }

    pub fn solve_part1(&self) -> i32 {
        0
    }

    pub fn solve_part2(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day2.txt");
        let result = solution.solve_part1();

        assert_eq!(result, 0);
    }

    #[test]
    fn part_two() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day2.txt");
        let result = solution.solve_part2();

        assert_eq!(result, 0);
    }

    #[test]
    fn both_parts() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day2.txt");
        let result = solution.solve_full();

        assert_eq!(result, (0,0));
    }
}
