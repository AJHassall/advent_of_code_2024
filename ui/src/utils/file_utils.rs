use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn read_lines(filename: &str) -> Result<impl Iterator<Item = String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().filter_map(|line| line.ok()))
}