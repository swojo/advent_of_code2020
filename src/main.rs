use std::env;

mod tools;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn exec_day(day: u8) {
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        _ => ()
    }
}

fn main() {
    let day = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap_or(1);
    
    exec_day(day);
}
