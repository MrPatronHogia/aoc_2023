use day_kind::{DayKind, GetDayNameTrait, GetFileContentTrait, RunDayTrait};
mod day_kind;
mod days;
mod error;
mod part_kind;
mod prelude;
mod reader;

fn main() {
    let days = vec![DayKind::Day1(
        "./src/inputs/day_1.txt",
        &[part_kind::PartKind::Part1],
    )];

    run(days);
}

fn run(days: Vec<DayKind>) {
    for day in days {
        let name = day.get_day();

        println!("Day name: {}", name);

        println!();
        println!("------------------------");
        println!();
        let context = day.get_content();
        match context {
            Ok(content) => day.run(&content),
            Err(error) => println!("Error: could not parse file",),
        }
    }
}
