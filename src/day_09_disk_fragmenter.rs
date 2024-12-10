use std::fs;

use anyhow::Result as Result;
use itertools::Itertools;

type Fragments = Vec<Option<usize>>;

#[derive(Copy, Clone, Debug)]
struct DataFragment {
    size: u8,
    id: Option<usize>
}

impl DataFragment {

    #[inline]
    fn new(size: u8, id: Option<usize>) -> Self {
        Self { size, id }
    }

    #[inline]
    fn is_none(&self) -> bool {
        self.id.is_none()
    }

    #[inline]
    fn is_some(&self) -> bool {
        self.id.is_some()
    }
}


pub fn solve_day_09(file: &str) -> Result<(usize, u32)> {
    let data: Vec<u8> = fs::read(file)?;

    let data: Vec<u8> = data[0..data.len()-1]
        .iter()
        .map(| byte | byte - b'0')
        .collect();


    let fragments = create_single_fragments(&data);
    let defragged_data = defragment_single_blocks(&fragments);
    let checksum = calculate_checksum(&defragged_data);

    Ok((checksum, 0))
}

fn create_single_fragments(data: &[u8]) -> Fragments {
    let mut fragments: Fragments = Vec::new();

    let mut is_value: bool = true;
    let mut id_value: usize = 0;

    for val in data.iter() {
        match is_value {
            true => {
                (0..*val).for_each(| _ | fragments.push(Some(id_value)));
                id_value += 1;
            },
            false => {
                (0..*val).for_each(| _ | fragments.push(None))
            }
        }
        is_value ^= true;
    }

    fragments
}

fn defragment_single_blocks(fragments: &Fragments) -> Vec<usize> {
    let mut new_fragments = fragments.clone();
    let mut last_idx: usize = fragments.len();

    for empty_idx in fragments.iter().positions(| val | val.is_none()) {
        let filled_idx = new_fragments[0..last_idx]
            .iter()
            .rposition(| val | val.is_some())
            .unwrap();

        if empty_idx > filled_idx {
            break  // reached the turning point where all None values are at the end.
        }

        new_fragments.swap(empty_idx, filled_idx);
        last_idx = filled_idx;
    }

    new_fragments
        .into_iter()
        .flatten()
        .collect()
}

fn create_fragment_blocks(data: &[u8]) -> Vec<DataFragment> {
    let mut fragments: Vec<DataFragment> = Vec::new();

    let mut is_value: bool = true;
    let mut id_value: usize = 0;

    for val in data.iter() {
        match is_value {
            true => {
                fragments.push(DataFragment::new(*val, Some(id_value)));
                id_value += 1;
            },
            false => fragments.push(DataFragment::new(*val, None))
        };
        is_value ^= true;
    }

    fragments
}

fn defragment_complete_blocks(fragments: Vec<DataFragment>) -> Vec<DataFragment> {
    let mut new_fragments= fragments.clone();
    let mut insert_indices: Vec<usize> = Vec::new();

    for filled_idx_raw in fragments.iter().positions(| val | val.is_some()).rev() {
        let filled_idx = filled_idx_raw + insert_indices.iter().filter(| val | **val > filled_idx_raw).count();

        for empty_idx in new_fragments.iter().positions(| val | val.is_none()) {
            if new_fragments[empty_idx].size >= new_fragments[filled_idx].size {

                new_fragments.insert(empty_idx, new_fragments[filled_idx]);
                new_fragments[empty_idx + 1].size -= new_fragments[empty_idx].size;
                new_fragments.remove(filled_idx + 1);
                insert_indices.push(empty_idx);
                break
            }
        }
    }

    // FIXME Blocks are in the right order,
    //   but there are empty blocks missing in the middle so the checksum will not be correct.
    new_fragments
}

fn calculate_checksum(data: &[usize]) -> usize {
    let mut checksum: usize = 0;

    data
        .iter()
        .enumerate()
        .for_each(| (i, &val) | checksum += i * val);

    checksum
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u8] = b"2333133121414131402";

    #[test]
    fn test_defragment_single_blocks() {
        let data: Vec<u8> = EXAMPLE.iter().map(| val | val - b'0').collect();
        let fragments = create_single_fragments(&data);
        let defragged = defragment_single_blocks(&fragments);
        assert_eq!(calculate_checksum(&defragged), 1928);
    }

    #[test]
    fn test_defragment_complete_blocks() {
        let data: Vec<u8> = EXAMPLE.iter().map(| val | val - b'0').collect();
        let fragments = create_fragment_blocks(&data);
        let defragged = defragment_complete_blocks(fragments);
        println!("{:?}", defragged);
    }
}
