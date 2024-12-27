#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn main() {
	let input = input_test!();

	let output = input
		.lines()
		.map(|report_str| {
			report_str
				.split(' ')
				.map(|level_str| level_str.parse::<isize>().unwrap())
				.collect::<Vec<isize>>()
		})
		.map(|report| {
			((report.windows(2).all(|ll| ll[0] < ll[1]))
				|| report.windows(2).all(|ll| ll[0] > ll[1]))
				&& report
					.windows(2)
					.all(|ll| (1..=3).contains(&isize::abs(ll[0] - ll[1])))
		})
		.map(|x| x as usize)
		.sum::<usize>();

	println!("{}", format!("{output:#?}").bright_blue().bold())
}
