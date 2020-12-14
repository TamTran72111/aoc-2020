use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("File does not exist")
}

pub fn read_lines(file_name: &str) -> Vec<String> {
    let content = read_file(file_name);
    content.split('\n').map(|line| line.to_string()).collect()
}

pub fn read_int_array<T>(file_name: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let content = read_file(file_name);
    content
        .split('\n')
        .map(|num| num.parse::<T>().unwrap())
        .collect()
}

pub fn read_block(file_name: &str) -> Vec<String> {
    let content = read_file(file_name);
    content
        .split("\n\n")
        .map(|block| block.to_string())
        .collect()
}
