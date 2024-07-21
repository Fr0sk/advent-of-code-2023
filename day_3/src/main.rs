use std::collections::HashMap;

const INPUT_DATA: &str = include_str!("input.txt");
//const EXAMPLE_DATA: &str = include_str!("example.txt");

#[derive(Debug, Default, Clone, Copy)]
struct Bounds {
    start: usize,
    end: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize
}


fn main() {
    println!("Part 1: {}", part_1(INPUT_DATA)); //521601
    println!("Part 2: {}", part_2(INPUT_DATA)); //80694070
}

fn part_1(data: &str) -> u32{
    let lines: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut sum: u32 = 0;

    for (i, line) in lines.iter().enumerate() {
        for number in get_numbers(line) {
            if !check_all_symbols(&lines, i, number, false).is_empty() {
                sum += get_number(line, &number);
            }
        }
    }

    sum
}

fn part_2(data: &str) -> u32 {
    let lines: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut sum: u32 = 0;
    let mut asterisks: HashMap<Coords, Vec<u32>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        for number in get_numbers(line) {
            for coord in check_all_symbols(&lines, i, number, true) {
                if !asterisks.contains_key(&coord) {
                    asterisks.insert(coord, Vec::new());
                }
                asterisks.get_mut(&coord).expect("Vector should be created").push(get_number(line, &number));
            }
            
        }
    }

    for numbers in asterisks.values() {
        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    sum
}

fn has_symbol(line: &Vec<char>, bounds: &Bounds, only_asterisk: bool) -> Option<usize> {
    assert!(bounds.end < line.len());

    for i in bounds.start..=bounds.end {
        let c = line[i];
        if only_asterisk && c == '*' || !only_asterisk && c != '.' && !c.is_ascii_digit() {
            return Some(i);
        }
    }

    None
}

fn get_numbers(line: &Vec<char>) -> Vec<Bounds> {
    let mut bounds: Vec<Bounds> = Vec::new();
    
    let mut start: i32 = -1;

    for (i, c) in line.iter().enumerate() {
        if c.is_ascii_digit() && start == -1{
            start = i as i32;
        } else if !c.is_ascii_digit() && start > -1 {
            bounds.push(Bounds {
                start: start as usize,
                end: i - 1,
            });
            start = -1;
        }
    }

    if start > -1 {
        bounds.push(Bounds {
            start: start as usize,
            end: line.len() - 1
        });
    }

    bounds
}

fn get_number(line: &Vec<char>, bounds: &Bounds) -> u32 {
    assert!(bounds.end < line.len());

    let result = line[bounds.start..=bounds.end].iter().collect::<String>().parse();
    match result {
        Ok(number) => number,
        Err(_) => 0,
    }
}

fn check_all_symbols(lines: &Vec<Vec<char>>, line_index: usize, number_bounds: Bounds, only_asterisk: bool) -> Vec<Coords> {
    let mut symbols_coords: Vec<Coords> = Vec::new();

    let expanded = Bounds{
        start: number_bounds.start.saturating_sub(1),
        end: (number_bounds.end + 1).min(lines.first().expect("lines should contain elements").len() - 1)
    };

    if line_index > 0 {
        if let Some(c) = check_symbol(lines, line_index - 1, expanded, only_asterisk) {
            symbols_coords.push(c);
        }
    }
    if line_index < lines.len() - 1 {
        if let Some(c) = check_symbol(lines, line_index + 1, expanded, only_asterisk) {
            symbols_coords.push(c);
        }
    }
    if let Some(c) = check_symbol(lines, line_index, expanded, only_asterisk) {
        symbols_coords.push(c);
    }

    symbols_coords
}

fn check_symbol(lines: &Vec<Vec<char>>, line_index: usize, bounds: Bounds, only_asterisk: bool) -> Option<Coords> {
    match has_symbol(&lines[line_index], &bounds, only_asterisk) {
        Some(x) => Some(Coords{
            x: x,
            y: line_index
        }),
        None => None,
    }
}