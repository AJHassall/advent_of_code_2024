

pub trait Solution: Send + Sync{
    fn print_day(&self){
      panic!();
    }
    fn part1(&self) -> String;
    fn part2(&self) -> String;

}