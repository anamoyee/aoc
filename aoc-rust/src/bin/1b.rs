use aoc::input;
use colored::*;

fn main() {
    let (l, r): (Vec<_>, Vec<_>) = input!()
        .lines()
        .map(|x| x.split_once(' ').unwrap())
        .map(|x| (x.0.trim(), x.1.trim()))
        .map(|x| (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap()))
        .unzip();

    let output = l
        .iter()
        .map(|x| x * r.iter().filter(|&y| y == x).count() as i32)
        .sum::<i32>();

    println!("{}", format!("{output}").bright_blue().bold())
}
