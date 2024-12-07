#[allow(unused)]
use aoc::{input, input_test};
use colored::*;

fn is_valid_page(page: &[u32], rules: &[[u32; 2]]) -> bool {
    for &[before, after] in rules {
        let idx_before = page.iter().position(|&item| item == before);
        let idx_after = page.iter().position(|&item| item == after);

        if idx_before.is_none() || idx_after.is_none() {
            continue;
        }

        let idx_before = idx_before.unwrap();
        let idx_after = idx_after.unwrap();

        if idx_before > idx_after {
            return false;
        }
    }

    true
}

fn main() {
    let input = input!().replace("\r\n", "\n");

    let [rules, pages]: [&str; 2] = input
        .splitn(2, "\n\n")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let rules = rules
        .lines()
        .map(|line| {
            line.split('|')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<[u32; 2]>>();

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let pages = pages
        .iter()
        .filter(|&page| is_valid_page(page, &rules))
        .collect::<Vec<&Vec<u32>>>();

    let out = pages
        .iter()
        .map(|&x| x[x.len().div_ceil(2) - 1])
        .sum::<u32>();

    // println!("{}", format!("{:#?}", rules).bright_blue().bold());
    println!("{}", format!("{:#?}", out).bright_blue().bold());
}
