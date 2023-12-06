use std::collections::HashMap;
use std::env::var;

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

fn part1(input: &str) -> Result<u64> {
    let lines: Vec<&str> = input.split("\n").collect();
    let seeds = match lines.first() {
        Some(value) => get_seed_numbers(&value)?,
        _ => {
            return Err(Error::Generic("Could not get seeds".to_string()));
        }
    };
    let mut map: HashMap<&str, Category> = HashMap::new();
    let mut cache: HashMap<String, u64> = HashMap::new();
    compose_categories(&mut map, &&lines[1..]);
    let mut destination = u64::MAX;
    for seed in seeds {
        let result_destionation = get_destination(&mut map, seed, "seed-to-soil map:", &mut cache)?;
        if result_destionation < destination {
            destination = result_destionation;
        }
    }
    Ok(destination)
}

fn part2(input: &str) -> Result<u64> {
    let lines: Vec<&str> = input.split("\n").collect();
    let seeds = match lines.first() {
        Some(value) => get_seed_numbers(&value)?,
        _ => {
            return Err(Error::Generic("Could not get seeds".to_string()));
        }
    };
    let mut map: HashMap<&str, Category> = HashMap::new();
    let mut cache: HashMap<String, u64> = HashMap::new();
    compose_categories(&mut map, &&lines[1..]);
    let mut destination = u64::MAX;
    iterate(&seeds, &mut map, &mut cache, &mut destination);
    Ok(destination)
}
fn iterate<'a>(
    seeds: &[u64],
    map: &mut HashMap<&'a str, Category<'a>>,
    cache: &mut HashMap<String, u64>,
    lowest_destination: &mut u64,
) {
    let start = seeds[0];
    let range = seeds[1];

    for seed in start..start + range {
        let result_destination = get_destination(map, seed, "seed-to-soil map:", cache);
        match result_destination {
            Ok(dest) => {
                if dest < *lowest_destination {
                    *lowest_destination = dest;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        };
    }

    if seeds.len() == 2 {
        return;
    }
    iterate(&seeds[2..], map, cache, lowest_destination);
}
fn get_destination<'a>(
    map: &mut HashMap<&'a str, Category<'a>>,
    source: u64,
    category_name: &str,
    cache: &mut HashMap<String, u64>,
) -> Result<u64> {
    // if cache.contains_key(&format!("{}{}", category_name, source)) {
    //     return Ok(*cache.get(&format!("{}{}", category_name, source)).unwrap());
    // }

    let category = map.get(category_name).unwrap();
    let destination = category.get_destination(source)?;
    match category.next_category_name {
        Some(name) => match get_destination(map, destination, name, cache) {
            Ok(new_dest) => {
                // if category_name == "seed-to-soil map:" {
                //     cache.insert(format!("{}{}", category_name, source), new_dest);
                // }
                Ok(new_dest)
            }
            Err(err) => Err(Error::Generic(err.to_string())),
        },

        None => Ok(destination),
    }
}
fn compose_categories<'a>(map: &mut HashMap<&'a str, Category<'a>>, lines: &[&'a str]) {
    match lines.first() {
        Some(line) => {
            if line.is_empty() {
                compose_categories(map, &lines[1..]);
            } else {
                let name = line;
                let compose_result = get_category_maps(&lines[1..]);
                match compose_result {
                    Ok(tuple) => {
                        let category: Category = Category {
                            name: name,
                            maps: tuple.0,
                            next_category_name: tuple.1,
                        };
                        map.insert(category.name, category);
                        compose_categories(map, &lines[(1 + tuple.2 as usize)..]);
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                };
            }
        }
        None => {}
    };
}

fn get_category_maps<'a>(lines: &[&'a str]) -> Result<(Vec<Map>, Option<&'a str>, u64)> {
    let mut skip_count = 0;
    let mut maps: Vec<Map> = Vec::new();
    for line in lines {
        if starts_with_number(&line) {
            let result: Result<Vec<u64>> = line
                .split_ascii_whitespace()
                .map(|x| {
                    x.parse::<u64>()
                        .map_err(|_| Error::Generic("Could not parse seed number".to_string()))
                })
                .collect();

            let numbers = result?;
            let map: Map = Map {
                destination: numbers[0],
                source: numbers[1],
                range: numbers[2],
            };
            maps.push(map);
            skip_count += 1;
        } else if !line.is_empty() {
            return Ok((maps, Some(line), skip_count));
        } else {
            skip_count += 1;
        }
    }

    Ok((maps, None, skip_count))
}

fn get_seed_numbers(text: &str) -> Result<Vec<u64>> {
    match text.split(":").last() {
        Some(value) => {
            let numbers: Result<Vec<u64>> = value
                .split_ascii_whitespace()
                .map(|x| {
                    x.parse::<u64>()
                        .map_err(|_| Error::Generic("Could not parse seed number".to_string()))
                })
                .collect();
            numbers
        }
        _ => Err(Error::Generic("Could not get seeds".to_string())),
    }
}

fn starts_with_number(s: &str) -> bool {
    s.chars().next().map_or(false, |c| c.is_numeric())
}
trait GetDestination {
    fn get_destination(&self, source: u64) -> Result<u64>;
}
impl GetDestination for Category<'_> {
    fn get_destination(&self, source: u64) -> Result<u64> {
        let matching_map = self.maps.iter().find(|map| {
            // (map.source..map.source + map.range)
            //     .collect::<Vec<u64>>()
            //     .contains(&map.source)
            source >= map.source && source < map.source + map.range
        });
        match matching_map {
            Some(value) => {
                let remainder = source - value.source;
                Ok(value.destination + remainder)
            }
            _ => Ok(source),
        }
    }
}
struct Category<'a> {
    name: &'a str,
    maps: Vec<Map>,
    next_category_name: Option<&'a str>,
}
struct Map {
    source: u64,
    destination: u64,
    range: u64,
}
mod tests {

    #[test]
    fn part1_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = super::part1(&input);
        match result {
            Ok(result) => assert_eq!(result, 35),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }

    #[test]
    fn part2_test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = super::part2(&input);
        match result {
            Ok(result) => assert_eq!(result, 46),
            Err(err) => assert!(false, "Error: {}", err),
        }
    }

    // #[test]
    // fn part2_test() {
    //     todo!()
    // }
}
