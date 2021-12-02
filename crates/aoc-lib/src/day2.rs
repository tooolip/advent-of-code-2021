use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Solution {
    instrs: Vec<String>,
    vals: Vec<i32>,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            instrs: Vec::new(),
            vals: Vec::new(),
        }
    }

    pub fn read_file(&mut self, filename: &str) {
        let file = File::open(filename).expect("Problem encountered opening file.");
        let reader = BufReader::new(file);

        for (_index, line) in reader.lines().enumerate() {
            let line = line.expect("Problem encountered unpacking line.");
            let line: Vec<&str> = line.split(" ").collect();
            let instr: String = line.iter().nth(0).unwrap().to_string();
            let val = line.iter().nth(1).unwrap().parse::<i32>().expect("Could not parse line as number.");

            self.instrs.push(instr);
            self.vals.push(val);
        };
    }

    pub fn solve_full(&self) -> (i32, i32) {
        (self.solve_part1(), self.solve_part2())
    }

    pub fn solve_part1(&self) -> i32 {
        let mut position: (i32, i32) = (0, 0);

        for (idx, instr) in self.instrs.iter().enumerate() {
            match instr.as_str() {
                "forward" => position.0 = position.0 + self.vals.iter().nth(idx).unwrap(),
                "up" => position.1 = position.1 - self.vals.iter().nth(idx).unwrap(),
                "down" => position.1 = position.1 + self.vals.iter().nth(idx).unwrap(),
                _ => { println!("Error command not recognized."); },
            }
        }

        position.0 * position.1
    }

    pub fn solve_part2(&self) -> i32 {
        let mut position: (i32, i32, i32) = (0, 0, 0);

        for (idx, instr) in self.instrs.iter().enumerate() {
            match instr.as_str() {
                "forward" => {
                    position.0 = position.0 + self.vals.iter().nth(idx).unwrap();
                    position.1 = position.1 + (position.2 * self.vals.iter().nth(idx).unwrap());
                },
                "up" => {
                    position.2 = position.2 - self.vals.iter().nth(idx).unwrap();
                },
                "down" => {
                    position.2 = position.2 + self.vals.iter().nth(idx).unwrap();
                },
                _ => { println!("Error command not recognized."); },
            }
        }

        position.0 * position.1
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

        assert_eq!(result, 150);
    }

    #[test]
    fn part_two() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day2.txt");
        let result = solution.solve_part2();

        assert_eq!(result, 900);
    }

    #[test]
    fn both_parts() {
        let mut solution = Solution::new();
        solution.read_file("../../test_inputs/day2.txt");
        let result = solution.solve_full();

        assert_eq!(result, (150,900));
    }
}
