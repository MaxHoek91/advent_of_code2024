use std::collections::HashMap;
use std::fs;
use anyhow::Result as Result;

type Memory = HashMap<(u64, u8), u64>;

pub fn solve_day_10(file: &str) -> Result<(u64, u64)> {
    let data: Vec<u64> = fs::read_to_string(file)?
        .split_ascii_whitespace()
        .map(| val | val.parse().unwrap())
        .collect();
    
    let mut memory: Memory = HashMap::new();
    let stones_part_one: u64 = data
        .iter()
        .map(| val | counting_stones(*val, 25, &mut memory))
        .sum();
    
    let stones_part_two: u64 = data
        .into_iter()
        .map(| val | counting_stones(val, 75, &mut memory))
        .sum();
    
    Ok((stones_part_one, stones_part_two))
}

/// Count the stones using a recursive function with memoization.
fn counting_stones(stone: u64, blinks: u8, memory: &mut Memory) -> u64 {
    if blinks == 0 {
        return 1  // Base case
    }

    if let Some(val) = memory.get(&(stone, blinks)) {
        return *val // Return memoized value.
    }

    // Do conversion and recursion.
    let mut count = 0;  // TODO clippy complains, check how to best improve.
    if stone == 0 {
        count = counting_stones(1, blinks - 1, memory);
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let m = 10u64.pow((stone.ilog10() + 1) / 2);
        count = counting_stones(stone / m, blinks - 1, memory)
            + counting_stones(stone % m, blinks - 1, memory);
    } else {
        count = counting_stones(stone * 2024, blinks - 1, memory);
    }
    
    memory.insert((stone, blinks), count);  // Add to memory
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: [u64; 2] = [125, 17];

    #[test]
    fn test_blinks() {
        let mut memory: Memory = HashMap::new();
        let total_stones: u64 = EXAMPLE
            .into_iter()
            .map(| val | counting_stones(val, 25, &mut memory))
            .sum();
        
        assert_eq!(total_stones, 55312);
    }
}
