use aoc::day::Challenge;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
    aoc::logger::init();

    day9::play();

    day1::Day1::run().unwrap();
    day2::Day2::run().unwrap();
    day3::Day3::run().unwrap();
    day4::Day4::run().unwrap();
    day5::Day5::run().unwrap();
    day6::Day6::run().unwrap();
    day7::Day7::run().unwrap();
    day8::Day8::run().unwrap();
    day9::Day9::run().unwrap();
    day10::Day10::run().unwrap();
    day11::Day11::run().unwrap();
}
