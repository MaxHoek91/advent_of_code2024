use std::fs;

use anyhow::Result as Result;

pub fn solve_day_two(file: &str) -> Result<(u32, u32)> {
    let data = fs::read_to_string(file)?;

    let mut safe_rows_part_one: u32 = 0;
    let mut safe_rows_part_two: u32 = 0;

    for line in data.lines() {
        let row: Vec<u8> = line
            .split_ascii_whitespace()
            .map(| val | val.parse::<u8>().unwrap())
            .collect();

        if is_report_safe(&row) {
            safe_rows_part_one += 1;
            safe_rows_part_two += 1;
        } else if is_report_safe_with_single_error(&row) {
            safe_rows_part_two += 1;
        }
    }

    Ok((safe_rows_part_one, safe_rows_part_two))
}

/// Check if an array is sorted from low-to-high or high-to-low
/// With the constraint that the maximum difference between elements must be > 1 and <= 3
/// The > 1 check is implied by checking a < b instead of using <=
#[inline]
fn is_report_safe(row: &[u8]) -> bool {
    row.is_sorted_by(|a, b | a < b && a.abs_diff(*b) <= 3)
        || row.is_sorted_by(| a, b| b < a && a.abs_diff(*b) <= 3)
}

/// Check if an array is sorted while ignoring a single value.
/// Uses is_report_safe while removing a single value from the array.
#[inline]
fn is_report_safe_with_single_error(row: &[u8]) -> bool {
    for i in 0..row.len() {
        let mut new_row = Vec::from(row);
        new_row.remove(i);
        if is_report_safe(&new_row) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW_1: [u8; 5] = [7, 6, 4, 2, 1];
    const ROW_2: [u8; 5] = [1, 2, 7, 8, 9];
    const ROW_3: [u8; 5] = [9, 7, 6, 2, 1];
    const ROW_4: [u8; 5] = [1, 3, 2, 4, 5];
    const ROW_5: [u8; 5] = [8, 6, 4, 4, 1];
    const ROW_6: [u8; 5] = [1, 3, 6, 7, 9];
    const ROW_7: [u8; 6] = [20, 17, 18, 21, 23, 25];

    #[test]
    fn test_check_safe_report_part_one() {
        assert!(is_report_safe(&ROW_1));
        assert!(!is_report_safe(&ROW_2));
        assert!(!is_report_safe(&ROW_3));
        assert!(!is_report_safe(&ROW_4));
        assert!(!is_report_safe(&ROW_5));
        assert!(is_report_safe(&ROW_6));
    }

    #[test]
    fn test_check_safe_report_part_two() {
        assert!(is_report_safe_with_single_error(&ROW_1));
        assert!(!is_report_safe_with_single_error(&ROW_2));
        assert!(!is_report_safe_with_single_error(&ROW_3));
        assert!(is_report_safe_with_single_error(&ROW_4));
        assert!(is_report_safe_with_single_error(&ROW_5));
        assert!(is_report_safe_with_single_error(&ROW_6));

        assert!(is_report_safe_with_single_error(&ROW_7));
    }
}
