use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    //println!("{:?}", calculate_record_breaks(15,40));

    let lines: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            let numbers: Vec<&str> = line.split(":").skip(1).collect();
            numbers[0]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    // let mut product: Vec<i32> = Vec::new();
    // let mut transposed: Vec<Vec<i32>> = Vec::new();
    // for i in 0..lines[0].len() {
    //     let time = lines[0][i];
    //     let distance = lines[1][i];

    //     transposed.push(vec![time, distance]);

    //     let record_breaks = calculate_record_breaks(time, distance);
    //     //println!("Game: {} Time: {} Distance: {} Records: {}", i+1, time, distance, record_breaks);
    //     product.push(record_breaks);
    // }
    // println!("Part 1: {:?}", product.iter().product::<i32>());
    // println!("Transposed: {:?}", transposed);

    // let single_game: Vec<Vec<String>> = transposed
    //     .iter()
    //     .map(|game| game.iter().map(|g| g.to_string()))
    //     .collect();
    println!("Part 2: {:?}", calculate_record_breaks(56977793,499221010971440));


}

fn calculate_record_breaks(race_length: i64, record: i64) -> i64 {
    let mut record_breaks: i64 = 0;
    for time_pressed in 0..=race_length {
        let duration = time_pressed * (race_length - time_pressed);

        if duration > record {
            record_breaks = record_breaks + 1;
        }
    }
    record_breaks
}
