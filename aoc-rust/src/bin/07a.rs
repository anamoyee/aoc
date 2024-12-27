#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn sums_or_products(rest: &[i64]) -> Vec<i64> {
    let mut thisvec: Vec<i64> = Vec::new();

    match rest {
        [] => panic!("empty rest; whot??"),
        [one] => thisvec.push(*one),
        [one, two] => {
            thisvec.push(one + two);
            thisvec.push(one * two);
        }
        [rest @ .., one] => {
            let innervec = sums_or_products(rest);

            for two in innervec {
                thisvec.push(one * two);
                thisvec.push(one + two);
            }
        }
    };

    thisvec
}

fn main() {
    let input = input!().replace("\r\n", "\n");

    let input = input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(testvalue_str, rest)| {
            (
                testvalue_str.parse::<i64>().unwrap(),
                rest.trim()
                    .split(' ')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let mut correct_testvalue_sum = 0;

    for (testvalue, rest) in &input {
        if sums_or_products(rest).contains(testvalue) {
            correct_testvalue_sum += testvalue;
        }
    }

    println!("{}", correct_testvalue_sum)
}
