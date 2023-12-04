use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let games: Vec<Vec<Vec<i32>>> = contents
        .lines()
        .map(|line| {
            let without_game_index = line.split(": ").collect::<Vec<&str>>()[1];

            without_game_index
                .split(" | ")
                .map(|number_group| {
                    number_group
                        .split_whitespace()
                        .map(|number| number.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect();

    // PART 1

    let mut points = 0;
    let won_games = part1(games);
    let won_games2 = won_games.iter().cloned().collect();

    for correct_numbers in won_games {
        if correct_numbers > 0 {
            points = points + 2_i32.pow(correct_numbers as u32 - 1)
        }
    }
    println!("Part 1 Points: {:?}", points);

    // PART 2

    part2(won_games2);
}

fn part1(games: Vec<Vec<Vec<i32>>>) -> Vec<i32> {
    let won_per_game: Vec<i32> = games
        .iter()
        .map(|game| {
            let winning_numbers: HashSet<i32> = HashSet::from_iter(game[0].iter().cloned());
            let own_numbers = HashSet::from_iter(game[1].iter().cloned());

            let winner_count = winning_numbers.intersection(&own_numbers).count();
            //println!("{:?}", winner_count);
            winner_count as i32
        })
        .collect();

    won_per_game
}

fn part2(won_games: Vec<i32>) {
    let mut sum = 0;
    let mut q = VecDeque::from((0..won_games.len()).collect::<Vec<usize>>());

    while !q.is_empty() {
        let game_index: usize = q.pop_front().unwrap();
        let won_count: i32 = won_games[game_index];

        sum = sum + 1;

        if won_count > 0 {
            for i in 1..won_count + 1 {
                q.push_back(game_index + i as usize);
            }
        }
    }

    println!("Part 2 Number of cards: {}", sum);
}
