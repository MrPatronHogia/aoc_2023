use crate::{part_kind::PartKind, prelude::Result};

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
    let mut vec: Vec<Part> = Vec::new();
    let mut y = 0;
    input.lines().for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        let mut x = 0;
        let mut skip_count = 0;
        for (index, &char) in chars.iter().enumerate() {
            if (skip_count > 0) {
                skip_count -= 1;
                continue;
            }
            match char.is_ascii_digit() {
                true => {
                    let (number, count_to_skip): (u32, usize) =
                        get_number_and_positions(&char, &chars[index + 1..]);
                    print!("{}", number.to_string());
                    skip_count = count_to_skip;
                    let mut positions: Vec<Position> = Vec::new();
                    let position = Position { x: x, y: y };
                    x += 1;
                    positions.push(position);
                    for _ in 0..skip_count {
                        x += 1;
                        positions.push(Position { x: x, y: y });
                    }
                    vec.push(Part::Number(positions, number));
                }
                false => match char {
                    '.' => {
                        print!(".");
                        x += 1;
                    }
                    _ => {
                        print!("{}", char);
                        let position = Position { x: x, y: y };
                        vec.push(Part::Symbol(position, char));
                        x += 1;
                    }
                },
            }
        }
        y += 1;
        println!();
    });
    let sum = get_sum(&vec);
    Ok(sum)
}

fn get_sum(vec: &Vec<Part>) -> u32 {
    let mut sum = 0;
    for item in vec {
        for symbol_part in vec.iter().filter(|o| match o {
            Part::Symbol(_, _) => true,
            _ => false,
        }) {
            let is_adjecent_result = item.is_adjacent(&symbol_part);
            if is_adjecent_result.0 {
                match item {
                    Part::Number(positions, number) => {
                        match is_adjecent_result.1 {
                            Some(pos) => {
                                println!("Number is {}", number);
                                println!("Symbol is at x: {}, y: {}", pos.x, pos.y);
                            }
                            None => {}
                        };
                        sum += number;
                    }
                    _ => {}
                }
                break;
            }
        }
    }
    sum
}

fn get_number_and_positions(currrent_char: &char, other_chars: &[char]) -> (u32, usize) {
    let mut number_chars: Vec<char> = Vec::new();
    number_chars.push(currrent_char.clone());
    let next_char = other_chars.iter().next();
    match next_char {
        Some(_) => recursive(next_char, &other_chars[1..], &mut number_chars),
        None => {}
    }
    let number_string: String = number_chars.iter().collect();
    return (number_string.parse().unwrap(), number_chars.len() - 1);
    fn recursive(next: Option<&char>, other_chars: &[char], number_chars: &mut Vec<char>) {
        match next {
            Some(value) => {
                if value.is_ascii_digit() {
                    number_chars.push(*value);
                    if other_chars.len() > 1 {
                        recursive(other_chars.iter().next(), &other_chars[1..], number_chars);
                    } else if other_chars.len() == 1 {
                        let last_char = other_chars.last().unwrap();
                        if last_char.is_ascii_digit() {
                            number_chars.push(*last_char);
                        }
                    }
                }
            }
            None => {}
        }
    }
}

fn part2(input: &str) -> Result<u32> {
    todo!()
}
enum Part {
    Number(Vec<Position>, u32),
    Symbol(Position, char),
}
trait IsAdjecent {
    fn is_adjacent(&self, other: &Part) -> (bool, Option<Position>);
}
impl IsAdjecent for Part {
    fn is_adjacent(&self, other: &Part) -> (bool, Option<Position>) {
        match self {
            Part::Number(item_pos, value) => match other {
                Part::Symbol(other_pos, _) => {
                    if is_adjacent(&item_pos, &other_pos) {
                        return (true, Some(other_pos.clone()));
                    }
                    return (false, None);
                }
                _ => (false, None),
            },
            _ => (false, None),
        }
    }
}

fn is_adjacent(positions: &Vec<Position>, other_position: &Position) -> bool {
    positions.iter().any(|position| {
        (position.x == other_position.x
            && (position.y == other_position.y + 1 || position.y == other_position.y - 1))
            || (position.y == other_position.y
                && (position.x == other_position.x + 1 || position.x == other_position.x - 1))
            || (position.y == other_position.y + 1
                && (position.x == other_position.x + 1 || position.x == other_position.x - 1))
            || (position.y == other_position.y - 1
                && (position.x == other_position.x + 1 || position.x == other_position.x - 1))
            || (position.x == other_position.x + 1
                && (position.y == other_position.y + 1 || position.y == other_position.y - 1))
            || (position.x == other_position.x - 1
                && (position.y == other_position.y + 1 || position.y == other_position.y - 1))
    })
}
// fn is_adjacent(positions: &Vec<Position>, other_position: &Position) -> bool {
//     positions
//         .iter()
//         .any(|position| is_adjacent_single(position, other_position))
// }

// fn is_adjacent_single(position: &Position, other_position: &Position) -> bool {
//     let dx = (position.x - other_position.x).abs();
//     let dy = (position.y - other_position.y).abs();
//     (dx == 1 && dy <= 1) || (dy == 1 && dx <= 1)
// }
#[derive(Clone, Eq, Hash, PartialEq)]

struct Position {
    x: i32,
    y: i32,
}
#[cfg(test)]
mod tests {
    use super::{is_adjacent, Position};

    #[test]
    fn part1_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = super::part1(&input);
        match result {
            Ok(result) => assert_eq!(result, 4361),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }
    #[test]
    fn part1_test_2() {
        let input = "..............492.......&...*....91.......%...*....779.......-691.93.......@.........710.*....*...=...310..........97.....*./..........*....
..........568*.......708...216..............780......*....................312........*....438.....346....$..$327...=......5........756..855.";
        let result = super::part1(&input);
        match result {
            Ok(result) => assert_eq!(result, 8390),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }
    #[test]
    fn is_adjacent_test() {
        let positions: Vec<Position> = vec![
            Position { x: 5, y: 0 },
            Position { x: 6, y: 0 },
            Position { x: 7, y: 0 },
        ];
        let other_pos = Position { x: 3, y: 1 };
        let result = is_adjacent(&positions, &other_pos);

        assert_eq!(result, false);
    }
}
