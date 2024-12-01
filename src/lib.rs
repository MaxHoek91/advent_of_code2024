use std::path::Path;
use std::time::Instant;

pub mod day_1_historian_hysteria;

pub fn solve_all() {
    println!("Advent of Code 2024\n");

    day_1();
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
