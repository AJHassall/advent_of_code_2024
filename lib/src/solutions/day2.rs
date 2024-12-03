use std::i32;


fn is_safe(val: String) -> bool
{
    let numbers: Vec<i32> = val.split_whitespace()
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

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

pub fn day2_solution<'a, I>(vals: I) -> i32
where
    I: Iterator<Item = String>,
{
    let mut safe_count= 0;

    for val in vals {

        if is_safe(val) {
            safe_count+=1;
        }
    }

    safe_count
    
}


#[cfg(test)]
mod tests {

    
    use super::*;
    use std::{io::BufRead, iter};

    #[test]
    fn test_read_lines_mock() {
        let mock_lines: Vec<String> = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];


    
       
        assert_eq!(day2_solution(mock_lines.iter()), 2)

    }
}