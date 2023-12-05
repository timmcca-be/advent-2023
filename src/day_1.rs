use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref DIGITS_PATTERN: Regex = Regex::new(r"(\d).*?(\d)?\D*$").unwrap();
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;
    for line in lines {
        let captures = DIGITS_PATTERN
            .captures(&line)
            .expect("could not get digits");
        let first_digit_str = captures.get(1).expect("could not get first digit").as_str();
        let first_digit = first_digit_str
            .parse::<i32>()
            .expect("could not parse first digit");

        let last_digit = match captures.get(2) {
            Some(last_digit) => last_digit
                .as_str()
                .parse::<i32>()
                .expect("could not parse last digit"),
            None => first_digit,
        };

        sum += first_digit * 10 + last_digit;
    }

    println!("sum: {}", sum);
}

const DIGIT_PATTERN_STRING: &str = r"\d|zero|one|two|three|four|five|six|seven|eight|nine";

lazy_static! {
    static ref FIRST_DIGIT_PATTERN: Regex = Regex::new(DIGIT_PATTERN_STRING).unwrap();
    static ref LAST_DIGIT_PATTERN: Regex =
        Regex::new(&(r".*(?<digit>".to_owned() + &DIGIT_PATTERN_STRING + r")")).unwrap();
}

fn parse_digit(text: &str) -> Option<i32> {
    match text.parse::<i32>() {
        Ok(digit) => return Some(digit),
        Err(_) => {}
    }

    return match text {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    };
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut sum = 0;
    for line in lines {
        let first_digit_str = FIRST_DIGIT_PATTERN
            .find(&line)
            .expect("could not get first digit")
            .as_str();

        let last_digit_str = LAST_DIGIT_PATTERN
            .captures(&line)
            .expect("could not get last digit")
            .name("digit")
            .expect("could not get last digit")
            .as_str();

        let first_digit = parse_digit(&first_digit_str).expect("could not parse first digit");
        let last_digit = parse_digit(&last_digit_str).expect("could not parse last digit");

        sum += first_digit * 10 + last_digit;
    }

    println!("sum: {}", sum);
}
