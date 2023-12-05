use crate::{
    part_kind::PartKind,
    prelude::{Error, Result},
};
use std::{collections::HashMap, error};

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
fn part1(input: &str) -> Result<i32> {
    let green_max = 13;
    let blue_max = 14;
    let red_max = 12;

    let result = input.lines().map(|line| {
        let parts: Vec<&str> = line.split(":").collect();
        let game = match parts.first() {
            Some(game_part) => game_part,
            _ => return Err(Error::Generic("Could not parse game".to_string())),
        };
        let rest_result = match parts.last() {
            Some(bricks) => bricks,
            None => return Err(Error::Generic("Could not parse rest".to_string())),
        };

        let is_valid = rest_result.split(";").all(|part| {
            let mut green_total = 0;
            let mut blue_total = 0;
            let mut red_total = 0;
            part.split(",").all(|f| {
                let cube: Vec<&str> = f.trim().split(" ").collect();
                let score = cube.first().unwrap();
                let colour = cube.last().unwrap();

                match colour {
                    &"green" => {
                        green_total += score.parse::<i32>().unwrap();
                    }
                    &"blue" => {
                        blue_total += score.parse::<i32>().unwrap();
                    }
                    &"red" => {
                        red_total += score.parse::<i32>().unwrap();
                    }
                    _ => {}
                }
                green_max >= green_total && blue_max >= blue_total && red_max >= red_total
            })
        });
        if is_valid {
            let game_id = game.trim().split(" ").last().unwrap().parse().unwrap();
            return Ok(game_id);
        }

        Ok(0)
    });
    let mut sum = 0;
    for res in result {
        match res {
            Ok(res) => sum += res,
            Err(_) => return Err(Error::Generic("Could not parse result".to_string())),
        }
    }

    Ok(sum)
}

fn part2(input: &str) -> Result<i32> {
    let result = input.lines().map(|line| {
        let game_parts: Vec<&str> = line.split(":").collect();
        let values = game_parts.last();
        let mut map: HashMap<&str, i32> = HashMap::new();

        let parts = match values {
            Some(content) => content,
            None => return Err(Error::Generic("Could not parse values".to_string())),
        };

        parts.split(";").try_for_each(|f| {
            f.split(",").try_for_each(|x| {
                let cube: Vec<&str> = x.trim().split(" ").collect();
                let score = cube.first().unwrap();
                let colour = cube.last().unwrap();
                let color_value_result = map.get(colour);
                let score = match score.parse::<i32>() {
                    Ok(value) => value,
                    Err(_) => return Err(Error::Generic("Could not parse to number".to_string())),
                };
                match color_value_result {
                    Some(value) => {
                        if value < &score {
                            map.insert(&colour, score);
                        }
                    }
                    None => {
                        map.insert(&colour, score);
                    }
                };
                Ok(())
            })
        })?;
        Ok(map.values().fold(1, |acc, &val| acc * val))
    });

    let mut sum = 0;
    for res in result {
        match res {
            Ok(res) => sum += res,
            Err(_) => return Err(Error::Generic("Could not parse result".to_string())),
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::days::day_2::part1;
    use crate::days::day_2::part2;

    #[test]
    fn part1_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part1(&input);
        match result {
            Ok(result) => assert_eq!(result, 8),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }
    #[test]
    fn part2_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = part2(&input);
        match result {
            Ok(result) => assert_eq!(result, 2286),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }
}
