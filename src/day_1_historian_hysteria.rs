use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::fmt::Debug;

use anyhow::Result as Result;

pub fn solve_day_one(file: &str) -> Result<(u32, u32)> {
    let data = fs::read_to_string(file)?;

    // Store the 2 distance columns into two separate vectors.
    let mut distances_left: Vec<u32> = Vec::new();
    let mut distances_right: Vec<u32> = Vec::new();
    for line in data.lines() {
        let (left, right) =  match line.split_once(' ') {
            Some((l, r)) => (l, r),
            None => continue // No space found within row.
        };
        distances_left.push(parse_int(left));
        distances_right.push(parse_int(right));
    }
    
    // Calculate the sum of differences.
    let sum_of_distances = determine_sum_of_differences(
        &mut distances_left, &mut distances_right
    );

    // Calculate the similarity score.
    let similarity_score = determine_similarity_score(distances_left, distances_right);

    Ok((sum_of_distances, similarity_score))
}

/// Parse the string value to a number of type T.
#[inline]
fn parse_int<T>(value: &str) -> T
where T: FromStr, <T as FromStr>::Err: Debug {
    value.trim().parse::<T>().unwrap()
}

/// Determine the total sum of differences of two arrays.
/// 
/// First, sort both arrays from low-to-high.
/// Then iterate over both arrays, calculate the absolute difference, and return the sum.
fn determine_sum_of_differences(distances_left: &mut[u32], distances_right: &mut[u32]) -> u32 {
    distances_left.sort_unstable();
    distances_right.sort_unstable();

    distances_left
        .iter()
        .zip(distances_right)
        .map(| (left, right) | left.abs_diff(*right))
        .sum::<u32>()
}

/// Determine the similarity score of two arrays.
/// 
/// Count the occurrence of each number in the right array, and store in a map
/// Iterate over the left array and take left_value * occurrence in right.
fn determine_similarity_score(distances_left: Vec<u32>, distances_right: Vec<u32>) -> u32 {
    // Create the map.
    let mut distance_counts: HashMap<u32, u32> = HashMap::new();
    for right in distances_right.iter() {
        distance_counts
            .entry(*right)
            .and_modify(| value | *value += 1)
            .or_insert(1);
    }

    // Calculate the similarity.
    distances_left
        .iter()
        .map(
            | left | {
                match distance_counts.get(left) {
                    Some(value) => left * value,
                    None => 0 
                }
            }
        )
        .sum::<u32>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_sum_of_differences() {
        let mut distances_left = vec![3, 4, 2, 1, 3, 3];
        let mut distances_right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(determine_sum_of_differences(&mut distances_left, &mut distances_right), 11)
    }

    #[test]
    fn test_determine_similarity_score() {
        let distances_left = vec![3, 4, 2, 1, 3, 3];
        let distances_right = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(determine_similarity_score(distances_left, distances_right), 31)
    }
}
