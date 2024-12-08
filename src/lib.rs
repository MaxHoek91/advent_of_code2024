use std::path::Path;
use std::time::Instant;

pub mod day_01_historian_hysteria;
pub mod day_02_red_nosed_reports;
pub mod day_03_mull_it_over;
pub mod day_04_ceres_search;
pub mod day_05_print_queue;
pub mod day_06_guard_gallivant;
pub mod day_07_bridge_repair;
pub mod day_08_resonant_collinearity;

pub fn solve_all() {
    println!("Advent of Code 2024\n");

    day_01();
    day_02();
    day_03();
    day_04();
    day_05();
    day_06();
    day_07();
    day_08();
}

fn day_01() {
    let file = Path::new("./data/day_01_historian_hysteria.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_01_historian_hysteria::solve_day_01(file).unwrap();

    println!(
        "Day 1: Historian Hysteria\n\
        Run Time: {:?}\n\
        Sum of Distances 1: {}\n\
        Similarity Score 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_02() {
    let file = Path::new("./data/day_02_red_nosed_reports.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_02_red_nosed_reports::solve_day_02(file).unwrap();

    println!(
        "Day 2: Red-Nodes Reports\n\
        Run Time: {:?}\n\
        Safe Reports 1: {}\n\
        Safe Reports 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_03() {
    let file = Path::new("./data/day_03_mull_it_over.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_03_mull_it_over::solve_day_03(file).unwrap();

    println!(
        "Day 3: Mull It Over\n\
        Run Time: {:?}\n\
        Sum of instructions 1: {}\n\
        Sum of instructions 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_04() {
    let file = Path::new("./data/day_04_ceres_search.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_04_ceres_search::solve_day_04(file).unwrap();

    println!(
        "Day 4: Ceres Search\n\
        Run Time: {:?}\n\
        XMAS Count 1: {}\n\
        X-MAS Count 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_05() {
    let file = Path::new("./data/day_05_print_queue.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_05_print_queue::solve_day_05(file).unwrap();

    println!(
        "Day 5: Print Queue\n\
        Run Time: {:?}\n\
        Ordered Pages Sum 1: {}\n\
        Unordered Pages Sum 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_06() {
    let file = Path::new("./data/day_06_guard_gallivant.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_06_guard_gallivant::solve_day_06(file).unwrap();

    println!(
        "Day 6: Guard Gallivant\n\
        Run Time: {:?}\n\
        Tiles Covered 1: {}\n\
        Objects To Cause Loops 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_07() {
    let file = Path::new("./data/day_07_bridge_repair.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_07_bridge_repair::solve_day_07(file).unwrap();

    println!(
        "Day 7: Bridge Repair\n\
        Run Time: {:?}\n\
        Total Calibration Result 1: {}\n\
        Total Calibration Result 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_08() {
    let file = Path::new("./data/day_08_resonant_collinearity.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_08_resonant_collinearity::solve_day_08(file).unwrap();

    println!(
        "Day 8: Resonant Collinearity\n\
        Run Time: {:?}\n\
        # Antinodes 1: {}\n\
        # Antinodes 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}