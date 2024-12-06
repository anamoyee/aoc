#[allow(unused)]
use aoc::{input, input_test};
use colored::*;
use regex::Regex;

fn main() {
    let input = input!();

    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    let mut out = 0;
    re.captures_iter(&input).for_each(|caps| {
        out += caps[1].parse::<i32>().unwrap() * caps[2].parse::<i32>().unwrap();
    });

    println!("{}", format!("{:#?}", out).bright_blue().bold())
}
