use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
// trait for solution of each day
trait Solution {
    fn get_input(path: &str) -> Vec<String> {
        let input_file = File::open(path);
        let reader = BufReader::new(input_file.expect("Given path does not lead to a file"));

        reader.lines().map(|line| line.unwrap()).collect_vec()
    }

    // gets input and returns solution
    // return type must be able to be printed
    fn solve<T: Display>(inputs: Vec<String>) -> T;

    // calls get_input and print the solution
    fn print_solution(path: &str);
}

pub struct Day01Solution;

impl Solution for Day01Solution {
    fn solve<T: Display>(inputs: Vec<String>) -> T {}

    fn print_solution(path: &str) {}
}
