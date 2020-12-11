use std::fs;
use std::str::FromStr;

use crate::solutions::SeatOption;

pub fn read_all<T: FromStr>(file_name: &str) -> Vec<T> {
    fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|x| x.parse::<T>())
        .filter_map(Result::ok)
        .collect()
}

pub fn parse_as_seat_option(file_name: &str) -> Vec<Vec<SeatOption>> {
    fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|x| {
            let x: Vec<SeatOption> = x
                .chars()
                .map(|c| {
                    if c == 'L' {
                        SeatOption::Occupied
                    } else {
                        SeatOption::match_on_input(c)
                    }
                })
                .collect();
            x
        })
        .collect()
}
