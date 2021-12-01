fn main() {
    let day1_part1_solution = aoc_lib::day1::Solution::solve_part1("../inputs/day1.txt");
    match day1_part1_solution {
        Ok(v) => println!("1.1: {}", v),
        Err(e) => println!("Error: {:?}", e),
    }
    let day1_part2_solution = aoc_lib::day1::Solution::solve_part2("../inputs/day1.txt");
    match day1_part2_solution {
        Ok(v) => println!("1.2: {}", v),
        Err(e) => println!("Error: {}", e),
    }
}
