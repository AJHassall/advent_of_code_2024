

use super::solution::Solution;
pub struct Day7 {
    pub input: String,
}

impl Solution for Day7 {

    fn print_day(&self) {
        println!("day 7");
    }

    fn part1(&self) -> String{
        let mut total_of_valid = 0;
        for line in self.input.split('\n') {

            let add = |a :_, b: _| a+b;
            let mult = |a :_, b: _| a*b;

            match is_possible(line, vec![&add,&mult]) {
                Some(product)=>{
                    total_of_valid += product;

                }                
                _=>{}
            }
            
        }


        total_of_valid.to_string()
    }

    fn part2(&self) -> String {
        let mut total_of_valid = 0;
        for line in self.input.split('\n') {

            let add = |a :_, b: _| a+b;
            let mult = |a :_, b: _| a*b;
            let conc = |a :i64, b: i64| (a.to_string() + &b.to_string()).parse::<i64>().unwrap();

            match is_possible(line, vec![&add,&mult, &conc]) {
                Some(product)=>{
                    total_of_valid += product;

                }                
                _=>{}
            }
            
        }


        total_of_valid.to_string()
    }
}


fn is_possible(str: &str, methods: Vec<&dyn Fn(i64, i64) -> i64>) -> Option<i64>{

    let (total, numbers) = str.split_once(':').unwrap();

    let total: i64 = total.parse().unwrap();
    let number: Vec<i64> = numbers.split_whitespace().map(|n| n.parse().unwrap()).collect();

    let characters = methods;
    let length = number.len()-1; // Desired length of inner vectors

    let mut combinations = Vec::new();

    for i in 0..characters.len().pow(length as u32) {
        let mut combination = Vec::new();
        for j in 0..length {
            combination.push(characters[i / characters.len().pow(j as u32) % characters.len()]);
        }
        combinations.push(combination);
    }
    
    for c in combinations{
        let mut i = c.iter();

        let mut product = number.first().unwrap().clone();

        for s in number.iter().skip(1){
            if product > total {
                return None;
            }
            product = i.next().unwrap()(product, s.clone());
        }

        if product== total{
            return Some(product);
        }
    }

    None


}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current module

    #[test]
    fn is_possible_test() {

        let add = |a :_, b: _| a+b;
        let mult = |a :_, b: _| a*b;
        let conc = |a :i64, b: i64| (a.to_string() + &b.to_string()).parse::<i64>().unwrap();

        let input = "190: 10 19";

        assert_eq!(is_possible(input, vec![&add, &mult] ), Some(190));

        let input = "3267: 81 40 27";

        assert_eq!(is_possible(input, vec![&add, &mult]), Some(3267));
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

        let solution: Day7 = Day7{input: input.to_string()};

        let result = solution.part1();

        assert_eq!(result, "3749");
    }

}