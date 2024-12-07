pub fn part_1(input: &str) -> i32{

    let lines = input.split('\n').collect::<Vec<&str>>();

    let (rules, pages) = lines.split_at(lines.iter().position(|&x| x =="").unwrap());

    let rules: &Vec<(i32, i32)> = &rules.iter().map(|x| {

        let mut l = x.split('|');
        let i = l.next().unwrap().parse::<i32>().unwrap();
        let j = l.next().unwrap().parse::<i32>().unwrap();

        (i, j)



    }).collect::<Vec<(i32,i32)>>();

    let mut sum = 0;

    //for each page
    'outer: for r in 1..pages.len()  {
        let i = &pages[r].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for c in i{
            if !rules
                .clone()
                .iter()
                .map(|(l,_)| l.clone())
                .collect::<Vec<i32>>()
                .contains(&c) {

                continue;
                
            }
        }

        for (l, r) in rules{

            let j = i
                                .iter()
                                .filter(|x| x == &l || x == &r)
                                .collect::<Vec<&i32>>();

            if j.len() < 2 {
                continue;
            }

            if j[0] != l{
                continue 'outer;
            }


//            println!("{:?}", i)

            
        }

        let middle_index = i.len() / 2;

        // Access the middle element and convert it to a number
        sum += i[middle_index];


    }

    sum
}


#[cfg(test)]
mod tests {
    use crate::solutions::day5::part_1;


    
    #[test]
    fn test_part_1_solution() {
        let input =  
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part_1(input), 143)

    }
}