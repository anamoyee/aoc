use aoc::prelude::*;

fn main() {
    let ranges = input!()
        .trim()
        .split(',')
        .map(|s| {
            s.split_once('-')
                .map(|(s1, s2)| s1.parse::<u64>().unwrap()..=s2.parse::<u64>().unwrap())
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut sum: u128 = 0;

    for range in ranges {
        for i in range {
            let i_stri = i.to_string();
            let i_stri_len = i_stri.len();

            if i_stri_len % 2 != 0 {
                continue;
            }

            let (first, second) = i_stri.split_at(i_stri_len / 2);

            if first == second {
                sum += i as u128;
            }
        }
    }

    ans!(sum);
}
