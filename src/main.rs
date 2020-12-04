use std::env;

mod read_file;
mod solutions;
use read_file::read_all;
use solutions::{
    correct_password, correct_password_second_solution, missing_passport_sol_1,
    missing_passport_sol_2, tree_problem_1_and_2, two_sum,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day.as_str() {
        "day1" => {
            let input = read_all::<i32>("inputs/input1.in");
            println!("{:?}", two_sum(input));
        }
        "day2" => {
            let input = &read_all::<String>("inputs/input2.in");
            println!("Day 2 solution 1 result: {}", correct_password(input));
            println!(
                "Day 2 solution 2 result: {}",
                correct_password_second_solution(input)
            );
        }
        "day3" => {
            let input = &read_all::<String>("inputs/input3.in");
            println!(
                "Day 3 solution 1 result: {}",
                tree_problem_1_and_2(input, 3, 1)
            );

            struct TreeInput {
                r: i32,
                d: i32,
            }

            let inputs = vec![
                TreeInput { r: 1, d: 1 },
                TreeInput { r: 3, d: 1 },
                TreeInput { r: 5, d: 1 },
                TreeInput { r: 7, d: 1 },
                TreeInput { r: 1, d: 2 },
            ];
            let mut prob_2_sol: u64 = 1;
            for entry in inputs {
                prob_2_sol *= tree_problem_1_and_2(input, entry.r, entry.d);
            }
            println!("Day 3 solution 2 result: {}", prob_2_sol);
        }
        "day4" => {
            let input = &read_all::<String>("inputs/input4.in");
            println!("Day 4 solution 1 result: {}", missing_passport_sol_1(input));

            let text = input.join("\n");
            println!("Day 4 solution 2 result: {}", missing_passport_sol_2(text));
        }
        _ => println!("Wrong argument!"),
    }
}
