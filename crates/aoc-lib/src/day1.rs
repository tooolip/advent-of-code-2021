use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}

pub struct Solution;

impl Solution {
    pub fn solve_part1(filename: &str) -> Result<i32, Error> {

        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => return Err(Error::IoError(e)),
        };
        let reader = BufReader::new(file);

        let mut counter = 0;
        let mut last_val = -1;
        for (_index, line) in reader.lines().enumerate() {

            let val = match line {
                Ok(v) => v,
                Err(e) => return Err(Error::IoError(e)),
            };
            let val = match val.parse::<i32>() {
                Ok(v) => v,
                Err(e) => return Err(Error::ParseIntError(e)),
            };

            if val > last_val && last_val != -1 {
                counter += 1;
            }

            last_val = val;
        }

        Ok(counter)
    }

    pub fn solve_part2(filename: &str) -> Result<i32, &'static str> {

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut counter = 0;
        let mut last_three_vals = Vec::with_capacity(3);
        for (index, line) in reader.lines().enumerate() {

            let val = line.unwrap().parse::<i32>().unwrap();

            if index > 2 {
                let sum_a: i32 = last_three_vals.clone().iter().sum();
                let sum_b: i32 = last_three_vals.clone().into_iter().nth(1).unwrap() + last_three_vals.clone().into_iter().nth(2).unwrap() + val;

                if sum_b > sum_a {
                    counter += 1;
                }
    
                last_three_vals.remove(0);
            }
            
            last_three_vals.push(val);
        }

        Ok(counter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = Solution::solve_part1("../../test_inputs/day1.txt").unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn part_two() {
        let result = Solution::solve_part2("../../test_inputs/day1.txt");
        assert_eq!(result, Ok(5));
    }
}
