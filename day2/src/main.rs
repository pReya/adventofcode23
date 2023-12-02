use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let data = prepare(&contents);
    part1(&data);
    part2(&data);
}

fn prepare(input: &str) -> Vec<Vec<Vec<i32>>> {
    let lines: Vec<&str> = input.lines().collect();

    // Prepare data structure with game data
    let games: Vec<&str> = lines
        .iter()
        .map(|l| -> &str {
            let line_parts: Vec<&str> = l.split(": ").collect();
            &line_parts[1]
        })
        .collect();

    let game_data: Vec<Vec<Vec<i32>>> = games
        .iter()
        .map(|game| -> Vec<Vec<i32>> {
            game.split("; ")
                .map(|round| -> Vec<i32> {
                    let colors: Vec<Vec<&str>> = round
                        .split(", ")
                        .map(|color_and_value| -> Vec<&str> {
                            color_and_value.split(" ").collect()
                        })
                        .collect();

                    let mut r_g_b: Vec<i32> = vec![0, 0, 0];

                    for color in colors {
                        if color[1] == "red" {
                            r_g_b[0] = color[0].parse::<i32>().unwrap()
                        } else if color[1] == "green" {
                            r_g_b[1] = color[0].parse::<i32>().unwrap()
                        } else if color[1] == "blue" {
                            r_g_b[2] = color[0].parse::<i32>().unwrap()
                        }
                    }

                    r_g_b
                })
                .collect()
        })
        .collect();

    game_data
}

fn part1(input: &Vec<Vec<Vec<i32>>>) {
    // Calculate solution
    // [red, green, blue]
    let bag = vec![12, 13, 14];
    let mut sum: i32 = 0;

    for (i, game) in input.iter().enumerate() {
        let mut game_impossible = false;
        for showing in game {
            if !((showing[0] <= bag[0]) && (showing[1] <= bag[1]) && (showing[2] <= bag[2])) {
                println!("Game {} is impossible. {:?}", i as i32 + 1, showing);
                game_impossible = true;
                break;
            }
        }
        if game_impossible != true {
            sum = sum + (i as i32 + 1)
        }
    }
    println!("Sum Part 1: {}", sum);
}

fn part2(input: &Vec<Vec<Vec<i32>>>) {
    let mut power: i32 = 0;

    for game in input {
        let mut min = vec![0, 0, 0];
        
        for showing in game {
            if showing[0] > min[0] {
                min[0] = showing[0];
            }
            if showing[1] > min[1] {
                min[1] = showing[1];
            }
            if showing[2] > min[2] {
                min[2] = showing[2];
            }
        }

        power = power + min.iter().product::<i32>();

    }
    println!("Power Part 2: {}", power);
}
