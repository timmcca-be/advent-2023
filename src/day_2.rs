use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref IMPOSSIBLE_GAME_PATTERN: Regex =
        Regex::new(r"13 red|14 (?:red|green)|(?:1[5-9]|[2-9]\d|\d{3}) (?:red|green|blue)").unwrap();
    static ref GAME_NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;
    for line in lines {
        if IMPOSSIBLE_GAME_PATTERN.is_match(&line) {
            continue;
        }

        let game_number = GAME_NUMBER_PATTERN
            .find(&line)
            .expect("could not get game number")
            .as_str()
            .parse::<i32>()
            .expect("could not parse game number");

        sum += game_number;
    }

    println!("sum: {}", sum);
}

lazy_static! {
    static ref COLOR_COUNT_PATTERN: Regex =
        Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").unwrap();
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;
    for line in lines {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for capture in COLOR_COUNT_PATTERN.captures_iter(&line) {
            let count = capture
                .name("count")
                .expect("could not get number")
                .as_str()
                .parse::<i32>()
                .expect("could not parse number");
            let color = capture.name("color").expect("could not get color").as_str();

            match color {
                "red" => red = std::cmp::max(red, count),
                "green" => green = std::cmp::max(green, count),
                "blue" => blue = std::cmp::max(blue, count),
                _ => panic!("unknown color"),
            };
        }

        sum += red * green * blue;
    }

    println!("sum: {}", sum);
}
