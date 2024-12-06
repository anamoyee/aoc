use colored::*;
use std::fs;
use std::path::Path;

pub fn _read_input(day: u8, suffix: &str) -> String {
    let crate_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(crate_root)
        .join("..")
        .join("inputs")
        .join(format!("{day}{suffix}.txt"));

    fs::read_to_string(&input_path).unwrap_or_else(|err| {
        panic!(
            "{}",
            format!("Failed to read input file at {:?}: {}", input_path, err)
                .bright_red()
                .bold()
        );
    })
}

#[macro_export]
macro_rules! input {
    () => {
        aoc::_read_input(
            file!()
                .replace("\\", "/")
                .split('/')
                .last()
                .unwrap()
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u8>()
                .expect(
                    "Failed to extract day from file name"
                        .bright_red()
                        .bold()
                        .as_ref(),
                ),
            "",
        )
    };
}

#[macro_export]
macro_rules! input_test {
    () => {
        aoc::_read_input(
            file!()
                .replace("\\", "/")
                .split('/')
                .last()
                .unwrap()
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<u8>()
                .expect(
                    "Failed to extract day from file name"
                        .bright_red()
                        .bold()
                        .as_ref(),
                ),
            "_test",
        )
    };
}
