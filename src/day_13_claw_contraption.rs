use std::fs;

use anyhow::Result as Result;

const TEXT_PATTERN: [[&str; 2]; 3] = [
    ["Button A: X+", ", Y+"],
    ["Button B: X+", ", Y+"],
    ["Prize: X=", ", Y="],
];

pub fn solve_day_13(file: &str) -> Result<(i64, i64)> {
    let data = fs::read_to_string(file)?;

    let mut total_presses_one = 0;
    let mut total_presses_two = 0;

    data
        .split("\n\n")
        .for_each(
            |machine| {
                let machine_values = parse_machine(machine);

                total_presses_one += calculate_presses_for_machine(&machine_values, 0.);
                total_presses_two += calculate_presses_for_machine(&machine_values, 1e13);
            }
        );

    Ok((total_presses_one, total_presses_two))
}

fn parse_machine(machine: &str) -> Vec<(f64, f64)> {
    machine
        .lines()
        .zip(TEXT_PATTERN)
        .map(
            | (line, pat) | {
                let (x,y ) = line
                    .trim_start_matches(pat[0])
                    .split_once(pat[1])
                    .unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            }
        )
        .collect()
}

fn calculate_presses_for_machine(input: &[(f64, f64)], offset: f64) -> i64 {
    let a = input[0];
    let b = input[1];
    let prize = (input[2].0 + offset, input[2].1 + offset);

    let presses_a = (prize.0 * b.1 - b.0 * prize.1) / (b.1 * a.0 - a.1 * b.0);
    let presses_b = (prize.1 * a.0 - prize.0 * a.1) / (a.0 * b.1 - b.0 * a.1);

    if presses_a.fract() > 0.0 || presses_b.fract() > 0.0 {
        return 0;
    }

    3 * (presses_a as i64) + presses_b as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n";

    #[test]
    fn test_example_input() {
        let total_presses: i64 = EXAMPLE
            .split("\n\n")
            .map(
                |machine| {
                    let machine_values = parse_machine(machine);
                    calculate_presses_for_machine(&machine_values, 0.)
                }
            )
            .sum();

        assert_eq!(total_presses, 480);
    }
}
