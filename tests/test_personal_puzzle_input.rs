use std::path::Path;

use advent_of_code_2024::*;

#[test]
fn test_day_01() {
    let file = Path::new("./data/day_01_historian_hysteria.txt").to_str().unwrap();
    let value = day_01_historian_hysteria::solve_day_01(file).unwrap();

    assert_eq!(value.0, 1646452);
    assert_eq!(value.1, 23609874);
}

#[test]
fn test_day_02() {
    let file = Path::new("./data/day_02_red_nosed_reports.txt").to_str().unwrap();
    let value = day_02_red_nosed_reports::solve_day_02(file).unwrap();

    assert_eq!(value.0, 306);
    assert_eq!(value.1, 366);
}

#[test]
fn test_day_03() {
    let file = Path::new("./data/day_03_mull_it_over.txt").to_str().unwrap();
    let value = day_03_mull_it_over::solve_day_03(file).unwrap();

    assert_eq!(value.0, 183788984);
    assert_eq!(value.1, 62098619);
}

#[test]
fn test_day_04() {
    let file = Path::new("./data/day_04_ceres_search.txt").to_str().unwrap();
    let value = day_04_ceres_search::solve_day_04(file).unwrap();

    assert_eq!(value.0, 2378);
    assert_eq!(value.1, 1796);
}

#[test]
fn test_day_05() {
    let file = Path::new("./data/day_05_print_queue.txt").to_str().unwrap();
    let value = day_05_print_queue::solve_day_05(file).unwrap();

    assert_eq!(value.0, 4924);
    assert_eq!(value.1, 6085);
}

#[test]
fn test_day_06() {
    let file = Path::new("./data/day_06_guard_gallivant.txt").to_str().unwrap();
    let value = day_06_guard_gallivant::solve_day_06(file).unwrap();

    assert_eq!(value.0, 4967);
    assert_eq!(value.1, 1789);
}

#[test]
fn test_day_07() {
    let file = Path::new("./data/day_07_bridge_repair.txt").to_str().unwrap();
    let value = day_07_bridge_repair::solve_day_07(file).unwrap();

    assert_eq!(value.0, 932137732557);
    assert_eq!(value.1, 661823605105500);
}

#[test]
fn test_day_08() {
    let file = Path::new("./data/day_08_resonant_collinearity.txt").to_str().unwrap();
    let value = day_08_resonant_collinearity::solve_day_08(file).unwrap();

    assert_eq!(value.0, 289);
    assert_eq!(value.1, 1030);
}

#[test]
fn test_day_09() {
    let file = Path::new("./data/day_09_disk_fragmenter.txt").to_str().unwrap();
    let value = day_09_disk_fragmenter::solve_day_09(file).unwrap();

    assert_eq!(value.0, 6471961544878);
    assert_eq!(value.1, 0);
}

#[test]
fn test_day_10() {
    let file = Path::new("./data/day_10_hoof_it.txt").to_str().unwrap();
    let value = day_10_hoof_it::solve_day_10(file).unwrap();

    assert_eq!(value.0, 607);
    assert_eq!(value.1, 1384);
}

#[test]
fn test_day_11() {
    let file = Path::new("./data/day_11_plutonian_pebbles.txt").to_str().unwrap();
    let value = day_11_plutonian_pebbles::solve_day_11(file).unwrap();

    assert_eq!(value.0, 186424);
    assert_eq!(value.1, 219838428124832);
}

#[test]
fn test_day_12() {
    let file = Path::new("./data/day_12_garden_groups.txt").to_str().unwrap();
    let value = day_12_garden_groups::solve_day_12(file).unwrap();

    assert_eq!(value.0, 0);
    assert_eq!(value.1, 0);
}

#[test]
fn test_day_13() {
    let file = Path::new("./data/day_13_claw_contraption.txt").to_str().unwrap();
    let value = day_13_claw_contraption::solve_day_13(file).unwrap();

    assert_eq!(value.0, 27157);
    assert_eq!(value.1, 104015411578548);
}

#[test]
fn test_day_16() {
    let file = Path::new("./data/day_16_reindeer_maze.txt").to_str().unwrap();
    let value = day_16_reindeer_maze::solve_day_16(file).unwrap();

    assert_eq!(value.0, 103512);
    assert_eq!(value.1, 554);
}