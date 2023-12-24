use std::fmt::Display;
use std::time::Instant;

use regex::Regex;

use aoc_2023::get_input;

fn get_sum_of_parts(input: &[String]) -> u32 {
    let symbol_re = Regex::new(r"([^0-9.])").expect("Invalid regex");
    let mut sum: u32 = 0;

    for i in 0..input.len() {
        let empty_line = (0..input[0].len()).map(|_| ".").collect::<String>();
        let previous = if i == 0 { &empty_line } else { &input[i-1] };
        let current = &input[i];
        let next = if i == input.len() - 1 { &empty_line } else { &input[i+1] };

        let mut j = 0;
        while j < current.len() {
            if current.chars().nth(j).unwrap().is_numeric() {
                let mut k = j;

                while k < current.len() - 1 && current.chars().nth(k+1).unwrap().is_numeric() {
                    k += 1;
                }

                let number = current[j..k+1].parse::<u32>().unwrap();

                let left = if j == 0 { 0 } else { j - 1 };
                let right = if k == current.len() - 1 { k + 1 } else { k + 2 };

                if  !symbol_re.captures_iter(&previous[left..right]).collect::<Vec<_>>().is_empty() ||
                    !symbol_re.captures_iter(&current[left..right]).collect::<Vec<_>>().is_empty() ||
                    !symbol_re.captures_iter(&next[left..right]).collect::<Vec<_>>().is_empty()
                {
                    sum += number;
                }

                j = k;
            }
            j += 1;
        }
    }

    sum
}

struct Number {
    value: u32,
    start_index: u32,
    length: u32,
}

fn find_number_on_line(line: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut j = 0;

    while j < line.len() {
        if line.chars().nth(j).unwrap().is_numeric() {
            let mut k = j;

            while k < line.len() - 1 && line.chars().nth(k+1).unwrap().is_numeric() {
                k += 1;
            }

            numbers.push(Number {
                value: line[j..k+1].parse::<u32>().unwrap(),
                start_index: j.try_into().unwrap(),
                length: (k+1 - j).try_into().unwrap(),
            });

            j = k;
        }
        j += 1;
    }

    numbers
}

fn get_sum_of_gear_ratios(input: &[String]) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..input.len() {
        let empty_line = (0..input[0].len()).map(|_| ".").collect::<String>();
        let previous = if i == 0 { &empty_line } else { &input[i-1] };
        let current = &input[i];
        let next = if i == input.len() - 1 { &empty_line } else { &input[i+1] };

        for j in 0..input[i].len() {
            if current.chars().nth(j).unwrap() == '*' {
                let mut numbers: Vec<Number> = vec![];
                let index: u32 = j.try_into().unwrap();

                numbers.append(&mut find_number_on_line(previous));
                numbers.append(&mut find_number_on_line(current));
                numbers.append(&mut find_number_on_line(next));

                let valid_numbers: Vec<_> = numbers.iter().filter(|x| x.start_index + x.length - 1 >= index - 1 && x.start_index <= index + 1).collect();
                if valid_numbers.len() == 2 {
                    sum += valid_numbers[0].value * valid_numbers[1].value;
                }
            }
        }
    }

    sum
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = get_sum_of_parts(input);
    let p2 = get_sum_of_gear_ratios(input);

    (p1, p2)
}

fn main() {
    let input = get_input("day03.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_sum_of_parts;
    use crate::get_sum_of_gear_ratios;

    #[test]
    fn test_p1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let res = get_sum_of_parts(&input.lines().map(|l| l.to_string()).collect::<Vec<String>>());

        assert_eq!(4361, res);
    }

    #[test]
    fn test_p2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

        let res = get_sum_of_gear_ratios(&input.lines().map(|l| l.to_string()).collect::<Vec<String>>());

        assert_eq!(467835, res);
    }
}
