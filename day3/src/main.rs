use std::{fs, collections::HashSet};

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    //part1(&contents);
    part2(&contents);
}

fn part1(input: &str) {
    let mut symbol_map =
        vec![vec![false; input.lines().nth(0).unwrap().len()]; input.lines().count()];
    let mut number_list: Vec<NumberPosition> = vec![];

    let mut current_number: Option<NumberPosition> = None;
    for (i, line) in input.lines().enumerate() {
        let line: Vec<char> = line.chars().collect();
        if let Some(number_position) = current_number.as_ref() {
            number_list.push(number_position.clone());
            current_number = None;
        }
        for (j, cell) in line.iter().enumerate() {
            if *cell != '.' {
                if !cell.is_ascii_digit() {
                    // Control chars
                    symbol_map[i][j] = true;
                    if let Some(number_position) = current_number.as_ref() {
                        number_list.push(number_position.clone());
                        current_number = None;
                    }
                } else {
                    // Digits
                    if let Some(number_position) = current_number.as_mut() {
                        number_position.x_end = j;
                        number_position.value =
                            number_position.value.clone() + cell.to_string().as_str();
                    } else {
                        current_number = Some(NumberPosition {
                            x_start: j,
                            x_end: j,
                            y: i,
                            value: cell.to_string(),
                        });
                    }
                }
            } else {
                if let Some(number_position) = current_number.as_ref() {
                    number_list.push(number_position.clone());
                    current_number = None;
                }
            }
        }
    }

    //println!("{:?}", &number_list);

    let sum: u32 = number_list
        .iter()
        .filter(|number_position| {
            return number_position.check_around(&symbol_map);
        })
        .map(|number_position| {
            //println!("{:?}",number_position);
            number_position.value.parse::<u32>().unwrap()
        })
        .sum();

    println!("Part 1: {}", sum);
}

#[derive(Debug, Clone)]
struct NumberPosition {
    x_start: usize,
    x_end: usize,
    y: usize,
    value: String,
}

fn part2(input: &str) {
    let mut gear_coords: Vec<Vec<i32>> = Vec::new();

    let plan: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    //println!("{:?}", get_number_at_coord(3, 5, &plan));

    for (i, line) in input.lines().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            if cell == '*' {
                gear_coords.push(vec![i as i32, j as i32]);
            }
        }
    }
    let mut sum = 0;

    for gear_coord in gear_coords {
        let digits = find_digits_around_coord(gear_coord[0], gear_coord[1], &plan);
        //let mut numbers: Vec<i32> = Vec::new();
        let mut unique_numbers = HashSet::new();

        for digit in digits {
            let number = get_number_at_coord(digit[0], digit[1], &plan);
            unique_numbers.insert(number);
        }
        let numbers:Vec<i32> = unique_numbers.iter().map(|n| n[0].parse::<i32>().unwrap()).collect();
        if numbers.len() == 2 {
            sum = sum + numbers.iter().product::<i32>();
        }
        println!("Numbers {:?}", numbers);
    }
    println!("Sum {:?}", sum);

    //println!("{:?}", gear_coords);
}

fn find_digits_around_coord(x: i32, y: i32, grid: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let top_left_coords = vec![x - 1, y - 1];
    let top_coords = vec![x - 1, y];
    let top_right_coords = vec![x - 1, y + 1];
    let bottom_left_coords = vec![x + 1, y - 1];
    let bottom_coords = vec![x + 1, y];
    let bottom_right_coords = vec![x + 1, y + 1];
    let left_coords = vec![x, y - 1];
    let right_coords = vec![x, y + 1];

    let surrounding = vec![
        top_left_coords,
        top_coords,
        top_right_coords,
        bottom_left_coords,
        bottom_coords,
        bottom_right_coords,
        left_coords,
        right_coords,
    ];

    let digits: Vec<Vec<i32>> = surrounding
        .into_iter()
        .filter(|coord| {
            let found_char = grid[coord[0] as usize][coord[1] as usize];
            found_char.is_ascii_digit()
        })
        .collect();

    println!("Char {:?}", grid[x as usize][y as usize]);
    println!("Digits around {:?}", digits);

    digits
}

fn get_number_at_coord(x: i32, y: i32, grid: &Vec<Vec<char>>) -> Vec<String> {
    let line = &grid[x as usize];

    let c = line[y as usize];
    //println!("{:?}", c);
    if c.is_ascii_digit() {
        let mut start_idx = y;
        let mut end_idx = y;

        while line
            .get((start_idx - 1) as usize)
            .or(Some(&'x'))
            .unwrap()
            .is_ascii_digit()
        {
            start_idx = start_idx - 1;
        }

        while line
            .get((end_idx + 1) as usize)
            .or(Some(&'x'))
            .unwrap()
            .is_ascii_digit()
        {
            end_idx = end_idx + 1;
        }

        vec![
            String::from_iter(&line[(start_idx as usize)..((end_idx + 1) as usize)]),
            start_idx.to_string(),
            end_idx.to_string(),
        ]
    } else {
        vec!["0".to_string()]
    }
}

impl NumberPosition {
    fn check_around(&self, symbol_map: &Vec<Vec<bool>>) -> bool {
        for i in self.x_start..(self.x_end + 1) {
            if NumberPosition::check_pos(self.y, i, symbol_map) {
                return true;
            }
        }
        false
    }

    fn check_pos(x: usize, y: usize, symbol_map: &Vec<Vec<bool>>) -> bool {
        *symbol_map
            .get(if x > 0 { x - 1 } else { x })
            .or(Some(&vec![]))
            .unwrap()
            .get(y)
            .or(Some(&false))
            .unwrap()
            || *symbol_map
                .get(if x > 0 { x - 1 } else { x })
                .or(Some(&vec![]))
                .unwrap()
                .get(if y > 0 { y - 1 } else { y })
                .or(Some(&false))
                .unwrap()
            || *symbol_map
                .get(if x > 0 { x - 1 } else { x })
                .or(Some(&vec![]))
                .unwrap()
                .get(y + 1)
                .or(Some(&false))
                .unwrap()
            || *symbol_map
                .get(x + 1)
                .or(Some(&vec![]))
                .unwrap()
                .get(y)
                .or(Some(&false))
                .unwrap()
            || *symbol_map
                .get(x + 1)
                .or(Some(&vec![]))
                .unwrap()
                .get(if y > 0 { y - 1 } else { y })
                .or(Some(&false))
                .unwrap()
            || *symbol_map
                .get(x + 1)
                .or(Some(&vec![]))
                .unwrap()
                .get(y + 1)
                .or(Some(&false))
                .unwrap()
            || *symbol_map
                .get(x)
                .or(Some(&vec![]))
                .unwrap()
                .get(if y > 0 { y - 1 } else { y })
                .or(Some(&false))
                .unwrap()
            || *symbol_map
                .get(x)
                .or(Some(&vec![]))
                .unwrap()
                .get(y + 1)
                .or(Some(&false))
                .unwrap()
    }
}
