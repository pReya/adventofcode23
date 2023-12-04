use std::{collections::HashSet, fs};

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

    //println!("{:?}", games);

    part1(games);
    //part2(&contents);
}

fn part1(games: Vec<Vec<Vec<i32>>>) {
    let mut points = 0;

    for game in games {
        let winning_numbers: HashSet<i32> = HashSet::from_iter(game[0].iter().cloned());
        let own_numbers = HashSet::from_iter(game[1].iter().cloned());

        let winner_count = winning_numbers.intersection(&own_numbers).count();
        //println!("{:?}", winner_count);
        if winner_count > 0 {
            points = points + 2_i32.pow(winner_count as u32 - 1)
        }
    }
    println!("Part 1 Points {:?}", points);
}
