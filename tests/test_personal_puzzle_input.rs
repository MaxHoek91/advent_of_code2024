use std::path::Path;

use advent_of_code_2024::*;

#[test]
fn test_day_1() {
    let file = Path::new("./data/day_1_historian_hysteria.txt").to_str().unwrap();
    let value = day_1_historian_hysteria::solve_day_one(file).unwrap();
    
    assert_eq!(value.0, 1646452);
    assert_eq!(value.1, 23609874);
}

#[test]
fn test_day_2() {
    let file = Path::new("./data/day2_red_nosed_reports.txt").to_str().unwrap();
    let value = day_2_red_nosed_reports::solve_day_2(file).unwrap();
    
    assert_eq!(value.0, 306);
    assert_eq!(value.1, 366);
}