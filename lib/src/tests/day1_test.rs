mod day1;

#[cfg(test)]
mod tests {
    use super::day1::*;

    #[test]
    fn test_split_lists() {
        let input_data = "10 20\n30 40\n5 15";
        let expected_output = vec![(5, 15), (10, 20), (30, 40)];

        let mut reader = io::Cursor::new(input_data.as_bytes());
        let sorted_pairs = day1::split_lists(&mut reader).unwrap();

        assert_eq!(sorted_pairs, expected_output);
    }
}