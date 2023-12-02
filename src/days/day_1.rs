use crate::part_kind::PartKind;

pub fn run(input: &str, parts: &[PartKind]) {
    for part in parts {
        match part {
            PartKind::Part1 => {
                let result = part1(input);
                println!("Result is {}", result);
            }
            PartKind::Part2 => {
                let result = part2(input);
                println!("Result is {}", result);
            }
        }
    }
}
fn part1(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let mut vec = Vec::new();
        let first_char = line.chars().into_iter().find(|c| c.is_numeric());
        let second_char = line.chars().rev().into_iter().find(|c| c.is_numeric());
        match first_char {
            None => continue,
            Some(k) => vec.push(k),
        }

        match second_char {
            None => continue,
            Some(k) => vec.push(k),
        }
        let number = vec.iter().collect::<String>().parse::<i32>().unwrap();
        println!("Number is {}", number);
        result += number;
    }
    result
}
fn part2(input: &str) -> i32 {
    let mut result = 0;

    for line in input.lines() {
        let mut vec = Vec::new();
        let first_char = line.chars().into_iter().find(|c| c.is_numeric());
        let second_char = line.chars().rev().into_iter().find(|c| c.is_numeric());
        match first_char {
            None => continue,
            Some(k) => vec.push(k),
        }

        match second_char {
            None => continue,
            Some(k) => vec.push(k),
        }
        let number = vec.iter().collect::<String>().parse::<i32>().unwrap();
        println!("Number is {}", number);
        result += number;
    }
    result
}
#[cfg(test)]
mod tests {
    use crate::days::day_1::part1;

    #[test]
    fn part1_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = part1(&input);
        assert_eq!(result, 142);
    }
}
