use std::io::{self, Write};

fn main()  {
    const DAY: i32 = 2;

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    write!(stdout, "Select day: ").expect("Problem encountered writing to stdout.");
    stdout.flush().expect("Problem encountered flushing stdout.");

    stdin.read_line(&mut input).expect("Problem encountered reading line.");
    let day = input.trim().parse::<i32>().expect("Input is not a number.");

    match day {
        1 => {
            let mut solution = aoc_lib::day1::Solution::new();
            solution.read_file("../inputs/day1.txt");
            let result = solution.solve_full();
            println!("{}, {}", result.0, result.1);
        },
        2 => {
            println!("Day 2 is coming!");
        },
        _ => println!("Invalid input: {}. Try a number 1-{}!", day, DAY),
    }
}
