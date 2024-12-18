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

                "day13" => {
                    day13::solve(data);
                }

                "day14" => {
                    day14::solve(data);
                }

                "day15" => {
                    day15::solve(data);
                }

                "day16" => {
                    day16::solve(data);
                }

                "day17" => {
                    day17::solve(data);
                }

                "day18" => {
                    day18::solve(data);
                }

                "day19" => {
                    day19::solve(data);
                }

                "day20" => {
                    day20::solve(data);
                }

                "day21" => {
                    day21::solve(data);
                }

                "day22" => {
                    day22::solve(data);
                }

                "day23" => {
                    day23::solve(data);
                }

                "day24" => {
                    day24::solve(data);
                }

                "day25" => {
                    day25::solve(data);
                }

                _ => {
                    panic!("Invalid day");
                }
            }
        }
    }
}
