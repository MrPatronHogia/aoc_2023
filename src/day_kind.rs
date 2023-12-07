use crate::part_kind::PartKind;
use crate::prelude::Result;
use crate::reader::file::read_file;

#[derive(Debug)]
pub enum DayKind<'a> {
    Day1(&'a str, &'a [PartKind]),
    Day2(&'a str, &'a [PartKind]),
    Day3(&'a str, &'a [PartKind]),
    Day4(&'a str, &'a [PartKind]),
    Day5(&'a str, &'a [PartKind]),
    Day6(&'a str, &'a [PartKind]),
}
pub trait GetDayNameTrait {
    fn get_day(&self) -> String;
}

pub trait RunDayTrait {
    fn run(&self, input: &str);
}
pub trait GetFileContentTrait {
    fn get_content(&self) -> Result<String>;
}

impl<'a> GetDayNameTrait for DayKind<'a> {
    fn get_day(&self) -> String {
        match self {
            DayKind::Day1(_, _) => String::from("1"),
            DayKind::Day2(_, _) => String::from("2"),
            DayKind::Day3(_, _) => String::from("3"),
            DayKind::Day4(_, _) => String::from("4"),
            DayKind::Day5(_, _) => String::from("5"),
            DayKind::Day6(_, _) => String::from("6"),
        }
    }
}

impl<'a> RunDayTrait for DayKind<'a> {
    fn run(&self, input: &str) {
        match self {
            DayKind::Day1(_, parts) => crate::days::day_1::run(input, parts),
            DayKind::Day2(_, parts) => crate::days::day_2::run(input, parts),
            DayKind::Day3(_, parts) => crate::days::day_3::run(input, parts),
            DayKind::Day4(_, parts) => crate::days::day_4::run(input, parts),
            DayKind::Day6(_, parts) => crate::days::day_6::run(input, parts),
            _ => print!(""),
        }
    }
}

impl<'a> GetFileContentTrait for DayKind<'a> {
    fn get_content(&self) -> Result<String> {
        match self {
            DayKind::Day1(file_name, _) => read_file(&file_name),
            DayKind::Day2(file_name, _) => read_file(&file_name),
            DayKind::Day3(file_name, _) => read_file(&file_name),
            DayKind::Day4(file_name, _) => read_file(&file_name),
            DayKind::Day5(file_name, _) => read_file(&file_name),
            DayKind::Day6(file_name, _) => read_file(&file_name),
        }
    }
}
