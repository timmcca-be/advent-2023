use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

#[derive(Parser)]
struct Args {
    command: String,
    input_path: String,
}

fn main() {
    let args = Args::parse();

    let file = File::open(args.input_path).unwrap();
    let lines = io::BufReader::new(file).lines().map(|line| line.unwrap());

    match args.command.as_str() {
        "day-1-step-1" => day_1::step_1(lines),
        "day-1-step-2" => day_1::step_2(lines),
        "day-2-step-1" => day_2::step_1(lines),
        "day-2-step-2" => day_2::step_2(lines),
        "day-3-step-1" => day_3::step_1(lines),
        "day-3-step-2" => day_3::step_2(lines),
        "day-4-step-1" => day_4::step_1(lines),
        "day-4-step-2" => day_4::step_2(lines),
        "day-5-step-1" => day_5::step_1(lines),
        "day-5-step-2" => day_5::step_2(lines),
        "day-6-step-1" => day_6::step_1(lines),
        "day-6-step-2" => day_6::step_2(lines),
        "day-7-step-1" => day_7::step_1(lines),
        "day-7-step-2" => day_7::step_2(lines),
        _ => panic!("unknown command"),
    };
}
