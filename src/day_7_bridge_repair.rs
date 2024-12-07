use std::fs;
use std::ops::{Add, Mul};

use anyhow::Result as Result;

pub fn solve_day_7(file: &str) -> Result<(u64, u64)> {
    let data = fs::read_to_string(file)?;

    let mut total_of_valid_equations = 0;
    let mut total_valid_with_concat = 0;

    for line in data.lines() {
        let (target, series) = line.split_once(": ").unwrap();
        let target: u64 = target.parse().unwrap();
        let series: Vec<u64> = series
            .split_ascii_whitespace()
            .map(| val | val.parse().unwrap())
            .collect();

        if let Some(value) = check_valid_equation(&series, target, false) {
            total_of_valid_equations += value;
            total_valid_with_concat += value;
        } else if let Some(value) = check_valid_equation(&series, target, true) {
            total_valid_with_concat += value;
        }
    }

    Ok((total_of_valid_equations, total_valid_with_concat))
}


/// Check if the series fits to the target value.
/// 
/// For part 1 use_concat = false, for part 2 use concat = true
///
/// Keep track of all attempts
/// For each attempt generate the addition, multiplication, (and concatenation)
/// Remove the attempts that go out of bounds
/// At the end check if one of the attempts matches the target number.
/// 
/// Notes
/// You cannot easily check the upper and lower bounds.
/// I thought that the series.sum() would be the lowest and series.product() would the highest.
/// However, this doesn't work if the number 1 occurs within the series.
#[inline]
fn check_valid_equation(series: &[u64], target: u64, use_concat: bool) -> Option<u64> {
    let mut attempts: Vec<u64> = vec![series[0]];
    for number in series.iter().skip(1) {
        std::mem::take(&mut attempts)
            .iter()
            .for_each(
                |val|
                    {
                        let new_num = number.add(val);
                        if new_num <= target {
                            attempts.push(new_num);
                        }
                        let new_num = number.mul(val);
                        if new_num <= target {
                            attempts.push(new_num);
                        }

                        if use_concat {
                            let new_num: u64 = format!("{}{}", val, number).parse().unwrap();
                            if new_num <= target {
                                attempts.push(new_num);
                            }
                        }
                    }
            );
    }

    match attempts.contains(&target) {
        true => Some(target),
        false => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: [u64; 2] = [10, 19];
    const EXAMPLE2: [u64; 3] = [81, 40, 27];
    const EXAMPLE3: [u64; 2] = [17, 5];
    const EXAMPLE4: [u64; 2] = [15, 6];
    const EXAMPLE5: [u64; 4] = [6, 8, 6, 15];
    const EXAMPLE6: [u64; 3] = [16, 10, 13];
    const EXAMPLE7: [u64; 3] = [17, 8, 14];
    const EXAMPLE8: [u64; 4] = [9, 7, 18, 13];
    const EXAMPLE9: [u64; 4] = [11, 6, 16, 20];

    #[test]
    fn test_check_valid_equations_part_one() {
        assert_eq!(check_valid_equation(&EXAMPLE1, 190, false), Some(190));
        assert_eq!(check_valid_equation(&EXAMPLE2, 3267, false), Some(3267));
        assert_eq!(check_valid_equation(&EXAMPLE3, 83, false), None);
        assert_eq!(check_valid_equation(&EXAMPLE4, 156, false), None);
        assert_eq!(check_valid_equation(&EXAMPLE5, 7290, false), None);
        assert_eq!(check_valid_equation(&EXAMPLE6, 161011, false), None);
        assert_eq!(check_valid_equation(&EXAMPLE7, 192, false), None);
        assert_eq!(check_valid_equation(&EXAMPLE8, 21037, false), None);
        assert_eq!(check_valid_equation(&EXAMPLE9, 292, false), Some(292));
    }

    #[test]
    fn test_check_valid_equations_part_two() {
        assert_eq!(check_valid_equation(&EXAMPLE3, 83, true), None);
        assert_eq!(check_valid_equation(&EXAMPLE4, 156, true), Some(156));
        assert_eq!(check_valid_equation(&EXAMPLE5, 7290, true), Some(7290));
        assert_eq!(check_valid_equation(&EXAMPLE6, 161011, true), None);
        assert_eq!(check_valid_equation(&EXAMPLE7, 192, true), Some(192));
        assert_eq!(check_valid_equation(&EXAMPLE8, 21037, true), None);
    }
}
