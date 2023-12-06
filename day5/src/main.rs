use std::{collections::HashMap, fs};

fn main() {
    let contents =
        fs::read_to_string("src/input2.txt").expect("Should have been able to read the file");

    let mut maps: HashMap<&str, Vec<Vec<i32>>> = HashMap::new();

    let blocks: Vec<Vec<Vec<&str>>> = contents
        .split("\n\n")
        .map(|b| {
            let mut lines: Vec<&str> = b.lines().collect();
            let numbers: Vec<Vec<i32>> = lines
                .split_off(1)
                .iter()
                .map(|n| n.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect()).collect();

            let bla:Vec<Vec<i32>> = Vec::new();

            return vec![lines, numbers];
        })
        .collect();

    // for block in blocks {
    //     if block.len() > 1 {
    //         maps.insert(block[0], block[1].map(||))
    //     }
    // }

    println!("{:?}", blocks);
}
