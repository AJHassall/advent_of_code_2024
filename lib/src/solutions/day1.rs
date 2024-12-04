use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

fn print_sum_of_differences(sorted_pairs: Vec<(i32, i32)>) -> i32{
    let mut sum = 0;

    for (x, y) in sorted_pairs {
        sum += (x - y).abs();
    }

    sum
}

fn split_lists(path: &str) -> Result<Vec<(i32, i32)>, io::Error>{

    let file = File::open(path)?;
    let reader = BufReader::new(file);



    let mut left_list = Vec::new();
    let mut right_list = Vec::new();


    for line in reader.lines()  {


        let line = line?;

        if line.is_empty() {
            break;
        }

        let mut numbers = line.split_whitespace();
        let left = numbers.next().unwrap().parse::<i32>().unwrap();
        left_list.push(left);
        let right = numbers.next().unwrap().parse::<i32>().unwrap();
        right_list.push(right);


    }

    left_list.sort();
    right_list.sort();

    let sorted_pairs: Vec<_> = left_list.iter()
        .zip(right_list.iter()).map(|(&left, &right)| (left, right)).collect();

    Ok(sorted_pairs)



}


pub fn day1_solution(path: &str) -> i32{    
    
    print_sum_of_differences(match split_lists(&path){
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return -1;
        }
    })
}