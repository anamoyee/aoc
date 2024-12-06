#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn is_valid_report(report: &[isize]) -> bool {
    if !report.windows(2).all(|arr| arr[0] < arr[1])
        && !report.windows(2).all(|arr| arr[0] > arr[1])
    {
        return false;
    }

    if !report
        .windows(2)
        .all(|arr| (1..=3).contains(&isize::abs(arr[0] - arr[1])))
    {
        return false;
    }

    true
}

fn main() {
    let input = input!();

    let out = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|number_str| number_str.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = out
        .iter()
        .map(|report| {
            let mut outer_vec = Vec::new();

            for i in 0..report.len() {
                let mut inner_vec = Vec::new();

                for (j, &el) in report.iter().enumerate() {
                    if i == j {
                        continue;
                    }

                    inner_vec.push(el);
                }
                outer_vec.push(inner_vec);
            }

            outer_vec
        })
        .collect::<Vec<_>>();

    let out = out
        .iter()
        .map(|report_permutations| {
            report_permutations
                .iter()
                .any(|report| is_valid_report(report))
        })
        .filter(|&x| x)
        .count();

    println!("{}", format!("{:#?}", out).bright_blue().bold())
}
