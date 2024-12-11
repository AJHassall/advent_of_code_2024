
use std::{collections::HashMap, hash::Hash};
use itertools::Itertools;

use super::solution::Solution;
pub struct Day8{
    pub input: String
}

impl Solution for Day8 {

    fn part1(&self) -> String {

        let (y_upper, x_upper, node_locations) = get_all_node_positions(&self.input);

        let mut anti_node_positions = vec![];

        for (key, value) in &node_locations{
            let new_anti_nodes = value.iter().tuple_combinations().map(|x|{

                let (a,b)= x;

                let (y0, x0) = a;
                let (y1, x1) = b;

                //let (dy, dx) = (y1 - y0, x1 - y0);

                return vec![
                    ((y0 - (y1 - y0)), (x0 - (x1 - x0))),
                    ((y1 - (y0 - y1), x1 - (x0 - x1))),
                ]

            });
            
            anti_node_positions.extend(new_anti_nodes.into_iter().flatten());
        }

        println!("{:?}", anti_node_positions.len());//.iter().unique().collect::<Vec<_>>());
        println!("{:?}", anti_node_positions.iter().unique().collect::<Vec<_>>().len());

        println!("{}", anti_node_positions.iter()
        .unique()
        .filter(|(y,x)| *y >= 0 && y < &y_upper && *x >= 0 && x <  &x_upper).collect::<Vec<_>>().len());

        println!("{}", anti_node_positions.iter()
            .unique()
                .filter(|(y,x)| *y >= 0 && y < &y_upper && *x >= 0 && x <  &x_upper)  
            .filter(|&&x | 
                !node_locations.values().flatten().contains(&x))
            .collect::<Vec<_>>().len());

        let r = anti_node_positions.iter()
             .unique()
             .filter(|(y,x)| *y >= 0 && y <&y_upper && *x >= 0 && x <  &x_upper)  
            .collect::<Vec<_>>();
        
        let mut grid  = self.input.split('\n').map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

        for (y, x) in &r[..]  {
            grid[*y as usize][*x as usize] = '#';
        }


        println!("========");

        for (y, row) in  grid.iter().enumerate(){
            for (x, cell) in row.iter().enumerate(){
                print!("{}", cell);
        
            }
            println!();
        }


        r.len().to_string()

      //  r.collect::<Vec<_>>().len().to_string()
        
    //    "".to_string()
    }

    fn part2(&self) -> String {
        "".to_string()
    }

    fn print_day(&self) {

        println!("day 8")
        
    }
    
}

fn get_all_node_positions(input: &str) -> (i32, i32, HashMap<char, Vec<(i32, i32)>>){

    let grid  = input.split('\n').map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let mut node_positions = HashMap::new();

    for (y, row) in  grid.iter().enumerate(){
        for (x, cell) in row.iter().enumerate(){
            print!("{}", cell);
            if *cell != '.' {
                node_positions.entry(cell.to_owned()).or_insert_with(Vec::new).push((y as i32,  x as i32));
            }
        }
        println!();
    }

    let y_upper = grid.len() as i32;
    let x_upper = grid.iter()
                            .next()
                            .unwrap()
                            .len() as i32;

    (y_upper, x_upper, node_positions)

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution_day_8() {
        let input = 
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let solution = Day8{input: input.to_string()};


        assert_eq!(solution.part1(), "14");

    }
}