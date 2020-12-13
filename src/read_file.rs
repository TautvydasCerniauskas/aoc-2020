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

pub fn read_file_for_navigation(file_name: &str) -> Vec<(String, i32)> {
    fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|i| {
            let (action, value) = i.split_at(1);
            let value = value.parse::<i32>().unwrap();
            (action.to_string(), value)
        })
        .collect::<Vec<(String, i32)>>()
}
