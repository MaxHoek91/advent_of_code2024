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
    let file = Path::new("./data/day_2_red_nosed_reports.txt").to_str().unwrap();
    let value = day_2_red_nosed_reports::solve_day_two(file).unwrap();
    
    assert_eq!(value.0, 306);
    assert_eq!(value.1, 366);
}

#[test]
fn test_day_3() {
    let file = Path::new("./data/day_3_mull_it_over.txt").to_str().unwrap();
    let value = day_3_mull_it_over::solve_day_three(file).unwrap();
    
    assert_eq!(value.0, 183788984);
    assert_eq!(value.1, 62098619);
}

#[test]
fn test_day_4() {
    let file = Path::new("./data/day_4_ceres_search.txt").to_str().unwrap();
    let value = day_4_ceres_search::solve_day_four(file).unwrap();

    assert_eq!(value.0, 2378);
    assert_eq!(value.1, 1796);
}

#[test]
fn test_day_5() {
    let file = Path::new("./data/day_5_print_queue.txt").to_str().unwrap();
    let value = day_5_print_queue::solve_day_5(file).unwrap();

    assert_eq!(value.0, 4924);
    assert_eq!(value.1, 6085);
}

#[test]
fn test_day_6() {
    let file = Path::new("./data/day_6_guard_gallivant.txt").to_str().unwrap();
    let value = day_6_guard_gallivant::solve_day_6(file).unwrap();

    assert_eq!(value.0, 4967);
    assert_eq!(value.1, 1789);
}

#[test]
fn test_day_7() {
    let file = Path::new("./data/day_7_bridge_repair.txt").to_str().unwrap();
    let value = day_7_bridge_repair::solve_day_7(file).unwrap();

    assert_eq!(value.0, 932137732557);
    assert_eq!(value.1, 661823605105500);
}