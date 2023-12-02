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

pub fn step_2(content: &str) {
    let pattern: Regex =
        Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").expect("could not parse pattern");

    let mut sum = 0;
    for line in content.lines() {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for capture in pattern.captures_iter(line) {
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
