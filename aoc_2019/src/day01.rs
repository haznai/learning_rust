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
    // return type must be able to be printed
    fn solve_first_part(inputs: Vec<String>) -> T;

    // calls get_input and print the solution
    fn print_solution(path: &str) {
        println!(
            "Solution for part 1: {}",
            Self::solve_first_part(Self::get_input(path)),
        );
    }
}

pub struct Day01Solution;

impl Solution<i128> for Day01Solution {
    fn solve_first_part(inputs: Vec<String>) -> i128 {
        inputs
            .into_iter()
            .map(|s| s.parse::<i128>().unwrap().div(3).sub(2))
            .sum()
    }
}
