use std::future::IntoFuture;

use crate::part_kind::PartKind;
use crate::prelude::{Error, Result};

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
    let distances = get_line_numbers(&input.lines().collect::<Vec<&str>>().first())?;
    let records = get_line_numbers(&input.lines().collect::<Vec<&str>>().last())?;
    let result = distances
        .iter()
        .enumerate()
        .map(|(index, distance)| {
            let record = records[index];
            get_winning_outcomes(&distance, &record)
        })
        .collect::<Vec<u32>>()
        .iter()
        .fold(1, |acc, x| acc * x);

    Ok(result)
}

fn part2(input: &str) -> Result<u64> {
    todo!()
}
fn get_number_from_string(text: &str) -> Result<u32> {
    text.parse::<u32>()
        .map_err(|_| Error::Generic("Could not parse seed number".to_string()))
    // match text.parse::<u32>() text.parse::<u32>() {
    //     Ok(number) => Ok(number),
    //     Err(_) => Err(Error::Generic("Could not parse to number".to_string()))
    // }
}
fn get_line_numbers(text_option: &Option<&&str>) -> Result<Vec<u32>> {
    match text_option {
        Some(text) => {
            let mut numbers: Vec<u32> = Vec::new();
            for number_text in text.split_ascii_whitespace().skip(1).collect::<Vec<&str>>() {
                let number = get_number_from_string(&number_text).unwrap();
                numbers.push(number);
            }
            Ok(numbers)
        }
        None => Err(Error::Generic("Could not parse line".to_string())),
    }
}

fn get_winning_outcomes(current: &u32, record: &u32) -> u32 {
    let mut outcomes = 0;
    for number in 0..*current {
        let calculated: u32 = (current - number) * number;
        if calculated > *record {
            outcomes += 1;
        }
    }
    outcomes
}
mod tests {

    #[test]
    fn part1_test() {
        let input = "Time:      7  15   30
                            Distance:  9  40  200";
        let result = super::part1(&input);
        match result {
            Ok(result) => assert_eq!(result, 288),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }

    #[test]
    fn part2_test() {
        todo!()
    }
}
