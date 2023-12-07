use day_kind::{DayKind, GetDayNameTrait, GetFileContentTrait, RunDayTrait};
mod day_kind;
mod days;
mod error;
mod part_kind;
mod prelude;
mod reader;

fn main() {
    let day = DayKind::Day6("./src/inputs/day_6.txt", &[part_kind::PartKind::Part1]);
    run_day(day);
}

fn run(days: Vec<DayKind>) {
    for day in days {
        run_day(day);
    }
}

fn run_day(day: DayKind<'_>) {
    let name = day.get_day();

    println!("Day name: {}", name);

    println!();
    println!("------------------------");
    println!();
    let context = day.get_content();
    match context {
        Ok(content) => day.run(&content),
        Err(_) => println!("Error: could not parse file",),
    }
}
