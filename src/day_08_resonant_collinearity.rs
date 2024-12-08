use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;
use anyhow::Result as Result;

type NodeMap = HashMap<u8, Coordinates>;
type Coordinates = Vec<(i32, i32)>;
type Antinodes = HashSet<(i32, i32)>;

pub fn solve_day_08(file: &str) -> Result<(usize, usize)> {
    let data = fs::read(file)?;

    let (node_map, max_row, max_col) = generate_node_sets(&data);
    let antinode_count_one = count_antinodes_without_resonance(&node_map, max_row, max_col);
    let antinode_count_two =  count_antinodes_with_resonance(&node_map, max_row, max_col);

    Ok((antinode_count_one, antinode_count_two))
}

/// Create a map where all nodes of the same key are grouped.
fn generate_node_sets(map: &[u8]) -> (NodeMap, i32, i32) {
    let mut node_map: NodeMap = HashMap::new();

    let mut row = 0;
    let mut col = 0;

    let mut max_row = 0;
    let mut max_col = 0;

    map
        .iter()
        .for_each(
            | char | {
                match char {
                    b'.' => col += 1,
                    b'\n' => { row += 1; col = 0; }
                    _ => {
                        node_map
                            .entry(*char)
                            .and_modify(| val | val.push((row, col)))
                            .or_insert(vec![(row, col)]);
                        col += 1;
                    }
                }
                max_row = max_row.max(row);
                max_col = max_col.max(col);
            }
        );

    // Remove 1 from maxes, last row is newline and each row ends in newline.
    (node_map, max_row - 1, max_col - 1)
}

/// Count the antinodes without resonance.
/// 
/// Here is it important to remove the coordinates themselves from the nodes
/// Because those are technically "resonant" nodes.
/// 
/// Finally, do a bounds check on the nodes.
fn count_antinodes_without_resonance(node_map: &NodeMap, max_row: i32, max_col: i32) -> usize {
    let mut antinodes: Antinodes = HashSet::new();

    for (_key, coordinates) in node_map.iter() {
        let mut new_antinodes = count_antinodes_for_key(coordinates);
        
        // Remove the nodes with resonance unto themselves.
        coordinates
            .iter()
            .for_each(| node | { new_antinodes.remove(node); });
        
        antinodes = antinodes
            .union(&new_antinodes)
            .copied()
            .collect();
    }

    // Remove out of bounds entries.
    antinodes
        .iter()
        .filter(
            | (row, col ) | {
                *row >= 0 && *row <= max_row && *col >= 0 && *col <= max_col
            }
        )
        .count()
}

/// Count the antinodes with resonance.
/// 
/// To get the resonance we hardcoded just 50 steps in each direction.
/// This is very ugly, but it worked and was fast enough.
fn count_antinodes_with_resonance(node_map: &NodeMap, max_row: i32, max_col: i32) -> usize {
    let mut antinodes: Antinodes = HashSet::new();

    for (_key, val) in node_map.iter() {
        let new_antinodes = count_antinodes_for_key_with_resonance(val);
        antinodes = antinodes
            .union(&new_antinodes)
            .copied()
            .collect();
    }

    // Remove out of bounds entries.
    antinodes
        .iter()
        .filter(
            | (row, col ) | {
                *row >= 0 && *row <= max_row && *col >= 0 && *col <= max_col
            }
        )
        .count()
}

/// Get the combinations of coordinates.
/// Calculate the delta. Insert all deltas in the set.
/// This also contains the coordinates themselves which need to be removed later.
#[inline]
fn count_antinodes_for_key(coordinates: &Coordinates) -> Antinodes {
    let mut antinodes: Antinodes = HashSet::new();
    
    coordinates
        .iter()
        .combinations(2)
        .for_each(
            | pair | {
                let dy = pair[0].0 - pair[1].0;
                let dx = pair[0].1 - pair[1].1;
                
                antinodes.insert((pair[0].0 + dy, pair[0].1 + dx));
                antinodes.insert((pair[0].0 - dy, pair[0].1 - dx));
                antinodes.insert((pair[1].0 + dy, pair[1].1 + dx));
                antinodes.insert((pair[1].0 - dy, pair[1].1 - dx));
            }
        );

    antinodes
}

/// Get the combinations of coordinates.
/// Calculate the deltas for many resonance values.
/// This could be done better, but this worked.
#[inline]
fn count_antinodes_for_key_with_resonance(coordinates: &Coordinates) -> Antinodes {
    let mut antinodes: Antinodes = HashSet::new();

    coordinates
        .iter()
        .combinations(2)
        .for_each(
            | pair | {
                for resonance in 1..50 {  // TODO instead of hard coding 50 loops use a nicer exit condition.
                    let dy = (pair[0].0 - pair[1].0) * resonance;
                    let dx = (pair[0].1 - pair[1].1) * resonance;
                    
                    antinodes.insert((pair[0].0 + dy, pair[0].1 + dx));
                    antinodes.insert((pair[0].0 - dy, pair[0].1 - dx));
                    antinodes.insert((pair[1].0 + dy, pair[1].1 + dx));
                    antinodes.insert((pair[1].0 - dy, pair[1].1 - dx));
                }
            }
        );

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] =
        b"............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n";

    #[test]
    fn test_count_antinodes_without_resonance() {
        let (node_map, max_row, max_col) = generate_node_sets(EXAMPLE);
        assert_eq!(count_antinodes_without_resonance(&node_map, max_row, max_col), 14);
    }

    #[test]
    fn test_count_antinodes_with_resonance() {
        let (node_map, max_row, max_col) = generate_node_sets(EXAMPLE);
        assert_eq!(count_antinodes_with_resonance(&node_map, max_row, max_col), 34);
    }
}
