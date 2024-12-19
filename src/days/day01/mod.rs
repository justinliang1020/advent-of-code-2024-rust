use std::collections;
use std::iter::zip;

#[allow(dead_code)]
fn part1(input: &str) -> u32 {
    let mut a1: Vec<u32> = Vec::new();
    let mut a2: Vec<u32> = Vec::new();
    for line in input.lines() {
        let mut line_split = line.split_whitespace();
        a1.push(line_split.next().unwrap().parse().unwrap());
        a2.push(line_split.next().unwrap().parse().unwrap());
    }
    a1.sort();
    a2.sort();

    let mut res: u32 = 0;
    for iter in zip(a1, a2) {
        if iter.0 > iter.1 {
            res += iter.0 - iter.1;
        } else {
            res += iter.1 - iter.0;
        }
    }

    res
}

#[allow(dead_code)]
fn part2(input: &str) -> u32 {
    let mut map = collections::HashMap::new();

    for line in input.lines() {
        let second_num: u32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        map.entry(second_num).and_modify(|e| *e += 1).or_insert(1);
    }

    let res = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap()
        })
        .fold(0, |acc, n| acc + n * map.get(&n).unwrap_or(&0));

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const REAL_INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(REAL_INPUT), 1579939)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(REAL_INPUT), 20351745)
    }
}
