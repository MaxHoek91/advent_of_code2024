use std::collections::HashMap;
use std::fs;

use anyhow::Result as Result;

pub fn solve_day_one(file: &str) -> Result<(u32, u32)> {
    let data = fs::read_to_string(file)?;

    let mut distances_left: Vec<u32> = Vec::new();
    let mut distances_right: Vec<u32> = Vec::new();

    for line in data.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        distances_left.push(left.trim().parse::<u32>().unwrap());
        distances_right.push(right.trim().parse::<u32>().unwrap());
    }
    let sum_of_distances = determine_sum_of_differences(
        &mut distances_left, &mut distances_right
    );
    
    let similarity_score = determine_similarity_score(distances_left, distances_right);

    Ok((sum_of_distances, similarity_score))
}

fn determine_sum_of_differences(distances_left: &mut[u32], distances_right: &mut[u32]) -> u32 {
    distances_left.sort_unstable();
    distances_right.sort_unstable();

    let mut sum_of_differences: u32 = 0;
    for (left, right) in distances_left.iter().zip(distances_right) {
        sum_of_differences += left.abs_diff(*right);
    }
    sum_of_differences
}

fn determine_similarity_score(distances_left: Vec<u32>, distances_right: Vec<u32>) -> u32 {
    let mut distance_counts: HashMap<u32, u32> = HashMap::new();
    for right in distances_right.iter() {
        distance_counts
            .entry(*right)
            .and_modify(| value | *value += 1)
            .or_insert(1);
    }

    let mut similarity_score: u32 = 0;
    for left in distances_left.iter() {
        match distance_counts.get(left) {
            Some(value) => {similarity_score += left * value}
            None => continue
        }
    }

    similarity_score
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
