use std::path::Path;
use std::time::Instant;

pub mod day_1_historian_hysteria;
pub mod day_2_red_nosed_reports;

pub fn solve_all() {
    println!("Advent of Code 2024\n");

    day_1();
    day_2();
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
    let file = Path::new("./data/day2_red_nosed_reports.txt").to_str().unwrap();
    let timer = Instant::now();
    let value = day_2_red_nosed_reports::solve_day_2(file).unwrap();

    println!(
        "Day 1: Red-Nodes Reports\n\
        Run Time: {:?}\n\
        Safe Reports 1: {}\n\
        Safe Reports 2: {}\n",
        timer.elapsed(), value.0, value.1
    );
}
