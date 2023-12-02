use crate::prelude::Result;
use crate::reader::file::read_file;
use crate::{days::days_1::day_1, part_kind::PartKind};

#[derive(Debug)]
pub enum DayKind<'a> {
    Day1(&'a str, &'a [PartKind]),
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
            _ => String::from("Unknown"),
        }
    }
}

impl<'a> RunDayTrait for DayKind<'a> {
    fn run(&self, input: &str) {
        match self {
            DayKind::Day1(_, parts) => day_1(input, parts),
            _ => print!(""),
        }
    }
}

impl<'a> GetFileContentTrait for DayKind<'a> {
    fn get_content(&self) -> Result<String> {
        match self {
            DayKind::Day1(fileName, _) => read_file(&fileName),
        }
    }
}
