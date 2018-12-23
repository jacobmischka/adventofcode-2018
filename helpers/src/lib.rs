use std::fs;

pub fn get_input() -> String {
    fs::read_to_string("./input.txt").expect("Oops!")
}
