use std::path::Path;
use std::time::Instant;

pub mod day_1_historian_hysteria;
pub mod day_2_red_nosed_reports;
pub mod day_3_mull_it_over;
pub mod day_4_ceres_search;

pub fn solve_all() {
    println!("Advent of Code 2024\n");

    day_1();
    day_2();
    day_3();
    day_4();
}

fn day_1() {
    let file = Path::new("./data/day_1_historian_hysteria.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_1_historian_hysteria::solve_day_one(file).unwrap();

    println!(
        "Day 1: Historian Hysteria\n\
        Run Time: {:?}\n\
        Sum of Distances 1: {}\n\
        Similarity Score 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_2() {
    let file = Path::new("./data/day_2_red_nosed_reports.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_2_red_nosed_reports::solve_day_two(file).unwrap();

    println!(
        "Day 2: Red-Nodes Reports\n\
        Run Time: {:?}\n\
        Safe Reports 1: {}\n\
        Safe Reports 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_3() {
    let file = Path::new("./data/day_3_mull_it_over.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_3_mull_it_over::solve_day_three(file).unwrap();

    println!(
        "Day 3: Mull It Over\n\
        Run Time: {:?}\n\
        Sum of instructions 1: {}\n\
        Sum of instructions 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}

fn day_4() {
    let file = Path::new("./data/day_4_ceres_search.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_4_ceres_search::solve_day_four(file).unwrap();

    println!(
        "Day 4: Ceres Search\n\
        Run Time: {:?}\n\
        XMAS Count 1: {}\n\
        X-MAS Count 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}