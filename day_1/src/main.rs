mod day1;

fn main() {
    day1::print_sum_of_differences(match day1::split_lists(){
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
        }
    });
}
