use std::fs::read_to_string;
use std::path::Path;

mod question1;

fn main() {
    let lines = read_lines("./inputs/question1.txt");
    question1::question1(lines);
}

fn read_lines<P: AsRef<Path>>(file_path: P) -> Vec<String>
{
    let mut result = Vec::new();

    for line in read_to_string(file_path).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}