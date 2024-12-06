use std::collections::{HashMap, HashSet};
use std::fs;

use anyhow::Result as Result;

type LabMap = HashMap<(i32, i32), Tile>;
type Start = ((i32, i32), (i32, i32));  // (coordinates, direction)

/// Positions in the Map.
#[derive(Eq, PartialEq)]
enum Tile {
    Open,
    Obstacle
}

/// Possible routes in the lab.
enum Route {
    Looping,
    Finite
}

pub fn solve_day_6(file: &str) -> Result<(usize, u32)> {
    let data = fs::read(file)?;
    let (mut lab_map, start) = create_lab_map(&data);
    let (tiles_covered, visited_tiles) = traverse_lab_part_one(&lab_map, &start);
    let obstacles_causing_loops = traverse_with_obstacles(&mut lab_map, &start, &visited_tiles);

    Ok((tiles_covered, obstacles_causing_loops))
}

/// Create a map of the lab as HashMap and provide the start location (and direction).
fn create_lab_map(data: &[u8]) -> (LabMap, Start) {
    let mut lab_map: LabMap = HashMap::new();
    let mut start: Option<(i32, i32)> = None;

    let mut row: i32 = 0;
    let mut col: i32 = 0;
    data
        .iter()
        .for_each(
            | char | {
                match char {
                    b'\n' => {row += 1; col = 0},
                    b'#' => { lab_map.insert((row, col), Tile::Obstacle); col += 1; },
                    b'.' => { lab_map.insert((row, col), Tile::Open); col += 1; },
                    b'^' => {
                        lab_map.insert((row, col), Tile::Open);
                        start = Some((row, col));
                        col += 1;
                    },
                    _ => unreachable!()
                };
            }
        );

    (lab_map, (start.unwrap(), (-1, 0)))  // We must have a start so just unwrap.
}

/// Take the map and place an obstacle on one of the visited tiles.
/// Check if we get into an infinite loop and count.
fn traverse_with_obstacles(
    lab_map: &mut LabMap, start: &Start, visited_tiles: &[(i32, i32)]
) -> u32 {
    let mut obstacles_causing_loops: u32 = 0;
    for tile in visited_tiles.iter() {
        *lab_map.get_mut(tile).unwrap() = Tile::Obstacle;  // Create new obstacle.

        match traverse_lab_part_two(lab_map, start) {
            Route::Looping => obstacles_causing_loops += 1,
            Route::Finite => {}  // Not relevant.
        }

        *lab_map.get_mut(tile).unwrap() = Tile::Open;  // Undo the new obstacle.
    }

    obstacles_causing_loops
}

/// Traverse the lab keeping track of each visited tile in a HashSet.
///
/// Once we go out of bounds (coordinate not in map), count the visited tiles.
///
/// For part two where we place obstacles provide an array of all visited tiles.
/// Here we cannot place an obstacle on the start so remove that from the set.
fn traverse_lab_part_one(lab_map: &LabMap, start: &Start) -> (usize, Vec<(i32, i32)>) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut step = *start;

    loop {
        let (coord, direction) = step;
        visited.insert(coord);

        let next_coord = (coord.0 + direction.0, coord.1 + direction.1);
        match lab_map.get(&next_coord) {
            Some(tile) => {
                match *tile {
                    Tile::Open => step = (next_coord, direction),
                    Tile::Obstacle => step = (coord, get_new_direction(&direction))
                }
            }
            None => break
        }
    }
    let visited_tiles_count = visited.len();
    visited.remove(&start.0);  // For part two get the visited tiles minus the start.

    (visited_tiles_count, Vec::from_iter(visited))
}


/// Traverse the lab keeping track of only the obstacles and directions we visited them.
///
/// If we visit an obstacle in the same direction as before we found an infinite looping route.
///
/// If we go out of bounds we found a finite route.
fn traverse_lab_part_two(lab_map: &LabMap, start: &Start) -> Route {
    let mut visited_obstacles: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
    let mut step = *start;

    loop {
        let(coord, direction) = step;

        let next_coord = (coord.0 + direction.0, coord.1 + direction.1);
        match lab_map.get(&next_coord) {
            Some(tile) => {
                match *tile {
                    Tile::Open => step = (next_coord, direction),
                    Tile::Obstacle => {
                        if !visited_obstacles.insert((coord, direction)) {
                            return Route::Looping
                        }

                        step = (coord, get_new_direction(&direction))
                    }
                }
            }
            None => return Route::Finite
        }
    }
}

/// Get a new direction based on the previous direction.
///
/// We always turn 90 degrees (to the right).
#[inline]
fn get_new_direction(direction: &(i32, i32)) -> (i32, i32) {
    match direction {
        (-1, 0) => (0, 1),   // Up -> Right
        (0, 1) => (1, 0),    // Right -> Down
        (1, 0) => (0, -1),   // Down -> Left
        (0, -1) => (-1, 0),  // Left -> Up
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] =
        b"....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...";

    #[test]
    fn test_traverse_lab() {
        let (lab_map, start) = create_lab_map(EXAMPLE);
        let (tiles_covered, _) = traverse_lab_part_one(&lab_map, &start);
        assert_eq!(tiles_covered, 41);
    }

    #[test]
    fn test_lab_loops() {
        let (mut lab_map, start) = create_lab_map(EXAMPLE);
        let (_, visited_tiles) = traverse_lab_part_one(&lab_map, &start);
        assert_eq!(traverse_with_obstacles(&mut lab_map, &start, &visited_tiles), 6);
    }
}
