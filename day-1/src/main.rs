use clap::Parser;
use regex::Regex;

#[derive(Parser)]
struct Args {
    input_path: String,
}

fn main() {
    let args = Args::parse();

    let content = std::fs::read_to_string(&args.input_path)
        .expect("could not read file");

    let digit_pattern: Regex = Regex::new(r"^.*?(\d).*?(\d)?\D*$")
        .expect("could not parse digit pattern");

    let mut sum = 0;
    for line in content.lines() {
        let captures = digit_pattern.captures(line).expect("no digits in line");
        let first_digit = captures.get(1).expect("could not get first digit")
            .as_str().parse::<i32>().expect("could not parse first digit");
        let last_digit = match captures.get(2) {
            Some(m) => m.as_str().parse::<i32>().expect("could not parse last digit"),
            None => first_digit
        };

        sum += first_digit * 10 + last_digit;
    }

    println!("sum: {}", sum);
}
