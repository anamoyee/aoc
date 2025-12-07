use std::ops::Range;

use aoc::prelude::*;

fn vis(
    batteries: &Vec<u8>,
    _slice_to_search: &[u8],
    range: Range<usize>,
    found_i: &usize,
    found_value: &u8,
) {
    let x = batteries
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");

    eprintln!(
        " {} -> v[{:>2}]={} ({})",
        format!("{}", x).blue().bold(),
        format!("{}", found_i).blue().bold(),
        format!("{}", found_value).blue().bold(),
        format!("{:#?}", range).blue(),
    );

    let mut bar: Vec<char> = vec![' '; batteries.len() + 2];
    bar[*found_i + 1] = '^';
    bar[range.start - 1] = '[';
    bar[range.end + 1] = ']';
    eprintln!("{}", bar.iter().collect::<String>());
}

fn main() {
    let soom = input!()
        .split("\n")
        .map(|bank_str| {
            let batteries = bank_str
                .chars()
                .map(|num_char| num_char.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();

            let mut begin_i = 0usize;
            let mut this_bank_sum = 0u64;

            for reserve_from_the_end in (0..=11usize).rev() {
                let slice_to_search = &batteries[begin_i..(batteries.len() - reserve_from_the_end)];

                let (largest_i, &largest_value) = slice_to_search
                    .iter()
                    .enumerate()
                    .max_by_key(|&(i, &n)| (n, -(i as isize)))
                    .map(|(i, n)| (i + begin_i, n))
                    .unwrap();

                begin_i = largest_i + 1;
                this_bank_sum *= 10;
                this_bank_sum += largest_value as u64;

                vis(
                    &batteries,
                    slice_to_search,
                    begin_i..(batteries.len() - reserve_from_the_end),
                    &largest_i,
                    &largest_value,
                );
            }

            this_bank_sum
        })
        .sum::<u64>();

    ans!(soom);
}
