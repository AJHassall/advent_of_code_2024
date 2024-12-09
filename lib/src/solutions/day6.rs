
use std::{collections::{hash_set, HashSet}, i32};

use super::solution::Solution;
pub struct Day6 {
    pub input: String,
}

fn string_to_2d_vector(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    grid
}

fn get_character_direction(character: char) -> (i32, i32){

    let r = match character {
        '<' => (0, 1),
        '>' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => (0,0)
    };

    r
}

fn get_character_position(map:  &Vec<Vec<char>>) -> (i32, i32){
    let characterDirections = ['v', '<','>','^'];

    for y in 0.. map.len(){
        for x in 0..map[0].len(){
            if characterDirections.iter().any(|d: &char| d == &map[y][x]) {
                return (y as i32, x as i32);
            }
        }
    }

    println!("map has no character");

    panic!()
}

fn position_in_bounds(y: i32, x :i32, map: &Vec<Vec<char>>) -> bool{
    return (y >= 0 && y < map.len() as i32) && (x >= 0 && x < map[0].len() as i32);
}

fn rotate(dy: i32, dx: i32) -> (i32, i32){
    (dx, dy* -1)
}

fn go(y: i32, x: i32, dy: i32, dx: i32) -> (i32, i32){
    (y + dy, x + dx)
}

fn brute_force_find_infinite_loops(input: &str) -> String {
    let mapOrig: &Vec<Vec<char>> = &string_to_2d_vector(input);
    let mut sum = 0;
    
    let characterDirections = ['v', '<','>','^'];
    for (y,x) in get_distinct_positions(input)  {
        
        let mut map = mapOrig.clone();
        
            if map[y as usize][x as usize] == '#' || characterDirections.iter().any(|f| *f == map[y as usize][x as usize]) {
                continue;
            }

            map[y as usize][x as usize] = '#';

            let (mut y, mut x) = get_character_position(&map);
            let (mut dy, mut dx) = get_character_direction(map[y as usize][x as usize]);
    
            let mut distinct_position = HashSet::new();
            distinct_position.insert((y, x, dy, dx));

            while position_in_bounds(y as i32 + dy, x as i32 + dx, &map) {
                
                while map[(y + dy) as usize][(x + dx) as usize] == '#' {
                    //rotate 90 deg clock
                    (dy, dx) = rotate(dy, dx);
                }
                (y, x) = go(y, x, dy, dx);

                if !distinct_position.insert((y, x, dy, dx)){
                    sum+=1;
                    break;
                }
                
                
            }
        }
    
    
    String::from(sum.to_string())
}

fn get_distinct_positions(input: &str) ->HashSet<(i32, i32)>{

    let map = &string_to_2d_vector(input);


    let (mut y, mut x) = get_character_position(&map);
    let (mut dy, mut dx) = get_character_direction(map[y as usize][x as usize]);

    let mut distinct_position = HashSet::new();

    distinct_position.insert((y, x));

    while position_in_bounds(y as i32 + dy, x as i32 + dx, map) {        
        //rotate 90 deg clock till not facing wall
        while map[(y + dy) as usize][(x + dx) as usize] == '#' {
            (dy, dx) = rotate(dy, dx);
        }
        
        (y, x) = go(y, x, dy, dx);
        distinct_position.insert((y, x));

    }
    
    distinct_position
}


impl Solution for Day6 {

    fn print_day(&self) {
        println!("Day 6");
    }
    fn part1(&self) -> String{

        String::from(get_distinct_positions(&self.input).len().to_string())        
    }

    fn part2(&self) -> String {
        String::from(brute_force_find_infinite_loops(&self.input))
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current module

    #[test]
    fn test_solve() {
        let input = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."; 

        let solution = Day6{input: input.to_string()};


        let result = solution.part1();
        // Assert your expected output
        assert_eq!(result, "41"); // Replace with your expected output
    }

    #[test]
    fn test_part2() {
        let input = 
"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."; 

        let solution = Day6{input: input.to_string()};


        let result = solution.part2();
        // Assert your expected output
        assert_eq!(result, "6"); // Replace with your expected output
    }
}