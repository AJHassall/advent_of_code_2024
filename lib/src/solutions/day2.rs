

fn is_safe(val: String) -> bool
{
    let numbers: Vec<i32> = val.split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();


    if check_safe(numbers) {
        return true;
    }
    
    false
}

pub fn day2_solution<'a, I>(vals: I) -> i32
where
I: Iterator<Item = String>,
{
    let mut safe_count= 0;

    for val in vals.into_iter(){
        
        if is_safe(val.to_string()) {
            safe_count+=1;
        }
    }
    
    safe_count
    
}


pub fn day2_solution_part_2<'a, I>(vals: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut safe_count= 0;

    for val in vals.into_iter() {
    
        if is_safe(val.clone()) {
            safe_count+=1;
        }

        else {
            if is_safe_part_2(&val) {
                safe_count+=1;
            }
        }
    }

    safe_count
    
}
fn generate_mutations(arr: &[i32]) -> Vec<Vec<i32>> {
    let mut mutations = Vec::new();

    for i in 0..arr.len() {
        let mut mutation = arr.to_vec();
        mutation.remove(i);
        mutations.push(mutation);

    }

    mutations

}

fn is_safe_part_2(val: &String) -> bool
{
    let numbers: Vec<i32> = val.split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

    let mutations = generate_mutations(&numbers);

    for mutation in mutations{

        if check_safe(mutation) {
            return true;
        }
    }
    false

}

fn check_safe(numbers: Vec<i32>) -> bool {
    let mut previous_number = numbers[0];

    let mut is_increasing = true;
    let mut is_decreasing = true;

  

    for item in numbers.into_iter().skip(1)  {

        if (item - previous_number).abs() > 3 || (item - previous_number).abs() == 0{
            return false;
        }

        is_increasing &= item >= previous_number;
        is_decreasing &= item <= previous_number;

        previous_number = item;

    }

    is_increasing || is_decreasing
}


#[cfg(test)]
mod tests {

    
    use super::*;

    #[test]
    fn test_solution_day_2() {
        let mock_lines: Vec<String> = vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ];

        assert_eq!(day2_solution(mock_lines.clone().into_iter()), 2)

    }

    #[test]
    fn test_solution_day_2_part2() {
        let mock_lines: Vec<String> = vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ];

        assert_eq!(day2_solution_part_2(mock_lines.clone().into_iter()), 4)

    }
}