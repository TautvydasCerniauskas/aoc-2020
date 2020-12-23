use regex::Regex;
use std::collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet};

// Day 1
pub fn two_sum(input: &Vec<i32>) -> Option<i32> {
    let target: i32 = 2020;

    let mut hm: HashMap<i32, i32> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        hm.insert(*line, i as i32);
    }

    println!("{:?}", input);
    for line in input.iter() {
        let compliment = target - line;

        for (idx, line1) in input.iter().enumerate() {
            let compliment1 = compliment - line1;
            if let Some(&index) = hm.get(&compliment1) {
                if index != idx as i32 {
                    Some(line * line1 * compliment1);
                }
            }
        }
    }
    None
}

// Day 2
pub fn correct_password(input: &Vec<String>) -> Option<i32> {
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
    Some(good_ones)
}

fn take<T: Copy>(vec: &Vec<T>, index: usize) -> T {
    *vec.get(index).unwrap()
}

// Day 2
pub fn correct_password_second_solution(input: &Vec<String>) -> Option<i32> {
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
    Some(good_ones)
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

fn has_fields(e: &CustomEntry) -> bool {
    e.len() == 8 || (e.len() == 7 && !e.contains_key("cid"))
}

fn atoi(s: &str) -> Option<i32> {
    i32::from_str_radix(s, 10).ok()
}

type CustomEntry<'t> = HashMap<&'t str, &'t str>;

fn parse_entries(s: &str) -> Vec<CustomEntry> {
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

    fn is_valid(&self, e: &CustomEntry) -> bool {
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

// Day 10
pub fn adapter_problem(input: &Vec<usize>) -> Option<usize> {
    let mut input = input.clone();
    &input.push(0);
    &input.sort();
    let mut jumps = [0; 3];
    input.windows(2).for_each(|adapters| {
        jumps[adapters[1] - adapters[0] - 1] += 1;
    });
    Some(jumps[0] * (jumps[2] + 1))
}

pub fn adapter_problem_2(input: &Vec<usize>) -> Option<usize> {
    let input = input.iter().copied().chain(Some(0)).collect::<HashSet<_>>();
    let max = *input.iter().max()?;
    let mut paths_heaps = BinaryHeap::new();
    let mut paths_count = HashMap::new();
    paths_count.insert(max, 1);
    paths_heaps.push(max);
    while let Some(path) = paths_heaps.pop() {
        let count = paths_count.remove(&path)?;
        if path == 0 {
            return Some(count);
        }
        for step in 1..=3 {
            if path < step {
                continue;
            }
            let next = path - step;
            if !input.contains(&next) {
                continue;
            }
            match paths_count.entry(next) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += count;
                }
                Entry::Vacant(entry) => {
                    entry.insert(count);
                    paths_heaps.push(next);
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SeatOption {
    Free,
    Occupied,
    Floor,
}

impl SeatOption {
    pub fn match_on_input(input: char) -> Self {
        match input {
            'L' => SeatOption::Free,
            '#' => SeatOption::Occupied,
            _ => SeatOption::Floor,
        }
    }

    fn count_adjacent(input: &mut Vec<Vec<usize>>, rows: usize, cols: usize, x: usize, y: usize) {
        // We are only searching adjacent 3x3
        for dy in 0..3 {
            for dx in 0..3 {
                if dx == 1 && dy == 1 {
                    continue;
                }
                if (y + dy >= 1) && (x + dx >= 1) && (y + dy <= rows) && (x + dx <= cols) {
                    input[y + dy - 1][x + dx - 1] += 1;
                }
            }
        }
    }

    fn count_neighbors(input: &Vec<Vec<SeatOption>>, rows: usize, cols: usize) -> Vec<Vec<usize>> {
        let mut counts = vec![vec![0; cols]; rows];
        for y in 0..rows {
            for x in 0..cols {
                if input[y][x] == SeatOption::Occupied {
                    SeatOption::count_adjacent(&mut counts, rows, cols, x, y);
                }
            }
        }
        counts
    }
}

// Day 11
pub fn seat_problem(input: &Vec<Vec<SeatOption>>) -> Option<usize> {
    let mut input = input.clone();
    let first_value = input.get(0)?;
    let rows = input.len();
    let cols = first_value.len();
    loop {
        let mut changes = false;
        let occupied_neighbors = SeatOption::count_neighbors(&input, rows, cols);
        for y in 0..rows {
            for x in 0..cols {
                match (input[y][x], occupied_neighbors[y][x]) {
                    (SeatOption::Free, 0) => {
                        changes = true;
                        input[y][x] = SeatOption::Occupied;
                    }
                    (SeatOption::Occupied, n) if n >= 4 => {
                        changes = true;
                        input[y][x] = SeatOption::Free;
                    }
                    _ => (),
                }
            }
        }
        if !changes {
            break;
        }
    }
    let res = *&input.iter().fold(0, |acc, row| {
        acc + row.iter().filter(|&&ch| ch == SeatOption::Occupied).count()
    });
    Some(res)
}

// Day 12 navigation problem
pub fn navigation_problem(input: &Vec<(String, i32)>) -> Option<usize> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut angle = 0;
    for instruction in input {
        match instruction.0.as_str() {
            "F" => match angle {
                0 => x += instruction.1,
                90 => y += instruction.1,
                180 => x -= instruction.1,
                270 => y -= instruction.1,
                _ => panic!("invalid angle"),
            },
            "N" => y -= instruction.1,
            "S" => y += instruction.1,
            "E" => x += instruction.1,
            "W" => x -= instruction.1,
            "L" => angle = (angle + 360 - instruction.1) % 360,
            "R" => angle = (angle + instruction.1) % 360,
            _ => unreachable!(),
        }
    }

    Some((x.abs() + y.abs()) as usize)
}

pub fn navigation_problem_2(input: &Vec<(String, i32)>) -> Option<usize> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut wpx: i32 = 10;
    let mut wpy: i32 = -1;
    for instruction in input {
        match instruction.0.as_str() {
            "F" => {
                x += wpx * instruction.1;
                y += wpy * instruction.1;
            }
            "N" => wpy -= instruction.1,
            "S" => wpy += instruction.1,
            "E" => wpx += instruction.1,
            "W" => wpx -= instruction.1,
            "L" => {
                let (new_wpx, new_wpy) = match instruction.1 {
                    90 => (wpy, -wpx),
                    180 => (-wpx, -wpy),
                    270 => (-wpy, wpx),
                    _ => panic!("invalid angle"),
                };
                wpx = new_wpx;
                wpy = new_wpy;
            }
            "R" => {
                let (new_wpx, new_wpy) = match instruction.1 {
                    90 => (-wpy, wpx),
                    180 => (-wpx, -wpy),
                    270 => (wpy, -wpx),
                    _ => panic!("invalid angle"),
                };
                wpx = new_wpx;
                wpy = new_wpy;
            }
            _ => unreachable!(),
        }
    }

    Some((x.abs() + y.abs()) as usize)
}

// Day 13
pub fn bus_problem(input: &Vec<String>) -> Option<usize> {
    let timestamp = input[0].parse::<usize>().unwrap();
    let buses = input[1]
        .split(",")
        .filter_map(|c| c.parse::<usize>().ok())
        .collect::<HashSet<usize>>();

    let mut bus_id: usize = 0;
    let mut minutes: usize = 10000000;

    for bus in buses.iter() {
        let new_value = bus + (timestamp - (timestamp % bus));
        let diff = new_value - timestamp;
        if diff < minutes {
            bus_id = *bus;
            minutes = diff;
        }
    }
    Some(bus_id * minutes)
}

struct Bus {
    bus_id: usize,
    offset: usize,
}

pub fn bus_departure_time(input: &Vec<String>) -> Option<usize> {
    let _ = input[0].parse::<usize>().unwrap();

    let mut x = 1;
    let mut earliest_timestamp = 0;

    let buses = input[1]
        .split(",")
        .enumerate()
        .filter_map(|(offset, bus_id)| {
            bus_id
                .parse::<usize>()
                .ok()
                .map(|id| Bus { bus_id: id, offset })
        })
        .collect::<Vec<Bus>>();

    for bus in buses {
        let mut xn = x;
        while (xn + (bus.offset + earliest_timestamp)) % bus.bus_id != 0 {
            xn += x;
        }

        earliest_timestamp += xn;
        x *= bus.bus_id;
    }

    Some(earliest_timestamp)
}

// Day 14
pub fn memory_problem(input: &Vec<String>) -> Option<usize> {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut and_or = (0, 0);
    for line in input.iter() {
        if line.starts_with("mas") {
            and_or = line
                .split(" = ")
                .skip(1)
                .next()
                .unwrap()
                .bytes()
                .rev()
                .enumerate()
                .fold((usize::MAX, 0), |(and, or), (i, b)| match b {
                    b'0' => (and & !(1 << i), or),
                    b'1' => (and, or | 1 << i),
                    _ => (and, or),
                });
        } else {
            &line[4..]
                .split(']')
                .collect::<Vec<&str>>()
                .windows(2)
                .for_each(|l| {
                    let l0 = l[0].parse::<usize>().unwrap();
                    let m = l[1]
                        .replace(&[']', '=', ' '][..], "")
                        .parse::<usize>()
                        .unwrap();
                    mem.insert(l0, m & and_or.0 | and_or.1);
                });
        }
    }
    Some(mem.values().sum())
}

// Day 15
pub fn number_game(input: &Vec<usize>) -> Option<usize> {
    println!("{}", nth(30000000, &input));
    Some(nth(2020, &input))
}

fn nth(n: usize, numbers: &Vec<usize>) -> usize {
    let mut last_seen = HashMap::new();
    let mut next: usize = 0;

    for i in 1..n {
        let next0 = match numbers.iter().next() {
            Some(num) => *num,
            None => next,
        };
        let next1 = match last_seen.get(&next0) {
            Some(ts) => i - ts,
            None => 0,
        };
        last_seen.insert(next0, i);
        next = next1;
    }
    next
}

// Day 16
pub fn train_ticket_problem(input: &Vec<String>) -> Option<usize> {
    let mut values: HashSet<usize> = HashSet::new();
    let mut results: Vec<usize> = vec![];
    let mut tickets_start_at_next_line = false;
    for line in input.iter() {
        let l: Vec<&str> = line.split(": ").collect();
        let mut first_range = Vec::new();
        let mut second_range = Vec::new();

        let ranges = l.get(1);
        if ranges.is_some() {
            let r = ranges.unwrap().split(" or ").collect::<Vec<&str>>();
            first_range = r[0]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            second_range = r[1]
                .split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
        }

        if !first_range.is_empty() && !second_range.is_empty() {
            (first_range[0]..first_range[1] + 1).for_each(|x| {
                values.insert(x);
                ()
            });
            (second_range[0]..second_range[1] + 1).for_each(|x| {
                values.insert(x);
                ()
            });
        }
        if !tickets_start_at_next_line {
            tickets_start_at_next_line = line.contains("nearby");
        }
        if tickets_start_at_next_line {
            line.split(",").for_each(|x| {
                let x = x.parse::<usize>().unwrap_or(0);
                if !values.contains(&x) {
                    results.push(x);
                }
                ()
            });
        }
    }
    Some(results.iter().sum())
}
