mod puzzles;

use crate::puzzles::*;
use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            panic!("No arguments provided");
        }
        _ => {
            let day = args[1].as_str();
            let data = fs::read_to_string(format!("inputs/{}.txt", day)).unwrap();

            match day {
                "day01" => {
                    day01::solve(data);
                }

                "day02" => {
                    day02::solve(data);
                }

                "day03" => {
                    day03::solve(data);
                }

                "day04" => {
                    day04::solve(data);
                }

                "day05" => {
                    day05::solve(data);
                }

                "day06" => {
                    day06::solve(data);
                }

                "day07" => {
                    day07::solve(data);
                }

                "day08" => {
                    day08::solve(data);
                }

                "day09" => {
                    day09::solve(data);
                }

                "day10" => {
                    day10::solve(data);
                }

                "day11" => {
                    day11::solve(data);
                }

                "day12" => {
                    day12::solve(data);
                }

                _ => {
                    panic!("Invalid day");
                }
            }
        }
    }
}
