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

            for chunk_size in 1..i_stri_len {
                if i_stri_len % chunk_size != 0 {
                    continue;
                }

                if i_stri
                    .chars()
                    .collect::<Vec<_>>()
                    .chunks(chunk_size)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .map(|item| Some(item))
                    .reduce(|acc, item| if acc == item { acc } else { None })
                    .unwrap()
                    .is_some()
                {
                    sum += i as u128;
                    break;
                }
            }
        }
    }

    ans!(sum);
}
