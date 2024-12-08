
use day6::Day6;
use lib::solutions::*;
use solution::Solution;
use utils::file_utils;

mod utils;

fn main() {

    println!("day 1");

    let day1 = day1::day1_solution("../ui/input/day1_input.txt");

    println!("    part 1 answer: {}", day1);

    let day1 = day1::day1_solution_part_2("../ui/input/day1_input.txt");

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

    println!("day 3");

    let vals3 = file_utils::read_lines("../ui/input/day3_input.txt");

    let v: &str = &String::from(vals3.unwrap().collect::<Vec<String>>().concat());

    let day3_answer = day3::calculate_from_corrupted_memory(&v);

    println!("    part 1 answer: {}", day3_answer);

    let day3_answer_part2 = day3::calculate_from_corrupted_memory_filtered(&v);

    println!("    part 2 answer: {}", day3_answer_part2);




    println!("day 4");
    let vals4 = file_utils::read_lines("../ui/input/day4_input.txt");

    let v: &str = &String::from(vals4.unwrap().collect::<Vec<String>>().join("\n"));
    let input = &day4::string_to_2d_vector(v);

    let day4_answer = day4::word_search(&input, "XMAS");

    println!("    part 1 answer: {}", day4_answer);

    let day4_answer_part2 = day4::pattern_search(&input);

    println!("    part 2 answer: {}", day4_answer_part2);


    println!("day 5");
    let vals5 = file_utils::read_lines("../ui/input/day5_input.txt");

    let v: &str = &String::from(vals5.unwrap().collect::<Vec<String>>().join("\n"));

    let day5_answer = day5::part_1(&v);

    println!("    part 1 answer: {}", day5_answer);

    let day5_answer = day5::part_2(&v);

    println!("    part 2 answer: {}", day5_answer);


    let mut v:  Vec<Box<dyn Solution>> = vec![];

    v.push(Box::new(Day6{
        
    }));

    for i in v{
        i.part1();
    }



}
