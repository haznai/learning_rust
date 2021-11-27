use itertools::Itertools;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Div, Sub};

// trait for solution of each day
pub trait Solution<T: Display> {
    fn get_input(path: &str) -> Vec<String> {
        let input_file = File::open(path);
        let reader = BufReader::new(input_file.expect("Given path does not lead to a file"));

        reader.lines().map(|line| line.unwrap()).collect_vec()
    }

    // gets input and returns solution
    fn solve_first_part(inputs: &Vec<String>) -> T;
    fn solve_second_part(inputs: &Vec<String>) -> T;

    // calls get_input and print the solution
    fn print_solution(path: &str) {
        let inputs = Self::get_input(path);
        println!("Solution for part 1: {}", Self::solve_first_part(&inputs),);

        println!("Solution for part 2: {}", Self::solve_second_part(&inputs),);
    }
}

pub struct Day01Solution;

impl Solution<i128> for Day01Solution {
    fn solve_first_part(inputs: &Vec<String>) -> i128 {
        inputs
            .iter()
            .map(|s| s.parse::<i128>().unwrap().div(3).sub(2))
            .sum()
    }

    fn solve_second_part(inputs: &Vec<String>) -> i128 {
        // recursive function that calculates fuel
        fn recursive_fuel(i: i128) -> i128 {
            if i.is_negative() {
                0
            } else {
                i + recursive_fuel(i.div(3).sub(2))
            }
        }

        inputs
            .iter()
            .map(|s| s.parse::<i128>().unwrap().div(3).sub(2))
            .map(recursive_fuel)
            .sum()
    }
}
