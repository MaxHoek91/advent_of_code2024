use std::collections::HashSet;
use std::fs;

use anyhow::Result as Result;

type TopoGraphicMap = Vec<Vec<u8>>;
type Coordinate = (i32, i32, u8);

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0),  // North
    (0, 1),   //  East
    (1, 0),   // South
    (0, -1)   // West
];

pub fn solve_day_10(file: &str) -> Result<(u32, u32)> {
    let data = fs::read(file)?;

    let mut trail_score: u32 = 0;
    let mut trail_rating: u32 = 0;

    let (topographic_map, starts) = create_topographic_map(&data);
    starts
        .iter()
        .for_each(
            | start | {
                trail_score += hike_trail(&topographic_map, start, true);
                trail_rating += hike_trail(&topographic_map, start, false);
            }
        );


    Ok((trail_score, trail_rating))
}

/// Create a map of the hiking trail elevation, also track the starting positions (height=0).
fn create_topographic_map(data: &[u8]) -> (TopoGraphicMap, Vec<Coordinate>) {
    let mut topographic_map: TopoGraphicMap = Vec::new();
    let mut row: Vec<u8> = Vec::new();

    let mut starts: Vec<Coordinate> = Vec::new();
    let mut x_val = 0;
    let mut y_val = 0;

    data
        .iter()
        .for_each(
            | char | {
                match char {
                    b'\n' => {
                        topographic_map.push(row.clone());
                        row = Vec::new();
                        x_val = 0; y_val += 1;
                    },
                    b'0' => {
                        row.push(char - b'0');
                        starts.push((y_val, x_val, char - b'0'));
                        x_val += 1
                    }
                    _ => {
                        row.push(char - b'0');
                        x_val += 1;
                    }
                }
            }
        );

    (topographic_map, starts)
}

/// Hike the trail by breadth-first-search, until we reach a summit (height=9).
/// 
/// Single Route determines if we allow a single route to a summit
/// or if we can have multiple routes to the same summit.
fn hike_trail(
    topographic_map: &TopoGraphicMap, starting_point: &Coordinate, single_route: bool
) -> u32 {
    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut queue: Vec<Coordinate> = vec![*starting_point];
    let mut goals_reached: u32 = 0;

    while let Some(current) = queue.pop() {
        if single_route && !visited.insert(current) {
            continue
        };

        let (row, col, val) = current;

        if val == 9 {
            goals_reached += 1;
            continue
        }

        for direction in DIRECTIONS {
            let new_row = row + direction.0;
            let new_col = col + direction.1;

            if let Some(slice) = topographic_map.get(new_row as usize) {
                if let Some(&new_val) = slice.get(new_col as usize) {
                    if new_val == val + 1 {
                        queue.push((new_row, new_col, new_val));
                    }
                }
            }
        }
    }

    goals_reached
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] =
        b"89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n";

    #[test]
    fn test_part_one() {
        let (map, starts) = create_topographic_map(EXAMPLE);
        let mut goals_reached = 0;

        starts
            .iter()
            .for_each(
                | start | {
                    goals_reached += hike_trail(&map, start, true);
                }
            );

        assert_eq!(goals_reached, 36);
    }

    #[test]
    fn test_part_two() {
        let (map, starts) = create_topographic_map(EXAMPLE);
        let mut goals_reached = 0;

        starts
            .iter()
            .for_each(
                | start | {
                    goals_reached += hike_trail(&map, start, false);
                }
            );

        assert_eq!(goals_reached, 81);
    }
}
