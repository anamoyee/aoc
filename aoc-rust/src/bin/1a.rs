use aoc::input;
use colored::*;

fn main() {
	let (mut l, mut r): (Vec<_>, Vec<_>) = input!()
		.lines()
		.map(|x| x.split_once(' ').unwrap())
		.map(|x| (x.0.trim(), x.1.trim()))
		.map(|x| (x.0.parse::<i32>().unwrap(), x.1.parse::<i32>().unwrap()))
		.unzip();

	l.sort();
	r.sort();

	let output = (l.into_iter().zip(r))
		.map(|x| (x.0 - x.1).unsigned_abs())
		.sum::<u32>();

	println!("{}", format!("{output}").bright_blue().bold())
}
