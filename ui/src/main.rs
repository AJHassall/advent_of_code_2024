use lib::solutions::*;
use utils::file_utils;

mod utils;

fn main() {
    day1::day1_solution("../ui/input/day1_input.txt");

    let vals = file_utils::read_lines("../ui/input/day2_input.txt")
    .unwrap() ; // Unwrap for simplicity (handle errors in real code)

    let r = day2::day2_solution(vals);

    println!("{}", r);
}
