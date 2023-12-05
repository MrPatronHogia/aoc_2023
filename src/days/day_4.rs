use crate::{
    part_kind::PartKind,
    prelude::{Error, Result},
};
use std::collections::HashMap;

pub fn run(input: &str, parts: &[PartKind]) {
    for part in parts {
        match part {
            PartKind::Part1 => {
                let result = part1(input);
                match result {
                    Ok(result) => println!("Result for part 1 is {}", result),
                    Err(err) => println!("Error: {}", err),
                }
            }
            PartKind::Part2 => {
                let result = part2(input);
                match result {
                    Ok(result) => println!("Result for part 2 is {}", result),
                    Err(err) => println!("Error: {}", err),
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<u32> {
    let map_games_result = get_games(input);
    let mut score = 0;
    map_games_result
        .iter()
        .for_each(|game_result| match game_result {
            Ok(game) => score += game.play(),
            Err(_) => {
                println!("Error");
            }
        });

    Ok(score)
}

fn get_games(input: &str) -> Vec<Result<Game>> {
    let map_games_result: Vec<Result<Game>> = input
        .lines()
        .map(|line| {
            let game_split: Vec<&str> = line.split(":").collect();
            let game_id = get_game_id(&game_split)?;

            let game = get_game(game_split, game_id)?;

            Ok(game)
        })
        .collect();
    map_games_result
}

fn get_game(game_split: Vec<&str>, game_id: u32) -> Result<Game> {
    match get_winning_numbers_and_numbers(&game_split) {
        Ok(tuple) => Ok(Game {
            id: game_id,
            winning_numbers: tuple.0,
            numbers: tuple.1,
        }),
        Err(_) => Err(Error::Generic("Not implemented".to_string())),
    }
}

fn get_game_id(game_split: &Vec<&str>) -> Result<u32> {
    Ok(match game_split.first() {
        Some(game_id) => {
            let game_parts: Vec<&str> = game_id.trim().split_ascii_whitespace().collect();
            match game_parts.last() {
                Some(game_id) => match game_id.parse::<u32>() {
                    Ok(game_id) => game_id,
                    Err(_) => {
                        return Err(Error::Generic("Could not parse to game id".to_string()));
                    }
                },
                None => return Err(Error::Generic("Could not parse game".to_string())),
            }
        }
        None => return Err(Error::Generic("Could not parse game".to_string())),
    })
}

fn get_winning_numbers_and_numbers(game_split: &Vec<&str>) -> Result<(Vec<u32>, Vec<u32>)> {
    match game_split.last() {
        Some(parts) => {
            let number_parts: Vec<&str> = parts.split("|").collect();
            let winning_numbers: Vec<u32> = match number_parts.first() {
                Some(part) => match get_numbers(part) {
                    Ok(numbers) => numbers,
                    Err(err) => return Err(err),
                },
                _ => {
                    return Err(Error::Generic(
                        "Could not parse winning numbers".to_string(),
                    ));
                }
            };
            let numbers: Vec<u32> = match number_parts.last() {
                Some(part) => match get_numbers(part) {
                    Ok(numbers) => numbers,
                    Err(err) => return Err(err),
                },
                _ => return Err(Error::Generic("Could not parse numbers".to_string())),
            };

            Ok((winning_numbers, numbers))
        }
        None => {
            return Err(Error::Generic("Could not parse to game id".to_string()));
        }
    }
}
fn get_numbers(part_string: &str) -> Result<Vec<u32>> {
    let parts = part_string
        .trim()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    parts
        .iter()
        .map(|part| match parse_to_number(part.trim()) {
            Ok(number) => Ok(number),
            _ => Err(Error::Generic("Could not parse to number".to_string())),
        })
        .collect()
}

fn parse_to_number(input: &str) -> Result<u32> {
    match input.parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => {
            return Err(Error::Generic("Could not parse to number".to_string()));
        }
    }
}

trait ScratchGame {
    fn play(&self) -> u32;
    fn matching_numbers(&self) -> u32;
}

impl ScratchGame for Game {
    fn play(&self) -> u32 {
        let mut score = 0;
        self.numbers.iter().for_each(|number| {
            let is_winning = self
                .winning_numbers
                .iter()
                .any(|winning_number| number == winning_number);
            if is_winning {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        });
        score
    }

    fn matching_numbers(&self) -> u32 {
        let mut score = 0;
        self.numbers.iter().for_each(|number| {
            let is_winning = self
                .winning_numbers
                .iter()
                .any(|winning_number| number == winning_number);
            if is_winning {
                score += 1;
            }
        });
        score
    }
}
fn part2(input: &str) -> Result<u32> {
    let game_results = get_games(input);
    let mut copies: HashMap<u32, u32> = HashMap::new();
    let mut game_count = 0;
    game_results
        .iter()
        .for_each(|game_result| match game_result {
            Ok(game) => {
                let score = game.matching_numbers();
                let current_game_count = match copies.get(&game.id) {
                    Some(v) => *v,
                    None => 0,
                };
                for index in 1..score + 1 {
                    let game_copy_id = game.id + index;
                    let copy_count = match copies.get(&game_copy_id) {
                        Some(v) => *v,
                        None => 0,
                    };
                    let new_copy_count = copy_count + (current_game_count * 1) + 1;
                    copies.insert(game_copy_id, new_copy_count);
                }
                game_count += 1;
            }
            Err(_) => {
                println!("Error");
            }
        });
    Ok(copies.values().into_iter().sum::<u32>() + game_count)
}

#[derive(Debug)]
struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

mod tests {
    #[test]
    fn part1_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = super::part1(&input);
        match result {
            Ok(result) => assert_eq!(result, 13),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }
    #[test]
    fn part2_test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = super::part2(&input);
        match result {
            Ok(result) => assert_eq!(result, 30),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }
}
