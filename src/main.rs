use clap::Parser;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

#[derive(Parser)]
struct Args {
    command: String,
    input_path: String,
}

fn main() {
    let args = Args::parse();

    let content = std::fs::read_to_string(&args.input_path).expect("could not read file");

    match args.command.as_str() {
        "day-1-step-1" => day_1::step_1(&content),
        "day-1-step-2" => day_1::step_2(&content),
        "day-2-step-1" => day_2::step_1(&content),
        "day-2-step-2" => day_2::step_2(&content),
        "day-3-step-1" => day_3::step_1(&content),
        "day-3-step-2" => day_3::step_2(&content),
        "day-4-step-1" => day_4::step_1(&content),
        "day-4-step-2" => day_4::step_2(&content),
        _ => panic!("unknown command"),
    };
}
