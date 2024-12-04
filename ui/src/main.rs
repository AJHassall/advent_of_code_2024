use lib::solutions::*;
use utils::file_utils;

mod utils;

fn main() {

    println!("day 1");

    let day1 = day1::day1_solution("../ui/input/day1_input.txt");

    println!("    part 1 answer: {}", day1);

    println!("day 2");

    let day2_input = file_utils::read_lines("../ui/input/day2_input.txt")
    .unwrap();

    let day2_part1 = day2::day2_solution(day2_input);

    println!("    part 1 answer: {}", day2_part1);


    let vals2 = file_utils::read_lines("../ui/input/day2_input.txt")
    .unwrap();

    let day2_part2 = day2::day2_solution_part_2(vals2);

    println!("    part 2 answer: {}", day2_part2);
}
