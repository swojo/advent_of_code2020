use std::io;
use std::env;

mod day1;
mod day2;

fn exec_day(day: u8) {
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
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
