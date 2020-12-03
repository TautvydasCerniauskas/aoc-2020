use std::fs;
use std::str::FromStr;
pub fn read_all<T: FromStr>(file_name: &str) -> Vec<T> {
    fs::read_to_string(file_name)
        .expect("File not found!")
        .lines()
        .map(|x| x.parse::<T>())
        .filter_map(Result::ok)
        .collect()
}
