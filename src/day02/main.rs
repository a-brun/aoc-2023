use std::fmt::Display;
use std::time::Instant;

use std::collections::HashMap;

use aoc_2023::get_input_as_string;

fn get_game_code(input: &str) -> u32 {
    let available_cubes = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let games: Vec<_> = input.split("\n").filter(|a| !a.is_empty()).collect();
    let mut code: u32 = 0;

    for game in games.iter() {
        let parts: Vec<&str> = game.split(":").collect();
        let game_id: u32 = parts[0][5..].parse::<u32>().unwrap_or(0);
        let mut is_valid: bool = true;

        for draw_group in parts[1].split(";") {
            for draw in draw_group.split(",") {
                let draw_elements: Vec<_> = draw.split(" ").collect();
                let amount: u32 = draw_elements[1].parse::<u32>().unwrap();
                let color: &str = draw_elements[2];

                if available_cubes.get(color).unwrap() < &amount {
                    is_valid = false;
                }
            }
        }

        if is_valid {
            code += game_id;
        }
    }

    code
}

fn get_minimal_set_coefficient(input: &str) -> u32 {
    let games: Vec<_> = input.split("\n").filter(|a| !a.is_empty()).collect();
    let mut code: u32 = 0;

    for game in games.iter() {
        let mut minimal_sets = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        let parts: Vec<_> = game.split(":").collect();

        for draw_group in parts[1].split(";") {
            for draw in draw_group.split(",") {
                let draw_elements: Vec<_> = draw.split(" ").collect();
                let amount: u32 = draw_elements[1].parse::<u32>().unwrap();
                let color: &str = draw_elements[2];

                if minimal_sets.get(color).unwrap() < &amount {
                    minimal_sets.insert(color, amount);
                }
            }
        }

        code += minimal_sets.get("red").unwrap() * minimal_sets.get("green").unwrap() * minimal_sets.get("blue").unwrap(); 
    }

    code
}

fn solve(input: &str) -> (impl Display, impl Display) {
    let p1 = get_game_code(input);
    let p2 = get_minimal_set_coefficient(input);

    (p1, p2)
}

fn main() {
    let input = get_input_as_string("day02.txt");

    let start = Instant::now();

    let (r1, r2) = solve(&input);

    let t = start.elapsed().as_nanos() as f64 / 1000.0;

    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
    println!("Duration: {:.3}μs", t);
}

#[cfg(test)]
mod tests {
    use crate::get_game_code;
    use crate::get_minimal_set_coefficient;

    #[test]
    fn test_p1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = get_game_code(&input);

        assert_eq!(8, res);
    }

    #[test]
    fn test_p2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let res = get_minimal_set_coefficient(&input);

        assert_eq!(2286, res);
    }

}

