use regex::Regex;

pub fn step_1(content: &str) {
    let impossible_pattern: Regex =
        Regex::new(r"13 red|14 (?:red|green)|(?:1[5-9]|[2-9]\d|\d{3}) (?:red|green|blue)")
            .expect("could not parse impossible game pattern");
    let game_number_pattern: Regex = Regex::new(r"\d+").expect("could not parse game number");

    let mut sum = 0;
    for line in content.lines() {
        if impossible_pattern.is_match(line) {
            continue;
        }

        let game_number = game_number_pattern
            .find(line)
            .expect("could not get game number")
            .as_str()
            .parse::<i32>()
            .expect("could not parse game number");

        sum += game_number;
    }

    println!("sum: {}", sum);
}
