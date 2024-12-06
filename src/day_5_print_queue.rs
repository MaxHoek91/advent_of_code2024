use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

use anyhow::Result as Result;

type OrderingMap = HashMap<u8, Vec<u8>>;

pub fn solve_day_5(file: &str) -> Result<(u32, u32)> {
    let data = fs::read_to_string(file)?;

    // Split the data into the part with ordering_rules and pages.
    let (ordering_rules, pages) = data.split_once("\n\n").unwrap();

    let ordering_map = create_ordering_map(ordering_rules);

    let mut ordered_page_sum: u32 = 0;
    let mut unordered_page_sum: u32 = 0;

    for line in pages.lines() {
        let mut page_set: Vec<u8> = line
            .split(',')
            .map(|val| val.parse().unwrap())
            .collect();

        match check_order(&page_set, &ordering_map) {
            // Ordered pages may be counted directly.
            Some(page_number) => ordered_page_sum += page_number as u32,
            // Unordered pages must be sorted and then counted for part two.
            None => unordered_page_sum += sort_pages(&mut page_set, &ordering_map) as u32
        }
    }

    Ok((ordered_page_sum, unordered_page_sum))
}

/// Create a map for a | b rules. Because a may occur multiple times create a | Vec<b's>.
fn create_ordering_map(lines: &str) -> OrderingMap {
    let mut ordering_map: OrderingMap = HashMap::new();
    lines
        .lines()
        .for_each(
            | line | {
                let (left, right) = match line.split_once('|') {
                    Some((l, r)) => (l.parse().unwrap(), r.parse().unwrap()),
                    None => unreachable!()
                };

                ordering_map
                    .entry(left)
                    .and_modify(| val | val.push(right))
                    .or_insert(vec![right]);
            }
        );

    ordering_map
}

/// Check if the pages are ordered according to the OrderingMap.
///
/// Iterate over the pages, and get the corresponding value from the OrderingMap.
/// Missing values can be ignored.
///
/// For each page check if any of the previous pages occur in the OrderingMap values.
/// If this is the case we have an invalid sorting.
///
/// Otherwise, return the middle page.
fn check_order(pages: &[u8], ordering_map: &OrderingMap) -> Option<u8> {
    for (i, page) in pages.iter().skip(1).enumerate() {
        let rules =  match ordering_map.get(page) {
            Some(r) => r,
            None => continue
        };

        if pages[..i + 1].iter().any(|val| rules.contains(val)) {
            return None;
        }
    }

    Some(pages[pages.len() / 2]) // return the value in the middle.
}

/// Sort the pages according to the OrderingMap using a custom sorter.
///
/// When we encounter "a | b" then a must come before b therefore return Ordering::Less
///
/// In all other cases we don't care, so we can return Ordering::Equal
///
/// Because of how the ordering map was constructed we cannot easily check for "b | a",
/// as a result we do not use Ordering::Greater
fn sort_pages(pages: &mut [u8], ordering_map: &OrderingMap) -> u8 {
    pages.sort_unstable_by(
        | a, b | {
            match ordering_map.get(a) {
                Some(vec) if vec.contains(b) => Ordering::Less,
                Some(_) => Ordering::Equal,
                None => Ordering::Equal
            }
        }
    );

    pages[pages.len() / 2] // return the value in the middle.
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAP: &str =
        "47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13";

    const EXAMPLE1: [u8; 5] = [75, 47, 61, 53, 29];
    const EXAMPLE2: [u8; 5] = [97, 61, 53, 29, 13];
    const EXAMPLE3: [u8; 3] = [75, 29, 13];
    const EXAMPLE4: [u8; 5] = [75, 97, 47, 61, 53];
    const EXAMPLE5: [u8; 3] = [61, 13, 29];
    const EXAMPLE6: [u8; 5] = [97, 13, 75, 29, 47];

    #[test]
    fn test_page_ordering() {
        let ordering_map = create_ordering_map(MAP);

        assert_eq!(check_order(&EXAMPLE1, &ordering_map), Some(61));
        assert_eq!(check_order(&EXAMPLE2, &ordering_map), Some(53));
        assert_eq!(check_order(&EXAMPLE3, &ordering_map), Some(29));
        assert_eq!(check_order(&EXAMPLE4, &ordering_map), None);
        assert_eq!(check_order(&EXAMPLE5, &ordering_map), None);
        assert_eq!(check_order(&EXAMPLE6, &ordering_map), None);
    }

    #[test]
    fn test_page_sorting() {
        let ordering_map = create_ordering_map(MAP);

        let mut ex = EXAMPLE4;
        assert_eq!(sort_pages(&mut ex, &ordering_map), 47);

        let mut ex = EXAMPLE5;
        assert_eq!(sort_pages(&mut ex, &ordering_map), 29);

        let mut ex = EXAMPLE6;
        assert_eq!(sort_pages(&mut ex, &ordering_map), 47);
    }
}
