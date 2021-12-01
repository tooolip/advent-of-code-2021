use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

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
        let mut counter: i32 = 0;
        let mut last_val: i32 = -1;

        for val in &self.vals {
            // Additional check: first value means no increment.
            if val > &last_val && last_val != -1 {
                counter += 1;
            }

            last_val = *val;
        }

        counter
    }

    pub fn solve_part2(&self) -> i32 {
        let mut counter: i32 = 0;
        let mut last_three_vals: Vec<i32> = Vec::with_capacity(3);

        for (idx, val) in self.vals.iter().enumerate() {
            // Check: last_three_vals is fully populated before first use.
            if idx > 2 {
                let sum_a: i32 = last_three_vals.iter().sum();
                // Neat shortcut, instead of bulky vector accesses, just subtract 1st value from sum_a to get sum of 2nd and 3rd.
                let sum_b: i32 = sum_a + val - last_three_vals.first().expect("Vector seems empty.");

                if sum_b > sum_a { counter += 1; }
                
                last_three_vals.remove(0);
            }

            last_three_vals.push(*val);
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day1.txt");
        let result = solution.solve_part1();

        assert_eq!(result, 7);
    }

    #[test]
    fn part_two() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day1.txt");
        let result = solution.solve_part2();

        assert_eq!(result, 5);
    }

    #[test]
    fn both_parts() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day1.txt");
        let result = solution.solve_full();

        assert_eq!(result, (7,5));
    }
}
