

use super::solution::Solution;
pub struct Day7 {
    pub input: String,
}




impl Solution for Day7 {

    fn print_day(&self) {
        
    }
    fn part1(&self) -> String{
        "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}


fn is_possible(str: &str) -> bool{

    let (total, numbers) = str.split_once(':').unwrap();

    let total: i32 = total.parse().unwrap();
    let number: Vec<i32> = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();

    let add = |a :i32, b: i32| a+b;
    let mult = |a :i32, b: i32| a*b;


    let characters = vec![add, mult];
    let length = 2; // Desired length of inner vectors

    let mut combinations = Vec::new();

    for i in 0..characters.len().pow(length as u32) {
        let mut combination = Vec::new();
        for j in 0..length {
            combination.push(characters[i / characters.len().pow(j as u32) % characters.len()]);
        }
        combinations.push(combination);
    }
    //output should be [[add, mult], [mult, add]]

    for c in combinations{
        let mut result =0;
        for func in c{
            result = func(number[0], number[1]);
            
            for l in number.iter().skip(2){

               
                result = func(result, *l )
            
            }
        }

        if result == total {
            return true;
        }
        println!("{}", result);
    }

    total == number.iter().sum();

    false


}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current module

    #[test]
    fn is_possible_test() {
        // let input = "190: 10 19";

        // assert_eq!(is_possible(input), true);

        let input = "3267: 81 40 27";

        assert_eq!(is_possible(input), true);
    }


    #[test]
    fn test_part1_solution() {
        let input = 
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let solution = Day7{input: input.to_string()};

        let result = solution.part1();

    //    assert_eq!(result, "3749");
    }

}