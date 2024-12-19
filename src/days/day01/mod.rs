use std::collections;

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
    fn test_part2() {
        assert_eq!(part2(REAL_INPUT), 20351745)
    }
}
