mod day01;
use day01::{Day01Solution, Solution};

mod day02;
use day02::Day02solution;

extern crate pest;
#[macro_use]
extern crate pest_derive;

fn main() {
    Day01Solution::print_solution("input/day01.txt");
    Day02solution::print_solution("input/day02.txt");
}
