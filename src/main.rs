use std::env;

mod benchmark;
mod read_file;
mod solutions;
use benchmark::benchmarked_main;
use read_file::read_all;
use solutions::{
    bag_problem, boarding_problem, boarding_problem_2, computer_problem, computer_problem_2,
    correct_password, correct_password_second_solution, encoder_problem, encoder_problem_2,
    missing_passport_sol_1, missing_passport_sol_2, question_problem, question_problem_2,
    tree_problem_1_and_2, two_sum,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    const ITERATIONS: usize = 100;
    match day.as_str() {
        "day1" => {
            benchmarked_main(read_all, two_sum, "inputs/input1.in", ITERATIONS);
        }
        "day2" => {
            benchmarked_main(read_all, correct_password, "inputs/input2.in", ITERATIONS);
            println!("\n");
            benchmarked_main(
                read_all,
                correct_password_second_solution,
                "inputs/input2.in",
                ITERATIONS,
            );
        }
        "day3" => {
            println!("\n");
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
        "day5" => {
            let input = &read_all::<String>("inputs/input5.in");
            println!(
                "Day 5 solution 1 result: {}",
                boarding_problem(&input).iter().max().unwrap()
            );

            let result = boarding_problem_2(&input);
            println!("Day 5 solution 2 result: {:?}", result);
        }
        "day6" => {
            let input = &read_all::<String>("inputs/input6.in");
            println!("Day 6 solution 1 result: {}", question_problem(&input));

            println!("Day 6 solution 2 result: {}", question_problem_2(&input));
        }
        "day7" => {
            let input = &read_all::<String>("inputs/input7.in");
            bag_problem(&input);
        }
        "day8" => {
            let input = &read_all::<String>("inputs/input8.in");
            println!("Part 1 solution: {}", computer_problem(&input));
            println!("Part 2 solution: {}", computer_problem_2(&input).unwrap());
        }
        "day9" => {
            let input = &read_all::<String>("inputs/input9.in");
            println!("Part 1 solution: {}", encoder_problem(&input, 25));
            println!("Part 2 solution: {}", encoder_problem_2(&input, 25));
        }
        _ => println!("Wrong argument!"),
    }
}
