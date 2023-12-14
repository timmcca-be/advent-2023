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
mod day_8;
mod day_9;

mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;

mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

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
        "day-8-step-1" => day_8::step_1(lines),
        "day-8-step-2" => day_8::step_2(lines),
        "day-9-step-1" => day_9::step_1(lines),
        "day-9-step-2" => day_9::step_2(lines),
        "day-10-step-1" => day_10::step_1(lines),
        "day-10-step-2" => day_10::step_2(lines),
        "day-11-step-1" => day_11::step_1(lines),
        "day-11-step-2" => day_11::step_2(lines),
        "day-12-step-1" => day_12::step_1(lines),
        "day-12-step-2" => day_12::step_2(lines),
        "day-13-step-1" => day_13::step_1(lines),
        "day-13-step-2" => day_13::step_2(lines),
        "day-14-step-1" => day_14::step_1(lines),
        "day-14-step-2" => day_14::step_2(lines),
        "day-15-step-1" => day_15::step_1(lines),
        "day-15-step-2" => day_15::step_2(lines),
        "day-16-step-1" => day_16::step_1(lines),
        "day-16-step-2" => day_16::step_2(lines),
        "day-17-step-1" => day_17::step_1(lines),
        "day-17-step-2" => day_17::step_2(lines),
        "day-18-step-1" => day_18::step_1(lines),
        "day-18-step-2" => day_18::step_2(lines),
        "day-19-step-1" => day_19::step_1(lines),
        "day-19-step-2" => day_19::step_2(lines),
        "day-20-step-1" => day_20::step_1(lines),
        "day-20-step-2" => day_20::step_2(lines),
        "day-21-step-1" => day_21::step_1(lines),
        "day-21-step-2" => day_21::step_2(lines),
        "day-22-step-1" => day_22::step_1(lines),
        "day-22-step-2" => day_22::step_2(lines),
        "day-23-step-1" => day_23::step_1(lines),
        "day-23-step-2" => day_23::step_2(lines),
        "day-24-step-1" => day_24::step_1(lines),
        "day-24-step-2" => day_24::step_2(lines),
        "day-25-step-1" => day_25::step_1(lines),
        "day-25-step-2" => day_25::step_2(lines),
        _ => panic!("unknown command"),
    };
}
