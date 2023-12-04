use regex::Regex;

fn parse_digit(text: &str) -> Option<i32> {
    return match text {
        "zero" => Some(0),
        "0" => Some(0),
        "one" => Some(1),
        "1" => Some(1),
        "two" => Some(2),
        "2" => Some(2),
        "three" => Some(3),
        "3" => Some(3),
        "four" => Some(4),
        "4" => Some(4),
        "five" => Some(5),
        "5" => Some(5),
        "six" => Some(6),
        "6" => Some(6),
        "seven" => Some(7),
        "7" => Some(7),
        "eight" => Some(8),
        "8" => Some(8),
        "nine" => Some(9),
        "9" => Some(9),
        _ => None,
    };
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let digits_pattern: Regex =
        Regex::new(r"(\d).*?(\d)?\D*$").expect("could not parse digit pattern");

    let mut sum = 0;
    for line in lines {
        let captures = digits_pattern
            .captures(&line)
            .expect("could not get digits");
        let first_digit_str = captures.get(1).expect("could not get first digit").as_str();
        let first_digit = parse_digit(&first_digit_str).expect("could not parse first digit");

        let last_digit = match captures.get(2) {
            Some(last_digit) => {
                parse_digit(&last_digit.as_str()).expect("could not parse last digit")
            }
            None => first_digit,
        };

        sum += first_digit * 10 + last_digit;
    }

    println!("sum: {}", sum);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let digits_string = r"\d|zero|one|two|three|four|five|six|seven|eight|nine";

    let first_digit_pattern: Regex =
        Regex::new(digits_string).expect("could not parse first digit pattern");
    let last_digit_pattern: Regex =
        Regex::new(&(r".*(?<digit>".to_owned() + &digits_string + r")"))
            .expect("could not parse last digit pattern");

    let mut sum = 0;
    for line in lines {
        let first_digit_str = first_digit_pattern
            .find(&line)
            .expect("could not get first digit")
            .as_str();

        let last_digit_captures = last_digit_pattern
            .captures(&line)
            .expect("could not get last digit");
        let last_digit_str = last_digit_captures
            .name("digit")
            .expect("could not get last digit")
            .as_str();

        let first_digit = parse_digit(&first_digit_str).expect("could not parse first digit");
        let last_digit = parse_digit(&last_digit_str).expect("could not parse last digit");

        sum += first_digit * 10 + last_digit;
    }

    println!("sum: {}", sum);
}
