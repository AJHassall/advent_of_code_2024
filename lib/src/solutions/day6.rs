
use super::solution::Solution;
pub struct Day6 {
    input: String,
}

impl Solution for Day6 {

    fn part1(&self) -> &str{
        ""
    }

}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the current module

    #[test]
    fn test_solve() {
        let solution = Day6("");
        let input = "your_example_input"; // Replace with your test input

        let result = solution.part1();
        // Assert your expected output
        assert_eq!(result, "expected_output"); // Replace with your expected output
    }
}