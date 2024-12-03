use std::fs;
use regex::{Captures, Regex};
use anyhow::Result as Result;

pub fn solve_day_three(file: &str) -> Result<(u32, u32)> {
    let data = fs::read_to_string(file)?;
    Ok((parse_mul(&data), parse_mul_with_do(&data)))
}

/// Parse the "mul(x,y)" values in the data using a regular expression and calculate the sum.
fn parse_mul(data: &str) -> u32 {
    let regexpr = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    regexpr
        .captures_iter(data)
        .map(
            | capture | {
                let val_left = extract_value(&capture, 1);
                let val_right = extract_value(&capture, 2);
                val_left * val_right
            }
        )
        .sum()
}

/// Parse the mul values including the do and don't conditional statements.
fn parse_mul_with_do(data: &str) -> u32 {
    let regexpr = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();

    let mut mul_active: bool = true;
    let mut total: u32 = 0;

    for capture in regexpr.captures_iter(data) {
        match capture.get(0).unwrap().as_str() {
            "do()" => mul_active = true,
            "don't()" => mul_active = false,
            _ if mul_active => {
                let val_left = extract_value(&capture, 1);
                let val_right = extract_value(&capture, 2);
                total += val_left * val_right;
            }
            _ => continue // mull is inactive, just continue.
        }
    }

    total
}

/// Extract the numeric value from the regex capture group.
#[inline]
fn extract_value(capture: &Captures, index: usize) -> u32 {
    capture.get(index).unwrap().as_str().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_ONE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_TWO: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse_mul(EXAMPLE_ONE), 161);
    }

    #[test]
    fn test_parse_mul_with_do() {
        assert_eq!(parse_mul_with_do(EXAMPLE_TWO), 48);
    }
}
