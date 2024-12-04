use std::collections::HashMap;
use std::fs;

use anyhow::Result as Result;

type LetterMap = HashMap<(i32, i32), u8>;
type StartCoordsOne = Vec<((i32, i32), (i32, i32))>;
type StartCoordsTwo = Vec<(i32, i32)>;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),  // North West
    (0, -1),   // North
    (1, -1),   // North East
    (-1, 0),   // West
    (1, 0),    // East
    (-1, 1),   // South West
    (0, 1),    // South
    (1, 1)     // South East
];
const DIAGS: [(i32, i32); 4] = [
    (-1, -1),  // North West
    (1, 1),     // South East
    (1, -1),  // North East
    (-1, 1)   // South West
];

pub fn solve_day_four(file: &str) -> Result<(u32, u32)> {
    let data = fs::read(file)?;

    let (map, starts_one, start_two) = prepare_map(&data);
    let xmas_count = search_xmas(&map, starts_one);
    let x_mas_count = search_x_mas(&map, start_two);

    Ok((xmas_count, x_mas_count))
}

/// Create a map of with coordinates for each letter.
/// Additionally create start coordinates for X (part one) and A (part two).
fn prepare_map(data: &[u8]) -> (LetterMap, StartCoordsOne, StartCoordsTwo) {
    let mut letter_map: LetterMap = HashMap::new();
    let mut start_coords_one: StartCoordsOne = Vec::new();
    let mut start_coords_two: StartCoordsTwo = Vec::new();

    let mut row = 0;
    let mut col = 0;
    for &char in data.iter() {
        match char {
            b'\n' => { row += 1; col = 0; },
            b'X' => {
                DIRECTIONS  // For each direction we can search add a start coordinate.
                    .iter()
                    .for_each(
                        | dir | start_coords_one.push(((row, col), *dir))
                    );

                letter_map.insert((row, col), char);
                col += 1
            },
            b'A' => {
                start_coords_two.push((row, col));
                letter_map.insert((row, col), char);
                col += 1
            },
            _ => {
                letter_map.insert((row, col), char);
                col += 1
            }
        }
    }

    (letter_map, start_coords_one, start_coords_two)
}

/// Do a search through the coordinates to see if X->M, M->A, and A->S
fn search_xmas(map: &LetterMap, starts: StartCoordsOne) -> u32 {
    let mut xmas_count: u32 = 0;
    
    for ((x, y), (dx, dy)) in starts.iter() {
        let m = map.get(&(x + dx, y + dy));
        if m != Some(&b'M') {
            continue
        }
        
        let a = map.get(&(x + (dx * 2), y + (dy * 2)));
        if a != Some(&b'A') {
            continue
        }
        
        let s = map.get(&(x + (dx * 3), y + (dy * 3)));
        if s == Some(&b'S') {
            xmas_count += 1;
        }
    }

    xmas_count
}

/// Check is around the start coordinate we find M and S on the diagonals.
fn search_x_mas(map: &LetterMap, starts: StartCoordsTwo) -> u32 {
    let mut x_mas_count: u32 = 0;

    for coord in starts.iter() {
        let diag_chars: Vec<&u8> = DIAGS
            .iter()
            .filter_map(| diag | map.get(&(coord.0 + diag.0, coord.1 + diag.1)))
            .collect();

        if diag_chars.len() != 4 {
            continue
        }

        if (diag_chars[0..2].contains(&&b'M') && diag_chars[0..2].contains(&&b'S'))
            && (diag_chars[2..].contains(&&b'M') && diag_chars[2..].contains(&&b'S')) {
            x_mas_count += 1;
        }
    }

    x_mas_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] =
        b"MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX";

    #[test]
    fn test_search_xmas() {
        let (map, starts, _) = prepare_map(EXAMPLE);
        assert_eq!(search_xmas(&map, starts), 18);
    }

    #[test]
    fn test_search_x_mas() {
        let (map, _, starts) = prepare_map(EXAMPLE);
        assert_eq!(search_x_mas(&map, starts), 9);
    }
}
