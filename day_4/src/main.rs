const INPUT_DATA: &str = include_str!("input.txt");
const EXAMPLE_DATA: &str = include_str!("example.txt");


#[derive(Debug, Default)]
struct Card {
    own_numbers: Vec<u32>,
    winning_numbers: Vec<u32>
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut own_numbers: Vec<u32> = Vec::new();
        let mut winning_numbers: Vec<u32> = Vec::new();

        let (_, info) = line.split_once(":").expect("Malformed Card (missing :)");
        let (winning, own) = info.split_once("|").expect("Malformed Card (missing |)");

        for number in winning.split_whitespace() {
            winning_numbers.push(number.parse().expect("Malformed number"));
        }

        for number in own.split_whitespace() {
            own_numbers.push(number.parse().expect("Malformed number"));
        }

        Self { 
            own_numbers: own_numbers, 
            winning_numbers: winning_numbers
        }
    }

    fn count_winning(&self) -> u32 {
        self.winning_numbers.iter()
            .fold(0, |acc, w| if self.own_numbers.contains(w) {acc + 1} else {acc})
    }
}

fn main() {
    println!("Part 1: {}", part_1(INPUT_DATA));
    println!("Part 2: {}", part_2(INPUT_DATA));
}


fn part_1(data: &str) -> u32 {
    let mut sum = 0;

    for line in data.lines() {
        let count = Card::parse(line).count_winning();
        if count > 0 {
            sum += 2u32.pow(count - 1);
        }
    }

    sum
}

fn part_2(data: &str) -> u32 {
    let mut cards: Vec<u32> = vec![0; data.lines().count()];

    for (i, line) in data.lines().enumerate() {
        let winnings = Card::parse(line).count_winning() as usize;
        cards[i] += 1;
        
        for j in 1..=winnings {
            if i + j < cards.len() {
                cards[i + j] += cards[i];
            } else {
                break;
            }
        }
    }
    
    cards.iter().sum()
}