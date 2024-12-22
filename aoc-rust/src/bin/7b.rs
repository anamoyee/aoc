#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn generate_combinations(operators: &[char], length: usize) -> Vec<Vec<char>> {
    if length == 0 {
        return vec![vec![]];
    }

    let smaller_combinations = generate_combinations(operators, length - 1);
    let mut combinations = Vec::new();

    for comb in smaller_combinations {
        for &op in operators {
            let mut new_comb = comb.clone();
            new_comb.push(op);
            combinations.push(new_comb);
        }
    }

    combinations
}

fn evaluate_expressions(numbers: &[i64]) -> Vec<(String, i64)> {
    if numbers.len() < 2 {
        panic!("The list must contain at least two numbers.");
    }

    let operators = ['*', '+', '|'];
    let operator_combinations = generate_combinations(&operators, numbers.len() - 1);

    let mut results = Vec::new();

    for operator_set in operator_combinations {
        let mut expression = numbers[0].to_string();
        let mut current_value = numbers[0];

        for (i, &op) in operator_set.iter().enumerate() {
            let next_number = numbers[i + 1];
            expression.push(op);
            expression.push_str(&next_number.to_string());

            current_value = match op {
                '+' => current_value + next_number,
                '|' => format!("{}{}", current_value, next_number)
                    .parse::<i64>()
                    .unwrap(),
                '*' => current_value * next_number,
                _ => unreachable!(),
            };
        }

        results.push((expression, current_value));
    }

    results
}

fn any_equals(testvalue: &i64, numbers: &[i64]) -> i64 {
    let results = evaluate_expressions(numbers);

    for (_, value) in results {
        if *testvalue == value {
            return value;
        }
    }

    0
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

    for (i, (testvalue, rest)) in input.iter().enumerate() {
        println!("{i}");
        correct_testvalue_sum += any_equals(testvalue, rest);
    }

    println!("{}", correct_testvalue_sum)
}
