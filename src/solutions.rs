use regex::Regex;
use std::collections::{HashMap, HashSet};

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
pub fn boarding_problem(input: &Vec<String>) -> Vec<i32> {
    input
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(7);
            let res_first = handle_first(first, 0.0_f32, 127.0_f32);
            let res_second = handle_first(second, 0.0_f32, 7.0_f32);
            res_first * 8 + res_second
        })
        .collect()
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

pub fn boarding_problem_2(input: &Vec<String>) -> i32 {
    let result = boarding_problem(input);
    let first = result.iter().min().unwrap();
    let last = result.iter().max().unwrap();
    let new_set: Vec<i32> = (*first..*last).collect();
    *take(
        &new_set
            .iter()
            .filter(|x| !result.contains(&x))
            .collect::<Vec<_>>(),
        0,
    )
}

// Day 6
pub fn question_problem(input: &Vec<String>) -> usize {
    let mut result = 0;
    let mut hm: HashMap<char, i32> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        for c in line.chars() {
            hm.insert(c, 1);
        }

        if line.len() == 0 || i == input.len() - 1 {
            result += hm.len();
            hm.clear();
        }
    }
    result
}

pub fn question_problem_2(input: &Vec<String>) -> usize {
    let mut hm: HashMap<char, i32> = HashMap::new();
    let mut result = 0;
    let mut groups = 0;

    for (i, line) in input.iter().enumerate() {
        let break_condition = line.len() == 0 || i == input.len() - 1;
        groups += 1;
        for c in line.chars() {
            hm.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        if break_condition {
            let value_to_check_against = if i == input.len() - 1 {
                groups
            } else {
                groups - 1
            };

            result += hm
                .values()
                .filter(|v| *v == &value_to_check_against)
                .count();

            // Reset values
            groups = 0;
            hm.clear();
        }
    }
    result
}

#[derive(Debug)]
struct Bag {
    head: String,
    tail: HashMap<String, i32>,
}

impl Bag {
    fn build_bags(row: Vec<&str>) -> Self {
        let head = take(&row, 0)[..take(&row, 0).len() - 5].to_string();
        let tail = take(&row, 1);
        if tail == "no other bags." {
            return Bag {
                head,
                tail: HashMap::new(),
            };
        }

        let right_side: HashMap<String, i32> = take(&row, 1)
            .split(", ")
            .map(|c| {
                let capture = Regex::new(r"^(\d+) ((\w+ ?)+) bags?\.?$")
                    .unwrap()
                    .captures(c)
                    .unwrap();
                let count = atoi(capture.get(1).unwrap().as_str()).unwrap();
                let bag = capture.get(2).unwrap().as_str().to_string();
                (bag, count)
            })
            .collect();
        Bag {
            head,
            tail: right_side,
        }
    }
}

// Day 7
pub fn bag_problem(input: &Vec<String>) {
    let mut bags: Vec<Bag> = Vec::new();
    for line in input.iter() {
        let row: Vec<&str> = line.split(" contain ").collect();
        bags.push(Bag::build_bags(row));
    }
    println!(
        "Solution to the first problem: {}",
        contains(&bags, &HashSet::new(), "shiny gold").len()
    );
    println!(
        "Solution to second problem: {}",
        search_bags(&bags, &mut HashMap::new(), "shiny gold")
    );
}
fn search_bags(bags: &Vec<Bag>, contains_map: &mut HashMap<String, i32>, bag: &str) -> i32 {
    bags.iter()
        .find(|b| b.head == bag)
        .unwrap()
        .tail
        .clone()
        .iter()
        .map(|(k, v)| {
            let req = match contains_map.get(k) {
                Some(&req) => req,
                None => {
                    let req = search_bags(bags, contains_map, k) + 1;
                    &contains_map.insert(k.to_string(), req);
                    req
                }
            };
            v * req
        })
        .sum()
}

fn contains(bags: &Vec<Bag>, contains_map: &HashSet<String>, bag: &str) -> HashSet<String> {
    let mut res_map: HashSet<String> = bags
        .iter()
        .filter(|b| b.tail.contains_key(bag))
        .filter(|b| !contains_map.contains(&b.head))
        .map(|b| &b.head)
        .map(|b| b.into())
        .collect();

    let extra: HashSet<String> = res_map
        .iter()
        .flat_map(|b| contains(bags, &res_map, b))
        .collect();
    res_map.extend(extra.iter().cloned());
    res_map
}

// Day 8
#[derive(Debug, Clone)]
struct Operation {
    head: String,
    tail: i32,
}

#[derive(Debug, Clone)]
enum Outcome {
    InfiniteLoop(isize),
    Terminate(isize),
}

impl Operation {
    fn new(input: &Vec<String>) -> Vec<Self> {
        input
            .iter()
            .map(|s| {
                let s: Vec<&str> = s.split(" ").collect();
                let operation = take(&s, 0);
                let value = take(&s, 1);
                Operation {
                    head: operation.to_string(),
                    tail: atoi(value).unwrap(),
                }
            })
            .collect()
    }
}

pub fn computer_problem(input: &Vec<String>) -> isize {
    let operations: Vec<Operation> = Operation::new(input);
    process_operations(operations).unwrap()
}

fn process_operations(operations: Vec<Operation>) -> Option<isize> {
    let mut index: i32 = 0;
    let mut acc: isize = 0;
    let mut visited = HashSet::new();
    Some(loop {
        if !visited.insert(index) {
            break acc;
        } else {
            match operations.get(index as usize).unwrap().head.as_str() {
                "acc" => {
                    acc += operations.get(index as usize).unwrap().tail as isize;
                    index += 1;
                    ()
                }
                "jmp" => {
                    index += operations.get(index as usize).unwrap().tail;
                    ()
                }
                "nop" => index += 1,
                _ => (),
            }
        }
    })
}

pub fn computer_problem_2(input: &Vec<String>) -> Option<i32> {
    let operations: Vec<Operation> = Operation::new(input);
    let (_, jumps) = process_operation_2(operations.clone(), None);
    for index in jumps {
        let (outcome, _) = process_operation_2(operations.clone(), Some(index as isize));
        if let Outcome::Terminate(acc) = outcome {
            return Some(acc as i32);
        };
    }
    None
}

fn process_operation_2(operations: Vec<Operation>, hack: Option<isize>) -> (Outcome, HashSet<i32>) {
    let mut indexes = HashSet::new();
    let mut index: i32 = 0;
    let mut acc: isize = 0;
    let mut visited = HashSet::new();
    let outcome = loop {
        if visited.insert(index) {
            let operation = operations.get(index as usize).unwrap();
            let opcode = if let Some(hack) = hack {
                if index == hack as i32 {
                    "nop"
                } else {
                    operation.head.as_str()
                }
            } else {
                operation.head.as_str()
            };
            match opcode {
                "acc" => {
                    acc += operation.tail as isize;
                    index += 1;
                }
                "jmp" => {
                    indexes.insert(index);
                    index += operation.tail;
                }
                "nop" => index += 1,
                _ => unreachable!(),
            }
            if index as usize == operations.len() {
                break Outcome::Terminate(acc);
            }
        } else {
            break Outcome::InfiniteLoop(acc);
        }
    };
    (outcome, indexes)
}

// Day 9
pub fn encoder_problem(input: &Vec<String>, preamble: usize) -> usize {
    let mut index = 0;
    loop {
        let chunk_start = index;
        let chunk_end = index + preamble;
        if chunk_end > input.len() - 1 {
            break 0;
        }
        let value_to_check = atoi(input.get(chunk_end).unwrap()).unwrap();

        let input_range = &input[chunk_start..chunk_end];
        let result = find_if_sum_exists(input_range, value_to_check);
        if let None = result {
            return value_to_check as usize;
        }

        index += 1;
    }
}

fn find_if_sum_exists(input: &[String], value_to_check_against: i32) -> Option<bool> {
    for l in input.iter() {
        let l = atoi(l).unwrap();
        let remainder = value_to_check_against - l;
        if input.contains(&remainder.to_string()) {
            return Some(true);
        }
    }
    None
}

pub fn encoder_problem_2(input: &Vec<String>, preamble: usize) -> i32 {
    let mut index = 0;
    let invalid_number = encoder_problem(input, preamble);
    let mut return_set = vec![];
    loop {
        let return_sum: i32 = return_set.iter().sum();
        if return_sum < invalid_number as i32 {
            let value_to_insert = atoi(input.get(index).unwrap()).unwrap();
            return_set.push(value_to_insert);
            index += 1;
        }

        if return_sum > invalid_number as i32 {
            return_set.remove(0);
        }

        if return_set.iter().sum::<i32>() == invalid_number as i32 {
            return return_set.iter().min().unwrap() + return_set.iter().max().unwrap();
        }
    }
}
