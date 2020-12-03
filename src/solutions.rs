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
