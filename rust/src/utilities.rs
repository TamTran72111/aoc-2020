use std::fs;

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("File does not exist")
}

pub fn read_lines(file_name: &str) -> Vec<String> {
    let content = read_file(file_name);
    content.split('\n').map(|line| line.to_string()).collect()
}

pub fn read_int_array(file_name: &str) -> Vec<i32> {
    let content = read_file(file_name);
    content
        .split('\n')
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}
