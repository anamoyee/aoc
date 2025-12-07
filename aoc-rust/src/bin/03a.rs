use aoc::prelude::*;

fn main() {
    let soom = input!()
        .split("\n")
        .map(|bank| {
            let batteries = bank
                .chars()
                .map(|num_char| num_char.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>();

            let mut largest_value: u8 = 0;

            for (b1_i, b1_value) in batteries.iter().enumerate() {
                let batteries_later = &batteries[b1_i + 1..];

                if batteries_later.is_empty() {
                    continue;
                }

                for b2_value in batteries_later {
                    let total_value = b1_value * 10 + b2_value; // still fits u8

                    largest_value = u8::max(largest_value, total_value);
                }
            }

            largest_value as u32
        })
        .sum::<u32>();

    ans!(soom);
}
