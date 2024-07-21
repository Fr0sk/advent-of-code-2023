use std::{fs, u32};

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input_data = fs::read_to_string("input.txt").expect("File not found");
    
    let result = part_1(&input_data);
    println!("Part 1: {result}");

    let result = part_2(&input_data);
    println!("Part 2: {result}");
}

fn part_1(input_data: &str) -> u32 {
    let lines = input_data.lines();
    let mut sum = 0;

    for line in lines {
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        for char in line.chars() {
            if char.is_numeric() {
                first = char
                    .to_digit(10)
                    .expect(&format!("Unexpected {char} char type"));
                break;
            }
        }

        for char in line.chars().rev() {
            if char.is_numeric() {
                last = char
                    .to_digit(10)
                    .expect(&format!("Unexpected {char} char type"));
                break;
            }
        }

        sum += first * 10 + last;
    }

    sum
}


fn part_2(input_data: &str) -> u32 {
    let lines = input_data.lines();

    let mut sum = 0;

    for mut line in lines {
        let mut first: u32 = 0;
        let mut last: u32 = 0;

        while line.len() > 0 {
            last = match digit_from_str_start(line) {
                Some(digit) => digit,
                None => last,
            };
            
            if first == 0 {
                first = last;
            }

            line = &line[1..];
        }

        sum += first * 10 + last;
    }

    sum
}

fn digit_from_str_start(string: &str) -> Option<u32> {
    match string.chars().next() {
        Some(char) => {
            match char.to_digit(10) {
                Some(digit) => Some(digit),
                None => {
                    for (index, number) in NUMBERS.iter().enumerate() {
                        if string.starts_with(number) {
                            return Some(index as u32 + 1);
                        }
                    }
                    None
                }
            }
        }
        None => None,
    }
}
