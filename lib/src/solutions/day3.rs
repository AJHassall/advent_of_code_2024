
use regex::Regex;

fn filter_disabled_instructions<F>(func: F) -> impl Fn(&str) -> i32
where
    F: Fn(&str) -> i32,
{
    move |x|{

        let re = Regex::new(r"don't\(\)(.*?)(do\(\)|$)").unwrap();

        let replaced_text = re.replace_all(x, "");

        func(&replaced_text)
    }
}

pub fn calculate_from_corrupted_memory(input: &str) -> i32{

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(input) {
            let group = cap.get(0).unwrap();
            let s = group.as_str();
            let numbers: Vec<i32> = s[4..s.len() - 1].split(',')
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect();

            let product = numbers[0] * numbers[1];
            sum += product;
        }

        sum

}

pub fn calculate_from_corrupted_memory_filtered(input: &str) -> i32{
    let decorated_function = 
        filter_disabled_instructions(calculate_from_corrupted_memory);

    decorated_function(input)
}

#[cfg(test)]
mod tests {

    
    use super::*;

    #[test]
    fn test_solution_day_3() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(calculate_from_corrupted_memory(input), 161)

    }

    #[test]
    fn test_solution_day_3_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";


        assert_eq!(calculate_from_corrupted_memory_filtered(input), 48)

    }
}