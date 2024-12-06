
pub fn pattern_search(input: &Vec<Vec<char>>) -> i32
{
    let mut count = 0;
    for x in 1..input.len()-1{
        for y in 1..input[x].len()-1{

            if input[x][y] != 'A' {
                continue;
            }

            let s = [
                [input[x-1][y-1]],
                [input[x]  [y]  ],
                [input[x+1][y+1]]
            ].concat();

            let s1 = [
                [input[x+1][y-1]],
                [input[x]  [y]  ],
                [input[x-1][y+1]]
            ].concat();

            let s: String = s.into_iter().collect();
            let s1: String = s1.into_iter().collect();

            if (s.matches("MAS").count() ==1 
            || s.matches("SAM").count() ==1) &&
            (s1.matches("MAS").count() ==1 
            || s1.matches("SAM").count() ==1) {
                count +=1;
            }
        }
    }

    count
}

pub fn word_search(input: &Vec<Vec<char>>, seek: &str) -> i32{

    let mut words_found = 0;

    //for each character
    for x in 0..input.len(){
        for y in 0..input[x].len(){

            //println!("new cell");

            //for each direction
            for dx in -1..2{
                for dy in -1..2{

                   //println!("   new direction dx:{} dy:{}", dx, dy);

                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let mut search_failed: bool = false;

                    let mut cx = x as i32;
                    let mut cy = y as i32;
                    
                    for c in seek.chars(){ 
                        if cx < 0 || cx > input.len() as i32 -1  {
                            search_failed = true;
                            break;
                        }

                        if cy < 0 || cy > input[x].len() as i32 -1  {
                            search_failed = true;
                            break;
                        }

                        if input[cx as usize][cy as usize]!=c {
                            search_failed = true;
                        }
                        
                        if search_failed{
                            break;
                        }

                        cx = cx as i32 + dx;
                        cy = cy as i32 + dy;
                    }

                    if search_failed{
                        continue;
                    }

                    words_found +=1;
                }           
            }            
        }
    }


    words_found
}

pub fn string_to_2d_vector(input: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    grid
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solution_day_4() {
        let input = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid = string_to_2d_vector(input);
     assert_eq!(word_search(&grid, "XMAS"), 18)

    }

    #[test]
    fn test_solution_day_4_simple() {
        let input = "XMAS";

        let grid = string_to_2d_vector(input);

        assert_eq!(word_search(&grid, "XMAS"), 1);
    }

    #[test]
    fn test_solution_day_4_part_2() {
        let input = 
"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let grid = string_to_2d_vector(input);
     assert_eq!(pattern_search(&grid), 9)

    }


}