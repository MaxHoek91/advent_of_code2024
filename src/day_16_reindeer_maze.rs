use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

use anyhow::Result as Result;

type Map = Vec<Vec<u8>>;
type TilesVisited = Vec<(i32, i32)>;

#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Step {
    row: i32,
    col: i32,
    dy: i32,
    dx: i32,
}

impl Step {
    fn new(row: i32, col: i32, dy: i32, dx: i32) -> Self {
        Self { row, col, dy, dx }
    }

    fn start(row: i32, col: i32) -> Self {
        Self { row, col, dy: 0, dx: 1 }
    }

    fn end(row: i32, col: i32) -> Self {
        Self { row, col, dy: 0, dx: 0 }
    }

    fn new_directions(&self) -> [(i32, i32); 2] {
        if self.dy == 0 {      // going left or right
            [(-1, 0), (1, 0)]  // return up and down
        } else {               // going up or down
            [(0, 1), (0, -1)]  // return left and right
        }
    }

    fn found_end(&self, end: &Step) -> bool {
        self.col == end.col && self.row == end.row
    }
}

pub fn solve_day_16(file: &str) -> Result<(u32, usize)> {
    let data: Vec<u8> = fs::read(file)?;
    let (map, start, end) = create_map(&data);
    let (shortest_route, total_steps) = search_graph(&map, &start, &end);

    Ok((shortest_route, total_steps))
}

/// Create a nested vector of the map and store the start and end positions.
fn create_map(data: &[u8]) -> (Map, Step, Step) {
    let mut map: Map = Vec::new();
    let mut row: Vec<u8> = Vec::new();

    let mut start: Step = Step::default();
    let mut end: Step = Step::default();

    let mut x_val = 0;
    let mut y_val = 0;

    data
        .iter()
        .for_each(
            | char | {
                match char {
                    b'\n' => {
                        map.push(row.clone());
                        row = Vec::new();
                        x_val = 0; y_val += 1;
                    },
                    b'S' => {
                        start = Step::start(y_val, x_val);
                        row.push(b'.');
                        x_val += 1
                    }
                    b'E' => {
                        end = Step::end(y_val, x_val);
                        row.push(b'.');
                        x_val += 1
                    }
                    _ => {
                        row.push(*char);
                        x_val += 1;
                    }
                }
            }
        );

    (map, start, end)
}

/// Use Dijkstra to search the graph to find the shortest path.
/// TODO the tiles visited cloning stuff is extremely slow! Should be improved.
fn search_graph(map: &Map, start: &Step, end: &Step) -> (u32, usize) {
    let mut visited: HashMap<Step, (Reverse<u32>, TilesVisited)> = HashMap::new();
    let mut queue: BinaryHeap<(Reverse<u32>, Step, TilesVisited)> = BinaryHeap::from([(Reverse(0), *start, Vec::new())]);
    let mut best_score: u32 = u32::MAX;
    let mut total_tiles_visited: usize = 0;

    while let Some((current_cost, current_step, mut tiles_visited)) = queue.pop() {
        if current_cost.0 > best_score {
            continue
        }

        match visited.get(&current_step) {
            // Old route was better (because of Reverse we check > instead of <).
            Some((cost, _)) if *cost > current_cost => continue,

            // Found an identical route, so we have 2 paths to reach here, so combine the visited tiles.
            Some((cost, previous_steps)) if *cost == current_cost => {
                tiles_visited.append(&mut previous_steps.clone());
            },
            // New route is better or first time here.
            _ => {visited.insert(current_step, (current_cost, tiles_visited.clone()));},
        };

        tiles_visited.push((current_step.row, current_step.col));

        if current_step.found_end(end) && current_cost.0 <= best_score {
            best_score = current_cost.0;

            tiles_visited.sort_unstable();
            tiles_visited.dedup();
            total_tiles_visited = total_tiles_visited.max(tiles_visited.len());
            continue
        }

        // straight
        let new_row = current_step.row + current_step.dy;
        let new_col = current_step.col + current_step.dx;
        if let Some(slice) = map.get(new_row as usize) {
            if let Some(&new_position) = slice.get(new_col as usize) {
                if new_position == b'.' {
                    queue.push(
                        (
                            Reverse(current_cost.0 + 1),
                            Step::new(new_row, new_col, current_step.dy, current_step.dx),
                            tiles_visited.clone()
                        )
                    )
                }
            }
        }

        // Turn 90 degrees
        for turn in  current_step.new_directions() {
            let new_row = current_step.row + turn.0;
            let new_col = current_step.col + turn.1;
            if let Some(slice) = map.get(new_row as usize) {
                if let Some(&new_position) = slice.get(new_col as usize) {
                    if new_position == b'.' {
                        queue.push(
                            (
                                Reverse(current_cost.0 + 1001),
                                Step::new(new_row, new_col, turn.0, turn.1),
                                tiles_visited.clone()
                            )
                        )
                    }
                }
            }
        }
    }

    (best_score, total_tiles_visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &[u8] =
        b"###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############\n";

    const EXAMPLE2: &[u8] =
        b"#################\n\
        #...#...#...#..E#\n\
        #.#.#.#.#.#.#.#.#\n\
        #.#.#.#...#...#.#\n\
        #.#.#.#.###.#.#.#\n\
        #...#.#.#.....#.#\n\
        #.#.#.#.#.#####.#\n\
        #.#...#.#.#.....#\n\
        #.#.#####.#.###.#\n\
        #.#.#.......#...#\n\
        #.#.###.#####.###\n\
        #.#.#...#.....#.#\n\
        #.#.#.#####.###.#\n\
        #.#.#.........#.#\n\
        #.#.#.#########.#\n\
        #S#.............#\n\
        #################\n";

    #[test]
    fn test_example_one() {
        let (map, start, end) = create_map(EXAMPLE1);
        let (score, steps) = search_graph(&map, &start, &end);
        assert_eq!(score, 7036);
        assert_eq!(steps, 45);
    }

    #[test]
    fn test_example_two() {
        let (map, start, end) = create_map(EXAMPLE2);
        let (score, steps) = search_graph(&map, &start, &end);
        assert_eq!(score, 11048);
        assert_eq!(steps, 64);
    }
}
