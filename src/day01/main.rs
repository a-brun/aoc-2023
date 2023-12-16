use std::fmt::Display;
use std::time::Instant;

use aoc_2023::get_input_as_string;

fn get_total_calibration_value(input: &str) -> u32 {
    let mut total: u32 = 0;
    let calibrations: Vec<_> = input.split("\n").collect::<Vec<_>>();

    for calibration in calibrations.iter() {
        let digits: Vec<_> = calibration.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();

        let first: &u32 = digits.first().unwrap_or(&0);
        let last: &u32 = digits.last().unwrap_or(&0);

        total += first * 10 + last
    }

    total
}

fn get_total_calibration_value_with_letters(input: &str) -> u32 {
    let mut total: u32 = 0;
    let calibrations: Vec<_> = input.split("\n").collect::<Vec<_>>();

    for calibration in calibrations.iter() {
        let mut digits: Vec<u32> = vec![];
        let length = calibration.len();
        for i in 0..length {
            if calibration[i..length].starts_with("one") {
                digits.push(1);
            } else if calibration[i..length].starts_with("two") {
                digits.push(2);
            } else if calibration[i..length].starts_with("three") {
                digits.push(3);
            } else if calibration[i..length].starts_with("four") {
                digits.push(4);
            } else if calibration[i..length].starts_with("five") {
                digits.push(5);
            } else if calibration[i..length].starts_with("six") {
                digits.push(6);
            } else if calibration[i..length].starts_with("seven") {
                digits.push(7);
            } else if calibration[i..length].starts_with("eight") {
                digits.push(8);
            } else if calibration[i..length].starts_with("nine") {
                digits.push(9);
            } else {
                let c: char = calibration.chars().nth(i).unwrap();
                if c.is_numeric() {
                    digits.push(c.to_digit(10).unwrap_or(0))
                }
            }
        }


        let first: &u32 = digits.first().unwrap_or(&0);
        let last: &u32 = digits.last().unwrap_or(&0);

        total += first * 10 + last
    }

    total
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_total_calibration_value(input);
    let p2 = get_total_calibration_value_with_letters(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day01.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_total_calibration_value;
    use crate::get_total_calibration_value_with_letters;

    #[test]
    fn test_p1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let res = get_total_calibration_value(&input);

        assert_eq!(142, res);
    }

    #[test]
    fn test_p2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let res = get_total_calibration_value_with_letters(&input);

        assert_eq!(281, res);
    }

}

