const INPUT_DATA: &str = include_str!("input.txt");
//const EXAMPLE_DATA: &str = include_str!("example.txt");

#[derive(Debug, Default, Clone, Copy)]
struct Cubes {
    r: u32,
    g: u32,
    b: u32
}

#[derive(Debug, Default, Clone, Copy)]
struct Game {
    id: u32,
    cubes: Cubes
}


fn main() {
    println!("Part 1: {}", part_1());
    println!("Part 2: {}", part_2());
}


fn part_1() -> u32 {
    let max_cubes = Cubes {
        r: 12,
        g: 13,
        b: 14
    };

    let mut sum: u32 = 0;

    for line in INPUT_DATA.lines() {
        let game = parse_game(line);
        
        if game.cubes.r <= max_cubes.r && game.cubes.g <= max_cubes.g && game.cubes.b <= max_cubes.b {
            sum += game.id;
        }
    }

    sum
}

fn part_2() -> u32 {
    let mut sum: u32 = 0;

    for line in INPUT_DATA.lines() {
        let game = parse_game(line);
        sum += game.cubes.r * game.cubes.g * game.cubes.b;
    }

    sum
}

fn parse_game(line: &str) -> Game {
    let mut game = Game::default();
    
    let line = line.strip_prefix("Game ").expect("This line should start with the word Game");
    let (id, rounds) = line.split_once(": ").expect("Game ID separator not found");
    
    game.id = id.parse().expect("ID is not numerical");

    for round in rounds.split("; ") {
        for draw in round.split(", ") {
            let (number, color) = draw.split_once(" ").expect("Expected <space> between number and color");
            let number: u32 = number.parse().expect("Could not parse number");
            
            match color {
                "red" => game.cubes.r = game.cubes.r.max(number),
                "green" => game.cubes.g = game.cubes.g.max(number),
                "blue" => game.cubes.b = game.cubes.b.max(number),
                _ => println!("Unidentified color {}", color)
            }
        }
    }

    game
}

