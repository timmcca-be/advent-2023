use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_PATTERN: Regex = Regex::new(r"\d+").unwrap();
}

fn get_num_valid_integer_hold_times(time: i64, distance: i64) -> i64 {
    // h = minimum hold time
    // t = race time
    // d = target distance

    // h * (t - h) = d
    // -h^2 + th - d = 0
    // h^2 - th + d = 0
    // h = (t - sqrt(t^2 - 4d)) / 2
    let time_float = time as f64;
    let distance_float = distance as f64;
    let minimum_hold_time =
        (time_float - (time_float * time_float - 4.0 * distance_float).sqrt()) / 2.0;

    // the max hold time will always be time - minimum_hold_time, since it's a parabola centered at time / 2.
    return time - 1 - (minimum_hold_time.floor() as i64) * 2;
}

pub fn step_1(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();
    let times_string = lines_iterator.next().unwrap();
    let distances_string = lines_iterator.next().unwrap();

    let times = NUMBER_PATTERN
        .find_iter(&times_string)
        .map(|value| value.as_str().parse::<i64>().unwrap());
    let distances = NUMBER_PATTERN
        .find_iter(&distances_string)
        .map(|value| value.as_str().parse::<i64>().unwrap());

    let mut product = 1;
    for (time, distance) in times.zip(distances) {
        product *= get_num_valid_integer_hold_times(time, distance);
    }

    println!("product: {}", product);
}

pub fn step_2(lines: impl IntoIterator<Item = String>) {
    let mut lines_iterator = lines.into_iter();
    let time_string = lines_iterator.next().unwrap();
    let distance_string = lines_iterator.next().unwrap();

    let time = NUMBER_PATTERN
        .find_iter(&time_string)
        .map(|value| value.as_str())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = NUMBER_PATTERN
        .find_iter(&distance_string)
        .map(|value| value.as_str())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    println!(
        "result: {}",
        get_num_valid_integer_hold_times(time, distance)
    );
}
