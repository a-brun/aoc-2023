use std::fmt::Display;
use std::time::Instant;

use std::collections::HashMap;

use aoc_2023::get_input;

fn get_sum_game_card_points(input: &[String]) -> u32 {
    input.iter().map(|line| {
        let parts = line.split(": ").collect::<Vec<_>>()[1].split(" | ").collect::<Vec<_>>();
        let winning_numbers = get_numbers_from_line(parts[0]);
        let card_numbers = get_numbers_from_line(parts[1]);
        let mut points: u32 = 0;

        for n in card_numbers.iter() {
            if winning_numbers.contains(&n) {
                points = if points == 0 { 1 } else { points * 2 };
            }
        }

        points
    }).sum()
}

fn get_count_of_scratch_cards(input: &[String]) -> u32 {
    let mut cards_count = HashMap::<u32, u32>::from([(1, 1)]);

    for line in input.iter() {
        let line_parts = line.split(": ").collect::<Vec<_>>();
        let card_id = line_parts[0].strip_prefix("Card ").unwrap().parse::<u32>().unwrap();
        let parts = line_parts[1].split(" | ").collect::<Vec<_>>();
        let winning_numbers = get_numbers_from_line(parts[0]);
        let card_numbers = get_numbers_from_line(parts[1]);

        for n in (card_id + 1)..(card_id + <usize as TryInto<u32>>::try_into(card_numbers.iter().filter(|x| winning_numbers.contains(&x)).collect::<Vec<_>>().len()).unwrap()) {
            let (_, current_card_count): (_, &u32) = cards_count.get_key_value(&card_id).unwrap();
            cards_count.entry(n).and_modify(|n| *n += current_card_count).or_insert(1);
        }

    }
    3
}

fn get_numbers_from_line(line: &str) -> Vec<u32> {
    line.split(" ").filter(|a| !a.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect()
}

fn solve(input: &[String]) -> (impl Display, impl Display) {
    let p1 = get_sum_game_card_points(input);

    (p1, 0)
}

fn main() {
    let input = get_input("day04.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_sum_game_card_points;
    use crate::get_count_of_scratch_cards;

    #[test]
    fn test_p1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = get_sum_game_card_points(&input.lines().map(|l| l.to_string()).collect::<Vec<String>>());

        assert_eq!(13, res);
    }

    #[test]
    fn test_p2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let res = get_count_of_scratch_cards(&input.lines().map(|l| l.to_string()).collect::<Vec<String>>());

        assert_eq!(45000, res);
    }
}
