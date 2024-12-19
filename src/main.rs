use std::collections;
use std::fs;

// for part b of question 1

fn main() {
    let file_path = "src/test.txt";
    println!("In file {file_path}");

    let mut map = collections::HashMap::new();

    for line in fs::read_to_string(file_path).unwrap().lines() {
        let second_num: i32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        map.entry(second_num).and_modify(|e| *e += 1).or_insert(1);
    }

    let res = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .fold(0, |acc, n| acc + n * map.get(&n).unwrap_or(&0));

    println!("{res}")
}
