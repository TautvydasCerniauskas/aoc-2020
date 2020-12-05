use regex::Regex;
use std::collections::HashMap;

// Day 1
pub fn two_sum(input: Vec<i32>) -> i32 {
    let target: i32 = 2020;

    let mut hm: HashMap<i32, i32> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        hm.insert(*line, i as i32);
    }

    for line in input.iter() {
        let compliment = target - line;

        for (idx, line1) in input.iter().enumerate() {
            let compliment1 = compliment - line1;
            if let Some(&index) = hm.get(&compliment1) {
                if index != idx as i32 {
                    return line * line1 * compliment1;
                }
            }
        }
    }
    0
}

// Day 2
pub fn correct_password(input: &Vec<String>) -> i32 {
    let mut good_ones = 0;

    for line in input.iter() {
        let line = line.replace(&['(', ')', ',', '\"', '.', ';', ':', '-', '\''][..], " ");
        let line: Vec<&str> = line.split(" ").collect();
        let low_range: i32 = take(&line, 0).parse::<i32>().unwrap();
        let high_range = take(&line, 1).parse::<i32>().unwrap();
        let c = take(&line, 2);
        let value = take(&line, 4);

        let v = value.matches(c).count() as i32;
        if v >= low_range && v <= high_range {
            good_ones += 1;
        }
    }
    good_ones
}

fn take<T: Copy>(vec: &Vec<T>, index: usize) -> T {
    *vec.get(index).unwrap()
}

// Day 2
pub fn correct_password_second_solution(input: &Vec<String>) -> i32 {
    let mut good_ones = 0;

    for line in input.iter() {
        let line = line.replace(&['(', ')', ',', '\"', '.', ';', ':', '-', '\''][..], " ");
        let line: Vec<&str> = line.split(" ").collect();
        let low_range: usize = take(&line, 0).parse::<usize>().unwrap();
        let high_range = take(&line, 1).parse::<usize>().unwrap();
        let c: &str = take(&line, 2);
        let value = take(&line, 4);

        if let Some(low_value) = value.chars().nth(low_range - 1) {
            if let Some(high_value) = value.chars().nth(high_range - 1) {
                // TODO: Maybe combine these 2?
                if low_value.to_string() == c && high_value.to_string() != c {
                    good_ones += 1;
                }
                if low_value.to_string() != c && high_value.to_string() == c {
                    good_ones += 1;
                }
            }
        }
    }
    good_ones
}

// Day 3
pub fn tree_problem_1_and_2(input: &Vec<String>, right: i32, down: i32) -> u64 {
    let mut trees = 0;
    let mut index = 0;
    for (i, line) in input.iter().enumerate() {
        if i % down as usize == 0 {
            let max_index = line.len() - 1;
            let line: Vec<char> = line.chars().collect();
            let c = take(&line, index);
            if c == '#' {
                trees += 1;
            }
            index += right as usize;
            if index > max_index {
                index = (index - max_index) - 1;
            }
        }
    }
    trees
}

// Day 4
pub fn missing_passport_sol_1(input: &Vec<String>) -> u64 {
    let mut value_to_parse = String::new();
    let valid_keys: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut result = 0;
    for (i, line) in input.iter().enumerate() {
        value_to_parse.push_str(line);
        value_to_parse.push(' ');
        if i == input.len() - 1 || line.len() == 0 {
            let keys: Vec<_> = value_to_parse
                .split([' ', ':'].as_ref())
                .filter(|c| !c.is_empty())
                .step_by(2)
                .collect();

            if valid_keys.iter().all(|item| keys.contains(item)) {
                result += 1;
            }
            value_to_parse.clear();
        }
    }
    result
}

// Day 4 problem 2
pub fn missing_passport_sol_2(input: String) -> usize {
    let entries = parse_entries(&input);
    let v = Validator::new();
    entries.iter().filter(|e| v.is_valid(e)).count()
}

fn has_fields(e: &Entry) -> bool {
    e.len() == 8 || (e.len() == 7 && !e.contains_key("cid"))
}

fn atoi(s: &str) -> Option<i32> {
    i32::from_str_radix(s, 10).ok()
}

type Entry<'t> = HashMap<&'t str, &'t str>;

fn parse_entries(s: &str) -> Vec<Entry> {
    let mut entries = Vec::new();
    for line in s.split("\n\n") {
        let mut m = HashMap::new();
        for segment in line.split_ascii_whitespace() {
            let mut ent = segment.split(':');
            let k = ent.next().unwrap();
            let v = ent.next().unwrap();
            m.insert(k, v);
        }
        entries.push(m);
    }
    entries
}

struct Validator {
    res: HashMap<&'static str, Regex>,
}

impl Validator {
    fn new() -> Self {
        let mut res = HashMap::new();
        let regex = |s| Regex::new(s).unwrap();
        res.insert("byr", regex(r"^\d{4}$"));
        res.insert("iyr", regex(r"^\d{4}$"));
        res.insert("eyr", regex(r"^\d{4}$"));
        res.insert("hgt", regex(r"^\d+(cm|in)$"));
        res.insert("hcl", regex(r"^#[0-9a-f]{6}$"));
        res.insert("ecl", regex(r"^(amb|blu|brn|gry|grn|hzl|oth)$"));
        res.insert("pid", regex(r"^[0-9]{9}$"));
        Self { res }
    }

    fn is_valid(&self, e: &Entry) -> bool {
        let valid_fields = e
            .iter()
            .filter(|(&k, _)| k != "cid")
            .filter(|(&k, v)| self.res[k].is_match(v))
            .filter(|(&k, v)| match k {
                "byr" => atoi(v).map_or(false, |n| n >= 1920 && n <= 2002),
                "iyr" => atoi(v).map_or(false, |n| n >= 2010 && n <= 2020),
                "eyr" => atoi(v).map_or(false, |n| n >= 2020 && n <= 2030),
                "hgt" => match &v[v.len() - 2..] {
                    "cm" => atoi(&v[..3]).map_or(false, |n| n >= 150 && n <= 193),
                    "in" => atoi(&v[..2]).map_or(false, |n| n >= 59 && n <= 76),
                    _ => unreachable!(),
                },
                _ => true,
            })
            .count();
        return has_fields(e) && valid_fields == 7;
    }
}

// Day 5
pub fn boarding_problem(input: &Vec<String>) -> i32 {
    input
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(7);
            let res_first = handle_first(first, 0.0_f32, 127.0_f32);
            let res_second = handle_first(second, 0.0_f32, 7.0_f32);
            res_first * 8 + res_second
        })
        .max()
        .unwrap()
}

fn handle_first(first: &str, mut range_start: f32, mut range_end: f32) -> i32 {
    let mut result = 0.0;
    for (i, f) in first.chars().enumerate() {
        match f {
            'F' | 'L' => {
                if i == first.len() - 1 {
                    result = range_start;
                }
                let new_value = ((range_end - range_start) / 2.0).floor();
                range_end = range_end - (new_value + 1.0);
            }
            'B' | 'R' => {
                if i == first.len() - 1 {
                    result = range_end;
                }
                let new_value: f32 = ((range_end - range_start) / 2.0).ceil();
                range_start = range_start + new_value;
            }
            _ => unreachable!(),
        };
    }
    result as i32
}
