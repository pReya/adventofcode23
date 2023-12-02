use regex::Regex;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    // part1(&contents);
    part2(&contents);
}

fn part1(input: &str) {
    let regex = Regex::new(r"[^0-9]").unwrap();

    let sum: u32 = input
        .lines()
        .map(|l| {
            let digits: String = regex.replace_all(l, "").into_owned();
            let config_value = digits.chars().nth(0).unwrap().to_string()
                + digits.chars().last().unwrap().to_string().as_str();
            return config_value.parse::<u32>().unwrap();
        })
        .reduce(|acc, e| acc + e)
        .unwrap();

    println!("Part 1 Sum: {}", sum);
}

fn part2(input: &str) {
    let regex = Regex::new(r"[^0-9]").unwrap();
    let regex_one = Regex::new(r"\(one\)").unwrap();
    let regex_two = Regex::new(r"\(two\)").unwrap();
    let regex_three = Regex::new(r"\(three\)").unwrap();
    let regex_four = Regex::new(r"\(four\)").unwrap();
    let regex_five = Regex::new(r"\(five\)").unwrap();
    let regex_six = Regex::new(r"\(six\)").unwrap();
    let regex_seven = Regex::new(r"\(seven\)").unwrap();
    let regex_eight = Regex::new(r"\(eight\)").unwrap();
    let regex_nine = Regex::new(r"\(nine\)").unwrap();

    let words = r"one|two|three|four|five|six|seven|eight|nine";
    let regex_isolate_first = Regex::new(("(".to_owned() + words + ")").as_str()).unwrap();
    let regex_isolate_last = Regex::new(
        ("(".to_owned() + words.chars().rev().collect::<String>().as_str() + ")").as_str(),
    )
    .unwrap();

    let first_isolated = input
        .lines()
        .map(|l| {
            let isolate_first = regex_isolate_first.replace(l, r"($1)").to_string();
            let mut no_strings = regex_two.replace_all(&isolate_first, "2").into_owned();
            no_strings = regex_one.replace_all(&no_strings, "1").into_owned();
            no_strings = regex_three.replace_all(&no_strings, "3").into_owned();
            no_strings = regex_four.replace_all(&no_strings, "4").into_owned();
            no_strings = regex_five.replace_all(&no_strings, "5").into_owned();
            no_strings = regex_six.replace_all(&no_strings, "6").into_owned();
            no_strings = regex_seven.replace_all(&no_strings, "7").into_owned();
            no_strings = regex_nine.replace_all(&no_strings, "9").into_owned();
            no_strings = regex_eight.replace_all(&no_strings, "8").into_owned();
            no_strings
        })
        .map(|l| {
            let digits: String = regex.replace_all(&l, "").into_owned();
            let config_value = digits.chars().nth(0).unwrap().to_string();
            return config_value;
        })
        .collect::<Vec<String>>();

    let last_isolated = input
        .lines()
        .map(|l| {
            let mut isolate_first = regex_isolate_last
                .replace(l.chars().rev().collect::<String>().as_str(), r")$1(")
                .to_string();
            isolate_first = isolate_first.chars().rev().collect::<String>();
            let mut no_strings = regex_two.replace_all(&isolate_first, "2").into_owned();
            no_strings = regex_one.replace_all(&no_strings, "1").into_owned();
            no_strings = regex_three.replace_all(&no_strings, "3").into_owned();
            no_strings = regex_four.replace_all(&no_strings, "4").into_owned();
            no_strings = regex_five.replace_all(&no_strings, "5").into_owned();
            no_strings = regex_six.replace_all(&no_strings, "6").into_owned();
            no_strings = regex_seven.replace_all(&no_strings, "7").into_owned();
            no_strings = regex_nine.replace_all(&no_strings, "9").into_owned();
            no_strings = regex_eight.replace_all(&no_strings, "8").into_owned();
            no_strings
        })
        .map(|l| {
            let digits: String = regex.replace_all(&l, "").into_owned();
            let config_value = digits.chars().last().unwrap().to_string();
            return config_value;
        })
        .collect::<Vec<String>>();

    println!("{:?}", first_isolated);
    println!("{:?}", last_isolated);




    let mut sum : u32 = 0;
    for (i, el) in first_isolated.iter().enumerate() {
        sum = sum + (el.to_string() + last_isolated[i].as_str()).parse::<u32>().unwrap()  
    }

    println!("{}", sum)
}
